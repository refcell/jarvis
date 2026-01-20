import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as settingsService from '../services/settings';
import { useSettingsStore } from '../stores';
import type { Settings } from '../types';

export function useSettings() {
  const queryClient = useQueryClient();
  const { setSettings, setLoading, setError } = useSettingsStore();

  const settingsQuery = useQuery({
    queryKey: ['settings'],
    queryFn: async () => {
      setLoading(true);
      try {
        const settings = await settingsService.getSettings();
        setSettings(settings);
        return settings;
      } catch (error) {
        setError(error instanceof Error ? error.message : 'Failed to load settings');
        throw error;
      } finally {
        setLoading(false);
      }
    },
  });

  const saveSettings = useMutation({
    mutationFn: async (settings: Settings) => {
      await settingsService.saveSettings(settings);
      setSettings(settings);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['settings'] });
    },
  });

  const toggleNotifications = useMutation({
    mutationFn: async (enabled: boolean) => {
      await settingsService.toggleNotifications(enabled);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['settings'] });
    },
  });

  const updateCaptureInterval = useMutation({
    mutationFn: async (intervalSecs: number) => {
      await settingsService.updateCaptureInterval(intervalSecs);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['settings'] });
    },
  });

  return {
    settings: settingsQuery.data,
    isLoading: settingsQuery.isLoading,
    error: settingsQuery.error,
    saveSettings: saveSettings.mutate,
    isSaving: saveSettings.isPending,
    toggleNotifications: toggleNotifications.mutate,
    updateCaptureInterval: updateCaptureInterval.mutate,
  };
}
