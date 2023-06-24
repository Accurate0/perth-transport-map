import { useMemo } from "react";
import useDarkMode from "./useDarkMode";

export type RouteName =
  | "Armadale Line"
  | "Thornlie Line"
  | "Joondalup Line"
  | "Midland Line"
  | "Airport Line"
  | "Mandurah Line"
  | "Fremantle Line";

export const useGetRouteColour = () => {
  const { isDarkMode } = useDarkMode();

  return useMemo(
    () => (routeName: RouteName) => {
      switch (routeName) {
        case "Armadale Line":
        case "Thornlie Line":
          return "#fcbd12";
        case "Joondalup Line":
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
    [isDarkMode]
  );
};
