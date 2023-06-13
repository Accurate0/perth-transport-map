use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LiveTripResponse {
    pub live_trips: Vec<String>,
}

#[Object]
impl LiveTripResponse {
    pub async fn live_trips(&self) -> &Vec<String> {
        &self.live_trips
    }
}
