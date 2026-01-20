import { useState } from 'react';
import { Header } from '../components/layout';
import { TaskList } from '../components/dashboard';
import { Badge } from '../components/common';
import { useTasks } from '../hooks';
import type { TaskStatus } from '../types';
import styles from './History.module.css';

const statusFilters: { value: TaskStatus | 'all'; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'pending', label: 'Pending' },
  { value: 'in_progress', label: 'In Progress' },
  { value: 'completed', label: 'Completed' },
  { value: 'dismissed', label: 'Dismissed' },
  { value: 'snoozed', label: 'Snoozed' },
];

export function History() {
  const [filter, setFilter] = useState<TaskStatus | 'all'>('all');
  const { data: tasks, isLoading } = useTasks(filter);

  return (
    <div className={styles.page}>
      <Header title="History" />
      <div className={styles.content}>
        <div className={styles.filters}>
          {statusFilters.map((status) => (
            <button
              key={status.value}
              className={`${styles.filterButton} ${filter === status.value ? styles.active : ''}`}
              onClick={() => setFilter(status.value)}
            >
              {status.label}
              {status.value === 'all' && tasks && (
                <Badge variant="default" size="sm">
                  {tasks.length}
                </Badge>
              )}
            </button>
          ))}
        </div>

        <TaskList tasks={tasks} isLoading={isLoading} />
      </div>
    </div>
  );
}
