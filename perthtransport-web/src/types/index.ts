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

export interface RealTimeMessage {
  tripId: string;
  routeName: string;
  currentPosition: CurrentPosition;
  lastUpdated: string;
  startTime: string;
  transitStops: TransitStop[];
}

export interface CurrentPosition {
  latitude: number;
  longitude: number;
}

export interface TransitStop {
  position: Position;
  description: string;
  realTimeInfo?: RealTimeInfo;
}

export interface Position {
  latitude: number;
  longitude: number;
}

export interface RealTimeInfo {
  tripStatus: string;
  estimatedArrivalTime?: string;
  estimatedDepartureTime?: string;
}
