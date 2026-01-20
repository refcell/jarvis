import { motion, AnimatePresence } from 'framer-motion';
import { TaskCard } from './TaskCard';
import { EmptyState } from './EmptyState';
import { Spinner } from '../common';
import { useTaskStore } from '../../stores';
import type { Task } from '../../types';
import styles from './TaskList.module.css';

interface TaskListProps {
  tasks: Task[] | undefined;
  isLoading: boolean;
}

export function TaskList({ tasks, isLoading }: TaskListProps) {
  const { selectedTaskId, setSelectedTaskId } = useTaskStore();

  if (isLoading) {
    return (
      <div className={styles.loading}>
        <Spinner size="lg" />
        <span>Loading tasks...</span>
      </div>
    );
  }

  if (!tasks || tasks.length === 0) {
    return <EmptyState />;
  }

  return (
    <div className={styles.list}>
      <AnimatePresence mode="popLayout">
        {tasks.map((task) => (
          <motion.div
            key={task.id}
            layout
            exit={{ opacity: 0, scale: 0.95 }}
            transition={{ duration: 0.2 }}
          >
            <TaskCard
              task={task}
              selected={selectedTaskId === task.id}
              onClick={() => setSelectedTaskId(task.id)}
            />
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
