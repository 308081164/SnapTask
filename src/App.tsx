import React, { useEffect, useState, useCallback } from 'react';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/Layout/MainLayout';
import TaskBoardPage from '@/pages/TaskBoardPage';
import TimelinePage from '@/pages/TimelinePage';
import CalendarPage from '@/pages/CalendarPage';
import SettingsPage from '@/pages/SettingsPage';
import FloatingCardPage from '@/pages/FloatingCardPage';
import ScreenshotSelectPage from '@/pages/ScreenshotSelectPage';
import { ErrorBoundary } from '@/components/Common/ErrorBoundary';
import { ReminderToast } from '@/components/Reminder/ReminderToast';
import { useTaskStore } from '@/stores/taskStore';
import { useAIStore } from '@/stores/aiStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { registerEventListeners } from '@/lib/tauri';
import { screenshotApi } from '@/lib/tauri';
import { listen } from '@tauri-apps/api/event';
import { initTheme } from '@/utils/theme';
import { TaskStatus } from '@/types';
import type { ScreenshotEvent, ReminderEvent, SyncEvent } from '@/types';

interface ScreenshotTriggerEvent {
  mode: string;
  base64: string;
  size: number;
}

const App: React.FC = () => {
  const [isTauri, setIsTauri] = useState(false);
  const [activeReminder, setActiveReminder] = useState<{
    reminder: import('@/types').Reminder;
    task: import('@/types').Task;
  } | null>(null);

  useEffect(() => {
    initTheme();
  }, []);

  useEffect(() => {
    setIsTauri(!!window.__TAURI__);
  }, []);

  const { loadSettings } = useSettingsStore();
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  useEffect(() => {
    if (!isTauri) return;

    let cleanupFn: (() => void) | undefined;
    const unlisteners: Array<() => void> = [];

    // 监听热键触发的截图事件（来自后端 hotkey.rs）
    listen<ScreenshotTriggerEvent>('screenshot:trigger', async (event) => {
      console.log('Hotkey screenshot triggered:', event.payload);
      try {
        const { analyzeScreenshot } = useAIStore.getState();
        // base64 已经在后端生成，直接使用
        if (event.payload?.base64) {
          await analyzeScreenshot(event.payload.base64);
        } else {
          console.error('No base64 data in screenshot event');
        }
      } catch (error) {
        console.error('Hotkey screenshot failed:', error);
      }
    }).then((unlisten) => {
      unlisteners.push(unlisten);
    });

    // 监听显示区域选择窗口的事件
    listen('screenshot:show-select', async () => {
      console.log('Show screenshot select window');
      try {
        await screenshotApi.showSelectWindow();
      } catch (error) {
        console.error('Failed to show select window:', error);
      }
    }).then((unlisten) => {
      unlisteners.push(unlisten);
    });

    // 监听截图错误事件
    listen<{ error: string }>('screenshot:error', async (event) => {
      console.error('Screenshot error:', event.payload);
      alert(`截图失败: ${event.payload?.error || '未知错误'}`);
    }).then((unlisten) => {
      unlisteners.push(unlisten);
    });

    registerEventListeners({
      screenshot: (event: ScreenshotEvent) => {
        console.log('Screenshot captured via API:', event);
        const { analyzeScreenshot } = useAIStore.getState();
        if (event.image_base64) {
          analyzeScreenshot(event.image_base64);
        }
      },
      reminder: (event: ReminderEvent) => {
        setActiveReminder({
          reminder: event.reminder,
          task: event.task,
        });
      },
      sync: (event: SyncEvent) => {
        console.log('Sync event:', event);
      },
    }).then((cleanup) => {
      cleanupFn = cleanup;
    });

    return () => {
      if (cleanupFn) cleanupFn();
      unlisteners.forEach((fn) => fn());
    };
  }, [isTauri]);

  const handleReminderDismiss = useCallback(() => {
    setActiveReminder(null);
  }, []);

  const handleReminderViewDetail = useCallback(
    (task: import('@/types').Task) => {
      setActiveReminder(null);
      console.log('Navigate to task:', task.id);
    },
    []
  );

  const handleReminderMarkDone = useCallback(
    (taskId: string) => {
      const { updateTaskStatus } = useTaskStore.getState();
      updateTaskStatus(taskId, TaskStatus.Done);
    },
    []
  );

  return (
    <ErrorBoundary>
      <HashRouter>
        <Routes>
          {/* Floating card window (separate Tauri window) */}
          <Route path="/floating" element={<FloatingCardPage />} />
          
          {/* Screenshot select window (separate Tauri window) */}
          <Route path="/screenshot-select" element={<ScreenshotSelectPage />} />

          {/* Main app */}
          <Route path="/" element={<MainLayout />}>
            <Route index element={<TaskBoardPage />} />
            <Route path="timeline" element={<TimelinePage />} />
            <Route path="calendar" element={<CalendarPage />} />
            <Route path="settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </HashRouter>

      {/* Reminder Toast */}
      {activeReminder && (
        <ReminderToast
          reminder={activeReminder.reminder}
          task={activeReminder.task}
          onViewDetail={handleReminderViewDetail}
          onMarkDone={handleReminderMarkDone}
          onDismiss={handleReminderDismiss}
        />
      )}
    </ErrorBoundary>
  );
};

export default App;
