import { useMemo } from "react";
import useDarkMode from "./useDarkMode";

export type RouteName =
  | "Armadale Line"
  | "Thornlie-Cockburn Line"
  | "Yanchep Line"
  | "Midland Line"
  | "Airport Line"
  | "Mandurah Line"
  | "Fremantle Line"
  | "Ellenbrook Line";

export const useGetRouteColour = () => {
  const { isDarkMode } = useDarkMode();

  return useMemo(
    () => (routeName: RouteName) => {
      switch (routeName) {
        case "Ellenbrook Line":
          return isDarkMode ? "#d32737" : "#d32838";
        case "Armadale Line":
        case "Thornlie-Cockburn Line":
          return "#fcbd12";
        case "Yanchep Line":
          return "#91a333";
        case "Midland Line":
          return "#990033";
        case "Airport Line":
          return "#46c1b3";
        case "Mandurah Line":
          return "#e0701c";
        case "Fremantle Line":
          return isDarkMode ? "#01427c" : "#000099";
      }
    },
    [isDarkMode],
  );
};
