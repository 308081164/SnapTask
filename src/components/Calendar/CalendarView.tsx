import React, { useState, useMemo } from 'react';
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  addMonths,
  subMonths,
  isSameMonth,
  isSameDay,
  parseISO,
} from 'date-fns';
import { zhCN } from 'date-fns/locale';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import type { Task } from '@/types';
import { TaskStatus } from '@/types';
import styles from './CalendarView.module.css';

interface CalendarViewProps {
  tasks: Task[];
  onTaskClick: (task: Task) => void;
}

export const CalendarView: React.FC<CalendarViewProps> = ({ tasks, onTaskClick }) => {
  const [currentMonth, setCurrentMonth] = useState(new Date());
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);

  const activeTasks = useMemo(
    () => tasks.filter((t) => t.status !== TaskStatus.Done && t.deadline),
    [tasks]
  );

  // Build calendar days
  const calendarDays = useMemo(() => {
    const monthStart = startOfMonth(currentMonth);
    const monthEnd = endOfMonth(currentMonth);
    const calStart = startOfWeek(monthStart, { weekStartsOn: 1, locale: zhCN });
    const calEnd = endOfWeek(monthEnd, { weekStartsOn: 1, locale: zhCN });

    const days: Date[] = [];
    let day = calStart;
    while (day <= calEnd) {
      days.push(day);
      day = addDays(day, 1);
    }
    return days;
  }, [currentMonth]);

  // Group tasks by date
  const tasksByDate = useMemo(() => {
    const map = new Map<string, Task[]>();
    activeTasks.forEach((task) => {
      if (!task.deadline) return;
      const dateKey = format(parseISO(task.deadline), 'yyyy-MM-dd');
      const existing = map.get(dateKey) || [];
      existing.push(task);
      map.set(dateKey, existing);
    });
    return map;
  }, [activeTasks]);

  const getTasksForDate = (date: Date): Task[] => {
    const dateKey = format(date, 'yyyy-MM-dd');
    return tasksByDate.get(dateKey) || [];
  };

  const selectedDateTasks = selectedDate ? getTasksForDate(selectedDate) : [];

  const weekDays = ['一', '二', '三', '四', '五', '六', '日'];

  return (
    <div className={styles.container}>
      <div className={styles.calendarSection}>
        {/* Header */}
        <div className={styles.header}>
          <button
            className={styles.navBtn}
            onClick={() => setCurrentMonth(subMonths(currentMonth, 1))}
          >
            <ChevronLeft size={18} />
          </button>
          <h2 className={styles.monthTitle}>
            {format(currentMonth, 'yyyy年M月', { locale: zhCN })}
          </h2>
          <button
            className={styles.navBtn}
            onClick={() => setCurrentMonth(addMonths(currentMonth, 1))}
          >
            <ChevronRight size={18} />
          </button>
        </div>

        {/* Weekday Headers */}
        <div className={styles.weekHeader}>
          {weekDays.map((day) => (
            <div key={day} className={styles.weekDay}>
              {day}
            </div>
          ))}
        </div>

        {/* Calendar Grid */}
        <div className={styles.grid}>
          {calendarDays.map((day, index) => {
            const dayTasks = getTasksForDate(day);
            const isCurrentMonth = isSameMonth(day, currentMonth);
            const isSelected = selectedDate && isSameDay(day, selectedDate);
            const isTodayDate = isSameDay(day, new Date());

            return (
              <div
                key={index}
                className={`${styles.dayCell} ${!isCurrentMonth ? styles.otherMonth : ''} ${isSelected ? styles.selected : ''} ${isTodayDate ? styles.today : ''}`}
                onClick={() => setSelectedDate(day)}
              >
                <span className={styles.dayNumber}>{format(day, 'd')}</span>
                {dayTasks.length > 0 && (
                  <div className={styles.dayDots}>
                    {dayTasks.slice(0, 3).map((task, i) => (
                      <span
                        key={i}
                        className={styles.dot}
                        style={{
                          backgroundColor:
                            task.priority === 'urgent'
                              ? 'var(--color-danger)'
                              : task.priority === 'high'
                                ? '#f97316'
                                : task.priority === 'medium'
                                  ? '#eab308'
                                  : 'var(--color-success)',
                        }}
                      />
                    ))}
                    {dayTasks.length > 3 && (
                      <span className={styles.moreCount}>+{dayTasks.length - 3}</span>
                    )}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {/* Selected Date Tasks */}
      {selectedDate && (
        <div className={styles.taskPanel}>
          <h3 className={styles.panelTitle}>
            {format(selectedDate, 'M月d日 EEEE', { locale: zhCN })}
          </h3>
          {selectedDateTasks.length === 0 ? (
            <p className={styles.panelEmpty}>当天没有任务</p>
          ) : (
            <div className={styles.panelTaskList}>
              {selectedDateTasks.map((task) => (
                <div
                  key={task.id}
                  className={styles.panelTask}
                  onClick={() => onTaskClick(task)}
                >
                  <div className={styles.panelTaskDot} />
                  <div className={styles.panelTaskContent}>
                    <span className={styles.panelTaskTitle}>{task.title}</span>
                    <span className={styles.panelTaskTime}>
                      {format(parseISO(task.deadline!), 'HH:mm')}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
