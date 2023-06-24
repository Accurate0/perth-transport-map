import {
  GoogleMap,
  TransitLayerF,
  useLoadScript,
} from "@react-google-maps/api";
import { useMemo, useState } from "react";
import lightStyles from "./styles.light.json";
import darkStyles from "./styles.dark.json";
import useWebSocket from "../../hooks/useWebSocket";
import { RouteName, useGetRouteColour } from "../../hooks/useGetRouteColour";
import DarkModeToggle from "../../components/DarkModeToggle";
import useDarkMode from "../../hooks/useDarkMode";
import { AnimatedMarker } from "../../components/AnimatedMarker";
import { RealTimeMessage } from "../../types";
import { Typography } from "@mui/material";
import useHealthCheck from "../../hooks/useHealthCheck";
import HealthStatus from "../../components/HealthStatus";

interface RealTime {
  lat: number;
  lng: number;
  tripId: string;
  routeName: string;
  nextStopName?: string;
  nextStopEstimated?: Date;
}

const MapRoute = () => {
  const { isHealthy } = useHealthCheck();
  const { isDarkMode } = useDarkMode();

  const { isLoaded } = useLoadScript({
    googleMapsApiKey: import.meta.env.VITE_MAPS_API_KEY as string,
  });

  const onMessage = (data: string) => {
    const info = JSON.parse(data) as RealTimeMessage;

    setTrainState((prev) => {
      const nextStop = info.transitStops.find(
        (t) => t.realTimeInfo?.tripStatus === "Scheduled"
      );

      const nextStopEstimatedArrival =
        nextStop?.realTimeInfo?.estimatedArrivalTime;

      return [
        ...prev.filter((x) => x.tripId !== info.tripId),
        {
          lat: info.currentPosition.latitude,
          lng: info.currentPosition.longitude,
          tripId: info.tripId,
          routeName: info.routeName,
          nextStopName: nextStop?.description,
          nextStopEstimated: nextStopEstimatedArrival
            ? new Date(`1970-01-01T${nextStopEstimatedArrival}`)
            : undefined,
        },
      ];
    });
  };

  useWebSocket(onMessage);

  const [trainState, setTrainState] = useState<RealTime[]>([]);

  const mapOptions = useMemo<google.maps.MapOptions>(
    () => ({
      disableDefaultUI: true,
      clickableIcons: true,
      styles: (isDarkMode
        ? darkStyles
        : lightStyles) as google.maps.MapTypeStyle[],
      restriction: {
        latLngBounds: {
          north: -31.61983458918886,
          south: -32.56142128884333,
          west: 114.8536535903477,
          east: 116.59953695847219,
        },
      },
    }),
    [isDarkMode]
  );

  const mapCenter = useMemo(
    () => ({ lat: -31.957250462794217, lng: 115.86367878837541 }),
    []
  );

  const zoomLevel = 11;

  if (!isLoaded) {
    return null;
  }

  return (
    <>
      <DarkModeToggle />
      <HealthStatus isHealthy={isHealthy} />
      <GoogleMap
        options={mapOptions}
        zoom={zoomLevel}
        center={mapCenter}
        mapTypeId={google.maps.MapTypeId.ROADMAP}
        mapContainerStyle={{
          width: "100%",
          height: "100%",
        }}
      >
        <TransitLayerF />
        {trainState.map((t) => (
          <AnimatedMarker
            routeName={t.routeName}
            key={t.tripId}
            infoWindowChildren={
              <>
                <Typography variant="subtitle2">{t.routeName}</Typography>
                <Typography component="p" variant="caption">
                  Next: {t.nextStopName ?? "Unknown"}
                </Typography>
                <Typography component="p" variant="caption">
                  Estimated:{" "}
                  {t.nextStopEstimated?.toLocaleTimeString() ?? "Unknown"}
                </Typography>
              </>
            }
            position={new google.maps.LatLng({ ...t })}
          />
        ))}
      </GoogleMap>
    </>
  );
};

export default MapRoute;
