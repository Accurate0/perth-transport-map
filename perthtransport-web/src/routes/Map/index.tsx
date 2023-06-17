import { GoogleMap, Marker, useLoadScript } from "@react-google-maps/api";
import { useMemo, useState } from "react";
import mapStyles from "./styles.json";
import useWebSocket from "../../hooks/useWebSocket";
import { faSubway } from "@fortawesome/free-solid-svg-icons";
import { RouteName, getRouteColour } from "../../utils/getRouteColour";

const MapRoute = () => {
  const { isLoaded } = useLoadScript({
    googleMapsApiKey: import.meta.env.VITE_MAPS_API_KEY as string,
  });

  const onMessage = (data: string) => {
    const jsonData = JSON.parse(data);

    setTrainState((prev) => [
      ...prev.filter((x) => x.tripId !== jsonData["tripId"]),
      {
        // TODO: types :)
        lat: jsonData["currentPosition"]["latitude"],
        lng: jsonData["currentPosition"]["longitude"],
        tripId: jsonData["tripId"],
        routeName: jsonData["routeName"],
      },
    ]);
  };

  useWebSocket(onMessage);

  const [trainState, setTrainState] = useState<
    {
      lat: number;
      lng: number;
      tripId: string;
      routeName: string;
    }[]
  >([]);

  const mapOptions = useMemo<google.maps.MapOptions>(
    () => ({
      disableDefaultUI: true,
      clickableIcons: true,
      styles: mapStyles as google.maps.MapTypeStyle[],
    }),
    []
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
      <GoogleMap
        options={mapOptions}
        zoom={zoomLevel}
        center={mapCenter}
        mapTypeId={google.maps.MapTypeId.ROADMAP}
        mapContainerStyle={{ width: "100%", height: "100%" }}
      >
        {trainState.map((t) => (
          <Marker
            key={t.tripId}
            position={{ ...t }}
            icon={{
              path: faSubway.icon[4] as string,
              fillColor: getRouteColour(t.routeName as RouteName),
              fillOpacity: 1,
              anchor: new google.maps.Point(
                faSubway.icon[0] / 2, // width
                faSubway.icon[1] // height
              ),
              strokeWeight: 1,
              strokeColor: "#ffffff",
              scale: 0.055,
            }}
          />
        ))}
      </GoogleMap>
    </>
  );
};

export default MapRoute;
