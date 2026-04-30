import React, { useState } from 'react';
import { Minus, X, ExternalLink } from 'lucide-react';
import type { Task } from '@/types';
import { TaskStatus, TaskPriority } from '@/types';
import { PriorityBadge } from '@/components/Common/PriorityBadge';
import { formatDate, formatDeadlineCountdown, isOverdue } from '@/utils/date';
import { getPriorityOrder } from '@/utils/priority';
import styles from './FloatingCard.module.css';

interface FloatingCardProps {
  tasks: Task[];
  onTaskClick: (task: Task) => void;
  onMinimize?: () => void;
  onClose?: () => void;
  opacity?: number;
}

export const FloatingCard: React.FC<FloatingCardProps> = ({
  tasks,
  onTaskClick,
  onMinimize,
  onClose,
  opacity = 0.9,
}) => {
  const [isMinimized, setIsMinimized] = useState(false);

  // Group tasks by urgency
  const urgentTasks = tasks
    .filter(
      (t) =>
        t.status !== TaskStatus.Done &&
        (t.priority === TaskPriority.Urgent ||
          t.priority === TaskPriority.High ||
          (t.deadline && isOverdue(t.deadline)))
    )
    .sort((a, b) => getPriorityOrder(b.priority) - getPriorityOrder(a.priority));

  const upcomingTasks = tasks
    .filter(
      (t) =>
        t.status !== TaskStatus.Done &&
        !urgentTasks.find((ut) => ut.id === t.id)
    )
    .sort((a, b) => {
      if (a.deadline && b.deadline) {
        return new Date(a.deadline).getTime() - new Date(b.deadline).getTime();
      }
      return getPriorityOrder(b.priority) - getPriorityOrder(a.priority);
    });

  const handleMinimize = () => {
    setIsMinimized(true);
    onMinimize?.();
  };

  if (isMinimized) {
    return (
      <div
        className={styles.minimized}
        style={{ opacity }}
        onClick={() => setIsMinimized(false)}
        title="点击展开"
      >
        <span className={styles.minimizedCount}>{urgentTasks.length}</span>
      </div>
    );
  }

  return (
    <div className={styles.card} style={{ opacity }}>
      {/* Header */}
      <div className={styles.header}>
        <span className={styles.headerTitle}>待办事项</span>
        <div className={styles.headerActions}>
          <button className={styles.headerBtn} onClick={handleMinimize} title="最小化">
            <Minus size={14} />
          </button>
          {onClose && (
            <button className={styles.headerBtn} onClick={onClose} title="关闭">
              <X size={14} />
            </button>
          )}
        </div>
      </div>

      {/* Task List */}
      <div className={styles.taskList}>
        {urgentTasks.length > 0 && (
          <div className={styles.group}>
            <span className={styles.groupLabel}>紧急</span>
            {urgentTasks.slice(0, 5).map((task) => (
              <TaskItem key={task.id} task={task} onClick={() => onTaskClick(task)} />
            ))}
          </div>
        )}

        {upcomingTasks.length > 0 && (
          <div className={styles.group}>
            <span className={styles.groupLabel}>即将到期</span>
            {upcomingTasks.slice(0, 5).map((task) => (
              <TaskItem key={task.id} task={task} onClick={() => onTaskClick(task)} />
            ))}
          </div>
        )}

        {urgentTasks.length === 0 && upcomingTasks.length === 0 && (
          <div className={styles.empty}>暂无待办任务</div>
        )}
      </div>
    </div>
  );
};

interface TaskItemProps {
  task: Task;
  onClick: (task: Task) => void;
}

const TaskItem: React.FC<TaskItemProps> = ({ task, onClick }) => {
  const overdue = task.deadline && isOverdue(task.deadline);

  return (
    <div className={styles.taskItem} onClick={() => onClick(task)}>
      <div className={styles.taskItemLeft}>
        <PriorityBadge priority={task.priority} />
        <span className={styles.taskItemTitle}>{task.title}</span>
      </div>
      <div className={styles.taskItemRight}>
        {task.deadline && (
          <span className={`${styles.taskDeadline} ${overdue ? styles.overdue : ''}`}>
            {formatDate(task.deadline, 'MM/dd')}
          </span>
        )}
        <ExternalLink size={12} className={styles.taskLink} />
      </div>
    </div>
  );
};
