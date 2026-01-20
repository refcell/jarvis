import { invoke } from '@tauri-apps/api/core';
import type { CaptureContext } from '../types';

export async function checkScreenPermission(): Promise<boolean> {
  return invoke('check_screen_permission');
}

export async function requestScreenPermission(): Promise<void> {
  return invoke('request_screen_permission');
}

export async function captureScreen(): Promise<CaptureContext> {
  return invoke('capture_screen');
}

export async function getRecentCaptures(limit: number): Promise<CaptureContext[]> {
  return invoke('get_recent_captures', { limit });
}
