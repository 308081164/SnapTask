use tauri::State;
use crate::db::models::*;
use crate::db::client as client_db;
use crate::commands::task_commands::DbState;

/// 创建客户
#[tauri::command]
pub fn create_client(state: State<DbState>, input: CreateClientInput) -> Result<Client, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    client_db::create_client(&conn, &input).map_err(|e| format!("Failed to create client: {}", e))
}

/// 列出所有客户
#[tauri::command]
pub fn list_clients(state: State<DbState>) -> Result<Vec<Client>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    client_db::list_clients(&conn).map_err(|e| format!("Failed to list clients: {}", e))
}

/// 更新客户
#[tauri::command]
pub fn update_client(state: State<DbState>, input: UpdateClientInput) -> Result<Client, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    client_db::update_client(&conn, &input).map_err(|e| format!("Failed to update client: {}", e))
}

/// 删除客户
#[tauri::command]
pub fn delete_client(state: State<DbState>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    client_db::delete_client(&conn, &id).map_err(|e| format!("Failed to delete client: {}", e))
}
