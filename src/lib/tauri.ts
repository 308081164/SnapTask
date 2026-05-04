/**
 * Tauri API 封装层
 * 所有与 Tauri 后端的通信都通过此模块
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import type {
  Task,
  Client,
  Project,
  Reminder,
  AnalysisResult,
  AIConfig,
  SyncConfig,
  SyncStatus,
  ScreenshotEvent,
  ReminderEvent,
  SyncEvent,
  SearchFilters,
} from '@/types';

// ==================== 通用 invoke 封装 ====================

async function invokeCommand<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    console.error(`[Tauri] Command "${cmd}" failed:`, error);
    throw error;
  }
}

// ==================== 任务 API ====================

export const taskApi = {
  createTask(task: Partial<Task>): Promise<Task> {
    return invokeCommand('create_task', { task });
  },

  getTask(id: string): Promise<Task> {
    return invokeCommand('get_task', { id });
  },

  updateTask(id: string, updates: Partial<Task>): Promise<Task> {
    return invokeCommand('update_task', { id, updates });
  },

  deleteTask(id: string): Promise<void> {
    return invokeCommand('delete_task', { id });
  },

  listTasks(): Promise<Task[]> {
    return invokeCommand('list_tasks');
  },

  searchTasks(filters: SearchFilters): Promise<Task[]> {
    return invokeCommand('search_tasks', { filters });
  },

  updateTaskStatus(id: string, status: string): Promise<Task> {
    return invokeCommand('update_task_status', { id, status });
  },

  getUpcomingTasks(days: number): Promise<Task[]> {
    return invokeCommand('get_upcoming_tasks', { days });
  },
};

// ==================== 客户 API ====================

export const clientApi = {
  createClient(client: Partial<Client>): Promise<Client> {
    return invokeCommand('create_client', { client });
  },

  listClients(): Promise<Client[]> {
    return invokeCommand('list_clients');
  },

  updateClient(id: string, updates: Partial<Client>): Promise<Client> {
    return invokeCommand('update_client', { id, updates });
  },

  deleteClient(id: string): Promise<void> {
    return invokeCommand('delete_client', { id });
  },
};

// ==================== 项目 API ====================

export const projectApi = {
  createProject(project: Partial<Project>): Promise<Project> {
    return invokeCommand('create_project', { project });
  },

  listProjects(): Promise<Project[]> {
    return invokeCommand('list_projects');
  },

  updateProject(id: string, updates: Partial<Project>): Promise<Project> {
    return invokeCommand('update_project', { id, updates });
  },

  deleteProject(id: string): Promise<void> {
    return invokeCommand('delete_project', { id });
  },
};

// ==================== AI API ====================

export const aiApi = {
  analyzeScreenshot(imageBase64: string): Promise<AnalysisResult> {
    return invokeCommand('analyze_screenshot', { imageBase64 });
  },

  confirmAnalysis(result: AnalysisResult): Promise<Task[]> {
    return invokeCommand('confirm_analysis', { result });
  },

  getAiConfig(): Promise<AIConfig> {
    return invokeCommand('get_ai_config');
  },

  updateAiConfig(config: Partial<AIConfig>): Promise<AIConfig> {
    return invokeCommand('update_ai_config', { config });
  },
};

// ==================== 提醒 API ====================

export const reminderApi = {
  createReminder(reminder: Partial<Reminder>): Promise<Reminder> {
    return invokeCommand('create_reminder', { reminder });
  },

  listReminders(): Promise<Reminder[]> {
    return invokeCommand('list_reminders');
  },

  updateReminder(id: string, updates: Partial<Reminder>): Promise<Reminder> {
    return invokeCommand('update_reminder', { id, updates });
  },

  deleteReminder(id: string): Promise<void> {
    return invokeCommand('delete_reminder', { id });
  },
};

// ==================== 同步 API ====================

export const syncApi = {
  triggerSync(): Promise<void> {
    return invokeCommand('trigger_sync');
  },

  getSyncStatus(): Promise<SyncStatus> {
    return invokeCommand('get_sync_status');
  },

  getSyncConfig(): Promise<SyncConfig> {
    return invokeCommand('get_sync_config');
  },

  updateSyncConfig(config: Partial<SyncConfig>): Promise<SyncConfig> {
    return invokeCommand('update_sync_config', { config });
  },
};

// ==================== 截屏 API ====================

export const screenshotApi = {
  triggerScreenshot(): Promise<string> {
    return invokeCommand('trigger_screenshot');
  },

  getScreenshot(): Promise<string> {
    return invokeCommand('get_screenshot');
  },
};

// ==================== 设置 API ====================

export const settingsApi = {
  getSettings(): Promise<Record<string, unknown>> {
    return invokeCommand('get_settings');
  },

  updateSettings(settings: Record<string, unknown>): Promise<void> {
    return invokeCommand('update_settings', { settings });
  },
};

// ==================== 悬浮窗口 API ====================

export const floatingCardApi = {
  toggleFloatingCard(): Promise<boolean> {
    return invokeCommand('toggle_floating_card');
  },

  showFloatingCard(): Promise<void> {
    return invokeCommand('show_floating_card');
  },

  hideFloatingCard(): Promise<void> {
    return invokeCommand('hide_floating_card');
  },

  isFloatingCardVisible(): Promise<boolean> {
    return invokeCommand('is_floating_card_visible');
  },
};

// ==================== 事件监听 ====================

export interface EventListeners {
  screenshot?: (event: ScreenshotEvent) => void;
  reminder?: (event: ReminderEvent) => void;
  sync?: (event: SyncEvent) => void;
}

/**
 * 注册 Tauri 事件监听
 * 返回清理函数数组，调用可取消所有监听
 */
export async function registerEventListeners(
  handlers: EventListeners
): Promise<() => void> {
  const unlisteners: UnlistenFn[] = [];

  try {
    if (handlers.screenshot) {
      const unlisten = await listen<ScreenshotEvent>('screenshot-captured', (event) => {
        handlers.screenshot!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.reminder) {
      const unlisten = await listen<ReminderEvent>('reminder-triggered', (event) => {
        handlers.reminder!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.sync) {
      const unlisten = await listen<SyncEvent>('sync-status-changed', (event) => {
        handlers.sync!(event.payload);
      });
      unlisteners.push(unlisten);
    }
  } catch (error) {
    console.warn('[Tauri] Failed to register event listeners (may not be in Tauri environment):', error);
  }

  return () => {
    unlisteners.forEach((unlisten) => unlisten());
  };
}
