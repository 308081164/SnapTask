import React from 'react';
import { TaskPriority } from '@/types';
import { getPriorityColor, getPriorityLabel } from '@/utils/priority';
import styles from './PriorityBadge.module.css';

export interface PriorityBadgeProps {
  priority: TaskPriority;
  size?: 'sm' | 'md';
}

export const PriorityBadge: React.FC<PriorityBadgeProps> = ({ priority, size = 'sm' }) => {
  const color = getPriorityColor(priority);
  const label = getPriorityLabel(priority);

  return (
    <span
      className={`${styles.badge} ${styles[size]}`}
      style={{
        color,
        backgroundColor: `${color}18`,
        borderColor: `${color}30`,
      }}
    >
      <span className={styles.dot} style={{ backgroundColor: color }} />
      {label}
    </span>
  );
};
