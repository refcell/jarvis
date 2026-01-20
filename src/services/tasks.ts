import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../types';

export async function getActiveTasks(): Promise<Task[]> {
  return invoke('get_active_tasks');
}

export async function getAllTasks(): Promise<Task[]> {
  return invoke('get_all_tasks');
}

export async function getTask(id: string): Promise<Task | null> {
  return invoke('get_task', { id });
}

export async function createTask(
  title: string,
  description: string,
  context: string,
  priority: number
): Promise<Task> {
  return invoke('create_task', { title, description, context, priority });
}

export async function updateTaskStatus(id: string, status: string): Promise<Task> {
  return invoke('update_task_status', { id, status });
}

export async function snoozeTask(id: string, hours: number): Promise<Task> {
  return invoke('snooze_task', { id, hours });
}

export async function dismissTask(id: string): Promise<void> {
  return invoke('dismiss_task', { id });
}

export async function completeTask(id: string): Promise<void> {
  return invoke('complete_task', { id });
}

export async function deleteTask(id: string): Promise<void> {
  return invoke('delete_task', { id });
}
