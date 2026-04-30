import React from 'react';
import styles from './Tag.module.css';

export interface TagProps {
  children: React.ReactNode;
  color?: string;
  variant?: 'solid' | 'outline';
  size?: 'sm' | 'md';
  onRemove?: () => void;
}

export const Tag: React.FC<TagProps> = ({
  children,
  color,
  variant = 'solid',
  size = 'sm',
  onRemove,
}) => {
  const style: React.CSSProperties = color
    ? {
        '--tag-color': color,
        '--tag-bg': variant === 'solid' ? `${color}20` : 'transparent',
      } as React.CSSProperties
    : {};

  return (
    <span
      className={`${styles.tag} ${styles[variant]} ${styles[size]}`}
      style={style}
    >
      {children}
      {onRemove && (
        <button className={styles.removeBtn} onClick={onRemove} aria-label="移除">
          x
        </button>
      )}
    </span>
  );
};
