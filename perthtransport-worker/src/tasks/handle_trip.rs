use crate::auth::generate_realtime_auth_header;
use chrono::{DateTime, Utc};
use flume::Sender;
use http::header::AUTHORIZATION;
use perthtransport::{
    constants::TRANSPERTH_REAL_TIME_API,
    types::{
        config::ApplicationConfig,
        message::WorkerMessage,
        response::realtime::{RealTimeResponse, TransitStopStatus},
        transperth::realtime::{PTARealTimeRequest, PTARealTimeResponse},
    },
};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::time::Instant;

pub async fn handle_trip(
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
    worker_tx: Sender<WorkerMessage>,
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
        if let Some(first_stop) = pta_realtime_converted.transit_stops.first() {
            if first_stop.real_time_info.trip_status == TransitStopStatus::Scheduled {
                tracing::warn!("this trip is scheduled for first station, no point tracking");
                break Ok(());
            }
        }

        worker_tx
            .send_async(WorkerMessage {
                response: pta_realtime_converted,
                trip_id: trip_id.clone(),
            })
            .await?;

        tracing::info!("task sleeping");
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
