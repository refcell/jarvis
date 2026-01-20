import { NavLink, useLocation } from 'react-router-dom';
import { LayoutDashboard, History, Settings, Cpu } from 'lucide-react';
import { motion } from 'framer-motion';
import styles from './Sidebar.module.css';

const navItems = [
  { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard, path: '/' },
  { id: 'history', label: 'History', icon: History, path: '/history' },
  { id: 'settings', label: 'Settings', icon: Settings, path: '/settings' },
];

export function Sidebar() {
  const location = useLocation();

  return (
    <aside className={styles.sidebar}>
      <div className={styles.logo}>
        <Cpu className={styles.logoIcon} />
        <span className={styles.logoText}>Jarvis</span>
      </div>

      <nav className={styles.nav}>
        {navItems.map((item) => {
          const isActive = location.pathname === item.path;
          const Icon = item.icon;

          return (
            <NavLink key={item.id} to={item.path} className={styles.navLink}>
              <motion.div
                className={`${styles.navItem} ${isActive ? styles.active : ''}`}
                whileHover={{ x: 4 }}
                transition={{ duration: 0.15 }}
              >
                <Icon className={styles.navIcon} />
                <span>{item.label}</span>
                {isActive && (
                  <motion.div
                    className={styles.activeIndicator}
                    layoutId="activeNav"
                    transition={{ type: 'spring', stiffness: 500, damping: 30 }}
                  />
                )}
              </motion.div>
            </NavLink>
          );
        })}
      </nav>

      <div className={styles.footer}>
        <span className={styles.version}>v0.1.0</span>
      </div>
    </aside>
  );
}
