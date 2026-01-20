import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useCallback, useEffect, useRef } from 'react';
import * as captureService from '../services/capture';
import * as llmService from '../services/llm';
import { useWatchStore } from '../stores';
import { useSettingsStore } from '../stores';
import { ACTIVE_TASKS_QUERY_KEY } from './useTasks';

export function useScreenCapture() {
  const queryClient = useQueryClient();
  const { status, setStatus, setCapturing, setError, incrementCaptures, incrementTasksDetected } =
    useWatchStore();
  const { settings } = useSettingsStore();
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const permissionQuery = useQuery({
    queryKey: ['screen-permission'],
    queryFn: captureService.checkScreenPermission,
    staleTime: Infinity,
  });

  const requestPermission = useMutation({
    mutationFn: captureService.requestScreenPermission,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['screen-permission'] });
    },
  });

  const captureAndAnalyze = useMutation({
    mutationFn: async () => {
      setCapturing(true);
      setError(null);

      try {
        // Capture screen
        const context = await captureService.captureScreen();
        incrementCaptures();

        // Analyze with LLM and create tasks
        const tasks = await llmService.analyzeAndCreateTasks(context);
        if (tasks.length > 0) {
          incrementTasksDetected(tasks.length);
          queryClient.invalidateQueries({ queryKey: ACTIVE_TASKS_QUERY_KEY });
        }

        return { context, tasks };
      } finally {
        setCapturing(false);
      }
    },
    onError: (error) => {
      setError(error instanceof Error ? error.message : 'Capture failed');
      setCapturing(false);
    },
  });

  const startWatching = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    const intervalMs = (settings?.capture_interval_secs ?? 30) * 1000;

    // Capture immediately
    captureAndAnalyze.mutate();

    // Then set up interval
    intervalRef.current = setInterval(() => {
      captureAndAnalyze.mutate();
    }, intervalMs);

    setStatus({ ...status, is_watching: true });
  }, [settings?.capture_interval_secs, status]);

  const stopWatching = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }

    setStatus({
      ...status,
      is_watching: false,
      captures_since_start: 0,
      tasks_detected_since_start: 0,
    });
  }, [status]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  return {
    hasPermission: permissionQuery.data ?? false,
    isCheckingPermission: permissionQuery.isLoading,
    requestPermission: requestPermission.mutate,
    isRequestingPermission: requestPermission.isPending,
    captureAndAnalyze: captureAndAnalyze.mutate,
    isCapturing: captureAndAnalyze.isPending,
    startWatching,
    stopWatching,
    isWatching: status.is_watching,
    watchStatus: status,
  };
}
