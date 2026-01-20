import { Bell, BellOff } from 'lucide-react';
import { Card, Toggle } from '../common';
import { useSettings, useNotifications } from '../../hooks';
import styles from './NotificationSettings.module.css';

export function NotificationSettings() {
  const { settings, toggleNotifications } = useSettings();
  const { notificationsEnabled, checkPermission, requestNotificationPermission } =
    useNotifications();

  const handleToggle = async (enabled: boolean) => {
    if (enabled) {
      const hasPermission = await checkPermission();
      if (!hasPermission) {
        const granted = await requestNotificationPermission();
        if (!granted) return;
      }
    }
    toggleNotifications(enabled);
  };

  return (
    <Card>
      <div className={styles.header}>
        {notificationsEnabled ? (
          <Bell className={styles.icon} />
        ) : (
          <BellOff className={styles.iconOff} />
        )}
        <div>
          <h3 className={styles.title}>Notifications</h3>
          <p className={styles.description}>
            Get notified when new tasks are detected. Notifications are disabled by default.
          </p>
        </div>
      </div>

      <div className={styles.toggleRow}>
        <Toggle
          label="Enable notifications"
          checked={settings?.notifications_enabled ?? false}
          onChange={handleToggle}
        />
      </div>
    </Card>
  );
}
