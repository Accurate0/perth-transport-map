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
