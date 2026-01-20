import { Activity, Eye, ListTodo, Clock } from 'lucide-react';
import { Card } from '../common';
import { useWatchStore } from '../../stores';
import styles from './StatusDisplay.module.css';

export function StatusDisplay() {
  const { status, isCapturing } = useWatchStore();

  const formatTime = (isoString: string | null) => {
    if (!isoString) return 'Never';
    const date = new Date(isoString);
    return date.toLocaleTimeString();
  };

  return (
    <div className={styles.grid}>
      <Card className={styles.stat}>
        <Eye className={`${styles.icon} ${status.is_watching ? styles.active : ''}`} />
        <div className={styles.statContent}>
          <span className={styles.statValue}>
            {status.is_watching ? 'Active' : 'Inactive'}
          </span>
          <span className={styles.statLabel}>Watch Status</span>
        </div>
        {isCapturing && <span className={styles.capturing}>Capturing...</span>}
      </Card>

      <Card className={styles.stat}>
        <Activity className={styles.icon} />
        <div className={styles.statContent}>
          <span className={styles.statValue}>{status.captures_since_start}</span>
          <span className={styles.statLabel}>Captures</span>
        </div>
      </Card>

      <Card className={styles.stat}>
        <ListTodo className={styles.icon} />
        <div className={styles.statContent}>
          <span className={styles.statValue}>{status.tasks_detected_since_start}</span>
          <span className={styles.statLabel}>Tasks Detected</span>
        </div>
      </Card>

      <Card className={styles.stat}>
        <Clock className={styles.icon} />
        <div className={styles.statContent}>
          <span className={styles.statValue}>{formatTime(status.last_capture_at)}</span>
          <span className={styles.statLabel}>Last Capture</span>
        </div>
      </Card>
    </div>
  );
}
