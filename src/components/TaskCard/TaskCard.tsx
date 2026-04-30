import React from 'react';
import { Clock, Paperclip, User, Folder } from 'lucide-react';
import type { Task } from '@/types';
import { TaskStatus, TaskPriority } from '@/types';
import { PriorityBadge } from '@/components/Common/PriorityBadge';
import { Tag } from '@/components/Common/Tag';
import { formatDate, formatDeadlineCountdown, isOverdue } from '@/utils/date';
import { getPriorityOrder } from '@/utils/priority';
import styles from './TaskCard.module.css';

interface TaskCardProps {
  task: Task;
  onClick: (task: Task) => void;
  onStatusChange: (taskId: string, newStatus: TaskStatus) => void;
  clientName?: string;
  projectName?: string;
}

export const TaskCard: React.FC<TaskCardProps> = ({
  task,
  onClick,
  onStatusChange,
  clientName,
  projectName,
}) => {
  const overdue = task.deadline && isOverdue(task.deadline) && task.status !== TaskStatus.Done;
  const countdown = formatDeadlineCountdown(task.deadline);

  const handleStatusClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    const nextStatus =
      task.status === TaskStatus.Todo
        ? TaskStatus.InProgress
        : task.status === TaskStatus.InProgress
          ? TaskStatus.Done
          : TaskStatus.Todo;
    onStatusChange(task.id, nextStatus);
  };

  return (
    <div
      className={`${styles.card} ${overdue ? styles.overdue : ''}`}
      onClick={() => onClick(task)}
      draggable
      onDragStart={(e) => {
        e.dataTransfer.setData('text/plain', task.id);
        e.dataTransfer.effectAllowed = 'move';
      }}
    >
      <div className={styles.header}>
        <PriorityBadge priority={task.priority} />
        {task.source === 'screenshot' && (
          <span className={styles.sourceBadge}>
            <Paperclip size={10} />
            截屏
          </span>
        )}
      </div>

      <h4 className={styles.title}>{task.title}</h4>

      {task.description && (
        <p className={styles.description}>
          {task.description.length > 100
            ? task.description.substring(0, 100) + '...'
            : task.description}
        </p>
      )}

      <div className={styles.meta}>
        {task.deadline && (
          <span className={`${styles.deadline} ${overdue ? styles.deadlineOverdue : ''}`}>
            <Clock size={12} />
            {formatDate(task.deadline, 'MM/dd')}
            {countdown && <span className={styles.countdown}>{countdown}</span>}
          </span>
        )}
      </div>

      <div className={styles.footer}>
        <div className={styles.tags}>
          {clientName && (
            <span className={styles.metaTag}>
              <User size={11} />
              {clientName}
            </span>
          )}
          {projectName && (
            <span className={styles.metaTag}>
              <Folder size={11} />
              {projectName}
            </span>
          )}
          {task.tags.slice(0, 2).map((tag) => (
            <Tag key={tag} size="sm">{tag}</Tag>
          ))}
          {task.tags.length > 2 && (
            <span className={styles.moreTags}>+{task.tags.length - 2}</span>
          )}
        </div>

        <button
          className={`${styles.statusBtn} ${styles[task.status]}`}
          onClick={handleStatusClick}
          title={
            task.status === TaskStatus.Todo
              ? '标记为进行中'
              : task.status === TaskStatus.InProgress
                ? '标记为完成'
                : '标记为待办'
          }
        >
          {task.status === TaskStatus.Todo && <span className={styles.statusCircle} />}
          {task.status === TaskStatus.InProgress && <span className={styles.statusHalf} />}
          {task.status === TaskStatus.Done && <span className={styles.statusCheck}>&#10003;</span>}
        </button>
      </div>
    </div>
  );
};
