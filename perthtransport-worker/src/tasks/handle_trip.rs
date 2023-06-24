use crate::auth::generate_realtime_auth_header;
use anyhow::Context;
use chrono::{DateTime, Days, Timelike, Utc};
use flume::Sender;
use http::header::AUTHORIZATION;
use perthtransport::{
    constants::{TRANSPERTH_EARLY_HOURS, TRANSPERTH_REAL_TIME_API},
    types::{
        config::ApplicationConfig,
        message::{MessageContents, WorkerMessage},
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

        let pta_realtime = response.json::<PTARealTimeResponse>().await;

        // something at happens at around midnight only
        // TODO: refactor
        let pta_realtime = if pta_realtime
            .as_ref()
            .is_err_and(|_| TRANSPERTH_EARLY_HOURS.contains(&now_in_perth.hour()))
        {
            let now_in_perth = now.with_timezone(&chrono_tz::Australia::Perth);
            let datetime = now_in_perth
                .checked_sub_days(Days::new(1))
                .context("unable to sub 1 day")?
                .format("%Y-%m-%d")
                .to_string();

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

            response.json::<PTARealTimeResponse>().await
        } else {
            pta_realtime
        }?;

        let pta_realtime_converted = RealTimeResponse::try_from(pta_realtime)?;
        if let Some(first_stop) = pta_realtime_converted.transit_stops.first() {
            if first_stop
                .real_time_info
                .as_ref()
                .is_some_and(|x| x.trip_status == TransitStopStatus::Scheduled)
            {
                worker_tx
                    .send_async(WorkerMessage::DoNotTrack(trip_id))
                    .await?;

                tracing::warn!("this trip is scheduled for first station, no point tracking");
                break Ok(());
            }
        }

        worker_tx
            .send_async(WorkerMessage::HasMessage(MessageContents {
                response: pta_realtime_converted,
                trip_id: trip_id.clone(),
            }))
            .await?;

        let sleep_duration = 30;
        tracing::info!("task sleeping for {}", sleep_duration);
        tokio::time::sleep(Duration::from_secs(sleep_duration)).await;
    }
}
