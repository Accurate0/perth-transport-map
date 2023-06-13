use super::handle_trip;
use crate::task_manager::TaskManager;
use anyhow::Context;
use pta::types::{
    config::ApplicationConfig,
    message::{PubSubAction, PubSubMessage},
};
use redis::Msg;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{Instrument, Level};

pub async fn handle_message(
    http_client: Arc<reqwest_middleware::ClientWithMiddleware>,
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
                        task_manager
                            .create_websocket_session(message.socket_id)
                            .await
                    }
                    PubSubAction::TripAdd => {
                        let trip_id = message.trip_id.context("trip add must have trip id")?;

                        task_manager
                            .add_task_to_websocket_session(
                                message.socket_id,
                                trip_id.clone(),
                                || {
                                    let http_client = http_client.clone();
                                    let span =
                                        tracing::span!(Level::INFO, "trip_task", trip_id = trip_id);
                                    let trip_id_cloned = trip_id.clone();
                                    let redis_multiplexed = redis_multiplexed.clone();
                                    let config = config.clone();

                                    tokio::spawn(async move {
                                        if let Err(e) = handle_trip(
                                            http_client,
                                            redis_multiplexed,
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
                            .await
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
