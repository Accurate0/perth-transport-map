use super::handle_trip;
use crate::task_manager::TaskManager;
use flume::Sender;
use perthtransport::{
    constants::{
        ACTIVE_TRAINS_KEY, CACHE_KEY_PREFIX, DO_NOT_TRACK_KEY_PREFIX, PUBSUB_CHANNEL_OUT_PREFIX,
    },
    types::{
        config::ApplicationConfig,
        message::{PubSubAction, PubSubMessage, WorkerMessage},
    },
};
use redis::{AsyncCommands, Msg};
use std::{sync::Arc, time::Duration};
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
                        // TODO: use a background thread that automatically changes current trains for all websockets every x mins
                        let socket_id = message.socket_id.clone();
                        task_manager
                            .create_websocket_session(socket_id.clone())
                            .await?;

                        // TODO: fix this, long-ish lock hold
                        let mut redis_multiplexed_lock = redis_multiplexed.write().await;
                        let live_trip_ids = serde_json::from_str::<Vec<String>>(
                            &redis_multiplexed_lock
                                .get::<_, String>(ACTIVE_TRAINS_KEY)
                                .await?,
                        )?;

                        // TODO: why do i need a delay, seems like firefox only... very cool
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        for trip_id in live_trip_ids {
                            if redis_multiplexed_lock
                                .exists(format!("{}_{}", DO_NOT_TRACK_KEY_PREFIX, trip_id))
                                .await?
                            {
                                tracing::warn!("[{}] exists in cache as 'Do Not Track'", trip_id);
                                continue;
                            }

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
                                let cache_value = redis_multiplexed_lock
                                    .get::<_, String>(format!("{}_{}", CACHE_KEY_PREFIX, trip_id))
                                    .await;

                                if let Ok(cache_value) = cache_value {
                                    redis_multiplexed_lock
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
                        tracing::info!("ignoring event: {:?}", message);

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
