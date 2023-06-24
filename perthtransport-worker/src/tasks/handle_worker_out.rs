use crate::task_manager::TaskManager;
use flume::Receiver;
use perthtransport::{
    constants::{CACHE_KEY_PREFIX, DO_NOT_TRACK_KEY_PREFIX, PUBSUB_CHANNEL_OUT_PREFIX},
    types::message::WorkerMessage,
};
use redis::AsyncCommands;
use std::sync::Arc;

pub async fn handle_worker_out(
    mut redis: redis::aio::MultiplexedConnection,
    worker_rx: Receiver<WorkerMessage>,
    task_manager: Arc<TaskManager>,
) -> Result<(), anyhow::Error> {
    loop {
        let message = worker_rx.recv_async().await?;
        match message {
            WorkerMessage::HasMessage(message) => {
                let trip_id = message.trip_id.clone();

                let all_websockets = task_manager
                    .get_all_task_subscribers(message.trip_id.clone())
                    .await;

                let json_string = serde_json::to_string(&message.response)?;

                redis
                    .set_ex(
                        format!("{}_{}", CACHE_KEY_PREFIX, trip_id),
                        &json_string,
                        29,
                    )
                    .await?;

                match all_websockets {
                    Some(websockets) => {
                        if websockets.is_empty() {
                            tracing::info!("[{}] no associated websocket sessions", trip_id);
                            task_manager.abort_task(&trip_id).await
                        } else if message.publish {
                            for socket_id in websockets {
                                tracing::info!("[{}] sending to {}", trip_id, socket_id);
                                redis
                                    .publish(
                                        format!("{}_{}", PUBSUB_CHANNEL_OUT_PREFIX, socket_id),
                                        json_string.clone(),
                                    )
                                    .await?
                            }
                        }
                    }
                    // this one doesn't actually happen at the moment i believe
                    None => {
                        tracing::info!("[{}] no associated websocket sessions", trip_id);
                        task_manager.abort_task(&trip_id).await;
                    }
                }
            }
            WorkerMessage::DoNotTrack(trip_id) => {
                let expiry = 100;
                tracing::info!("setting {} to Do Not Track for {} seconds", trip_id, expiry);
                redis
                    .set_ex(
                        format!("{}_{}", DO_NOT_TRACK_KEY_PREFIX, trip_id),
                        "",
                        expiry,
                    )
                    .await?;
            }
        }
    }
}
