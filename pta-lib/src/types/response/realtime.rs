use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealTimeResponse {
    pub trip_id: String,
    pub current_position: GeoPosition,
    // TODO: datetime
    pub last_updated: String,
    pub start_time: String,

    pub transit_stops: Vec<TransitStop>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoPosition {
    pub latitude: String,
    pub longitude: String,
}

impl GeoPosition {
    pub fn try_from_str(value: &str) -> Result<Self, anyhow::Error> {
        let mut s = if value.contains(',') {
            value.split(", ")
        } else {
            // compatible types..
            #[allow(clippy::single_char_pattern)]
            value.split(" ")
        };

        Ok(Self {
            latitude: s
                .next()
                .context(format!("geoposition invalid: {}", value))?
                .to_owned(),
            longitude: s
                .next()
                .context(format!("geoposition invalid: {}", value))?
                .to_owned(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransitStop {
    pub position: GeoPosition,
    pub description: String,
    pub real_time_info: RealTimeInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RealTimeInfo {
    // TODO: this is an enum in reality
    pub trip_status: i64,
    // TODO: datetime
    pub estimated_arrival_time: Option<String>,
    pub estimated_departure_time: Option<String>,
}
