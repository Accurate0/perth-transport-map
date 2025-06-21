use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHealthStatus {
    pub worker_output_healthy: bool,
    pub task_manager_healthy: bool,
    pub active_trains_healthy: bool,
}

#[allow(unused)]
pub struct ServiceUnavailable(anyhow::Error);
impl IntoResponse for ServiceUnavailable {
    fn into_response(self) -> Response {
        StatusCode::SERVICE_UNAVAILABLE.into_response()
    }
}

impl<E> From<E> for ServiceUnavailable
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
