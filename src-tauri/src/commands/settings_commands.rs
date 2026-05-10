use tauri::State;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use crate::commands::task_commands::DbState;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: Option<String>,
    pub hotkeys: Option<serde_json::Value>,
    pub ai_config: Option<serde_json::Value>,
    pub sync_config: Option<serde_json::Value>,
    pub floating_card_opacity: Option<f64>,
}

#[tauri::command]
pub fn get_settings(state: State<DbState>) -> Result<AppSettings, String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    let get_setting = |key: &str| -> Option<String> {
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [key],
            |row| row.get::<_, String>(0),
        ).ok()
    };
    
    Ok(AppSettings {
        theme: get_setting("theme"),
        hotkeys: get_setting("hotkeys").and_then(|s| serde_json::from_str(&s).ok()),
        ai_config: get_setting("ai_api_key").map(|_| {
            serde_json::json!({
                "api_key": get_setting("ai_api_key").unwrap_or_default(),
                "model": get_setting("ai_model_name").unwrap_or_else(|| "qwen-vl-max".to_string()),
                "api_endpoint": get_setting("ai_api_endpoint").unwrap_or_else(|| "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions".to_string()),
                "max_tokens": get_setting("ai_max_tokens").and_then(|s| s.parse().ok()).unwrap_or(4096),
                "temperature": 0.3,
            })
        }),
        sync_config: get_setting("sync_server_url").map(|_| {
            serde_json::json!({
                "server_url": get_setting("sync_server_url").unwrap_or_default(),
                "sync_interval": get_setting("sync_interval").and_then(|s| s.parse().ok()).unwrap_or(30),
                "device_id": get_setting("sync_device_id").unwrap_or_default(),
                "auto_sync": get_setting("sync_auto_sync").and_then(|s| s.parse().ok()).unwrap_or(false),
            })
        }),
        floating_card_opacity: get_setting("floating_card_opacity").and_then(|s| s.parse().ok()),
    })
}

#[tauri::command]
pub fn update_settings(state: State<DbState>, settings: AppSettings) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    if let Some(ref theme) = settings.theme {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('theme', ?1, datetime('now'))",
            [theme],
        ).map_err(|e| format!("Failed to save theme: {}", e))?;
        info!("Theme saved: {}", theme);
    }
    
    if let Some(ref hotkeys) = settings.hotkeys {
        let hotkeys_str = serde_json::to_string(hotkeys).map_err(|e| format!("Failed to serialize hotkeys: {}", e))?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('hotkeys', ?1, datetime('now'))",
            [&hotkeys_str],
        ).map_err(|e| format!("Failed to save hotkeys: {}", e))?;
        info!("Hotkeys saved");
    }
    
    if let Some(ref ai_config) = settings.ai_config {
        if let Some(api_key) = ai_config.get("api_key").and_then(|v| v.as_str()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_api_key', ?1, datetime('now'))",
                [api_key],
            ).map_err(|e| format!("Failed to save AI API key: {}", e))?;
        }
        if let Some(model) = ai_config.get("model").and_then(|v| v.as_str()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_model_name', ?1, datetime('now'))",
                [model],
            ).map_err(|e| format!("Failed to save AI model: {}", e))?;
        }
        if let Some(endpoint) = ai_config.get("api_endpoint").and_then(|v| v.as_str()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_api_endpoint', ?1, datetime('now'))",
                [endpoint],
            ).map_err(|e| format!("Failed to save AI endpoint: {}", e))?;
        }
        if let Some(max_tokens) = ai_config.get("max_tokens").and_then(|v| v.as_i64()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('ai_max_tokens', ?1, datetime('now'))",
                [max_tokens.to_string()],
            ).map_err(|e| format!("Failed to save AI max_tokens: {}", e))?;
        }
        info!("AI config saved");
    }
    
    if let Some(ref sync_config) = settings.sync_config {
        if let Some(server_url) = sync_config.get("server_url").and_then(|v| v.as_str()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('sync_server_url', ?1, datetime('now'))",
                [server_url],
            ).map_err(|e| format!("Failed to save sync server URL: {}", e))?;
        }
        if let Some(interval) = sync_config.get("sync_interval").and_then(|v| v.as_i64()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('sync_interval', ?1, datetime('now'))",
                [interval.to_string()],
            ).map_err(|e| format!("Failed to save sync interval: {}", e))?;
        }
        if let Some(device_id) = sync_config.get("device_id").and_then(|v| v.as_str()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('sync_device_id', ?1, datetime('now'))",
                [device_id],
            ).map_err(|e| format!("Failed to save sync device ID: {}", e))?;
        }
        if let Some(auto_sync) = sync_config.get("auto_sync").and_then(|v| v.as_bool()) {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('sync_auto_sync', ?1, datetime('now'))",
                [auto_sync.to_string()],
            ).map_err(|e| format!("Failed to save sync auto_sync: {}", e))?;
        }
        info!("Sync config saved");
    }
    
    if let Some(opacity) = settings.floating_card_opacity {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('floating_card_opacity', ?1, datetime('now'))",
            [opacity.to_string()],
        ).map_err(|e| format!("Failed to save floating card opacity: {}", e))?;
        info!("Floating card opacity saved: {}", opacity);
    }
    
    info!("All settings saved successfully");
    Ok(())
}
