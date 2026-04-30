use tauri::State;
use std::sync::Arc;
use std::sync::Mutex;
use crate::sync::engine::SyncEngine;
use crate::sync::config::SyncConfig;
use crate::commands::task_commands::DbState;
use log::info;

/// 同步引擎的 Tauri State 包装
pub struct SyncEngineState(pub Arc<SyncEngine>);

/// 手动触发同步
#[tauri::command]
pub async fn trigger_sync(state: State<'_, SyncEngineState>) -> Result<(), String> {
    info!("Manual sync triggered");
    state.0.trigger_sync().await
}

/// 获取同步状态
#[tauri::command]
pub fn get_sync_status(state: State<'_, SyncEngineState>) -> Result<serde_json::Value, String> {
    let status = state.0.get_status();
    Ok(serde_json::to_value(&status).unwrap_or_default())
}

/// 获取同步配置
#[tauri::command]
pub fn get_sync_config(state: State<DbState>) -> Result<SyncConfig, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    let config = SyncConfig::from_db(&conn);
    Ok(config)
}

/// 更新同步配置
#[tauri::command]
pub fn update_sync_config(state: State<DbState>, config: SyncConfig) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    config.save_to_db(&conn).map_err(|e| format!("Failed to save sync config: {}", e))
}
