import React, { useState } from 'react';
import type { TaskPriority } from '@/types';
import { useTaskStore } from '@/stores/taskStore';
import { useClientStore } from '@/stores/clientStore';
import { useProjectStore } from '@/stores/projectStore';
import { Button } from '@/components/Common/Button';
import styles from './TaskForm.module.css';

interface TaskFormProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: () => void;
}

export const TaskForm: React.FC<TaskFormProps> = ({ isOpen, onClose, onCreated }) => {
  const { createTask } = useTaskStore();
  const { clients } = useClientStore();
  const { projects } = useProjectStore();

  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [priority, setPriority] = useState<TaskPriority>('medium');
  const [deadline, setDeadline] = useState('');
  const [clientId, setClientId] = useState('');
  const [projectId, setProjectId] = useState('');
  const [tagsInput, setTagsInput] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState('');

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) {
      setError('请输入任务标题');
      return;
    }

    setSubmitting(true);
    setError('');

    try {
      const tags = tagsInput
        .split(',')
        .map((t) => t.trim())
        .filter(Boolean);

      await createTask({
        title: title.trim(),
        description: description.trim(),
        priority,
        deadline: deadline || null,
        client_id: clientId || null,
        project_id: projectId || null,
        tags,
        source: 'manual' as const,
      });

      // Reset form
      setTitle('');
      setDescription('');
      setPriority('medium');
      setDeadline('');
      setClientId('');
      setProjectId('');
      setTagsInput('');
      onCreated();
    } catch (err) {
      setError(err instanceof Error ? err.message : '创建任务失败');
    } finally {
      setSubmitting(false);
    }
  };

  const handleCancel = () => {
    setTitle('');
    setDescription('');
    setPriority('medium');
    setDeadline('');
    setClientId('');
    setProjectId('');
    setTagsInput('');
    setError('');
    onClose();
  };

  return (
    <div className={styles.overlay} onClick={onCancel}>
      <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
        <h2 className={styles.title}>新建任务</h2>

        <form className={styles.form} onSubmit={handleSubmit}>
          <div className={styles.formGroup}>
            <label className={styles.label}>标题 *</label>
            <input
              className={styles.input}
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="输入任务标题"
              autoFocus
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>描述</label>
            <textarea
              className={styles.textarea}
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="输入任务描述（可选）"
              rows={3}
            />
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>优先级</label>
              <select
                className={styles.select}
                value={priority}
                onChange={(e) => setPriority(e.target.value as TaskPriority)}
              >
                <option value="low">低</option>
                <option value="medium">中</option>
                <option value="high">高</option>
                <option value="urgent">紧急</option>
              </select>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>截止日期</label>
              <input
                className={styles.input}
                type="datetime-local"
                value={deadline}
                onChange={(e) => setDeadline(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>客户</label>
              <select
                className={styles.select}
                value={clientId}
                onChange={(e) => setClientId(e.target.value)}
              >
                <option value="">无</option>
                {clients.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.name}
                  </option>
                ))}
              </select>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>项目</label>
              <select
                className={styles.select}
                value={projectId}
                onChange={(e) => setProjectId(e.target.value)}
              >
                <option value="">无</option>
                {projects.map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>标签（逗号分隔）</label>
            <input
              className={styles.input}
              value={tagsInput}
              onChange={(e) => setTagsInput(e.target.value)}
              placeholder="例如: 设计, 前端, 紧急"
            />
          </div>

          {error && <p className={styles.error}>{error}</p>}

          <div className={styles.actions}>
            <Button variant="primary" type="submit" loading={submitting}>
              创建任务
            </Button>
            <Button variant="ghost" type="button" onClick={handleCancel}>
              取消
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
};
