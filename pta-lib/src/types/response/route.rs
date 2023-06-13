use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResponse {
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub identifier: String,
    pub timetable_id: String,
}

#[Object]
impl RouteResponse {
    pub async fn routes(&self) -> &Vec<Route> {
        &self.routes
    }
}

#[Object]
impl Route {
    pub async fn identifier(&self) -> &String {
        &self.identifier
    }

    pub async fn timetable_id(&self) -> &String {
        &self.timetable_id
    }
}
