use crate::task_manager::TaskManager;
use futures_util::future::join_all;
use perthtransport::{constants::ACTIVE_TRAINS_KEY, query, types::config::ApplicationConfig};
use redis::AsyncCommands;
use std::{sync::Arc, time::Duration};

pub async fn handle_active_trains(
    _task_manager: Arc<TaskManager>,
    config: Arc<ApplicationConfig>,
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
    mut redis_multiplexed: redis::aio::MultiplexedConnection,
) -> Result<(), anyhow::Error> {
    loop {
        tracing::info!("caching currently active trains");
        let trains = vec![
            // Airport Line
            "PerthRestricted:RTG_16",
            // Midland Line
            "PerthRestricted:RTG_15",
            // Armadale Line
            "PerthRestricted:RTG_12",
            // Mandurah Line
            "PerthRestricted:RTG_14",
            // Thornlie Line
            "PerthRestricted:RTG_13",
            // Fremantle Line
            "PerthRestricted:RTG_11",
            // Joondalup Line
            "PerthRestricted:RTG_10",
        ];

        let live_trip_ids: Vec<String> = join_all(trains.iter().map(|timetable_id| {
            query::get_live_trips_for(timetable_id, &config, http_client.clone())
        }))
        .await
        .iter()
        .filter_map(|x| x.as_ref().ok())
        .flat_map(|x| x.live_trips.clone())
        .collect();

        // set in cache
        redis_multiplexed
            .set(ACTIVE_TRAINS_KEY, serde_json::to_string(&live_trip_ids)?)
            .await?;

        // TODO: update active websocket sessions too

        let sleep_duration = 600;
        tracing::info!("sleeping for {} seconds", sleep_duration);
        tokio::time::sleep(Duration::from_secs(sleep_duration)).await;
    }
}
