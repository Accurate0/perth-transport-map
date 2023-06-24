use crate::types::{AppState, ServiceUnavailable};
use anyhow::Context;
use axum::extract::State;
use futures_util::StreamExt;
use http::StatusCode;
use perthtransport::{
    constants::{PUBSUB_CHANNEL_WORKER_HEALTH_IN, PUBSUB_CHANNEL_WORKER_HEALTH_OUT},
    types::health::WorkerHealthStatus,
};
use redis::AsyncCommands;
use std::time::Duration;

pub async fn health_check(State(state): State<AppState>) -> Result<StatusCode, ServiceUnavailable> {
    let mut pubsub = state.redis.get_async_connection().await?.into_pubsub();
    let mut connection = state.redis.get_async_connection().await?;

    pubsub.subscribe(PUBSUB_CHANNEL_WORKER_HEALTH_OUT).await?;
    connection
        .publish(PUBSUB_CHANNEL_WORKER_HEALTH_IN, "hey")
        .await?;

    let worker_health = tokio::time::timeout(Duration::from_secs(5), pubsub.on_message().next())
        .await?
        .context("must contain message")?;

    let worker_health =
        serde_json::from_str::<WorkerHealthStatus>(&worker_health.get_payload::<String>()?)?;

    if worker_health.task_manager_healthy
        && worker_health.worker_output_healthy
        && worker_health.active_trains_healthy
    {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::error!(
            "task_manager: {}, worker_output: {}, active_trains: {}",
            worker_health.task_manager_healthy,
            worker_health.worker_output_healthy,
            worker_health.active_trains_healthy
        );

        Ok(StatusCode::SERVICE_UNAVAILABLE)
    }
}
