use serde::{Deserialize, Serialize};

/// AI 分析上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisContext {
    pub current_date: String,
    pub current_time: String,
    pub active_clients: Vec<String>,
    pub active_projects: Vec<String>,
}

/// AI 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub tasks: Vec<ExtractedTask>,
    pub ocr_text: Option<String>,
    pub scene_type: String,
    pub model_used: String,
    pub latency_ms: u64,
}

/// AI 提取的任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTask {
    pub title: String,
    pub description: Option<String>,
    pub client_name: Option<String>,
    pub deadline: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
    pub confidence: f64,
    pub reasoning: Option<String>,
}

/// AI 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelConfig {
    pub api_key: String,
    pub model_name: String,
    pub api_endpoint: String,
    pub max_tokens: u32,
    pub timeout_secs: u64,
}

impl Default for AIModelConfig {
    fn default() -> Self {
        AIModelConfig {
            api_key: String::new(),
            model_name: "qwen2.5-vl-plus".to_string(),
            api_endpoint: "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions".to_string(),
            max_tokens: 4096,
            timeout_secs: 15,
        }
    }
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: serde_json::Value,
}

/// DashScope API 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f64,
}

/// DashScope API 响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
