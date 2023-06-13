use crate::task_manager::TaskManager;
use futures_util::StreamExt;
use perthtransport::{
    constants::{PUBSUB_CHANNEL_OUT_PREFIX, PUBSUB_CHANNEL_WORKER_TASK_OUT},
    types::message::PubSubWorkerOutMessage,
};
use redis::AsyncCommands;
use std::sync::Arc;

pub async fn handle_worker_out(
    mut redis: redis::aio::MultiplexedConnection,
    pubsub_connection: redis::aio::Connection,
    task_manager: Arc<TaskManager>,
) -> Result<(), anyhow::Error> {
    let mut pubsub = pubsub_connection.into_pubsub();
    pubsub.subscribe(PUBSUB_CHANNEL_WORKER_TASK_OUT).await?;

    loop {
        let msg = pubsub.on_message().next().await;
        if let Some(msg) = msg {
            match msg.get_payload::<String>() {
                Ok(s) => {
                    let message = serde_json::from_str::<PubSubWorkerOutMessage>(&s)?;
                    let trip_id = message.trip_id.clone();

                    let all_websockets = task_manager
                        .get_all_task_subscribers(message.trip_id.clone())
                        .await;

                    match all_websockets {
                        Some(websockets) => {
                            if websockets.is_empty() {
                                tracing::info!("[{}] no associated websocket sessions", trip_id);
                                task_manager.abort_task(&trip_id).await
                            } else {
                                for socket_id in websockets {
                                    tracing::info!("[{}] sending to {}", trip_id, socket_id);
                                    redis
                                        .publish(
                                            format!("{}_{}", PUBSUB_CHANNEL_OUT_PREFIX, socket_id),
                                            serde_json::to_string(&message.response)?,
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
                Err(e) => tracing::error!("error opening message payload: {}", e),
            }
        }
    }
}
