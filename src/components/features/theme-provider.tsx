import { invoke } from "@tauri-apps/api/core";
import {
  createContext,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";

export type Theme = "light" | "dark" | "system";

type ThemeProviderState = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  isLoading: boolean;
};

const initialState: ThemeProviderState = {
  theme: "dark",
  setTheme: () => {},
  isLoading: false,
};

const ThemeProviderContext = createContext<ThemeProviderState>(initialState);

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setThemeState] = useState<Theme>("dark");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    invoke<Theme>("get_theme")
      .then((appTheme) => {
        setThemeState(appTheme);
      })
      .catch((error) =>
        console.error("Failed to get Theme from the app:", error),
      )
      .finally(() => setIsLoading(false));
  }, []);

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");

    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme:dark")
        .matches
        ? "dark"
        : "light";
      root.classList.add(systemTheme);
      return;
    }
    root.classList.add(theme);
  }, [theme]);

  const setTheme = (newThemeSelected: Theme) => {
    setThemeState(newThemeSelected);
    invoke("set_theme", { theme: newThemeSelected }).catch((error) =>
      console.error("Failed to save theme in the app.", error),
    );
  };

  if (isLoading)
    return <div className="bg-background min-h-screen min-w-screen" />;

  return (
    <ThemeProviderContext.Provider value={{ theme, setTheme, isLoading }}>
      {children}
    </ThemeProviderContext.Provider>
  );
}

export const useTheme = () => useContext(ThemeProviderContext);
