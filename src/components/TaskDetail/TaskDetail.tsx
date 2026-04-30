import React, { useState, useEffect } from 'react';
import { X, Trash2, Calendar, User, Folder, Tag, Clock, Paperclip } from 'lucide-react';
import { TaskStatus, TaskPriority } from '@/types';
import type { Task } from '@/types';
import { useTaskStore } from '@/stores/taskStore';
import { useClientStore } from '@/stores/clientStore';
import { useProjectStore } from '@/stores/projectStore';
import { PriorityBadge } from '@/components/Common/PriorityBadge';
import { Tag as TagComponent } from '@/components/Common/Tag';
import { Button } from '@/components/Common/Button';
import { formatDate, formatRelative, formatDeadlineCountdown, isOverdue } from '@/utils/date';
import styles from './TaskDetail.module.css';

interface TaskDetailProps {
  task: Task | null;
  isOpen: boolean;
  onClose: () => void;
  onUpdated: () => void;
}

export const TaskDetail: React.FC<TaskDetailProps> = ({
  task, isOpen, onClose, onUpdated,
}) => {
  const { updateTask, deleteTask } = useTaskStore();
  const { clients } = useClientStore();
  const { projects } = useProjectStore();
  const [isEditing, setIsEditing] = useState(false);
  const [editTitle, setEditTitle] = useState('');
  const [editDescription, setEditDescription] = useState('');
  const [editPriority, setEditPriority] = useState<TaskPriority>(TaskPriority.Medium);
  const [editDeadline, setEditDeadline] = useState('');
  const [editClientId, setEditClientId] = useState('');
  const [editProjectId, setEditProjectId] = useState('');
  const [editTags, setEditTags] = useState('');
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  useEffect(() => {
    if (task) {
      setEditTitle(task.title);
      setEditDescription(task.description);
      setEditPriority(task.priority);
      setEditDeadline(task.deadline || '');
      setEditClientId(task.client_id || '');
      setEditProjectId(task.project_id || '');
      setEditTags(task.tags.join(', '));
      setIsEditing(false);
      setShowDeleteConfirm(false);
    }
  }, [task]);

  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    document.addEventListener('keydown', handleEsc);
    return () => document.removeEventListener('keydown', handleEsc);
  }, [isOpen, onClose]);

  if (!isOpen || !task) return null;

  const client = clients.find((c) => c.id === task.client_id);
  const project = projects.find((p) => p.id === task.project_id);
  const overdue = task.deadline && isOverdue(task.deadline) && task.status !== TaskStatus.Done;

  const handleSave = async () => {
    try {
      await updateTask(task.id, {
        title: editTitle,
        description: editDescription,
        priority: editPriority,
        deadline: editDeadline || null,
        client_id: editClientId || null,
        project_id: editProjectId || null,
        tags: editTags
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean),
      });
      setIsEditing(false);
      onUpdated();
    } catch (error) {
      console.error('保存失败:', error);
    }
  };

  const handleDelete = async () => {
    try {
      await deleteTask(task.id);
      setShowDeleteConfirm(false);
      onClose();
      onUpdated();
    } catch (error) {
      console.error('删除失败:', error);
    }
  };

  const handleStatusChange = async (status: TaskStatus) => {
    try {
      await updateTask(task.id, { status });
      onUpdated();
    } catch (error) {
      console.error('状态更新失败:', error);
    }
  };

  return (
    <>
      <div className={styles.overlay} onClick={onClose} />
      <div className={styles.drawer}>
        <div className={styles.header}>
          <div className={styles.headerLeft}>
            <PriorityBadge priority={task.priority} size="md" />
          </div>
          <button className={styles.closeBtn} onClick={onClose}>
            <X size={20} />
          </button>
        </div>

        <div className={styles.content}>
          {isEditing ? (
            <div className={styles.editForm}>
              <div className={styles.formGroup}>
                <label className={styles.label}>标题</label>
                <input className={styles.input} value={editTitle} onChange={(e) => setEditTitle(e.target.value)} placeholder="任务标题" />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>描述</label>
                <textarea className={styles.textarea} value={editDescription} onChange={(e) => setEditDescription(e.target.value)} placeholder="任务描述" rows={4} />
              </div>
              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>优先级</label>
                  <select className={styles.select} value={editPriority} onChange={(e) => setEditPriority(e.target.value as TaskPriority)}>
                    <option value="low">低</option>
                    <option value="medium">中</option>
                    <option value="high">高</option>
                    <option value="urgent">紧急</option>
                  </select>
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.label}>截止日期</label>
                  <input className={styles.input} type="datetime-local" value={editDeadline} onChange={(e) => setEditDeadline(e.target.value)} />
                </div>
              </div>
              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>客户</label>
                  <select className={styles.select} value={editClientId} onChange={(e) => setEditClientId(e.target.value)}>
                    <option value="">无</option>
                    {clients.map((c) => (<option key={c.id} value={c.id}>{c.name}</option>))}
                  </select>
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.label}>项目</label>
                  <select className={styles.select} value={editProjectId} onChange={(e) => setEditProjectId(e.target.value)}>
                    <option value="">无</option>
                    {projects.map((p) => (<option key={p.id} value={p.id}>{p.name}</option>))}
                  </select>
                </div>
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>标签（逗号分隔）</label>
                <input className={styles.input} value={editTags} onChange={(e) => setEditTags(e.target.value)} placeholder="标签1, 标签2" />
              </div>
              <div className={styles.formActions}>
                <Button variant="primary" onClick={handleSave}>保存</Button>
                <Button variant="ghost" onClick={() => setIsEditing(false)}>取消</Button>
              </div>
            </div>
          ) : (
            <div className={styles.viewMode}>
              <h2 className={styles.title}>{task.title}</h2>
              {task.description && <p className={styles.description}>{task.description}</p>}
              <div className={styles.statusSection}>
                <span className={styles.sectionLabel}>状态</span>
                <div className={styles.statusButtons}>
                  {([TaskStatus.Todo, TaskStatus.InProgress, TaskStatus.Done] as TaskStatus[]).map((status) => (
                    <button key={status} className={`${styles.statusBtn} ${task.status === status ? styles.statusActive : ''}`} onClick={() => handleStatusChange(status)}>
                      {status === TaskStatus.Todo && '待办'}
                      {status === TaskStatus.InProgress && '进行中'}
                      {status === TaskStatus.Done && '已完成'}
                    </button>
                  ))}
                </div>
              </div>
              <div className={styles.metaGrid}>
                {task.deadline && (
                  <div className={`${styles.metaItem} ${overdue ? styles.metaOverdue : ''}`}>
                    <Calendar size={14} />
                    <div>
                      <span className={styles.metaLabel}>截止日期</span>
                      <span className={styles.metaValue}>{formatDate(task.deadline)}
                        {formatDeadlineCountdown(task.deadline) && <span className={styles.countdown}>{formatDeadlineCountdown(task.deadline)}</span>}
                      </span>
                    </div>
                  </div>
                )}
                {client && (
                  <div className={styles.metaItem}>
                    <User size={14} />
                    <div>
                      <span className={styles.metaLabel}>客户</span>
                      <span className={styles.metaValue}>{client.name}</span>
                    </div>
                  </div>
                )}
                {project && (
                  <div className={styles.metaItem}>
                    <Folder size={14} />
                    <div>
                      <span className={styles.metaLabel}>项目</span>
                      <span className={styles.metaValue}>{project.name}</span>
                    </div>
                  </div>
                )}
                <div className={styles.metaItem}>
                  <Clock size={14} />
                  <div>
                    <span className={styles.metaLabel}>创建时间</span>
                    <span className={styles.metaValue}>{formatRelative(task.created_at)}</span>
                  </div>
                </div>
              </div>
              {task.tags.length > 0 && (
                <div className={styles.tagsSection}>
                  <span className={styles.sectionLabel}>标签</span>
                  <div className={styles.tagsList}>
                    {task.tags.map((tag) => (<TagComponent key={tag} size="md">{tag}</TagComponent>))}
                  </div>
                </div>
              )}
              {task.change_history && task.change_history.length > 0 && (
                <div className={styles.historySection}>
                  <span className={styles.sectionLabel}>变更历史</span>
                  <div className={styles.historyList}>
                    {task.change_history.map((record) => (
                      <div key={record.id} className={styles.historyItem}>
                        <span className={styles.historyField}>{record.field}</span>
                        <span className={styles.historyChange}>
                          <span className={styles.oldValue}>{record.old_value || '(空)'}</span>
                          <span className={styles.arrow}>&rarr;</span>
                          <span className={styles.newValue}>{record.new_value || '(空)'}</span>
                        </span>
                        <span className={styles.historyTime}>{formatRelative(record.changed_at)}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
        {!isEditing && (
          <div className={styles.footer}>
            <Button variant="secondary" onClick={() => setIsEditing(true)}>编辑</Button>
            {showDeleteConfirm ? (
              <div className={styles.deleteConfirm}>
                <span className={styles.deleteText}>确认删除？</span>
                <Button variant="danger" size="sm" onClick={handleDelete}>删除</Button>
                <Button variant="ghost" size="sm" onClick={() => setShowDeleteConfirm(false)}>取消</Button>
              </div>
            ) : (
              <Button variant="ghost" onClick={() => setShowDeleteConfirm(true)}><Trash2 size={14} />删除</Button>
            )}
          </div>
        )}
      </div>
    </>
  );
};
