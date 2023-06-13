use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATimetableResponse {
    #[serde(rename = "Request")]
    pub request: PTARequest,
    #[serde(rename = "Status")]
    pub status: PTAStatus,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "TimetableTrips")]
    pub timetable_trips: Vec<PTATimetableTrip>,
    #[serde(rename = "StopPatterns")]
    pub stop_patterns: Vec<PTAStopPattern>,
    #[serde(rename = "Routes")]
    pub routes: Vec<PTARoute>,
    #[serde(rename = "TransitStops")]
    pub transit_stops: Vec<PTATransitStop>,
    #[serde(rename = "RunningDatePatterns")]
    pub running_date_patterns: Vec<PTARunningDatePattern>,
    #[serde(rename = "Notes")]
    pub notes: Vec<PTANote>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARequest {
    #[serde(rename = "ApiKey")]
    pub api_key: String,
    #[serde(rename = "Route")]
    pub route: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "DataSet")]
    pub data_set: String,
    #[serde(rename = "ReturnNotes")]
    pub return_notes: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAStatus {
    #[serde(rename = "Severity")]
    pub severity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATimetableTrip {
    #[serde(rename = "TripUid")]
    pub trip_uid: String,
    #[serde(rename = "Headsign")]
    pub headsign: String,
    #[serde(rename = "RouteUid")]
    pub route_uid: String,
    #[serde(rename = "RunningDatePatternId")]
    pub running_date_pattern_id: i64,
    #[serde(rename = "StopPatternId")]
    pub stop_pattern_id: i64,
    #[serde(rename = "TripStopTimings")]
    pub trip_stop_timings: Vec<TripStopTiming>,
    #[serde(rename = "Direction")]
    pub direction: String,
    #[serde(rename = "TripSourceId")]
    pub trip_source_id: String,
    #[serde(rename = "NoteIds")]
    pub note_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TripStopTiming {
    #[serde(rename = "ArrivalTime")]
    pub arrival_time: String,
    #[serde(rename = "DepartTime")]
    pub depart_time: String,
    #[serde(rename = "CanBoard")]
    pub can_board: bool,
    #[serde(rename = "CanAlight")]
    pub can_alight: bool,
    #[serde(rename = "IsTimingPoint")]
    pub is_timing_point: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAStopPattern {
    #[serde(rename = "StopPatternId")]
    pub stop_pattern_id: i64,
    #[serde(rename = "TransitStopUids")]
    pub transit_stop_uids: Vec<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATransitStop {
    #[serde(rename = "DataSet")]
    pub data_set: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "StopUid")]
    pub stop_uid: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "Zone")]
    pub zone: String,
    #[serde(rename = "SupportedModes")]
    pub supported_modes: String,
    #[serde(rename = "SupportedModeUids")]
    pub supported_mode_uids: String,
    #[serde(rename = "Routes")]
    pub routes: String,
    #[serde(rename = "ParentUid")]
    pub parent_uid: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARunningDatePattern {
    #[serde(rename = "RunningDatePatternId")]
    pub running_date_pattern_id: i64,
    #[serde(rename = "Pattern")]
    pub pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTANote {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Code")]
    pub code: String,
}
