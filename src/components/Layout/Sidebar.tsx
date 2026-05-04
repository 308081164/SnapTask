import React from 'react';
import { NavLink, useLocation } from 'react-router-dom';
import {
  LayoutDashboard,
  Clock,
  Calendar,
  Settings,
  PanelLeftClose,
  PanelLeftOpen,
  RefreshCw,
  Zap,
  StickyNote,
} from 'lucide-react';
import { useSyncStore } from '@/stores/syncStore';
import { floatingCardApi } from '@/lib/tauri';
import styles from './Sidebar.module.css';

interface SidebarProps {
  collapsed: boolean;
  onToggle: () => void;
}

const navItems = [
  { path: '/', label: '任务看板', icon: LayoutDashboard },
  { path: '/timeline', label: '时间线', icon: Clock },
  { path: '/calendar', label: '日历', icon: Calendar },
  { path: '/settings', label: '设置', icon: Settings },
];

export const Sidebar: React.FC<SidebarProps> = ({ collapsed, onToggle }) => {
  const location = useLocation();
  const { syncStatus, triggerSync } = useSyncStore();
  const [floatingVisible, setFloatingVisible] = React.useState(false);

  const handleToggleFloating = async () => {
    try {
      const visible = await floatingCardApi.toggleFloatingCard();
      setFloatingVisible(visible);
    } catch (e) {
      console.error('Failed to toggle floating card:', e);
    }
  };

  return (
    <aside className={`${styles.sidebar} ${collapsed ? styles.collapsed : ''}`}>
      {/* Logo */}
      <div className={styles.logo}>
        <Zap size={24} className={styles.logoIcon} />
        {!collapsed && <span className={styles.logoText}>SnapTask</span>}
      </div>

      {/* Navigation */}
      <nav className={styles.nav}>
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              `${styles.navItem} ${isActive ? styles.active : ''}`
            }
            title={collapsed ? item.label : undefined}
          >
            <item.icon size={20} className={styles.navIcon} />
            {!collapsed && <span className={styles.navLabel}>{item.label}</span>}
          </NavLink>
        ))}
      </nav>

      {/* Bottom Section */}
      <div className={styles.bottom}>
        {/* Sync Status */}
        <div className={styles.syncSection}>
          <button
            className={styles.syncBtn}
            onClick={triggerSync}
            disabled={syncStatus === 'syncing'}
            title={collapsed ? '同步' : undefined}
          >
            <RefreshCw
              size={16}
              className={`${styles.syncIcon} ${syncStatus === 'syncing' ? styles.syncing : ''}`}
            />
            {!collapsed && (
              <span className={styles.syncLabel}>
                {syncStatus === 'syncing' ? '同步中...' : '同步'}
              </span>
            )}
          </button>
          {!collapsed && (
            <span className={`${styles.syncStatus} ${styles[syncStatus]}`}>
              {syncStatus === 'idle' && '已同步'}
              {syncStatus === 'syncing' && '同步中'}
              {syncStatus === 'success' && '同步成功'}
              {syncStatus === 'error' && '同步失败'}
            </span>
          )}
        </div>

        {/* Floating Card Toggle */}
        <button
          className={`${styles.syncBtn} ${floatingVisible ? styles.floatingActive : ''}`}
          onClick={handleToggleFloating}
          title={collapsed ? '悬浮待办' : undefined}
        >
          <StickyNote size={16} className={styles.syncIcon} />
          {!collapsed && (
            <span className={styles.syncLabel}>悬浮待办</span>
          )}
        </button>

        {/* Collapse Toggle */}
        <button className={styles.collapseBtn} onClick={onToggle} title="折叠侧边栏">
          {collapsed ? <PanelLeftOpen size={18} /> : <PanelLeftClose size={18} />}
        </button>
      </div>
    </aside>
  );
};
