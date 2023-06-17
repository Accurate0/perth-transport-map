use crate::{
    constants::{TRANSPERTH_EARLY_HOURS, TRANSPERTH_TIMETABLE_ENDPOINT, TRANSPERTH_TRIP_LOOKUP},
    types::{
        config::ApplicationConfig,
        response::trip::LiveTripResponse,
        transperth::{timetable::PTATimetableResponse, trip::PTATripResponse},
    },
};
use anyhow::Context;
use chrono::{DateTime, Days, Timelike, Utc};
use http::header::HOST;
use reqwest_middleware::ClientWithMiddleware;
use std::{sync::Arc, time::SystemTime};
use tokio::time::Instant;

// TODO: cache this query
// using cached package probs, i'm lazy
pub async fn get_live_trips_for(
    timetable_id: &str,
    config: &ApplicationConfig,
    http_client: Arc<ClientWithMiddleware>,
) -> Result<LiveTripResponse, anyhow::Error> {
    let start = Instant::now();
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now_in_perth = now.with_timezone(&chrono_tz::Australia::Perth);
    let datetime = now_in_perth.format("%Y-%m-%d").to_string();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error")
        .as_millis();

    let response = http_client
        .get(TRANSPERTH_TIMETABLE_ENDPOINT)
        .header(HOST, "au-journeyplanner.silverrail.io".parse::<String>()?)
        .query(&[
            ("ApiKey", config.reference_data_api_key.as_str()),
            ("format", "json"),
            ("Route", timetable_id),
            ("StartDate", &datetime),
            ("EndDate", &datetime),
            ("ReturnNotes", "true"),
            ("_", &now.to_string()),
        ])
        .send()
        .await?;

    tracing::info!(
        "timetable request completed with status: {} in {} ms",
        response.status(),
        start.elapsed().as_millis()
    );

    let timetable_response = response.json::<PTATimetableResponse>().await?;
    let mut trip_ids: Vec<String> = timetable_response
        .timetable_trips
        .iter()
        .map(|t| t.trip_source_id.clone())
        .collect();

    // this is something the app does at around midnight?
    // TODO: refactor
    if TRANSPERTH_EARLY_HOURS.contains(&now_in_perth.hour()) {
        let datetime = now_in_perth
            .checked_sub_days(Days::new(1))
            .context("could not sub 1 day")?
            .format("%Y-%m-%d")
            .to_string();

        let response = http_client
            .get(TRANSPERTH_TIMETABLE_ENDPOINT)
            .header(HOST, "au-journeyplanner.silverrail.io".parse::<String>()?)
            .query(&[
                ("ApiKey", config.reference_data_api_key.as_str()),
                ("format", "json"),
                ("Route", timetable_id),
                ("StartDate", &datetime),
                ("EndDate", &datetime),
                ("ReturnNotes", "true"),
                ("_", &now.to_string()),
            ])
            .send()
            .await?;

        tracing::info!(
            "timetable request completed with status: {} in {} ms",
            response.status(),
            start.elapsed().as_millis()
        );

        let timetable_response = response.json::<PTATimetableResponse>().await?;
        trip_ids.append(
            &mut timetable_response
                .timetable_trips
                .iter()
                .map(|t| t.trip_source_id.clone())
                .collect(),
        );
    }

    let start = Instant::now();
    let response = http_client
        .get(TRANSPERTH_TRIP_LOOKUP)
        .header(
            HOST,
            "serviceinformation.transperth.info".parse::<String>()?,
        )
        .query(&[
            ("OperatingDate", datetime.as_str()),
            ("format", "json"),
            ("_", now.to_string().as_str()),
        ])
        .query(&[("TripIDs", trip_ids.join(","))])
        .send()
        .await?;

    tracing::info!(
        "trip request completed with status: {} in {} ms",
        response.status(),
        start.elapsed().as_millis()
    );

    let trip_response = response.json::<PTATripResponse>().await?;
    let live_trip_response = LiveTripResponse::from(trip_response);

    Ok(live_trip_response)
}
