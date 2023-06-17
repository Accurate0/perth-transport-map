use super::handle_trip;
use crate::task_manager::TaskManager;
use anyhow::Context;
use flume::Sender;
use futures_util::future::join_all;
use perthtransport::{
    constants::{CACHE_KEY_PREFIX, PUBSUB_CHANNEL_OUT_PREFIX},
    query,
    types::{
        config::ApplicationConfig,
        message::{PubSubAction, PubSubMessage, WorkerMessage},
    },
};
use redis::{AsyncCommands, Msg};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{Instrument, Level};

pub async fn handle_message(
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
    worker_tx: Sender<WorkerMessage>,
    redis_multiplexed: Arc<RwLock<redis::aio::MultiplexedConnection>>,
    task_manager: Arc<TaskManager>,
    message: Msg,
    config: Arc<ApplicationConfig>,
) -> Result<(), anyhow::Error> {
    match message.get_payload::<String>() {
        Ok(s) => {
            let message = serde_json::from_str::<PubSubMessage>(&s);
            if let Ok(message) = message {
                match message.action {
                    PubSubAction::Hello => {
                        let socket_id = message.socket_id.clone();
                        task_manager
                            .create_websocket_session(socket_id.clone())
                            .await?;

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

                        let live_trip_ids: Vec<String> =
                            join_all(trains.iter().map(|timetable_id| {
                                query::get_live_trips_for(
                                    timetable_id,
                                    &config,
                                    http_client.clone(),
                                )
                            }))
                            .await
                            .iter()
                            .filter_map(|x| x.as_ref().ok())
                            .flat_map(|x| x.live_trips.clone())
                            .collect();

                        for trip_id in live_trip_ids {
                            let task_created = task_manager
                                .add_task_to_websocket_session(
                                    socket_id.clone(),
                                    trip_id.clone(),
                                    || {
                                        let http_client = http_client.clone();
                                        let span = tracing::span!(
                                            Level::INFO,
                                            "trip_task",
                                            trip_id = trip_id
                                        );
                                        let trip_id_cloned = trip_id.clone();
                                        let config = config.clone();
                                        let worker_tx = worker_tx.clone();

                                        tokio::spawn(async move {
                                            if let Err(e) = handle_trip(
                                                http_client,
                                                worker_tx,
                                                config,
                                                trip_id_cloned,
                                            )
                                            .instrument(span)
                                            .await
                                            {
                                                tracing::error!("task failed with {}", e)
                                            }
                                        })
                                    },
                                )
                                .await?;

                            if !task_created {
                                tracing::info!("getting value from cache as task already exists");
                                let mut redis_multiplexed = redis_multiplexed.write().await;
                                let cache_value = redis_multiplexed
                                    .get::<_, String>(format!("{}_{}", CACHE_KEY_PREFIX, trip_id))
                                    .await;

                                if let Ok(cache_value) = cache_value {
                                    redis_multiplexed
                                        .publish(
                                            format!(
                                                "{}_{}",
                                                PUBSUB_CHANNEL_OUT_PREFIX, message.socket_id
                                            ),
                                            cache_value,
                                        )
                                        .await?
                                }
                            }
                        }

                        Ok(())
                    }
                    PubSubAction::TripAdd => {
                        // TODO: logic duplicated above
                        let trip_id = message.trip_id.context("trip add must have trip id")?;

                        let task_created = task_manager
                            .add_task_to_websocket_session(
                                message.socket_id.clone(),
                                trip_id.clone(),
                                || {
                                    let http_client = http_client.clone();
                                    let span =
                                        tracing::span!(Level::INFO, "trip_task", trip_id = trip_id);
                                    let trip_id_cloned = trip_id.clone();
                                    let config = config.clone();
                                    let worker_tx = worker_tx.clone();

                                    tokio::spawn(async move {
                                        if let Err(e) = handle_trip(
                                            http_client,
                                            worker_tx,
                                            config,
                                            trip_id_cloned,
                                        )
                                        .instrument(span)
                                        .await
                                        {
                                            tracing::error!("task failed with {}", e)
                                        }
                                    })
                                },
                            )
                            .await?;

                        if !task_created {
                            tracing::info!("getting value from cache as task already exists");
                            let mut redis_multiplexed = redis_multiplexed.write().await;
                            let cache_value = redis_multiplexed
                                .get::<_, String>(format!("{}_{}", CACHE_KEY_PREFIX, trip_id))
                                .await;

                            if let Ok(cache_value) = cache_value {
                                redis_multiplexed
                                    .publish(
                                        format!(
                                            "{}_{}",
                                            PUBSUB_CHANNEL_OUT_PREFIX, message.socket_id
                                        ),
                                        cache_value,
                                    )
                                    .await?
                            }
                        }

                        Ok(())
                    }
                    PubSubAction::Bye => {
                        task_manager
                            .destroy_websocket_session(message.socket_id)
                            .await
                    }
                }?
            }
        }
        Err(e) => tracing::error!("error getting payload: {}", e),
    }

    Ok(())
}
