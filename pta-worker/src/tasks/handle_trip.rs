use crate::auth::generate_realtime_auth_header;
use chrono::{DateTime, Utc};
use http::header::AUTHORIZATION;
use pta::{
    constants::{PUBSUB_CHANNEL_WORKER_TASK_OUT, TRANSPERTH_REAL_TIME_API},
    types::{
        config::ApplicationConfig,
        message::PubSubWorkerOutMessage,
        response::realtime::RealTimeResponse,
        transperth::realtime::{PTARealTimeRequest, PTARealTimeResponse},
    },
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::{sync::RwLock, time::Instant};

pub async fn handle_trip(
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
    redis: Arc<RwLock<MultiplexedConnection>>,
    config: Arc<ApplicationConfig>,
    trip_id: String,
) -> Result<(), anyhow::Error> {
    tracing::info!("task started");

    loop {
        let trip_id = trip_id.clone();
        tracing::info!("fetching realtime data");
        let start = Instant::now();
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now_in_perth = now.with_timezone(&chrono_tz::Australia::Perth);
        let datetime = now_in_perth.format("%Y-%m-%d").to_string();

        let response = http_client
            .post(TRANSPERTH_REAL_TIME_API)
            .json(&PTARealTimeRequest {
                trip_uid: trip_id.clone(),
                trip_date: datetime,
                is_mapping_data_returned: true,
                is_real_time_checked: true,
                return_notes: true,
            })
            .header(
                AUTHORIZATION,
                generate_realtime_auth_header(&config.realtime_api_key).await?,
            )
            .send()
            .await?;

        tracing::info!(
            "realtime request completed with status: {} in {} ms",
            response.status(),
            start.elapsed().as_millis()
        );

        let pta_realtime = response.json::<PTARealTimeResponse>().await?;
        let pta_realtime_converted = RealTimeResponse::try_from(pta_realtime)?;
        {
            let mut redis = redis.write().await;
            redis
                .publish(
                    PUBSUB_CHANNEL_WORKER_TASK_OUT,
                    serde_json::to_string(&PubSubWorkerOutMessage {
                        response: pta_realtime_converted,
                        trip_id: trip_id.clone(),
                    })?,
                )
                .await?;
        }

        tracing::info!("task sleeping");
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
