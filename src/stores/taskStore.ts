import { create } from 'zustand';
import type { Task, SearchFilters, TaskStatus } from '@/types';
import { taskApi } from '@/lib/tauri';

interface TaskState {
  tasks: Task[];
  loading: boolean;
  error: string | null;
  currentTask: Task | null;
  searchResults: Task[] | null;
  upcomingTasks: Task[];

  fetchTasks: () => Promise<void>;
  createTask: (task: Partial<Task>) => Promise<Task>;
  updateTask: (id: string, updates: Partial<Task>) => Promise<Task>;
  deleteTask: (id: string) => Promise<void>;
  updateTaskStatus: (id: string, status: TaskStatus) => Promise<Task>;
  setCurrentTask: (task: Task | null) => void;
  searchTasks: (filters: SearchFilters) => Promise<Task[]>;
  getUpcomingTasks: (days: number) => Promise<Task[]>;
  clearError: () => void;
}

export const useTaskStore = create<TaskState>((set, get) => ({
  tasks: [],
  loading: false,
  error: null,
  currentTask: null,
  searchResults: null,
  upcomingTasks: [],

  fetchTasks: async () => {
    set({ loading: true, error: null });
    try {
      const tasks = await taskApi.listTasks();
      set({ tasks, loading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : '获取任务列表失败';
      set({ error: message, loading: false });
    }
  },

  createTask: async (task) => {
    set({ loading: true, error: null });
    try {
      const newTask = await taskApi.createTask(task);
      set((state) => ({
        tasks: [newTask, ...state.tasks],
        loading: false,
      }));
      return newTask;
    } catch (error) {
      const message = error instanceof Error ? error.message : '创建任务失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  updateTask: async (id, updates) => {
    set({ loading: true, error: null });
    try {
      const updatedTask = await taskApi.updateTask(id, updates);
      set((state) => ({
        tasks: state.tasks.map((t) => (t.id === id ? updatedTask : t)),
        currentTask: state.currentTask?.id === id ? updatedTask : state.currentTask,
        loading: false,
      }));
      return updatedTask;
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新任务失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  deleteTask: async (id) => {
    set({ loading: true, error: null });
    try {
      await taskApi.deleteTask(id);
      set((state) => ({
        tasks: state.tasks.filter((t) => t.id !== id),
        currentTask: state.currentTask?.id === id ? null : state.currentTask,
        loading: false,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : '删除任务失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  updateTaskStatus: async (id, status) => {
    try {
      const updatedTask = await taskApi.updateTaskStatus(id, status);
      set((state) => ({
        tasks: state.tasks.map((t) => (t.id === id ? updatedTask : t)),
        currentTask: state.currentTask?.id === id ? updatedTask : state.currentTask,
      }));
      return updatedTask;
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新任务状态失败';
      set({ error: message });
      throw error;
    }
  },

  setCurrentTask: (task) => {
    set({ currentTask: task });
  },

  searchTasks: async (filters) => {
    set({ loading: true, error: null });
    try {
      const results = await taskApi.searchTasks(filters);
      set({ searchResults: results, loading: false });
      return results;
    } catch (error) {
      const message = error instanceof Error ? error.message : '搜索失败';
      set({ error: message, loading: false });
      return [];
    }
  },

  getUpcomingTasks: async (days) => {
    try {
      const tasks = await taskApi.getUpcomingTasks(days);
      set({ upcomingTasks: tasks });
      return tasks;
    } catch (error) {
      console.error('获取即将到期任务失败:', error);
      return [];
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
