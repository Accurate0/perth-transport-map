use crate::types::AppState;
use axum::{extract::State, Json};
use perthtransport::types::health::{ServiceUnavailable, WorkerHealthStatus};

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<WorkerHealthStatus>, ServiceUnavailable> {
    let active_trains_healthy = !state.active_trains_handle.is_finished();
    let worker_output_healthy = !state.worker_out_handle.is_finished();
    let task_manager_healthy =
        state.task_manager.is_healthy().await && !state.task_manager_handle.is_finished();

    tracing::info!(
        "task_manager: {}, worker_output: {}, active_trains: {}",
        task_manager_healthy,
        worker_output_healthy,
        active_trains_healthy
    );

    Ok(Json(WorkerHealthStatus {
        worker_output_healthy,
        task_manager_healthy,
        active_trains_healthy,
    }))
}
