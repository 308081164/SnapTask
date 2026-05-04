import { create } from 'zustand';
import type { AnalysisResult, AIConfig, Task } from '@/types';
import { aiApi } from '@/lib/tauri';

interface AIState {
  analyzing: boolean;
  currentResult: AnalysisResult | null;
  aiConfig: AIConfig;
  error: string | null;

  analyzeScreenshot: (imageBase64: string) => Promise<AnalysisResult>;
  confirmAnalysis: (result: AnalysisResult) => Promise<Task[]>;
  loadAiConfig: () => Promise<void>;
  updateAiConfig: (config: Partial<AIConfig>) => Promise<void>;
  clearResult: () => void;
  clearError: () => void;
}

const defaultAiConfig: AIConfig = {
  api_key: '',
  model: 'qwen-vl-max',
  api_endpoint: 'https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions',
  max_tokens: 4096,
  temperature: 0.3,
};

export const useAIStore = create<AIState>((set) => ({
  analyzing: false,
  currentResult: null,
  aiConfig: defaultAiConfig,
  error: null,

  analyzeScreenshot: async (imageBase64) => {
    set({ analyzing: true, error: null });
    try {
      const result = await aiApi.analyzeScreenshot(imageBase64);
      set({ currentResult: result, analyzing: false });
      return result;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'AI 分析失败';
      set({ error: message, analyzing: false });
      throw error;
    }
  },

  confirmAnalysis: async (result) => {
    set({ analyzing: true, error: null });
    try {
      const tasks = await aiApi.confirmAnalysis(result);
      set({ currentResult: null, analyzing: false });
      return tasks;
    } catch (error) {
      const message = error instanceof Error ? error.message : '确认分析结果失败';
      set({ error: message, analyzing: false });
      throw error;
    }
  },

  loadAiConfig: async () => {
    try {
      const config = await aiApi.getAiConfig();
      set({ aiConfig: config });
    } catch (error) {
      console.warn('加载 AI 配置失败，使用默认配置:', error);
    }
  },

  updateAiConfig: async (config) => {
    try {
      const updatedConfig = await aiApi.updateAiConfig(config);
      set({ aiConfig: updatedConfig });
    } catch (error) {
      const message = error instanceof Error ? error.message : '更新 AI 配置失败';
      set({ error: message });
      throw error;
    }
  },

  clearResult: () => {
    set({ currentResult: null });
  },

  clearError: () => {
    set({ error: null });
  },
}));
