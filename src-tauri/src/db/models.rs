use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    None,
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::None
    }
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::None => "none",
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
            TaskPriority::Urgent => "urgent",
        }
    }

    pub fn from_str_value(s: &str) -> Self {
        match s {
            "low" => TaskPriority::Low,
            "medium" => TaskPriority::Medium,
            "high" => TaskPriority::High,
            "urgent" => TaskPriority::Urgent,
            _ => TaskPriority::None,
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Archived,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Archived => "archived",
        }
    }

    pub fn from_str_value(s: &str) -> Self {
        match s {
            "in_progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            "archived" => TaskStatus::Archived,
            _ => TaskStatus::Pending,
        }
    }
}

/// 来源类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Screenshot,
    Manual,
    Import,
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Manual
    }
}

impl SourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceType::Screenshot => "screenshot",
            SourceType::Manual => "manual",
            SourceType::Import => "import",
        }
    }

    pub fn from_str_value(s: &str) -> Self {
        match s {
            "screenshot" => SourceType::Screenshot,
            "import" => SourceType::Import,
            _ => SourceType::Manual,
        }
    }
}

/// 任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub client_id: Option<String>,
    pub project_id: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<String>,       // JSON array string
    pub source_type: String,
    pub source_image: Option<String>, // base64 encoded
    pub ocr_text: Option<String>,
    pub ai_confidence: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub archived_at: Option<String>,
}

/// 客户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub client_id: Option<String>,
    pub color: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 提醒
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub task_id: String,
    pub reminder_time: String,
    pub message: Option<String>,
    pub status: String, // pending, fired, dismissed
    pub created_at: String,
    pub fired_at: Option<String>,
}

/// 变更记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRecord {
    pub id: String,
    pub entity_type: String,  // task, client, project, reminder
    pub entity_id: String,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: String,
}

/// 同步日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncLogEntry {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String, // create, update, delete
    pub data: Option<String>, // JSON serialized entity data
    pub timestamp: String,
    pub synced: bool,
    pub device_id: String,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

/// 创建任务的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub client_id: Option<String>,
    pub project_id: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub source_type: Option<String>,
    pub source_image: Option<String>,
    pub ocr_text: Option<String>,
    pub ai_confidence: Option<f64>,
}

/// 更新任务的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskInput {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub client_id: Option<String>,
    pub project_id: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// 创建客户的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClientInput {
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

/// 更新客户的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClientInput {
    pub id: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

/// 创建项目的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub client_id: Option<String>,
    pub color: Option<String>,
}

/// 更新项目的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub client_id: Option<String>,
    pub color: Option<String>,
    pub status: Option<String>,
}

/// 创建提醒的输入参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReminderInput {
    pub task_id: String,
    pub reminder_time: String,
    pub message: Option<String>,
}

/// 任务列表筛选参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFilter {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub client_id: Option<String>,
    pub project_id: Option<String>,
    pub deadline_start: Option<String>,
    pub deadline_end: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
