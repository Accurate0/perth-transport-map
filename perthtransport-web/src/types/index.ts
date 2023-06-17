export interface RouteResponse {
  route: Routes;
}

export interface Routes {
  routes: Route[];
}

export interface Route {
  identifier: string;
  timetableId: string;
}

export interface LiveTripsResponse {
  liveTrips: LiveTrips;
}

export interface LiveTrips {
  liveTrips: string[];
}
