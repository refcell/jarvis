import { Check, X, Clock, Play } from 'lucide-react';
import { Button } from '../common';
import { useTaskMutations } from '../../hooks';
import type { Task } from '../../types';
import styles from './TaskActions.module.css';

interface TaskActionsProps {
  task: Task;
}

export function TaskActions({ task }: TaskActionsProps) {
  const { complete, dismiss, snooze, updateStatus } = useTaskMutations();

  const handleComplete = (e: React.MouseEvent) => {
    e.stopPropagation();
    complete.mutate(task.id);
  };

  const handleDismiss = (e: React.MouseEvent) => {
    e.stopPropagation();
    dismiss.mutate(task.id);
  };

  const handleSnooze = (e: React.MouseEvent) => {
    e.stopPropagation();
    snooze.mutate({ id: task.id, hours: 1 });
  };

  const handleStart = (e: React.MouseEvent) => {
    e.stopPropagation();
    updateStatus.mutate({ id: task.id, status: 'in_progress' });
  };

  if (task.status === 'completed' || task.status === 'dismissed') {
    return null;
  }

  return (
    <div className={styles.actions}>
      <Button
        variant="ghost"
        size="sm"
        icon={<Check size={16} />}
        onClick={handleComplete}
        title="Complete"
      >
        Complete
      </Button>

      {task.status === 'pending' && (
        <Button
          variant="ghost"
          size="sm"
          icon={<Play size={16} />}
          onClick={handleStart}
          title="Start"
        >
          Start
        </Button>
      )}

      <Button
        variant="ghost"
        size="sm"
        icon={<Clock size={16} />}
        onClick={handleSnooze}
        title="Snooze 1 hour"
      >
        Snooze
      </Button>

      <Button
        variant="ghost"
        size="sm"
        icon={<X size={16} />}
        onClick={handleDismiss}
        title="Dismiss"
      >
        Dismiss
      </Button>
    </div>
  );
}
