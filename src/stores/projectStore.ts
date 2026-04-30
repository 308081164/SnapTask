import { create } from 'zustand';
import type { Project } from '@/types';
import { projectApi } from '@/lib/tauri';

interface ProjectState {
  projects: Project[];
  loading: boolean;
  error: string | null;

  fetchProjects: () => Promise<void>;
  createProject: (project: Partial<Project>) => Promise<Project>;
  updateProject: (id: string, updates: Partial<Project>) => Promise<Project>;
  deleteProject: (id: string) => Promise<void>;
  clearError: () => void;
}

export const useProjectStore = create<ProjectState>((set) => ({
  projects: [],
  loading: false,
  error: null,

  fetchProjects: async () => {
    set({ loading: true, error: null });
    try {
      const projects = await projectApi.listProjects();
      set({ projects, loading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : '获取项目列表失败';
      set({ error: message, loading: false });
    }
  },

  createProject: async (project) => {
    set({ loading: true, error: null });
    try {
      const newProject = await projectApi.createProject(project);
      set((state) => ({
        projects: [newProject, ...state.projects],
        loading: false,
      }));
      return newProject;
    } catch (error) {
      const message = error instanceof Error ? error.message : '创建项目失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  updateProject: async (id, updates) => {
    set({ loading: true, error: null });
    try {
      const updatedProject = await projectApi.updateProject(id, updates);
      set((state) => ({
        projects: state.projects.map((p) => (p.id === id ? updatedProject : p)),
        loading: false,
      }));
      return updatedProject;
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新项目失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  deleteProject: async (id) => {
    set({ loading: true, error: null });
    try {
      await projectApi.deleteProject(id);
      set((state) => ({
        projects: state.projects.filter((p) => p.id !== id),
        loading: false,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : '删除项目失败';
      set({ error: message, loading: false });
      throw error;
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
