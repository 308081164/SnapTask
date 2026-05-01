use anyhow::{Context, Result};
use crate::ai::models::*;
use crate::ai::prompts;
use rusqlite::Connection;
use log::{info, error};
/// 分析截屏图片
pub async fn analyze_screenshot(
    image_data: &[u8],
    context: &AnalysisContext,
    config: &AIModelConfig,
) -> Result<AnalysisResult> {
    let start = std::time::Instant::now();
    // 将图片编码为 base64
    let image_base64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        image_data,
    );
    info!("Image encoded to base64, size: {} bytes", image_base64.len());
    // 构建消息列表
    let mut messages = prompts::build_analysis_messages(context);
    messages.push(prompts::build_image_message(&image_base64));
    // 构建请求体
    let request = ChatCompletionRequest {
        model: config.model_name.clone(),
        messages,
        max_tokens: config.max_tokens,
        temperature: 0.3,
    };
    // 创建 HTTP 客户端
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .build()
        .context("Failed to create HTTP client")?;
    // 发送请求
    let response = client
        .post(&config.api_endpoint)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to AI API")?;
    let status = response.status();
    let body = response.text().await
        .context("Failed to read AI API response body")?;
    if !status.is_success() {
        error!("AI API returned error status {}: {}", status, body);
        anyhow::bail!(
            "AI API 请求失败 (HTTP {}): {}",
            status,
            body.chars().take(500).collect::<String>()
        );
    }
    // 解析响应
    let api_response: ChatCompletionResponse = serde_json::from_str(&body)
        .context(format!("Failed to parse AI API response: {}", &body.chars().take(500).collect::<String>()))?;
    let latency_ms = start.elapsed().as_millis() as u64;
    // 提取返回内容
    if api_response.choices.is_empty() {
        anyhow::bail!("AI API returned empty choices");
    }
    let content = &api_response.choices[0].message.content;
    let content_str = match content {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    // 解析 JSON 结果
    let parsed: serde_json::Value = serde_json::from_str(&content_str)
        .context(format!("Failed to parse AI response JSON: {}", content_str.chars().take(500).collect::<String>()))?;
    // 提取任务列表
    let tasks = parse_extracted_tasks(&parsed);
    // 提取 OCR 文本
    let ocr_text = parsed.get("ocr_text")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    // 提取场景类型
    let scene_type = parsed.get("scene_type")
        .and_then(|v| v.as_str())
        .unwrap_or("other")
        .to_string();
    info!(
        "AI analysis completed in {}ms, found {} tasks",
        latency_ms,
        tasks.len()
    );
    Ok(AnalysisResult {
        tasks,
        ocr_text,
        scene_type,
        model_used: api_response.model,
        latency_ms,
    })
}
/// 从 JSON 值中解析提取的任务列表
fn parse_extracted_tasks(parsed: &serde_json::Value) -> Vec<ExtractedTask> {
    let tasks_array = match parsed.get("tasks").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Vec::new(),
    };
    let mut tasks = Vec::new();
    for task_value in tasks_array {
        let title = task_value.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if title.is_empty() {
            continue;
        }
        let task = ExtractedTask {
            title,
            description: task_value.get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            client_name: task_value.get("client_name")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
            deadline: task_value.get("deadline")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
            priority: task_value.get("priority")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            tags: task_value.get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                }),
            confidence: task_value.get("confidence")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.5),
            reasoning: task_value.get("reasoning")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        };
        tasks.push(task);
    }
    tasks
}
/// 从数据库获取 AI 配置
pub fn get_ai_config_from_db(conn: &Connection) -> AIModelConfig {
    let mut config = AIModelConfig::default();
    // 尝试从数据库读取配置
    let api_key = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_api_key'",
        [],
        |row| row.get::<_, String>(0),
    ).ok();
    if let Some(key) = api_key {
        config.api_key = key;
    }
    // 尝试从环境变量读取（作为备选）
    if config.api_key.is_empty() {
        if let Ok(env_key) = std::env::var("DASHSCOPE_API_KEY") {
            config.api_key = env_key;
        }
    }
    let model_name = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_model_name'",
        [],
        |row| row.get::<_, String>(0),
    ).ok();
    if let Some(name) = model_name {
        config.model_name = name;
    }
    let api_endpoint = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_api_endpoint'",
        [],
        |row| row.get::<_, String>(0),
    ).ok();
    if let Some(endpoint) = api_endpoint {
        config.api_endpoint = endpoint;
    }
    let max_tokens = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_max_tokens'",
        [],
        |row| row.get::<_, String>(0),
    ).ok();
    if let Some(tokens) = max_tokens {
        if let Ok(t) = tokens.parse::<u32>() {
            config.max_tokens = t;
        }
    }
    let timeout = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_timeout_secs'",
        [],
        |row| row.get::<_, String>(0),
    ).ok();
    if let Some(t) = timeout {
        if let Ok(secs) = t.parse::<u64>() {
            config.timeout_secs = secs;
        }
    }
    config
}
/// 保存 AI 配置到数据库
pub fn save_ai_config_to_db(conn: &Connection, config: &AIModelConfig) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_api_key', ?1, datetime('now'))",
        rusqlite::params![config.api_key],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_model_name', ?1, datetime('now'))",
        rusqlite::params![config.model_name],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_api_endpoint', ?1, datetime('now'))",
        rusqlite::params![config.api_endpoint],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_max_tokens', ?1, datetime('now'))",
        rusqlite::params![config.max_tokens.to_string()],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_timeout_secs', ?1, datetime('now'))",
        rusqlite::params![config.timeout_secs.to_string()],
    )?;
    Ok(())
}
