import { Eye, EyeOff, Camera } from 'lucide-react';
import { Button, Card } from '../common';
import { useScreenCapture } from '../../hooks';
import styles from './WatchControls.module.css';

export function WatchControls() {
  const {
    hasPermission,
    requestPermission,
    isWatching,
    startWatching,
    stopWatching,
    captureAndAnalyze,
    isCapturing,
  } = useScreenCapture();

  if (!hasPermission) {
    return (
      <Card>
        <div className={styles.permission}>
          <Eye className={styles.icon} />
          <h3 className={styles.title}>Screen Recording Permission Required</h3>
          <p className={styles.description}>
            Jarvis needs permission to capture your screen to detect tasks.
          </p>
          <Button onClick={() => requestPermission()}>Grant Permission</Button>
        </div>
      </Card>
    );
  }

  return (
    <Card>
      <div className={styles.controls}>
        <Button
          variant={isWatching ? 'danger' : 'primary'}
          onClick={isWatching ? stopWatching : startWatching}
          icon={isWatching ? <EyeOff size={18} /> : <Eye size={18} />}
        >
          {isWatching ? 'Stop Watching' : 'Start Watching'}
        </Button>

        <Button
          variant="secondary"
          onClick={() => captureAndAnalyze()}
          loading={isCapturing}
          icon={<Camera size={18} />}
        >
          Capture Now
        </Button>
      </div>
    </Card>
  );
}
