import { invoke } from '@tauri-apps/api/core';
import type { Settings, WatchStatus } from '../types';

export async function getSettings(): Promise<Settings> {
  return invoke('get_settings');
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke('save_settings', { settings });
}

export async function getWatchStatus(): Promise<WatchStatus> {
  return invoke('get_watch_status');
}

export async function setWatching(enabled: boolean): Promise<void> {
  return invoke('set_watching', { enabled });
}

export async function updateCaptureInterval(intervalSecs: number): Promise<void> {
  return invoke('update_capture_interval', { intervalSecs });
}

export async function toggleNotifications(enabled: boolean): Promise<void> {
  return invoke('toggle_notifications', { enabled });
}
