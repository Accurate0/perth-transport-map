use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(Clone)]
pub struct AppState {
    pub redis: redis::Client,
}

// error type that converts any error into service unavailble
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
