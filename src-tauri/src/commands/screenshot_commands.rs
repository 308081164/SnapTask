use tauri::{AppHandle, Emitter, State};
use std::sync::{Mutex, Arc};
use crate::screenshot::capture;
use log::{info, error};

/// 截图缓存
pub struct ScreenshotCache(pub Arc<Mutex<Option<Vec<u8>>>>);

/// 触发截屏
#[tauri::command]
pub fn trigger_screenshot(
    app_handle: AppHandle,
    state: State<'_, ScreenshotCache>,
    mode: Option<String>,
) -> Result<String, String> {
    let mode = mode.unwrap_or_else(|| "full".to_string());
    info!("Screenshot triggered with mode: {}", mode);
    
    let result = match mode.as_str() {
        "full" => capture::capture_screen(),
        "area" => capture::capture_screen(),
        "window" => capture::capture_window(),
        _ => {
            return Err(format!("Unknown screenshot mode: {}", mode));
        }
    };
    
    match result {
        Ok(data) => {
            // 缓存截图
            if let Ok(mut cache) = state.0.lock() {
                *cache = Some(data.clone());
            }
            // 编码为 base64
            let base64_str = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &data,
            );
            // emit 事件通知前端
            if let Err(e) = app_handle.emit("screenshot:captured", serde_json::json!({
                "mode": mode,
                "size": data.len(),
                "base64": base64_str,
            })) {
                error!("Failed to emit screenshot:captured event: {}", e);
            }
            info!("Screenshot captured successfully, size: {} bytes", data.len());
            Ok(base64_str)
        }
        Err(e) => {
            error!("Screenshot capture failed: {}", e);
            Err(format!("截屏失败: {}", e))
        }
    }
}

/// 获取最近一次截图的 base64
#[tauri::command]
pub fn get_screenshot(state: State<'_, ScreenshotCache>) -> Result<Option<String>, String> {
    let cache = state.0.lock().map_err(|e| format!("Failed to acquire cache lock: {}", e))?;
    match cache.as_ref() {
        Some(data) => {
            let base64_str = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                data,
            );
            Ok(Some(base64_str))
        }
        None => Ok(None),
    }
}
