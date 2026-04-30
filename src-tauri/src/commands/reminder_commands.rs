use tauri::State;
use crate::db::models::*;
use crate::db::reminder as reminder_db;
use crate::commands::task_commands::DbState;

/// 创建提醒
#[tauri::command]
pub fn create_reminder(state: State<DbState>, input: CreateReminderInput) -> Result<Reminder, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    reminder_db::create_reminder(&conn, &input).map_err(|e| format!("Failed to create reminder: {}", e))
}

/// 列出所有提醒
#[tauri::command]
pub fn list_reminders(state: State<DbState>) -> Result<Vec<Reminder>, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    reminder_db::list_reminders(&conn).map_err(|e| format!("Failed to list reminders: {}", e))
}

/// 更新提醒状态
#[tauri::command]
pub fn update_reminder(state: State<DbState>, id: String, status: Option<String>, reminder_time: Option<String>, message: Option<String>) -> Result<Reminder, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;

    if let Some(new_status) = status {
        reminder_db::update_reminder_status(&conn, &id, &new_status)
            .map_err(|e| format!("Failed to update reminder status: {}", e))?;
    }

    if let Some(new_time) = reminder_time {
        conn.execute(
            "UPDATE reminders SET reminder_time = ?1 WHERE id = ?2",
            rusqlite::params![new_time, id],
        ).map_err(|e| format!("Failed to update reminder time: {}", e))?;
    }

    if let Some(new_message) = message {
        conn.execute(
            "UPDATE reminders SET message = ?1 WHERE id = ?2",
            rusqlite::params![new_message, id],
        ).map_err(|e| format!("Failed to update reminder message: {}", e))?;
    }

    reminder_db::get_reminder(&conn, &id).map_err(|e| format!("Failed to get reminder: {}", e))
}

/// 删除提醒
#[tauri::command]
pub fn delete_reminder(state: State<DbState>, id: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    reminder_db::delete_reminder(&conn, &id).map_err(|e| format!("Failed to delete reminder: {}", e))
}
