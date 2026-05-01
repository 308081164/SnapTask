use tauri::State;
use crate::ai::models::*;
use crate::ai::engine;
use crate::db::models::*;
use crate::db::task as task_db;
use crate::db::client as client_db;
use crate::db::project as project_db;
use crate::commands::task_commands::DbState;
use log::{info, error};
/// 分析截屏（接收 base64 图片，返回分析结果）
#[tauri::command]
pub async fn analyze_screenshot(
    state: State<'_, DbState>,
    image_base64: String,
) -> Result<AnalysisResult, String> {
    // 解码 base64 图片
    let image_data = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &image_base64,
    ).map_err(|e| format!("Failed to decode base64 image: {}", e))?;
    // 获取 AI 配置和上下文 (block scope to drop conn before .await)
    let (config, context) = {
        let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
        let config = engine::get_ai_config_from_db(&conn);
        if config.api_key.is_empty() {
            return Err("AI API Key 未配置，请在设置中配置 DashScope API Key".to_string());
        }
        // 构建分析上下文
        let now = chrono::Local::now();
        let active_clients = client_db::list_clients(&conn)
            .unwrap_or_default()
            .iter()
            .map(|c| c.name.clone())
            .collect();
        let active_projects = project_db::list_projects(&conn)
            .unwrap_or_default()
            .iter()
            .map(|p| p.name.clone())
            .collect();
        let context = AnalysisContext {
            current_date: now.format("%Y-%m-%d").to_string(),
            current_time: now.format("%H:%M").to_string(),
            active_clients,
            active_projects,
        };
        // conn is dropped here
        (config, context)
    };
    // 调用 AI 分析
    let result = engine::analyze_screenshot(&image_data, &context, &config)
        .await
        .map_err(|e| format!("AI 分析失败: {}", e))?;
    info!("Screenshot analysis completed: {} tasks found", result.tasks.len());
    Ok(result)
}
/// 确认分析结果并创建任务
#[tauri::command]
pub fn confirm_analysis(
    state: State<DbState>,
    tasks: Vec<ExtractedTask>,
    source_image: Option<String>,
    ocr_text: Option<String>,
) -> Result<Vec<Task>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let mut created_tasks = Vec::new();
    for extracted in &tasks {
        // 尝试匹配客户
        let client_id = if let Some(ref client_name) = extracted.client_name {
            match client_db::list_clients(&conn) {
                Ok(clients) => {
                    clients.iter()
                        .find(|c| c.name == *client_name)
                        .map(|c| c.id.clone())
                }
                Err(_) => None,
            }
        } else {
            None
        };
        let input = CreateTaskInput {
            title: extracted.title.clone(),
            description: extracted.description.clone(),
            priority: extracted.priority.clone(),
            client_id,
            project_id: None,
            deadline: extracted.deadline.clone(),
            tags: extracted.tags.clone(),
            source_type: Some("screenshot".to_string()),
            source_image: source_image.clone(),
            ocr_text: ocr_text.clone(),
            ai_confidence: Some(extracted.confidence),
        };
        match task_db::create_task(&conn, &input) {
            Ok(task) => {
                created_tasks.push(task);
            }
            Err(e) => {
                error!("Failed to create task from analysis: {}", e);
            }
        }
    }
    info!("Created {} tasks from AI analysis", created_tasks.len());
    Ok(created_tasks)
}
/// 获取 AI 配置
#[tauri::command]
pub fn get_ai_config(state: State<DbState>) -> Result<AIModelConfig, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let config = engine::get_ai_config_from_db(&conn);
    // 隐藏 API Key 的中间部分
    let mut safe_config = config.clone();
    if !safe_config.api_key.is_empty() {
        let len = safe_config.api_key.len();
        if len > 8 {
            safe_config.api_key = format!("{}...{}", &safe_config.api_key[..4], &safe_config.api_key[len-4..]);
        }
    }
    Ok(safe_config)
}
/// 更新 AI 配置
#[tauri::command]
pub fn update_ai_config(state: State<DbState>, config: AIModelConfig) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    engine::save_ai_config_to_db(&conn, &config).map_err(|e| format!("Failed to save AI config: {}", e))
}
