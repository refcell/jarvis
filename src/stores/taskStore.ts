import { create } from 'zustand';
import type { TaskStatus } from '../types';

interface TaskState {
  selectedTaskId: string | null;
  filter: TaskStatus | 'all';
  searchQuery: string;

  // Actions
  setSelectedTaskId: (id: string | null) => void;
  setFilter: (filter: TaskStatus | 'all') => void;
  setSearchQuery: (query: string) => void;
  clearSelection: () => void;
}

export const useTaskStore = create<TaskState>((set) => ({
  selectedTaskId: null,
  filter: 'all',
  searchQuery: '',

  setSelectedTaskId: (id) => set({ selectedTaskId: id }),
  setFilter: (filter) => set({ filter }),
  setSearchQuery: (query) => set({ searchQuery: query }),
  clearSelection: () => set({ selectedTaskId: null }),
}));
