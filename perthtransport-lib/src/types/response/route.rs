use async_graphql::{Enum, Object};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResponse {
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub r#type: RouteType,
    pub identifier: String,
    pub timetable_id: String,
}

#[derive(Serialize, Deserialize, Debug, Enum, Clone, Copy, PartialEq, Eq)]
pub enum RouteType {
    Bus,
    Train,
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

    pub async fn r#type(&self) -> &RouteType {
        &self.r#type
    }
}
