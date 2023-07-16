import { Button } from "@mui/material";
import useDarkMode from "../hooks/useDarkMode";
import { Favorite, HeartBroken } from "@mui/icons-material";
import { useMemo } from "react";

const HealthStatus = ({ isHealthy }: { isHealthy: boolean }) => {
  const { isDarkMode } = useDarkMode();

  const backgroundColour = useMemo(
    () => (isDarkMode ? "#000" : "#fff"),
    [isDarkMode]
  );

  return (
    <Button
      style={{
        position: "fixed",
        zIndex: 50,
        bottom: "114px",
        left: "18px",
        background: backgroundColour,
        maxHeight: "40px",
        minHeight: "40px",
        maxWidth: "40px",
        minWidth: "40px",
        cursor: "default",
      }}
      disableRipple
    >
      {isHealthy ? (
        <Favorite htmlColor="green" />
      ) : (
        <HeartBroken htmlColor="red" />
      )}
    </Button>
  );
};

export default HealthStatus;
