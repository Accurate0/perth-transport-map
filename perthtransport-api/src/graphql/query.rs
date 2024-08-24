use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use perthtransport::{
    constants::TRANSPERTH_ROUTE_ENDPOINT,
    query,
    types::{
        config::ApplicationConfig,
        response::{route::RouteResponse, trip::LiveTripResponse},
        transperth::route::PTARouteResponse,
    },
};
use reqwest_middleware::ClientWithMiddleware;
use std::{sync::Arc, time::SystemTime};

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

        let response = http_client
            .get(TRANSPERTH_ROUTE_ENDPOINT)
            .query(&[
                ("ApiKey", config.reference_data_api_key.as_str()),
                ("format", "json"),
                ("SearchTerm", &search_term),
                ("_", &now.to_string()),
            ])
            .send()
            .await?
            .error_for_status()?;

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

        query::get_live_trips_for(&timetable_id, config, http_client.clone()).await
    }
}
