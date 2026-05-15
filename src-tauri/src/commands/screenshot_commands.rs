use tauri::{AppHandle, Emitter, State, Manager};
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

/// 捕获指定区域的截图
#[tauri::command]
pub fn capture_area(
    app_handle: AppHandle,
    state: State<'_, ScreenshotCache>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<String, String> {
    info!("Area capture triggered: x={}, y={}, w={}, h={}", x, y, width, height);
    
    match capture::capture_area_windows(x, y, width, height) {
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
            if let Err(e) = app_handle.emit("screenshot:trigger", serde_json::json!({
                "mode": "area",
                "size": data.len(),
                "base64": base64_str,
            })) {
                error!("Failed to emit screenshot:trigger event: {}", e);
            }
            info!("Area screenshot captured successfully, size: {} bytes", data.len());
            Ok(base64_str)
        }
        Err(e) => {
            error!("Area screenshot capture failed: {}", e);
            Err(format!("区域截屏失败: {}", e))
        }
    }
}

/// 显示区域选择窗口
#[tauri::command]
pub fn show_select_window(app_handle: AppHandle) -> Result<(), String> {
    info!("Showing screenshot select window");
    if let Some(window) = app_handle.get_webview_window("screenshot-select") {
        window.show().map_err(|e| format!("Failed to show select window: {}", e))?;
        window.set_focus().map_err(|e| format!("Failed to focus select window: {}", e))?;
        Ok(())
    } else {
        Err("Screenshot select window not found".to_string())
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
