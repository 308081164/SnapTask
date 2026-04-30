import React, { useMemo } from 'react';
import { format, parseISO, isToday, isTomorrow, isThisWeek, startOfDay, differenceInDays } from 'date-fns';
import { zhCN } from 'date-fns/locale';
import type { Task } from '@/types';
import { TaskStatus } from '@/types';
import { PriorityBadge } from '@/components/Common/PriorityBadge';
import { EmptyState } from '@/components/Common/EmptyState';
import { Clock } from 'lucide-react';
import styles from './Timeline.module.css';

interface TimelineProps {
  tasks: Task[];
  onTaskClick: (task: Task) => void;
}

interface TimelineGroup {
  label: string;
  date: Date;
  tasks: Task[];
}

export const Timeline: React.FC<TimelineProps> = ({ tasks, onTaskClick }) => {
  const groups = useMemo(() => {
    const activeTasks = tasks.filter((t) => t.status !== TaskStatus.Done);
    const taskMap = new Map<string, Task[]>();

    activeTasks.forEach((task) => {
      const dateStr = task.deadline
        ? format(parseISO(task.deadline), 'yyyy-MM-dd')
        : 'unscheduled';
      const existing = taskMap.get(dateStr) || [];
      existing.push(task);
      taskMap.set(dateStr, existing);
    });

    const result: TimelineGroup[] = [];

    // Overdue
    const overdue = taskMap
      .get('overdue')
      ?.filter((t) => t.deadline && differenceInDays(parseISO(t.deadline), new Date()) < 0);
    if (overdue && overdue.length > 0) {
      result.push({ label: '已逾期', date: new Date(), tasks: overdue });
    }

    // Today
    const todayTasks = activeTasks.filter(
      (t) => t.deadline && isToday(parseISO(t.deadline))
    );
    if (todayTasks.length > 0) {
      result.push({ label: '今天', date: new Date(), tasks: todayTasks });
    }

    // Tomorrow
    const tomorrowTasks = activeTasks.filter(
      (t) => t.deadline && isTomorrow(parseISO(t.deadline))
    );
    if (tomorrowTasks.length > 0) {
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);
      result.push({ label: '明天', date: tomorrow, tasks: tomorrowTasks });
    }

    // This week
    const weekTasks = activeTasks.filter(
      (t) =>
        t.deadline &&
        isThisWeek(parseISO(t.deadline), { locale: zhCN }) &&
        !isToday(parseISO(t.deadline)) &&
        !isTomorrow(parseISO(t.deadline))
    );
    if (weekTasks.length > 0) {
      result.push({ label: '本周', date: new Date(), tasks: weekTasks });
    }

    // Later
    const laterTasks = activeTasks.filter(
      (t) =>
        t.deadline &&
        !isToday(parseISO(t.deadline)) &&
        !isTomorrow(parseISO(t.deadline)) &&
        !isThisWeek(parseISO(t.deadline), { locale: zhCN })
    );
    if (laterTasks.length > 0) {
      result.push({ label: '更晚', date: new Date(), tasks: laterTasks });
    }

    // Unscheduled
    const unscheduled = taskMap.get('unscheduled');
    if (unscheduled && unscheduled.length > 0) {
      result.push({ label: '未安排', date: new Date(), tasks: unscheduled });
    }

    return result;
  }, [tasks]);

  if (tasks.length === 0) {
    return (
      <EmptyState
        icon={<Clock size={28} />}
        title="暂无任务"
        description="创建你的第一个任务，或通过截屏快速添加"
      />
    );
  }

  return (
    <div className={styles.timeline}>
      {groups.map((group, groupIndex) => (
        <div key={groupIndex} className={styles.group}>
          <div className={styles.groupHeader}>
            <div className={styles.groupLine} />
            <span className={styles.groupLabel}>{group.label}</span>
            <span className={styles.groupCount}>{group.tasks.length}</span>
          </div>

          <div className={styles.taskList}>
            {group.tasks.map((task) => (
              <div
                key={task.id}
                className={styles.taskItem}
                onClick={() => onTaskClick(task)}
              >
                <div className={styles.taskDot} />
                <div className={styles.taskContent}>
                  <div className={styles.taskHeader}>
                    <PriorityBadge priority={task.priority} />
                    {task.deadline && (
                      <span className={styles.taskDate}>
                        {format(parseISO(task.deadline), 'MM/dd HH:mm')}
                      </span>
                    )}
                  </div>
                  <h4 className={styles.taskTitle}>{task.title}</h4>
                  {task.description && (
                    <p className={styles.taskDesc}>
                      {task.description.length > 80
                        ? task.description.substring(0, 80) + '...'
                        : task.description}
                    </p>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
};
