import { create } from "zustand";
import type { Submission } from "@/models/submission";

interface SharedState {
  refresh: number;
  setRefresh: () => void;

  history: Array<Submission>;
  saveHistory: (submission: Submission) => void;
}

export const useSharedStore = create<SharedState>((set, get) => ({
  refresh: 0,
  setRefresh: () => set({ refresh: get().refresh + 1 }),

  history: [],
  saveHistory: (submission: Submission) => {
    const history = get().history;

    const existingIndex = history.findIndex(
      (item) => item.id === submission.id
    );

    if (existingIndex !== -1) {
      const updatedHistory = [...history];
      updatedHistory[existingIndex] = {
        ...updatedHistory[existingIndex],
        ...submission,
      };
      set({ history: updatedHistory });
    } else {
      set({ history: [submission, ...history] });
    }
  },
}));
