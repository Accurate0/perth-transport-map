use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use chrono::{DateTime, Utc};
use perthtransport::{
    constants::{TRANSPERTH_ROUTE_ENDPOINT, TRANSPERTH_TIMETABLE_ENDPOINT, TRANSPERTH_TRIP_LOOKUP},
    types::{
        config::ApplicationConfig,
        response::{route::RouteResponse, trip::LiveTripResponse},
        transperth::{
            route::PTARouteResponse, timetable::PTATimetableResponse, trip::PTATripResponse,
        },
    },
};
use reqwest::header::HOST;
use reqwest_middleware::ClientWithMiddleware;
use std::{sync::Arc, time::SystemTime};
use tokio::time::Instant;

pub type PTASchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn route<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Bus number or train line name")] search_term: String,
    ) -> Result<RouteResponse, anyhow::Error> {
        // TODO: fetch dynamically
        let config: &ApplicationConfig = ctx.data_unchecked();
        let http_client: &Arc<ClientWithMiddleware> = ctx.data_unchecked();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get millis error")
            .as_millis();

        let start = Instant::now();
        let response = http_client
            .get(TRANSPERTH_ROUTE_ENDPOINT)
            .header(HOST, "au-journeyplanner.silverrail.io".parse::<String>()?)
            .query(&[
                ("ApiKey", config.reference_data_api_key.as_str()),
                ("format", "json"),
                ("SearchTerm", &search_term),
                ("_", &now.to_string()),
            ])
            .send()
            .await?;

        tracing::info!(
            "route request completed with status: {} in {} ms",
            response.status(),
            start.elapsed().as_millis()
        );

        let response = response.json::<PTARouteResponse>().await?;

        Ok(RouteResponse::from(response))
    }

    async fn live_trips<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Timetable ID of a route")] timetable_id: String,
    ) -> Result<LiveTripResponse, anyhow::Error> {
        // TODO: fetch dynamically
        let config: &ApplicationConfig = ctx.data_unchecked();
        let http_client: &Arc<ClientWithMiddleware> = ctx.data_unchecked();

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
                ("Route", &timetable_id),
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
        let trip_ids: Vec<String> = timetable_response
            .timetable_trips
            .iter()
            .map(|t| t.trip_source_id.clone())
            .collect();

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
}
