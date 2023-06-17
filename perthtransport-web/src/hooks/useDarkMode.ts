import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";

const darkModeAtom = atomWithStorage("darkMode", false);

const useDarkMode = () => {
  const [darkMode, setDarkMode] = useAtom(darkModeAtom);

  return {
    isDarkMode: darkMode,
    toggleDarkMode: () => setDarkMode((x) => !x),
  };
};

export default useDarkMode;
