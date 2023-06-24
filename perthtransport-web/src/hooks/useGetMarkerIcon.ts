import { faSubway } from "@fortawesome/free-solid-svg-icons";
import { RouteName, useGetRouteColour } from "./useGetRouteColour";

const useGetMarkerIcon = () => {
  const getRouteColour = useGetRouteColour();

  return (routeName: string) => ({
    path: faSubway.icon[4] as string,
    fillColor: getRouteColour(routeName as RouteName),
    fillOpacity: 1,
    anchor: new google.maps.Point(
      faSubway.icon[0] / 2, // width
      faSubway.icon[1] // height
    ),
    scale: 0.035,
  });
};

export default useGetMarkerIcon;
