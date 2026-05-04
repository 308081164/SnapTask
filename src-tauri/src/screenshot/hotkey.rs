use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent};
use log::{info, error};

/// 注册全局快捷键
/// 使用 Alt+Shift 组合，避免与 Windows 系统热键（Ctrl+Shift+S 语音输入等）及浏览器快捷键冲突
pub fn register_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Alt+Shift+S: 区域截图快捷键
    let area_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyS);
    if let Err(e) = app.global_shortcut().on_shortcut(area_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: area screenshot (Alt+Shift+S)");
        // 通知前端进行区域截图
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "area"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register area screenshot shortcut: {}", e);
    }
    // Alt+Shift+A: 全屏截图快捷键
    let full_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyA);
    if let Err(e) = app.global_shortcut().on_shortcut(full_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: full screenshot (Alt+Shift+A)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "full"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register full screenshot shortcut: {}", e);
    }
    // Alt+Shift+W: 当前窗口截图快捷键
    let window_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyW);
    if let Err(e) = app.global_shortcut().on_shortcut(window_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: window screenshot (Alt+Shift+W)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "window"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register window screenshot shortcut: {}", e);
    }
    info!("Global hotkeys registered successfully");
    Ok(())
}

/// 注销全局快捷键
pub fn unregister_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let area_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyS);
    let full_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyA);
    let window_shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyW);
    let _ = app.global_shortcut().unregister(area_shortcut);
    let _ = app.global_shortcut().unregister(full_shortcut);
    let _ = app.global_shortcut().unregister(window_shortcut);
    info!("Global hotkeys unregistered");
    Ok(())
}
