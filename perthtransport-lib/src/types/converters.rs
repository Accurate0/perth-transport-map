use anyhow::bail;

use super::{
    response::{
        realtime::{GeoPosition, RealTimeInfo, RealTimeResponse, TransitStop},
        route::{Route, RouteResponse},
        trip::LiveTripResponse,
    },
    transperth::{realtime::PTARealTimeResponse, route::PTARouteResponse, trip::PTATripResponse},
};

impl TryFrom<PTARealTimeResponse> for RealTimeResponse {
    type Error = anyhow::Error;

    fn try_from(value: PTARealTimeResponse) -> Result<Self, Self::Error> {
        let transit_stops = value
            .trip_stops
            .iter()
            .map(|t| -> Result<TransitStop, anyhow::Error> {
                Ok(TransitStop {
                    position: GeoPosition::try_from_str(&t.transit_stop.position)?,
                    description: t.transit_stop.description.clone(),
                    real_time_info: RealTimeInfo {
                        trip_status: t.real_time_info.real_time_trip_status,
                        estimated_arrival_time: t.real_time_info.estimated_arrival_time.clone(),
                        estimated_departure_time: t.real_time_info.estimated_departure_time.clone(),
                    },
                })
            })
            .collect::<Vec<_>>();

        if transit_stops.iter().any(|t| t.is_err()) {
            let errored = transit_stops.iter().filter(|t| t.is_err());
            for error in errored {
                tracing::info!("transit stop in error: {:#?}", error);
            }
            bail!("transit stop mapping error")
        }

        Ok(RealTimeResponse {
            trip_id: value.summary.trip_uid,
            current_position: GeoPosition::try_from_str(
                &value.summary.real_time_info.current_position,
            )?,
            last_updated: value.summary.real_time_info.last_updated,
            start_time: value.summary.trip_start_time,
            transit_stops: transit_stops
                .into_iter()
                .filter_map(|t| t.ok())
                .collect::<Vec<_>>(),
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