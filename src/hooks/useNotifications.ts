import { useCallback } from 'react';
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
import { useSettingsStore } from '../stores';
import type { Task } from '../types';

export function useNotifications() {
  const { settings } = useSettingsStore();

  const checkPermission = useCallback(async () => {
    return await isPermissionGranted();
  }, []);

  const requestNotificationPermission = useCallback(async () => {
    const permission = await requestPermission();
    return permission === 'granted';
  }, []);

  const notifyNewTask = useCallback(
    async (task: Task) => {
      if (!settings?.notifications_enabled) return;

      const hasPermission = await isPermissionGranted();
      if (!hasPermission) return;

      sendNotification({
        title: 'New Task Detected',
        body: task.title,
      });
    },
    [settings?.notifications_enabled]
  );

  const notifyMultipleTasks = useCallback(
    async (count: number) => {
      if (!settings?.notifications_enabled) return;
      if (count === 0) return;

      const hasPermission = await isPermissionGranted();
      if (!hasPermission) return;

      sendNotification({
        title: 'Tasks Detected',
        body: `${count} new task${count > 1 ? 's' : ''} detected from screen capture`,
      });
    },
    [settings?.notifications_enabled]
  );

  return {
    checkPermission,
    requestNotificationPermission,
    notifyNewTask,
    notifyMultipleTasks,
    notificationsEnabled: settings?.notifications_enabled ?? false,
  };
}
