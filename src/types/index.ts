// ==================== 枚举类型 ====================

export enum TaskPriority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Urgent = 'urgent',
}

export enum TaskStatus {
  Todo = 'todo',
  InProgress = 'in_progress',
  Done = 'done',
}

export enum SourceType {
  Screenshot = 'screenshot',
  Manual = 'manual',
  Import = 'import',
}

// ==================== 核心实体 ====================

export interface Task {
  id: string;
  title: string;
  description: string;
  priority: TaskPriority;
  status: TaskStatus;
  deadline: string | null; // ISO 8601
  client_id: string | null;
  project_id: string | null;
  tags: string[];
  source: SourceType;
  screenshot_url: string | null;
  created_at: string;
  updated_at: string;
  change_history: ChangeRecord[];
  reminder_id: string | null;
}

export interface Client {
  id: string;
  name: string;
  description: string;
  contact_info: string;
  created_at: string;
  updated_at: string;
}

export interface Project {
  id: string;
  name: string;
  description: string;
  client_id: string | null;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface Reminder {
  id: string;
  task_id: string;
  remind_at: string; // ISO 8601
  is_read: boolean;
  created_at: string;
}

export interface ChangeRecord {
  id: string;
  task_id: string;
  field: string;
  old_value: string;
  new_value: string;
  changed_at: string;
}

// ==================== AI 分析相关 ====================

export interface AnalysisResult {
  id: string;
  extracted_tasks: ExtractedTask[];
  raw_text: string;
  confidence: number; // 0-1
  model_used: string;
  analyzed_at: string;
}

export interface ExtractedTask {
  title: string;
  description: string;
  priority: TaskPriority;
  deadline: string | null;
  client_name: string | null;
  project_name: string | null;
  tags: string[];
  confidence: number; // 0-1
  raw_text_segment: string;
}

export interface AnalysisContext {
  available_clients: Client[];
  available_projects: Project[];
  recent_tasks: Task[];
}

// ==================== 同步相关 ====================

export type SyncStatus = 'idle' | 'syncing' | 'error' | 'success';

export interface SyncConfig {
  server_url: string;
  sync_interval: number; // 分钟
  device_id: string;
  auto_sync: boolean;
  last_sync_at: string | null;
}

// ==================== AI 配置 ====================

export interface AIConfig {
  api_key: string;
  model: string;
  api_endpoint: string;
  max_tokens: number;
  temperature: number;
}

// ==================== 设置 ====================

export interface Settings {
  theme: 'light' | 'dark';
  language: string;
  hotkeys: Record<string, string>;
  ai_config: AIConfig;
  sync_config: SyncConfig;
  floating_card_opacity: number;
  floating_card_position: { x: number; y: number };
}

// ==================== 搜索与筛选 ====================

export interface SearchFilters {
  query: string;
  priority: TaskPriority | null;
  status: TaskStatus | null;
  client_id: string | null;
  project_id: string | null;
  tags: string[];
  date_from: string | null;
  date_to: string | null;
}

// ==================== Tauri 事件 ====================

export interface ScreenshotEvent {
  image_base64: string;
  timestamp: string;
}

export interface ReminderEvent {
  reminder: Reminder;
  task: Task;
}

export interface SyncEvent {
  status: SyncStatus;
  message?: string;
  timestamp: string;
}
