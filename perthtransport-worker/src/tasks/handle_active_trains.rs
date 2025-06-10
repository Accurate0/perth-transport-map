use crate::{task_manager::TaskManager, tasks::handle_trip};
use flume::Sender;
use futures_util::future::join_all;
use perthtransport::{
    constants::{ACTIVE_TRAINS_KEY, ACTIVE_TRAIN_THREAD_SLEEP},
    query,
    types::{config::ApplicationConfig, message::WorkerMessage},
};
use redis::AsyncCommands;
use std::{sync::Arc, time::Duration};
use tracing::{Instrument, Level};

pub async fn handle_active_trains(
    task_manager: Arc<TaskManager>,
    config: Arc<ApplicationConfig>,
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
    mut redis_multiplexed: redis::aio::MultiplexedConnection,
    worker_tx: Sender<WorkerMessage>,
) -> Result<(), anyhow::Error> {
    loop {
        tracing::info!("caching currently active trains");
        let trains = [
            // Ellenbrook Line
            "PerthRestricted:RTG_16",
            // Airport Line
            "PerthRestricted:RTG_15",
            // Thornlie-Cockburn Line
            "PerthRestricted:RTG_12",
            // Midland Line
            "PerthRestricted:RTG_14",
            // Mandurah Line
            "PerthRestricted:RTG_13",
            // Armadale Line
            "PerthRestricted:RTG_11",
            // Fremantle Line
            "PerthRestricted:RTG_10",
            // Yanchep Line
            "PerthRestricted:RTG_9",
        ];

        let live_trip_ids: Vec<String> = join_all(trains.into_iter().map(|timetable_id| {
            query::get_live_trips_for(timetable_id, &config, http_client.clone())
        }))
        .await
        .iter()
        .inspect(|&x| {
            if x.is_err() {
                tracing::error!("error in fetching active train: {x:?}")
            }
        })
        .filter_map(|x| x.as_ref().ok())
        .flat_map(|x| x.live_trips.clone())
        .collect();

        tracing::info!("there are {} currently active trains", live_trip_ids.len());
        // set in cache
        redis_multiplexed
            .set(ACTIVE_TRAINS_KEY, serde_json::to_string(&live_trip_ids)?)
            .await?;

        let active_sockets = task_manager.get_all_socket_ids().await;
        for socket_id in active_sockets {
            for task_id in &live_trip_ids {
                task_manager
                    .add_task_to_websocket_session(socket_id.clone(), task_id.clone(), || {
                        let http_client = http_client.clone();
                        let span = tracing::span!(Level::INFO, "trip_task", trip_id = task_id);
                        let task_id_cloned = task_id.clone();
                        let config = config.clone();
                        let worker_tx = worker_tx.clone();

                        tokio::spawn(async move {
                            if let Err(e) =
                                handle_trip(http_client, worker_tx, config, task_id_cloned)
                                    .instrument(span)
                                    .await
                            {
                                tracing::error!("task failed with {}", e)
                            }
                        })
                    })
                    .await?;
            }
        }
        tracing::info!("sleeping for {} seconds", ACTIVE_TRAIN_THREAD_SLEEP);
        tokio::time::sleep(Duration::from_secs(ACTIVE_TRAIN_THREAD_SLEEP)).await;
    }
}
