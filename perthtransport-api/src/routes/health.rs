use crate::types::AppState;
use axum::extract::State;
use http::StatusCode;
use perthtransport::types::health::ServiceUnavailable;

pub async fn health_check(
    State(_state): State<AppState>,
) -> Result<StatusCode, ServiceUnavailable> {
    Ok(StatusCode::NO_CONTENT)
}
