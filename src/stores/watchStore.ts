import { create } from 'zustand';
import type { WatchStatus } from '../types';

interface WatchState {
  status: WatchStatus;
  isCapturing: boolean;
  lastError: string | null;

  // Actions
  setStatus: (status: WatchStatus) => void;
  setCapturing: (capturing: boolean) => void;
  setError: (error: string | null) => void;
  incrementCaptures: () => void;
  incrementTasksDetected: (count: number) => void;
}

export const useWatchStore = create<WatchState>((set) => ({
  status: {
    is_watching: false,
    last_capture_at: null,
    captures_since_start: 0,
    tasks_detected_since_start: 0,
  },
  isCapturing: false,
  lastError: null,

  setStatus: (status) => set({ status }),
  setCapturing: (capturing) => set({ isCapturing: capturing }),
  setError: (error) => set({ lastError: error }),
  incrementCaptures: () =>
    set((state) => ({
      status: {
        ...state.status,
        captures_since_start: state.status.captures_since_start + 1,
        last_capture_at: new Date().toISOString(),
      },
    })),
  incrementTasksDetected: (count) =>
    set((state) => ({
      status: {
        ...state.status,
        tasks_detected_since_start: state.status.tasks_detected_since_start + count,
      },
    })),
}));
