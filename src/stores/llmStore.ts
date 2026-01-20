import { create } from 'zustand';
import type { DetectedCLITools, LLMConfig, LLMProviderType } from '../types';

interface LLMState {
  config: LLMConfig | null;
  detectedTools: DetectedCLITools | null;
  isHealthy: boolean | null;
  isChecking: boolean;

  // Actions
  setConfig: (config: LLMConfig) => void;
  setDetectedTools: (tools: DetectedCLITools) => void;
  setHealthy: (healthy: boolean | null) => void;
  setChecking: (checking: boolean) => void;
  updateProvider: (providerType: LLMProviderType) => void;
}

export const useLLMStore = create<LLMState>((set) => ({
  config: null,
  detectedTools: null,
  isHealthy: null,
  isChecking: false,

  setConfig: (config) => set({ config }),
  setDetectedTools: (tools) => set({ detectedTools: tools }),
  setHealthy: (healthy) => set({ isHealthy: healthy }),
  setChecking: (checking) => set({ isChecking: checking }),
  updateProvider: (providerType) =>
    set((state) => ({
      config: state.config
        ? { ...state.config, provider_type: providerType }
        : null,
    })),
}));
