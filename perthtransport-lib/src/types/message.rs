use serde::Deserialize;
use serde::Serialize;

use super::response::realtime::RealTimeResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessage {
    pub trip_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubSubMessage {
    pub action: PubSubAction,
    pub socket_id: String,
    pub trip_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum PubSubAction {
    Hello,
    TripAdd,
    Bye,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMessage {
    pub response: RealTimeResponse,
    pub trip_id: String,
}
