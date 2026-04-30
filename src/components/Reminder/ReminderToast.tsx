import React, { useEffect, useState } from 'react';
import { Bell, Check, X } from 'lucide-react';
import type { Reminder, Task } from '@/types';
import { formatDeadlineCountdown } from '@/utils/date';
import { Button } from '@/components/Common/Button';
import styles from './ReminderToast.module.css';

interface ReminderToastProps {
  reminder: Reminder;
  task: Task;
  onViewDetail: (task: Task) => void;
  onMarkDone: (taskId: string) => void;
  onDismiss: () => void;
  autoCloseMs?: number;
}

export const ReminderToast: React.FC<ReminderToastProps> = ({
  reminder,
  task,
  onViewDetail,
  onMarkDone,
  onDismiss,
  autoCloseMs = 10000,
}) => {
  const [countdown, setCountdown] = useState(autoCloseMs / 1000);
  const [isClosing, setIsClosing] = useState(false);

  useEffect(() => {
    const timer = setInterval(() => {
      setCountdown((prev) => {
        if (prev <= 1) {
          clearInterval(timer);
          handleClose();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  const handleClose = () => {
    setIsClosing(true);
    setTimeout(() => onDismiss(), 300);
  };

  const handleViewDetail = () => {
    onViewDetail(task);
    handleClose();
  };

  const handleMarkDone = () => {
    onMarkDone(task.id);
    handleClose();
  };

  return (
    <div className={`${styles.toast} ${isClosing ? styles.closing : ''}`}>
      <div className={styles.progress} style={{ width: `${(countdown / (autoCloseMs / 1000)) * 100}%` }} />

      <div className={styles.content}>
        <div className={styles.icon}>
          <Bell size={18} />
        </div>

        <div className={styles.body}>
          <h4 className={styles.title}>{task.title}</h4>
          <p className={styles.message}>
            {task.deadline && formatDeadlineCountdown(task.deadline)}
          </p>
        </div>

        <button className={styles.closeBtn} onClick={handleClose}>
          <X size={14} />
        </button>
      </div>

      <div className={styles.actions}>
        <Button variant="primary" size="sm" onClick={handleViewDetail}>
          查看详情
        </Button>
        <Button variant="secondary" size="sm" onClick={handleMarkDone} icon={<Check size={12} />}>
          标记完成
        </Button>
      </div>
    </div>
  );
};
