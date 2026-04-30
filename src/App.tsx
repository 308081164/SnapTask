import React, { useEffect, useState, useCallback } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/Layout/MainLayout';
import { TaskBoardPage } from '@/pages/TaskBoardPage';
import { TimelinePage } from '@/pages/TimelinePage';
import { CalendarPage } from '@/pages/CalendarPage';
import { SettingsPage } from '@/pages/SettingsPage';
import { FloatingCardPage } from '@/pages/FloatingCardPage';
import { ErrorBoundary } from '@/components/Common/ErrorBoundary';
import { ReminderToast } from '@/components/Reminder/ReminderToast';
import { useTaskStore } from '@/stores/taskStore';
import { useAIStore } from '@/stores/aiStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { useReminderStore } from '@/stores/reminderStore';
import { registerEventListeners } from '@/lib/tauri';
import { initTheme } from '@/utils/theme';
import type { ScreenshotEvent, ReminderEvent, SyncEvent } from '@/types';

const App: React.FC = () => {
  const [isTauri, setIsTauri] = useState(false);
  const [activeReminder, setActiveReminder] = useState<{
    reminder: import('@/types').Reminder;
    task: import('@/types').Task;
  } | null>(null);

  // Initialize theme
  useEffect(() => {
    initTheme();
  }, []);

  // Check if running in Tauri
  useEffect(() => {
    setIsTauri(!!window.__TAURI__);
  }, []);

  // Load settings
  const { loadSettings } = useSettingsStore();
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Register Tauri event listeners
  useEffect(() => {
    if (!isTauri) return;

    const cleanup = registerEventListeners({
      screenshot: (event: ScreenshotEvent) => {
        const { analyzeScreenshot } = useAIStore.getState();
        analyzeScreenshot(event.image_base64);
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
    });

    return cleanup;
  }, [isTauri]);

  const handleReminderDismiss = useCallback(() => {
    setActiveReminder(null);
  }, []);

  const handleReminderViewDetail = useCallback(
    (task: import('@/types').Task) => {
      setActiveReminder(null);
      // Navigate to task detail - would use router navigation
      console.log('Navigate to task:', task.id);
    },
    []
  );

  const handleReminderMarkDone = useCallback(
    (taskId: string) => {
      const { updateTaskStatus } = useTaskStore.getState();
      updateTaskStatus(taskId, 'done');
    },
    []
  );

  return (
    <ErrorBoundary>
      <BrowserRouter>
        <Routes>
          {/* Floating card window (separate Tauri window) */}
          <Route path="/floating" element={<FloatingCardPage />} />

          {/* Main app */}
          <Route path="/" element={<MainLayout />}>
            <Route index element={<TaskBoardPage />} />
            <Route path="timeline" element={<TimelinePage />} />
            <Route path="calendar" element={<CalendarPage />} />
            <Route path="settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>

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
