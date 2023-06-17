import { Button } from "@mui/material";
import useDarkMode from "../hooks/useDarkMode";
import { DarkMode, LightMode } from "@mui/icons-material";
import { useMemo } from "react";

const DarkModeToggle = () => {
  const { isDarkMode, toggleDarkMode } = useDarkMode();

  const colour = useMemo(() => (isDarkMode ? "#fff" : "#000"), [isDarkMode]);
  const backgroundColour = useMemo(
    () => (isDarkMode ? "#000" : "#fff"),
    [isDarkMode]
  );

  return (
    <Button
      style={{
        position: "fixed",
        zIndex: 50,
        bottom: "18px",
        left: "18px",
        background: backgroundColour,
        maxHeight: "40px",
        minHeight: "40px",
        maxWidth: "40px",
        minWidth: "40px",
      }}
      onClick={toggleDarkMode}
      disableRipple
    >
      {isDarkMode ? (
        <DarkMode htmlColor={colour} />
      ) : (
        <LightMode htmlColor={colour} />
      )}
    </Button>
  );
};

export default DarkModeToggle;
