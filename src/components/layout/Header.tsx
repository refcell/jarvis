import { Eye, EyeOff, Loader2 } from 'lucide-react';
import { Button } from '../common';
import { useScreenCapture } from '../../hooks';
import styles from './Header.module.css';

interface HeaderProps {
  title: string;
}

export function Header({ title }: HeaderProps) {
  const {
    hasPermission,
    requestPermission,
    isWatching,
    startWatching,
    stopWatching,
    isCapturing,
    watchStatus,
  } = useScreenCapture();

  const handleWatchToggle = () => {
    if (!hasPermission) {
      requestPermission();
      return;
    }

    if (isWatching) {
      stopWatching();
    } else {
      startWatching();
    }
  };

  return (
    <header className={styles.header}>
      <h1 className={styles.title}>{title}</h1>

      <div className={styles.controls}>
        {isWatching && (
          <div className={styles.stats}>
            <span className={styles.stat}>
              <span className={styles.statValue}>{watchStatus.captures_since_start}</span>
              <span className={styles.statLabel}>captures</span>
            </span>
            <span className={styles.statDivider}>|</span>
            <span className={styles.stat}>
              <span className={styles.statValue}>{watchStatus.tasks_detected_since_start}</span>
              <span className={styles.statLabel}>tasks</span>
            </span>
          </div>
        )}

        <Button
          variant={isWatching ? 'danger' : 'primary'}
          onClick={handleWatchToggle}
          icon={
            isCapturing ? (
              <Loader2 className={styles.spinIcon} />
            ) : isWatching ? (
              <EyeOff size={18} />
            ) : (
              <Eye size={18} />
            )
          }
        >
          {!hasPermission
            ? 'Grant Permission'
            : isWatching
            ? 'Stop Watching'
            : 'Start Watching'}
        </Button>
      </div>
    </header>
  );
}
