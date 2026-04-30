import React from 'react';
import type { Task, TaskStatus } from '@/types';
import { TaskCard } from '@/components/TaskCard/TaskCard';
import { EmptyState } from '@/components/Common/EmptyState';
import styles from './TaskColumn.module.css';

interface TaskColumnProps {
  title: string;
  status: TaskStatus;
  tasks: Task[];
  onTaskClick: (task: Task) => void;
  onStatusChange: (taskId: string, newStatus: TaskStatus) => void;
  onDrop: (taskId: string, newStatus: TaskStatus) => void;
  getClientName: (clientId: string) => string;
  getProjectName: (projectId: string) => string;
}

export const TaskColumn: React.FC<TaskColumnProps> = ({
  title,
  status,
  tasks,
  onTaskClick,
  onStatusChange,
  onDrop,
  getClientName,
  getProjectName,
}) => {
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    const taskId = e.dataTransfer.getData('text/plain');
    if (taskId) {
      onDrop(taskId, status);
    }
  };

  return (
    <div
      className={styles.column}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
    >
      <div className={styles.header}>
        <h3 className={styles.title}>{title}</h3>
        <span className={styles.count}>{tasks.length}</span>
      </div>

      <div className={styles.taskList}>
        {tasks.length === 0 ? (
          <div className={styles.empty}>
            <span className={styles.emptyText}>暂无任务</span>
          </div>
        ) : (
          tasks.map((task) => (
            <TaskCard
              key={task.id}
              task={task}
              onClick={onTaskClick}
              onStatusChange={onStatusChange}
              clientName={task.client_id ? getClientName(task.client_id) : undefined}
              projectName={task.project_id ? getProjectName(task.project_id) : undefined}
            />
          ))
        )}
      </div>
    </div>
  );
};
