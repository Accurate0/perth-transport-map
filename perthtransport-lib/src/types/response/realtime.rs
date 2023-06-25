use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealTimeResponse {
    pub trip_id: String,
    pub route_name: String,
    pub current_position: GeoPosition,
    // TODO: datetime
    pub last_updated: String,
    pub start_time: String,
    pub next_stop: Option<TransitStop>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransitStop {
    pub position: GeoPosition,
    pub description: String,
    pub real_time_info: Option<RealTimeInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealTimeInfo {
    pub trip_status: TransitStopStatus,
    // TODO: datetime
    pub estimated_arrival_time: Option<String>,
    pub estimated_departure_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TransitStopStatus {
    Scheduled = 1,
    AtStation = 2,
    Completed = 3,
}
