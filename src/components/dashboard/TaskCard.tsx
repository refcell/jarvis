import { motion } from 'framer-motion';
import { Clock, Monitor, ChevronRight } from 'lucide-react';
import { Badge } from '../common';
import { TaskActions } from './TaskActions';
import { getPriorityColor, getPriorityLabel } from '../../styles/theme';
import type { Task } from '../../types';
import styles from './TaskCard.module.css';

interface TaskCardProps {
  task: Task;
  onClick?: () => void;
  selected?: boolean;
}

export function TaskCard({ task, onClick, selected = false }: TaskCardProps) {
  const priorityColor = getPriorityColor(task.current_priority);
  const priorityLabel = getPriorityLabel(task.current_priority);
  const age = getTaskAge(task.created_at);

  return (
    <motion.div
      className={`${styles.card} ${selected ? styles.selected : ''}`}
      onClick={onClick}
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      whileHover={{ scale: 1.01 }}
      transition={{ duration: 0.2 }}
    >
      <div className={styles.priorityBar} style={{ backgroundColor: priorityColor }} />

      <div className={styles.content}>
        <div className={styles.header}>
          <h3 className={styles.title}>{task.title}</h3>
          <Badge
            variant={
              task.current_priority >= 0.7
                ? 'danger'
                : task.current_priority >= 0.4
                ? 'warning'
                : 'success'
            }
            size="sm"
          >
            {priorityLabel}
          </Badge>
        </div>

        <p className={styles.description}>{task.description}</p>

        <div className={styles.meta}>
          <span className={styles.metaItem}>
            <Clock size={14} />
            {age}
          </span>
          {task.source_window && (
            <span className={styles.metaItem}>
              <Monitor size={14} />
              {task.source_window}
            </span>
          )}
        </div>

        <div className={styles.actions}>
          <TaskActions task={task} />
        </div>
      </div>

      <ChevronRight className={styles.chevron} />
    </motion.div>
  );
}

function getTaskAge(createdAt: string): string {
  const created = new Date(createdAt);
  const now = new Date();
  const diffMs = now.getTime() - created.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffDays > 0) return `${diffDays}d ago`;
  if (diffHours > 0) return `${diffHours}h ago`;
  if (diffMins > 0) return `${diffMins}m ago`;
  return 'Just now';
}
