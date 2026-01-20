import { Inbox, Eye } from 'lucide-react';
import { Button } from '../common';
import { useScreenCapture } from '../../hooks';
import styles from './EmptyState.module.css';

export function EmptyState() {
  const { hasPermission, requestPermission, startWatching, isWatching } = useScreenCapture();

  return (
    <div className={styles.container}>
      <Inbox className={styles.icon} />
      <h2 className={styles.title}>No tasks yet</h2>
      <p className={styles.description}>
        Start watching your screen to detect actionable tasks. Jarvis will analyze your screen
        content and identify tasks for you.
      </p>
      {!hasPermission ? (
        <Button onClick={() => requestPermission()} icon={<Eye size={18} />}>
          Grant Screen Permission
        </Button>
      ) : !isWatching ? (
        <Button onClick={startWatching} icon={<Eye size={18} />}>
          Start Watching
        </Button>
      ) : (
        <p className={styles.watching}>Watching for tasks...</p>
      )}
    </div>
  );
}
