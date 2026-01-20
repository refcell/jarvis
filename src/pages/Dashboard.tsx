import { Header } from '../components/layout';
import { TaskList } from '../components/dashboard';
import { StatusDisplay } from '../components/control';
import { useActiveTasks } from '../hooks';
import styles from './Dashboard.module.css';

export function Dashboard() {
  const { data: tasks, isLoading } = useActiveTasks();

  return (
    <div className={styles.page}>
      <Header title="Dashboard" />
      <div className={styles.content}>
        <StatusDisplay />
        <section className={styles.tasks}>
          <h2 className={styles.sectionTitle}>Active Tasks</h2>
          <TaskList tasks={tasks} isLoading={isLoading} />
        </section>
      </div>
    </div>
  );
}
