"use client";
import { GoogleMap, useLoadScript } from "@react-google-maps/api";
import { useMemo } from "react";
import mapStyles from "./styles.json";

const Map = () => {
  const libraries = useMemo(() => ["places"], []);
  const { isLoaded } = useLoadScript({
    googleMapsApiKey: process.env.NEXT_PUBLIC_MAPS_API_KEY as string,
    libraries: libraries as any,
  });
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
    />
  );
};

export default Map;
