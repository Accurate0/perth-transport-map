use std::time::Duration;

use crate::types::{AppState, ServiceUnavailable};
use axum::extract::State;
use futures_util::StreamExt;
use http::StatusCode;
use pta::{
    constants::{PUBSUB_CHANNEL_WORKER_HEALTH_IN, PUBSUB_CHANNEL_WORKER_HEALTH_OUT},
    types::health::WorkerHealthStatus,
};
use redis::AsyncCommands;

pub async fn health_check(State(state): State<AppState>) -> Result<StatusCode, ServiceUnavailable> {
    let mut pubsub = state.redis.get_async_connection().await?.into_pubsub();
    let mut connection = state.redis.get_async_connection().await?;

    pubsub.subscribe(PUBSUB_CHANNEL_WORKER_HEALTH_OUT).await?;
    connection
        .publish(PUBSUB_CHANNEL_WORKER_HEALTH_IN, "hey")
        .await?;

    let worker_health =
        tokio::time::timeout(Duration::from_secs(5), pubsub.on_message().next()).await;

    match worker_health {
        Ok(r) => match r {
            Some(m) => {
                let worker_health =
                    serde_json::from_str::<WorkerHealthStatus>(&m.get_payload::<String>()?)?;

                if worker_health.task_manager_healthy && worker_health.worker_output_healthy {
                    Ok(StatusCode::NO_CONTENT)
                } else {
                    tracing::error!(
                        "task_manager: {}, worker_output: {}",
                        worker_health.task_manager_healthy,
                        worker_health.worker_output_healthy
                    );

                    Ok(StatusCode::SERVICE_UNAVAILABLE)
                }
            }
            None => {
                tracing::error!("received message but it's empty");
                Ok(StatusCode::SERVICE_UNAVAILABLE)
            }
        },
        Err(_) => {
            tracing::error!("time out in worker health check");
            Ok(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
