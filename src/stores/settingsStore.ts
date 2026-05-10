import { create } from 'zustand';
import type { AIConfig, SyncConfig } from '@/types';
import { settingsApi } from '@/lib/tauri';

interface SettingsState {
  theme: 'light' | 'dark';
  hotkeys: Record<string, string>;
  aiConfig: AIConfig;
  syncConfig: SyncConfig;
  floatingCardOpacity: number;
  loading: boolean;
  error: string | null;

  loadSettings: () => Promise<void>;
  updateSettings: (settings: Partial<SettingsState>) => void;
  setTheme: (theme: 'light' | 'dark') => void;
  updateHotkey: (action: string, key: string) => void;
  updateAiConfig: (config: Partial<AIConfig>) => void;
  updateSyncConfig: (config: Partial<SyncConfig>) => void;
  saveAllSettings: () => Promise<void>;
  clearError: () => void;
}

const defaultHotkeys: Record<string, string> = {
  screenshot: 'Ctrl+Numpad1',
  newTask: 'Ctrl+Numpad4',
  search: 'Ctrl+Numpad5',
  toggleSidebar: 'Ctrl+Numpad0',
};

const defaultAiConfig: AIConfig = {
  api_key: '',
  model: 'qwen-vl-max',
  api_endpoint: 'https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions',
  max_tokens: 4096,
  temperature: 0.3,
};

const defaultSyncConfig: SyncConfig = {
  server_url: '',
  sync_interval: 30,
  device_id: '',
  auto_sync: false,
  last_sync_at: null,
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  theme: 'light',
  hotkeys: { ...defaultHotkeys },
  aiConfig: { ...defaultAiConfig },
  syncConfig: { ...defaultSyncConfig },
  floatingCardOpacity: 0.9,
  loading: false,
  error: null,

  loadSettings: async () => {
    set({ loading: true });
    try {
      const settings = await settingsApi.getSettings();
      set({
        theme: (settings.theme as 'light' | 'dark') || 'light',
        hotkeys: (settings.hotkeys as Record<string, string>) || defaultHotkeys,
        aiConfig: (settings.ai_config as AIConfig) || defaultAiConfig,
        syncConfig: (settings.sync_config as SyncConfig) || defaultSyncConfig,
        floatingCardOpacity: (settings.floating_card_opacity as number) || 0.9,
        loading: false,
      });
    } catch (error) {
      console.warn('加载设置失败，使用默认值:', error);
      set({ loading: false });
    }
  },

  updateSettings: (settings) => {
    set(settings);
  },

  setTheme: (theme) => {
    set({ theme });
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('snaptask-theme', theme);
  },

  updateHotkey: (action, key) => {
    set((state) => ({
      hotkeys: { ...state.hotkeys, [action]: key },
    }));
  },

  updateAiConfig: (config) => {
    set((state) => ({
      aiConfig: { ...state.aiConfig, ...config },
    }));
  },

  updateSyncConfig: (config) => {
    set((state) => ({
      syncConfig: { ...state.syncConfig, ...config },
    }));
  },

  saveAllSettings: async () => {
    const state = get();
    try {
      await settingsApi.updateSettings({
        theme: state.theme,
        hotkeys: state.hotkeys,
        ai_config: state.aiConfig,
        sync_config: state.syncConfig,
        floating_card_opacity: state.floatingCardOpacity,
      });
      console.log('设置已保存到数据库');
    } catch (error) {
      console.error('保存设置失败:', error);
      throw error;
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
