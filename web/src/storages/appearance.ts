import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

type Theme = "light" | "dark";

interface AppearanceState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useApperanceStore = create<AppearanceState>()(
  persist(
    (set) => ({
      theme: "dark",
      setTheme: (theme: Theme) => set({ theme }),
    }),
    {
      name: "apperance",
      storage: createJSONStorage(() => localStorage),
    }
  )
);
