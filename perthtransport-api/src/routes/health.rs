use crate::types::AppState;
use axum::extract::State;
use http::StatusCode;
use perthtransport::types::health::{ServiceUnavailable, WorkerHealthStatus};

pub async fn health_check(State(state): State<AppState>) -> Result<StatusCode, ServiceUnavailable> {
    let health_endpoint = format!("{}/{}", state.config.worker_api_base, "status/health");
    let response = state
        .http_client
        .get(health_endpoint)
        .send()
        .await?
        .json::<WorkerHealthStatus>()
        .await?;

    if response.active_trains_healthy
        && response.task_manager_healthy
        && response.worker_output_healthy
    {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::SERVICE_UNAVAILABLE)
    }
}
