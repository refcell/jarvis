import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as taskService from '../services/tasks';
import type { TaskStatus } from '../types';

export const TASKS_QUERY_KEY = ['tasks'];
export const ACTIVE_TASKS_QUERY_KEY = ['tasks', 'active'];

export function useActiveTasks() {
  return useQuery({
    queryKey: ACTIVE_TASKS_QUERY_KEY,
    queryFn: taskService.getActiveTasks,
    refetchInterval: 30000, // Refresh every 30 seconds for priority decay
  });
}

export function useAllTasks() {
  return useQuery({
    queryKey: TASKS_QUERY_KEY,
    queryFn: taskService.getAllTasks,
  });
}

export function useTasks(filter: TaskStatus | 'all' = 'all') {
  const { data: tasks, ...rest } = useAllTasks();

  const filteredTasks = tasks?.filter((task) => {
    if (filter === 'all') return true;
    return task.status === filter;
  });

  return { data: filteredTasks, ...rest };
}

export function useTaskMutations() {
  const queryClient = useQueryClient();

  const invalidateTasks = () => {
    queryClient.invalidateQueries({ queryKey: TASKS_QUERY_KEY });
    queryClient.invalidateQueries({ queryKey: ACTIVE_TASKS_QUERY_KEY });
  };

  const createTask = useMutation({
    mutationFn: ({
      title,
      description,
      context,
      priority,
    }: {
      title: string;
      description: string;
      context: string;
      priority: number;
    }) => taskService.createTask(title, description, context, priority),
    onSuccess: invalidateTasks,
  });

  const updateStatus = useMutation({
    mutationFn: ({ id, status }: { id: string; status: TaskStatus }) =>
      taskService.updateTaskStatus(id, status),
    onSuccess: invalidateTasks,
  });

  const snooze = useMutation({
    mutationFn: ({ id, hours }: { id: string; hours: number }) =>
      taskService.snoozeTask(id, hours),
    onSuccess: invalidateTasks,
  });

  const dismiss = useMutation({
    mutationFn: (id: string) => taskService.dismissTask(id),
    onSuccess: invalidateTasks,
  });

  const complete = useMutation({
    mutationFn: (id: string) => taskService.completeTask(id),
    onSuccess: invalidateTasks,
  });

  const remove = useMutation({
    mutationFn: (id: string) => taskService.deleteTask(id),
    onSuccess: invalidateTasks,
  });

  return {
    createTask,
    updateStatus,
    snooze,
    dismiss,
    complete,
    remove,
  };
}
