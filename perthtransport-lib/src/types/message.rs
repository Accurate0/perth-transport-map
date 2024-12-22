use serde::Deserialize;
use serde::Serialize;

use super::response::realtime::RealTimeResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketMessage {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PubSubMessage {
    pub action: PubSubAction,
    pub socket_id: String,
    pub trip_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PubSubAction {
    Hello,
    TripAdd,
    Bye,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContents {
    pub response: RealTimeResponse,
    pub trip_id: String,
    pub publish: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerMessage {
    HasMessage(Box<MessageContents>),
    DoNotTrack(String),
}
