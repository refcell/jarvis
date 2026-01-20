import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as llmService from '../services/llm';
import * as keychainService from '../services/keychain';
import { useLLMStore } from '../stores';
import type { LLMConfig } from '../types';

export function useLLMProvider() {
  const queryClient = useQueryClient();
  const { setConfig, setDetectedTools, setHealthy, setChecking } = useLLMStore();

  const configQuery = useQuery({
    queryKey: ['llm-config'],
    queryFn: async () => {
      const config = await llmService.getLLMConfig();
      setConfig(config);
      return config;
    },
  });

  const detectedToolsQuery = useQuery({
    queryKey: ['detected-cli-tools'],
    queryFn: async () => {
      const tools = await llmService.detectAvailableCLITools();
      setDetectedTools(tools);
      return tools;
    },
    staleTime: 60000, // Cache for 1 minute
  });

  const healthCheck = useMutation({
    mutationFn: async () => {
      setChecking(true);
      try {
        const healthy = await llmService.healthCheckLLM();
        setHealthy(healthy);
        return healthy;
      } finally {
        setChecking(false);
      }
    },
  });

  const updateConfig = useMutation({
    mutationFn: async (config: LLMConfig) => {
      await llmService.updateLLMConfig(config);
      setConfig(config);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['llm-config'] });
    },
  });

  const storeApiKey = useMutation({
    mutationFn: async ({ provider, apiKey }: { provider: string; apiKey: string }) => {
      await keychainService.storeApiKey(provider, apiKey);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['llm-config'] });
    },
  });

  const deleteApiKey = useMutation({
    mutationFn: async (provider: string) => {
      await keychainService.deleteApiKey(provider);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['llm-config'] });
    },
  });

  const hasApiKey = useQuery({
    queryKey: ['has-api-key', configQuery.data?.provider_type],
    queryFn: async () => {
      if (!configQuery.data) return false;
      return keychainService.hasApiKey(configQuery.data.provider_type);
    },
    enabled: !!configQuery.data,
  });

  return {
    config: configQuery.data,
    isLoadingConfig: configQuery.isLoading,
    detectedTools: detectedToolsQuery.data,
    isLoadingTools: detectedToolsQuery.isLoading,
    healthCheck: healthCheck.mutate,
    isCheckingHealth: healthCheck.isPending,
    isHealthy: healthCheck.data,
    resetHealthCheck: () => setHealthy(null),
    updateConfig: updateConfig.mutate,
    isUpdatingConfig: updateConfig.isPending,
    storeApiKey: storeApiKey.mutate,
    deleteApiKey: deleteApiKey.mutate,
    hasApiKey: hasApiKey.data ?? false,
  };
}
