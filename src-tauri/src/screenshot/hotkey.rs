use tauri::{
    AppHandle,
    Emitter,
    Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use log::{info, error};

/// 注册所有全局热键
pub fn register_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Ctrl+Shift+S: 选区截屏
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyS);
    if let Err(e) = app.global_shortcut().on_shortcut(area_shortcut, |_app, _shortcut, _event| {
        info!("Hotkey triggered: area screenshot (Ctrl+Shift+S)");
        // 发送截屏事件到前端
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "area"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register area screenshot shortcut: {}", e);
    }

    // Ctrl+Shift+A: 全屏截屏
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);
    if let Err(e) = app.global_shortcut().on_shortcut(full_shortcut, |_app, _shortcut, _event| {
        info!("Hotkey triggered: full screenshot (Ctrl+Shift+A)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "full"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register full screenshot shortcut: {}", e);
    }

    // Ctrl+Shift+W: 窗口截屏
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyW);
    if let Err(e) = app.global_shortcut().on_shortcut(window_shortcut, |_app, _shortcut, _event| {
        info!("Hotkey triggered: window screenshot (Ctrl+Shift+W)");
        if let Err(e) = _app.emit("screenshot:trigger", serde_json::json!({"mode": "window"})) {
            error!("Failed to emit screenshot:trigger event: {}", e);
        }
    }) {
        error!("Failed to register window screenshot shortcut: {}", e);
    }

    info!("Global hotkeys registered successfully");
    Ok(())
}

/// 注销所有全局热键
pub fn unregister_hotkeys(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let area_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyS);
    let full_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);
    let window_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyW);

    let _ = app.global_shortcut().unregister(area_shortcut);
    let _ = app.global_shortcut().unregister(full_shortcut);
    let _ = app.global_shortcut().unregister(window_shortcut);

    info!("Global hotkeys unregistered");
    Ok(())
}
