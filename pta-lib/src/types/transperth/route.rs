use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARouteResponse {
    #[serde(rename = "Request")]
    pub request: PTARequest,
    #[serde(rename = "Status")]
    pub status: PTAStatus,
    #[serde(rename = "Routes")]
    pub routes: Vec<PTARoute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARequest {
    #[serde(rename = "ApiKey")]
    pub api_key: String,
    #[serde(rename = "DataSet")]
    pub data_set: String,
    #[serde(rename = "SearchTerm")]
    pub search_term: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAStatus {
    #[serde(rename = "Severity")]
    pub severity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARoute {
    #[serde(rename = "RouteUid")]
    pub route_uid: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServiceProviderUid")]
    pub service_provider_uid: String,
    #[serde(rename = "TransportMode")]
    pub transport_mode: String,
    #[serde(rename = "TransportModeUid")]
    pub transport_mode_uid: String,
    #[serde(rename = "RouteTimetableGroupId")]
    pub route_timetable_group_id: String,
    #[serde(rename = "RouteSourceId")]
    pub route_source_id: String,
}
