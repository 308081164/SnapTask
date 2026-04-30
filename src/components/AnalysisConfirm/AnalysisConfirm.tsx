import React, { useState } from 'react';
import { Check, X, ChevronDown, ChevronUp } from 'lucide-react';
import type { AnalysisResult, ExtractedTask, TaskPriority } from '@/types';
import { PriorityBadge } from '@/components/Common/PriorityBadge';
import { Button } from '@/components/Common/Button';
import styles from './AnalysisConfirm.module.css';

interface AnalysisConfirmProps {
  result: AnalysisResult;
  onConfirm: (result: AnalysisResult) => void;
  onCancel: () => void;
}

export const AnalysisConfirm: React.FC<AnalysisConfirmProps> = ({
  result,
  onConfirm,
  onCancel,
}) => {
  const [editedTasks, setEditedTasks] = useState<ExtractedTask[]>(
    result.extracted_tasks.map((t) => ({ ...t }))
  );
  const [expandedIndex, setExpandedIndex] = useState<number>(0);
  const [submitting, setSubmitting] = useState(false);

  const updateTask = (index: number, updates: Partial<ExtractedTask>) => {
    setEditedTasks((prev) =>
      prev.map((t, i) => (i === index ? { ...t, ...updates } : t))
    );
  };

  const toggleExpand = (index: number) => {
    setExpandedIndex(expandedIndex === index ? -1 : index);
  };

  const handleConfirm = async () => {
    setSubmitting(true);
    try {
      await onConfirm({
        ...result,
        extracted_tasks: editedTasks,
      });
    } finally {
      setSubmitting(false);
    }
  };

  const confidencePercent = Math.round(result.confidence * 100);
  const confidenceColor =
    confidencePercent >= 80 ? 'var(--color-success)' :
    confidencePercent >= 50 ? 'var(--color-warning)' :
    'var(--color-danger)';

  return (
    <div className={styles.overlay} onClick={onCancel}>
      <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
        <div className={styles.header}>
          <h2 className={styles.title}>AI 分析结果</h2>
          <div className={styles.confidence}>
            <span className={styles.confidenceLabel}>置信度</span>
            <span className={styles.confidenceValue} style={{ color: confidenceColor }}>
              {confidencePercent}%
            </span>
          </div>
        </div>

        <p className={styles.subtitle}>
          AI 从截图中提取了 {editedTasks.length} 个任务，请确认或修正以下信息
        </p>

        <div className={styles.taskList}>
          {editedTasks.map((task, index) => (
            <div
              key={index}
              className={`${styles.taskItem} ${expandedIndex === index ? styles.expanded : ''}`}
            >
              <div className={styles.taskHeader} onClick={() => toggleExpand(index)}>
                <div className={styles.taskHeaderLeft}>
                  <span className={styles.taskIndex}>#{index + 1}</span>
                  <span className={styles.taskTitle}>{task.title}</span>
                  <PriorityBadge priority={task.priority} />
                </div>
                <div className={styles.taskHeaderRight}>
                  <span className={styles.taskConfidence}>
                    {Math.round(task.confidence * 100)}%
                  </span>
                  {expandedIndex === index ? (
                    <ChevronUp size={16} />
                  ) : (
                    <ChevronDown size={16} />
                  )}
                </div>
              </div>

              {expandedIndex === index && (
                <div className={styles.taskBody}>
                  <div className={styles.field}>
                    <label className={styles.fieldLabel}>标题</label>
                    <input
                      className={styles.fieldInput}
                      value={task.title}
                      onChange={(e) => updateTask(index, { title: e.target.value })}
                    />
                  </div>

                  <div className={styles.field}>
                    <label className={styles.fieldLabel}>描述</label>
                    <textarea
                      className={styles.fieldTextarea}
                      value={task.description}
                      onChange={(e) => updateTask(index, { description: e.target.value })}
                      rows={3}
                    />
                  </div>

                  <div className={styles.fieldRow}>
                    <div className={styles.field}>
                      <label className={styles.fieldLabel}>优先级</label>
                      <select
                        className={styles.fieldSelect}
                        value={task.priority}
                        onChange={(e) =>
                          updateTask(index, { priority: e.target.value as TaskPriority })
                        }
                      >
                        <option value="low">低</option>
                        <option value="medium">中</option>
                        <option value="high">高</option>
                        <option value="urgent">紧急</option>
                      </select>
                    </div>

                    <div className={styles.field}>
                      <label className={styles.fieldLabel}>截止日期</label>
                      <input
                        className={styles.fieldInput}
                        type="datetime-local"
                        value={task.deadline || ''}
                        onChange={(e) =>
                          updateTask(index, { deadline: e.target.value || null })
                        }
                      />
                    </div>
                  </div>

                  <div className={styles.fieldRow}>
                    <div className={styles.field}>
                      <label className={styles.fieldLabel}>客户</label>
                      <input
                        className={styles.fieldInput}
                        value={task.client_name || ''}
                        onChange={(e) =>
                          updateTask(index, { client_name: e.target.value || null })
                        }
                        placeholder="客户名称"
                      />
                    </div>

                    <div className={styles.field}>
                      <label className={styles.fieldLabel}>项目</label>
                      <input
                        className={styles.fieldInput}
                        value={task.project_name || ''}
                        onChange={(e) =>
                          updateTask(index, { project_name: e.target.value || null })
                        }
                        placeholder="项目名称"
                      />
                    </div>
                  </div>

                  <div className={styles.field}>
                    <label className={styles.fieldLabel}>标签（逗号分隔）</label>
                    <input
                      className={styles.fieldInput}
                      value={task.tags.join(', ')}
                      onChange={(e) =>
                        updateTask(index, {
                          tags: e.target.value
                            .split(',')
                            .map((t) => t.trim())
                            .filter(Boolean),
                        })
                      }
                    />
                  </div>

                  {task.raw_text_segment && (
                    <div className={styles.rawText}>
                      <label className={styles.fieldLabel}>原始文本</label>
                      <p className={styles.rawTextContent}>{task.raw_text_segment}</p>
                    </div>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>

        <div className={styles.footer}>
          <Button variant="ghost" onClick={onCancel}>
            <X size={14} />
            取消
          </Button>
          <Button variant="primary" onClick={handleConfirm} loading={submitting}>
            <Check size={14} />
            确认创建 ({editedTasks.length} 个任务)
          </Button>
        </div>
      </div>
    </div>
  );
};
