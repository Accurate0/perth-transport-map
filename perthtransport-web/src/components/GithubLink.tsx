import { Button } from "@mui/material";
import useDarkMode from "../hooks/useDarkMode";
import { GitHub } from "@mui/icons-material";
import { useMemo } from "react";

const GithubLink = () => {
  const { isDarkMode } = useDarkMode();

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
      href="https://github.com/Accurate0/perth-transport-map"
      disableRipple
    >
      {isDarkMode ? (
        <GitHub htmlColor={colour} />
      ) : (
        <GitHub htmlColor={colour} />
      )}
    </Button>
  );
};

export default GithubLink;
