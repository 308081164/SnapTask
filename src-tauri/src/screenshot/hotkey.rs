use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent};
use log::{info, error};
use crate::screenshot::capture;
use std::sync::Mutex;
use std::sync::Arc;

/// 截图缓存
pub struct ScreenshotCache(pub Arc<Mutex<Option<Vec<u8>>>>);

/// 注册全局快捷键
/// 使用 Ctrl + 数字键盘组合，几乎不与任何系统/应用热键冲突
pub fn register_hotkeys(app: &AppHandle, cache: Arc<Mutex<Option<Vec<u8>>>>) -> Result<(), Box<dyn std::error::Error>> {
    // Ctrl+Numpad1: 区域截图 - 显示选择窗口
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad1);
    if let Err(e) = app.global_shortcut().on_shortcut(area_shortcut, {
        let app_clone = app.clone();
        move |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
            info!("Hotkey triggered: show screenshot select (Ctrl+Numpad1)");
            // 显示区域选择窗口
            if let Some(window) = app_clone.get_webview_window("screenshot-select") {
                if let Err(e) = window.show() {
                    error!("Failed to show screenshot select window: {}", e);
                }
                if let Err(e) = window.set_focus() {
                    error!("Failed to focus screenshot select window: {}", e);
                }
            } else {
                error!("Screenshot select window not found");
            }
        }
    }) {
        error!("Failed to register area screenshot shortcut: {}", e);
    }
    
    // Ctrl+Numpad2: 全屏截图快捷键
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad2);
    if let Err(e) = app.global_shortcut().on_shortcut(full_shortcut, {
        let cache = cache.clone();
        let app_clone = app.clone();
        move |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
            info!("Hotkey triggered: full screenshot (Ctrl+Numpad2)");
            match capture::capture_screen() {
                Ok(data) => {
                    // 缓存截图
                    if let Ok(mut c) = cache.lock() {
                        *c = Some(data.clone());
                    }
                    // 编码为 base64
                    let base64_str = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
                    // 发送到前端
                    if let Err(e) = app_clone.emit("screenshot:trigger", serde_json::json!({
                        "mode": "full",
                        "base64": base64_str,
                        "size": data.len(),
                    })) {
                        error!("Failed to emit screenshot:trigger event: {}", e);
                    }
                    info!("Full screenshot captured: {} bytes", data.len());
                }
                Err(e) => {
                    error!("Failed to capture screenshot: {}", e);
                    if let Err(e) = app_clone.emit("screenshot:error", serde_json::json!({
                        "error": format!("截屏失败: {}", e),
                    })) {
                        error!("Failed to emit screenshot:error event: {}", e);
                    }
                }
            }
        }
    }) {
        error!("Failed to register full screenshot shortcut: {}", e);
    }
    
    // Ctrl+Numpad3: 当前窗口截图快捷键
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad3);
    if let Err(e) = app.global_shortcut().on_shortcut(window_shortcut, {
        let cache = cache.clone();
        let app_clone = app.clone();
        move |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
            info!("Hotkey triggered: window screenshot (Ctrl+Numpad3)");
            match capture::capture_window() {
                Ok(data) => {
                    // 缓存截图
                    if let Ok(mut c) = cache.lock() {
                        *c = Some(data.clone());
                    }
                    // 编码为 base64
                    let base64_str = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
                    // 发送到前端
                    if let Err(e) = app_clone.emit("screenshot:trigger", serde_json::json!({
                        "mode": "window",
                        "base64": base64_str,
                        "size": data.len(),
                    })) {
                        error!("Failed to emit screenshot:trigger event: {}", e);
                    }
                    info!("Window screenshot captured: {} bytes", data.len());
                }
                Err(e) => {
                    error!("Failed to capture screenshot: {}", e);
                    if let Err(e) = app_clone.emit("screenshot:error", serde_json::json!({
                        "error": format!("截屏失败: {}", e),
                    })) {
                        error!("Failed to emit screenshot:error event: {}", e);
                    }
                }
            }
        }
    }) {
        error!("Failed to register window screenshot shortcut: {}", e);
    }
    
    info!("Global hotkeys registered successfully");
    Ok(())
}

/// 注销全局快捷键
pub fn unregister_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad1);
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad2);
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad3);
    let _ = app.global_shortcut().unregister(area_shortcut);
    let _ = app.global_shortcut().unregister(full_shortcut);
    let _ = app.global_shortcut().unregister(window_shortcut);
    info!("Global hotkeys unregistered");
    Ok(())
}
