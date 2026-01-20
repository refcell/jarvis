import { useState, useEffect } from 'react';
import { Camera } from 'lucide-react';
import { Card, Button, Input } from '../common';
import { useSettings } from '../../hooks';
import styles from './CaptureSettings.module.css';

export function CaptureSettings() {
  const { settings, updateCaptureInterval } = useSettings();
  const [interval, setInterval] = useState(30);

  useEffect(() => {
    if (settings) {
      setInterval(settings.capture_interval_secs);
    }
  }, [settings]);

  const handleSave = () => {
    updateCaptureInterval(interval);
  };

  return (
    <Card>
      <div className={styles.header}>
        <Camera className={styles.icon} />
        <div>
          <h3 className={styles.title}>Screen Capture</h3>
          <p className={styles.description}>
            Configure how often Jarvis captures and analyzes your screen.
          </p>
        </div>
      </div>

      <div className={styles.field}>
        <Input
          label="Capture Interval (seconds)"
          type="number"
          min={10}
          max={300}
          value={interval}
          onChange={(e) => setInterval(parseInt(e.target.value) || 30)}
        />
        <p className={styles.hint}>
          Minimum: 10 seconds. Lower values use more resources but detect tasks faster.
        </p>
      </div>

      <div className={styles.actions}>
        <Button onClick={handleSave}>Save Settings</Button>
      </div>
    </Card>
  );
}
