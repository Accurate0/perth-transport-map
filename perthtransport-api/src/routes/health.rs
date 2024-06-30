use crate::types::AppState;
use axum::extract::State;
use http::StatusCode;
use perthtransport::types::health::{ServiceUnavailable, WorkerHealthStatus};
use std::time::Duration;

pub async fn health_check(State(state): State<AppState>) -> Result<StatusCode, ServiceUnavailable> {
    let health_endpoint = format!("{}/{}", state.config.worker_api_base, "status/health");
    let response = state
        .http_client
        .get(health_endpoint)
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .error_for_status()?
        .json::<WorkerHealthStatus>()
        .await?;

    tracing::info!("response: {:?}", response);

    if response.active_trains_healthy
        && response.task_manager_healthy
        && response.worker_output_healthy
    {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::SERVICE_UNAVAILABLE)
    }
}
