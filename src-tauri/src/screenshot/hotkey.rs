use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent};
use log::{info, error};

/// 注册全局快捷键
/// 使用 Ctrl + 数字键盘组合，几乎不与任何系统/应用热键冲突
pub fn register_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Ctrl+Numpad1: 区域截图快捷键
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad1);
    if let Err(e) = app.global_shortcut().on_shortcut(area_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: area screenshot (Ctrl+Numpad1)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "area"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register area screenshot shortcut: {}", e);
    }
    // Ctrl+Numpad2: 全屏截图快捷键
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad2);
    if let Err(e) = app.global_shortcut().on_shortcut(full_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: full screenshot (Ctrl+Numpad2)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "full"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register full screenshot shortcut: {}", e);
    }
    // Ctrl+Numpad3: 当前窗口截图快捷键
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad3);
    if let Err(e) = app.global_shortcut().on_shortcut(window_shortcut, |_app: &AppHandle, _shortcut, _event: ShortcutEvent| {
        info!("Hotkey triggered: window screenshot (Ctrl+Numpad3)");
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
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad1);
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad2);
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Numpad3);
    let _ = app.global_shortcut().unregister(area_shortcut);
    let _ = app.global_shortcut().unregister(full_shortcut);
    let _ = app.global_shortcut().unregister(window_shortcut);
    info!("Global hotkeys unregistered");
    Ok(())
}
