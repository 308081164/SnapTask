import { create } from 'zustand';
import type { Client } from '@/types';
import { clientApi } from '@/lib/tauri';

interface ClientState {
  clients: Client[];
  loading: boolean;
  error: string | null;

  fetchClients: () => Promise<void>;
  createClient: (client: Partial<Client>) => Promise<Client>;
  updateClient: (id: string, updates: Partial<Client>) => Promise<Client>;
  deleteClient: (id: string) => Promise<void>;
  clearError: () => void;
}

export const useClientStore = create<ClientState>((set) => ({
  clients: [],
  loading: false,
  error: null,

  fetchClients: async () => {
    set({ loading: true, error: null });
    try {
      const clients = await clientApi.listClients();
      set({ clients, loading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : '获取客户列表失败';
      set({ error: message, loading: false });
    }
  },

  createClient: async (client) => {
    set({ loading: true, error: null });
    try {
      const newClient = await clientApi.createClient(client);
      set((state) => ({
        clients: [newClient, ...state.clients],
        loading: false,
      }));
      return newClient;
    } catch (error) {
      const message = error instanceof Error ? error.message : '创建客户失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  updateClient: async (id, updates) => {
    set({ loading: true, error: null });
    try {
      const updatedClient = await clientApi.updateClient(id, updates);
      set((state) => ({
        clients: state.clients.map((c) => (c.id === id ? updatedClient : c)),
        loading: false,
      }));
      return updatedClient;
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新客户失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  deleteClient: async (id) => {
    set({ loading: true, error: null });
    try {
      await clientApi.deleteClient(id);
      set((state) => ({
        clients: state.clients.filter((c) => c.id !== id),
        loading: false,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : '删除客户失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
