import { create } from 'zustand';
import type { SyncStatus, SyncConfig } from '@/types';
import { syncApi } from '@/lib/tauri';

interface SyncState {
  syncStatus: SyncStatus;
  lastSyncAt: Date | null;
  syncConfig: SyncConfig;
  error: string | null;

  triggerSync: () => Promise<void>;
  getSyncStatus: () => Promise<void>;
  loadSyncConfig: () => Promise<void>;
  updateSyncConfig: (config: Partial<SyncConfig>) => Promise<void>;
  clearError: () => void;
}

const defaultSyncConfig: SyncConfig = {
  server_url: '',
  sync_interval: 30,
  device_id: '',
  auto_sync: false,
  last_sync_at: null,
};

export const useSyncStore = create<SyncState>((set) => ({
  syncStatus: 'idle',
  lastSyncAt: null,
  syncConfig: defaultSyncConfig,
  error: null,

  triggerSync: async () => {
    set({ syncStatus: 'syncing', error: null });
    try {
      await syncApi.triggerSync();
      const now = new Date();
      set({ syncStatus: 'success', lastSyncAt: now });
      // 3秒后恢复idle状态
      setTimeout(() => {
        set((state) => {
          if (state.syncStatus === 'success') return { syncStatus: 'idle' };
          return {};
        });
      }, 3000);
    } catch (error) {
      const message = error instanceof Error ? error.message : '同步失败';
      set({ syncStatus: 'error', error: message });
    }
  },

  getSyncStatus: async () => {
    try {
      const status = await syncApi.getSyncStatus();
      set({ syncStatus: status });
    } catch (error) {
      console.warn('获取同步状态失败:', error);
    }
  },

  loadSyncConfig: async () => {
    try {
      const config = await syncApi.getSyncConfig();
      set({ syncConfig: config });
    } catch (error) {
      console.warn('加载同步配置失败，使用默认配置:', error);
    }
  },

  updateSyncConfig: async (config) => {
    try {
      const updatedConfig = await syncApi.updateSyncConfig(config);
      set({ syncConfig: updatedConfig });
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新同步配置失败';
      set({ error: message });
      throw error;
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
