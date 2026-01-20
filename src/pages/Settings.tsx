import { Header } from '../components/layout';
import {
  LLMProviderConfig,
  NotificationSettings,
  CaptureSettings,
} from '../components/settings';
import styles from './Settings.module.css';

export function Settings() {
  return (
    <div className={styles.page}>
      <Header title="Settings" />
      <div className={styles.content}>
        <div className={styles.sections}>
          <LLMProviderConfig />
          <CaptureSettings />
          <NotificationSettings />
        </div>
      </div>
    </div>
  );
}
