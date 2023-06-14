"use client";
import { GoogleMap, Marker, useLoadScript } from "@react-google-maps/api";
import { useEffect, useMemo, useState } from "react";
import mapStyles from "./styles.json";
import useWebSocket from "../../hooks/useWebSocket";

const Map = () => {
  const { isLoaded } = useLoadScript({
    googleMapsApiKey: import.meta.env.VITE_MAPS_API_KEY as string,
  });

  const [trainState, setTrainState] = useState<{ lat: number; lng: number }>();

  const onMessage = (data: string) => {
    setTrainState({
      lat: parseFloat(JSON.parse(data)["currentPosition"]["latitude"]),
      lng: parseFloat(JSON.parse(data)["currentPosition"]["longitude"]),
    });
  };

  useEffect(() => console.log(trainState), [trainState]);

  useWebSocket(onMessage);

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
    <GoogleMap
      options={mapOptions}
      zoom={zoomLevel}
      center={mapCenter}
      mapTypeId={google.maps.MapTypeId.ROADMAP}
      mapContainerStyle={{ width: "100%", height: "100%" }}
    >
      {trainState && <Marker position={{ ...trainState }} />}
    </GoogleMap>
  );
};

export default Map;
