use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATripResponse {
    #[serde(rename = "GetTripInfosResult")]
    pub get_trip_infos_result: Vec<PTAGetTripInfosResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAGetTripInfosResult {
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    #[serde(rename = "Interruptions")]
    pub interruptions: Option<Vec<PTAInterruption>>,
    #[serde(rename = "Status")]
    // Live or Not Found
    pub status: String,
    #[serde(rename = "TripId")]
    pub trip_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAInterruption {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "InsertDate")]
    pub insert_date: String,
    #[serde(rename = "InterruptionId")]
    pub interruption_id: i64,
    #[serde(rename = "ModifyDate")]
    pub modify_date: String,
    #[serde(rename = "Resolved")]
    pub resolved: bool,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "URL")]
    pub url: String,
}
