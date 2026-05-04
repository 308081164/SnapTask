import React, { useEffect, useState } from 'react';
import { useTaskStore } from '@/stores/taskStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { FloatingCard } from '@/components/FloatingCard/FloatingCard';
import type { Task } from '@/types';
import { Window } from '@tauri-apps/api/window';

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

  const handleTaskClick = async (task: Task) => {
    // 点击任务时：打开主窗口并聚焦
    try {
      const main = await Window.getByLabel('main');
      if (main) {
        await main.show();
        await main.setFocus();
      }
    } catch (e) {
      console.error('Failed to open main window:', e);
    }
  };

  const handleClose = async () => {
    try {
      const current = Window.getCurrent();
      await current.hide();
    } catch (e) {
      console.error('Failed to hide floating window:', e);
    }
  };

  return (
    <div style={{ padding: 8 }}>
      <FloatingCard
        tasks={displayTasks.length > 0 ? displayTasks : tasks}
        onTaskClick={handleTaskClick}
        onClose={handleClose}
        opacity={floatingCardOpacity}
      />
    </div>
  );
};

export default FloatingCardPage;
