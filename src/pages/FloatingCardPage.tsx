import React, { useEffect, useState } from 'react';
import { useTaskStore } from '@/stores/taskStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { FloatingCard } from '@/components/FloatingCard/FloatingCard';
import type { Task } from '@/types';

const FloatingCardPage: React.FC = () => {
  const { tasks, fetchTasks, getUpcomingTasks } = useTaskStore();
  const { floatingCardOpacity } = useSettingsStore();
  const [displayTasks, setDisplayTasks] = useState<Task[]>([]);

  useEffect(() => {
    fetchTasks();
    getUpcomingTasks(7).then((upcoming) => {
      setDisplayTasks(upcoming);
    });
  }, [fetchTasks, getUpcomingTasks]);

  const handleTaskClick = (task: Task) => {
    // In Tauri environment, this would open the main window and navigate to the task
    console.log('Open task:', task.id);
    if (window.__TAURI__) {
      // Tauri-specific: open main window with task detail
      // This would use @tauri-apps/api to communicate with the main window
    }
  };

  return (
    <div style={{ padding: 16 }}>
      <FloatingCard
        tasks={displayTasks.length > 0 ? displayTasks : tasks}
        onTaskClick={handleTaskClick}
        opacity={floatingCardOpacity}
      />
    </div>
  );
};

export default FloatingCardPage;
