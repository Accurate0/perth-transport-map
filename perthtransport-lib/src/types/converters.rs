use super::{
    response::{
        realtime::{GeoPosition, RealTimeInfo, RealTimeResponse, TransitStop, TransitStopStatus},
        route::{Route, RouteResponse, RouteType},
        trip::LiveTripResponse,
    },
    transperth::{realtime::PTARealTimeResponse, route::PTARouteResponse, trip::PTATripResponse},
};
use anyhow::Context;
use core::panic;

impl TryFrom<PTARealTimeResponse> for RealTimeResponse {
    type Error = anyhow::Error;

    fn try_from(value: PTARealTimeResponse) -> Result<Self, Self::Error> {
        let next_stop = value
            .trip_stops
            .iter()
            .find(|x| {
                x.real_time_info
                    .as_ref()
                    .map(|rti| rti.real_time_trip_status)
                    .is_some_and(|status| status == TransitStopStatus::Scheduled as i64)
            })
            .and_then(|t| -> Option<TransitStop> {
                Some(TransitStop {
                    position: GeoPosition::try_from_str(&t.transit_stop.position).ok()?,
                    description: t.transit_stop.description.clone(),
                    real_time_info: t.real_time_info.as_ref().map(|rti| RealTimeInfo {
                        trip_status: rti.real_time_trip_status.into(),
                        // has 1. at the start when past midnight????????
                        // what the actual FUCK
                        estimated_arrival_time: rti
                            .estimated_arrival_time
                            .clone()
                            .map(|x| x.replace("1.", "")),
                        estimated_departure_time: rti.estimated_departure_time.clone(),
                    }),
                })
            });

        Ok(RealTimeResponse {
            route_name: value.summary.route_name,
            trip_id: value.summary.trip_uid,
            current_position: GeoPosition::try_from_str(
                &value.summary.real_time_info.current_position,
            )?,
            last_updated: value.summary.real_time_info.last_updated,
            start_time: value.summary.trip_start_time,
            next_stop,
        })
    }
}

impl From<PTARouteResponse> for RouteResponse {
    fn from(value: PTARouteResponse) -> Self {
        Self {
            routes: value
                .routes
                .iter()
                .map(|x| Route {
                    r#type: if x.code.is_empty() {
                        RouteType::Train
                    } else {
                        RouteType::Bus
                    },
                    identifier: if x.code.is_empty() {
                        x.name.clone()
                    } else {
                        x.code.clone()
                    },
                    timetable_id: x.route_timetable_group_id.clone(),
                })
                .collect(),
        }
    }
}

impl From<PTATripResponse> for LiveTripResponse {
    fn from(value: PTATripResponse) -> Self {
        Self {
            live_trips: value
                .get_trip_infos_result
                .iter()
                .filter(|x| x.status == "Live")
                .map(|x| format!("PerthRestricted:{}", x.trip_id))
                .collect(),
        }
    }
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
                .to_owned()
                .trim()
                .parse()?,
            longitude: s
                .next()
                .context(format!("geoposition invalid: {}", value))?
                .to_owned()
                .trim()
                .parse()?,
        })
    }
}

impl From<i64> for TransitStopStatus {
    fn from(value: i64) -> Self {
        match value {
            3 => Self::Completed,
            2 => Self::AtStation,
            1 => Self::Scheduled,
            _ => panic!("unknown transit stop status {}", value),
        }
    }
}
