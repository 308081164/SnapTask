use tauri::State;
use std::sync::Mutex;
use rusqlite::Connection;
use crate::db::models::*;
use crate::db::task as task_db;
/// 数据库连接的 Tauri State 包装
pub struct DbState(pub Mutex<Connection>);
/// 创建任务
#[tauri::command]
pub fn create_task(state: State<DbState>, input: CreateTaskInput) -> Result<Task, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    task_db::create_task(&conn, &input).map_err(|e| format!("Failed to create task: {}", e))
}
/// 获取单个任务
#[tauri::command]
pub fn get_task(state: State<DbState>, id: String) -> Result<Task, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    task_db::get_task(&conn, &id).map_err(|e| format!("Failed to get task: {}", e))
}
/// 更新任务
#[tauri::command]
pub fn update_task(state: State<DbState>, input: UpdateTaskInput) -> Result<Task, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    task_db::update_task(&conn, &input).map_err(|e| format!("Failed to update task: {}", e))
}
/// 删除任务
#[tauri::command]
pub fn delete_task(state: State<DbState>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    task_db::delete_task(&conn, &id).map_err(|e| format!("Failed to delete task: {}", e))
}
/// 列出任务
#[tauri::command]
pub fn list_tasks(state: State<DbState>, filter: Option<TaskFilter>) -> Result<Vec<Task>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let f = filter.unwrap_or(TaskFilter {
        status: None,
        priority: None,
        client_id: None,
        project_id: None,
        deadline_start: None,
        deadline_end: None,
        search: None,
        limit: None,
        offset: None,
    });
    task_db::list_tasks(&conn, &f).map_err(|e| format!("Failed to list tasks: {}", e))
}
/// 搜索任务
#[tauri::command]
pub fn search_tasks(state: State<DbState>, query: String, limit: Option<i64>) -> Result<Vec<Task>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let l = limit.unwrap_or(50);
    task_db::search_tasks(&conn, &query, l).map_err(|e| format!("Failed to search tasks: {}", e))
}
/// 更新任务状态
#[tauri::command]
pub fn update_task_status(state: State<DbState>, task_id: String, new_status: String) -> Result<Task, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    task_db::update_task_status(&conn, &task_id, &new_status)
        .map_err(|e| format!("Failed to update task status: {}", e))
}
/// 获取即将到期的任务
#[tauri::command]
pub fn get_upcoming_tasks(state: State<DbState>, days: Option<i64>) -> Result<Vec<Task>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let d = days.unwrap_or(7);
    task_db::get_upcoming_tasks(&conn, d).map_err(|e| format!("Failed to get upcoming tasks: {}", e))
}
