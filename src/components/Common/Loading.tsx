import React from 'react';
import styles from './Loading.module.css';

export interface LoadingProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
  fullScreen?: boolean;
}

export const Loading: React.FC<LoadingProps> = ({ size = 'md', text, fullScreen = false }) => {
  const content = (
    <div className={`${styles.container} ${fullScreen ? styles.fullScreen : ''}`}>
      <div className={`${styles.spinner} ${styles[size]}`} />
      {text && <p className={styles.text}>{text}</p>}
    </div>
  );

  if (fullScreen) {
    return <div className={styles.overlay}>{content}</div>;
  }

  return content;
};
