use perthtransport::{queue::MessageBus, types::config::ApplicationConfig};
use reqwest_middleware::ClientWithMiddleware;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub message_bus: MessageBus,
    pub http_client: Arc<ClientWithMiddleware>,
    pub config: ApplicationConfig,
}
