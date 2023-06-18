import {
  GoogleMap,
  MarkerF,
  TransitLayerF,
  useLoadScript,
} from "@react-google-maps/api";
import { useMemo, useState } from "react";
import lightStyles from "./styles.light.json";
import darkStyles from "./styles.dark.json";
import useWebSocket from "../../hooks/useWebSocket";
import { faSubway } from "@fortawesome/free-solid-svg-icons";
import { RouteName, getRouteColour } from "../../utils/getRouteColour";
import DarkModeToggle from "../../components/DarkModeToggle";
import useDarkMode from "../../hooks/useDarkMode";

const MapRoute = () => {
  const { isDarkMode } = useDarkMode();

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
      styles: (isDarkMode
        ? darkStyles
        : lightStyles) as google.maps.MapTypeStyle[],
      restriction: {
        latLngBounds: {
          north: -31.64983458918886,
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
          <MarkerF
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
              scale: 0.055,
            }}
          />
        ))}
      </GoogleMap>
    </>
  );
};

export default MapRoute;
