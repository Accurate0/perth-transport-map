use crate::task_manager::TaskManager;
use perthtransport::{
    constants::PUBSUB_CHANNEL_WORKER_HEALTH_OUT, types::health::WorkerHealthStatus,
};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};

pub async fn handle_healthcheck(
    worker_out_handle: Arc<JoinHandle<()>>,
    redis_multiplexed: Arc<RwLock<redis::aio::MultiplexedConnection>>,
    task_manager: Arc<TaskManager>,
) -> Result<(), anyhow::Error> {
    tracing::info!("health check received");
    let worker_output_healthy = !worker_out_handle.is_finished();
    let task_manager_healthy = task_manager.is_healthy().await;
    let mut redis_connection = redis_multiplexed.write().await;

    tracing::info!(
        "task_manager: {}, worker_output: {}",
        task_manager_healthy,
        worker_output_healthy
    );

    redis_connection
        .publish(
            PUBSUB_CHANNEL_WORKER_HEALTH_OUT,
            serde_json::to_string(&WorkerHealthStatus {
                worker_output_healthy,
                task_manager_healthy,
            })?,
        )
        .await?;
    tracing::info!("health check response complete");
    Ok::<(), anyhow::Error>(())
}
