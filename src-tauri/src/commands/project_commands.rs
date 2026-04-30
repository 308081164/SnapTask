use tauri::State;
use crate::db::models::*;
use crate::db::project as project_db;
use crate::commands::task_commands::DbState;

/// 创建项目
#[tauri::command]
pub fn create_project(state: State<DbState>, input: CreateProjectInput) -> Result<Project, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    project_db::create_project(&conn, &input).map_err(|e| format!("Failed to create project: {}", e))
}

/// 列出所有项目
#[tauri::command]
pub fn list_projects(state: State<DbState>) -> Result<Vec<Project>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    project_db::list_projects(&conn).map_err(|e| format!("Failed to list projects: {}", e))
}

/// 更新项目
#[tauri::command]
pub fn update_project(state: State<DbState>, input: UpdateProjectInput) -> Result<Project, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    project_db::update_project(&conn, &input).map_err(|e| format!("Failed to update project: {}", e))
}

/// 删除项目
#[tauri::command]
pub fn delete_project(state: State<DbState>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    project_db::delete_project(&conn, &id).map_err(|e| format!("Failed to delete project: {}", e))
}
