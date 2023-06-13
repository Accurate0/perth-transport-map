use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARealTimeRequest {
    #[serde(rename = "TripUid")]
    pub trip_uid: String,
    #[serde(rename = "TripDate")]
    pub trip_date: String,
    #[serde(rename = "IsMappingDataReturned")]
    pub is_mapping_data_returned: bool,
    #[serde(rename = "IsRealTimeChecked")]
    pub is_real_time_checked: bool,
    #[serde(rename = "ReturnNotes")]
    pub return_notes: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARealTimeResponse {
    #[serde(rename = "Request")]
    pub request: PTARequest,
    #[serde(rename = "Status")]
    pub status: PTAStatus,
    #[serde(rename = "Summary")]
    pub summary: PTASummary,
    #[serde(rename = "TripStops")]
    pub trip_stops: Vec<PTATripStop>,
    #[serde(rename = "ParentTransitStops")]
    pub parent_transit_stops: Vec<PTAParentTransitStop>,
    #[serde(rename = "Notes")]
    pub notes: Vec<PTANote>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARequest {
    #[serde(rename = "ApiKey")]
    pub api_key: String,
    #[serde(rename = "TripUid")]
    pub trip_uid: String,
    #[serde(rename = "TripDate")]
    pub trip_date: String,
    #[serde(rename = "DataSet")]
    pub data_set: serde_json::Value,
    #[serde(rename = "IsRealTimeChecked")]
    pub is_real_time_checked: bool,
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
pub struct PTASummary {
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "TripUid")]
    pub trip_uid: String,
    #[serde(rename = "TripCode")]
    pub trip_code: String,
    #[serde(rename = "TripSourceId")]
    pub trip_source_id: String,
    #[serde(rename = "RouteName")]
    pub route_name: String,
    #[serde(rename = "RouteCode")]
    pub route_code: String,
    #[serde(rename = "RouteUid")]
    pub route_uid: String,
    #[serde(rename = "ServiceProvider")]
    pub service_provider: ServiceProvider,
    #[serde(rename = "Headsign")]
    pub headsign: String,
    #[serde(rename = "Polyline")]
    pub polyline: String,
    #[serde(rename = "RealTimeInfo")]
    pub real_time_info: PTARealTimeInfo,
    #[serde(rename = "TripStartTime")]
    pub trip_start_time: String,
    #[serde(rename = "Direction")]
    pub direction: String,
    #[serde(rename = "RouteSourceId")]
    pub route_source_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProvider {
    #[serde(rename = "ServiceProviderUid")]
    pub service_provider_uid: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "TimeZone")]
    pub time_zone: String,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARealTimeInfo {
    #[serde(rename = "CurrentPosition")]
    pub current_position: String,
    #[serde(rename = "LastUpdated")]
    pub last_updated: String,
    #[serde(rename = "CurrentBearing")]
    pub current_bearing: i64,
    #[serde(rename = "VehicleId")]
    pub vehicle_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATripStop {
    #[serde(rename = "TransitStop")]
    pub transit_stop: PTATransitStop,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
    #[serde(rename = "DepartureTime")]
    pub departure_time: String,
    #[serde(rename = "CanBoard")]
    pub can_board: bool,
    #[serde(rename = "CanAlight")]
    pub can_alight: bool,
    #[serde(rename = "RealTimeInfo")]
    pub real_time_info: PTARealTimeInfo2,
    #[serde(rename = "IsTimingPoint")]
    pub is_timing_point: Option<bool>,
    #[serde(rename = "ArrivalTime")]
    pub arrival_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTATransitStop {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "StopUid")]
    pub stop_uid: String,
    #[serde(rename = "DataSet")]
    pub data_set: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "Zone")]
    pub zone: String,
    #[serde(rename = "SupportedModes")]
    pub supported_modes: String,
    #[serde(rename = "Routes")]
    pub routes: String,
    #[serde(rename = "ParentUid")]
    pub parent_uid: Option<String>,
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTARealTimeInfo2 {
    #[serde(rename = "RealTimeTripStatus")]
    pub real_time_trip_status: i64,
    #[serde(rename = "EstimatedDepartureTime")]
    pub estimated_departure_time: Option<String>,
    #[serde(rename = "EstimatedArrivalTime")]
    pub estimated_arrival_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PTAParentTransitStop {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "StopUid")]
    pub stop_uid: String,
    #[serde(rename = "DataSet")]
    pub data_set: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "SupportedModes")]
    pub supported_modes: String,
    #[serde(rename = "Routes")]
    pub routes: String,
    #[serde(rename = "IsParent")]
    pub is_parent: bool,
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
