import { create } from 'zustand';
import type { Reminder } from '@/types';
import { reminderApi } from '@/lib/tauri';

interface ReminderState {
  reminders: Reminder[];
  activeReminder: Reminder | null;
  loading: boolean;
  error: string | null;

  fetchReminders: () => Promise<void>;
  createReminder: (reminder: Partial<Reminder>) => Promise<Reminder>;
  updateReminder: (id: string, updates: Partial<Reminder>) => Promise<Reminder>;
  deleteReminder: (id: string) => Promise<void>;
  setActiveReminder: (reminder: Reminder | null) => void;
  clearError: () => void;
}

export const useReminderStore = create<ReminderState>((set) => ({
  reminders: [],
  activeReminder: null,
  loading: false,
  error: null,

  fetchReminders: async () => {
    set({ loading: true, error: null });
    try {
      const reminders = await reminderApi.listReminders();
      set({ reminders, loading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : '获取提醒列表失败';
      set({ error: message, loading: false });
    }
  },

  createReminder: async (reminder) => {
    try {
      const newReminder = await reminderApi.createReminder(reminder);
      set((state) => ({
        reminders: [newReminder, ...state.reminders],
      }));
      return newReminder;
    } catch (error) {
      const message = error instanceof Error ? error.message : '创建提醒失败';
      set({ error: message });
      throw error;
    }
  },

  updateReminder: async (id, updates) => {
    try {
      const updatedReminder = await reminderApi.updateReminder(id, updates);
      set((state) => ({
        reminders: state.reminders.map((r) =>
          r.id === id ? updatedReminder : r
        ),
        activeReminder:
          state.activeReminder?.id === id ? updatedReminder : state.activeReminder,
      }));
      return updatedReminder;
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新提醒失败';
      set({ error: message });
      throw error;
    }
  },

  deleteReminder: async (id) => {
    try {
      await reminderApi.deleteReminder(id);
      set((state) => ({
        reminders: state.reminders.filter((r) => r.id !== id),
        activeReminder:
          state.activeReminder?.id === id ? null : state.activeReminder,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : '删除提醒失败';
      set({ error: message });
      throw error;
    }
  },

  setActiveReminder: (reminder) => {
    set({ activeReminder: reminder });
  },

  clearError: () => {
    set({ error: null });
  },
}));
