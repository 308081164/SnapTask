use tauri::{AppHandle, Manager};

/// 打开/关闭悬浮待办窗口
#[tauri::command]
pub async fn toggle_floating_card(app: AppHandle) -> Result<bool, String> {
    let label = "floating-card";
    if let Some(window) = app.get_webview_window(label) {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            Ok(false)
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            Ok(true)
        }
    } else {
        // 从 tauri.conf.json 中读取 floating-card 窗口配置并创建
        let config = app.config().app.windows.iter()
            .find(|w| w.label == label)
            .cloned()
            .ok_or("floating-card window config not found")?;
        let window = tauri::WebviewWindowBuilder::from_config(&app, &config)
            .map_err(|e| format!("Failed to create floating window: {}", e))?
            .build()
            .map_err(|e| format!("Failed to build floating window: {}", e))?;
        let _ = window.show();
        let _ = window.set_focus();
        Ok(true)
    }
}

/// 显示悬浮待办窗口
#[tauri::command]
pub async fn show_floating_card(app: AppHandle) -> Result<(), String> {
    let label = "floating-card";
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let config = app.config().app.windows.iter()
            .find(|w| w.label == label)
            .cloned()
            .ok_or("floating-card window config not found")?;
        let window = tauri::WebviewWindowBuilder::from_config(&app, &config)
            .map_err(|e| format!("Failed to create floating window: {}", e))?
            .build()
            .map_err(|e| format!("Failed to build floating window: {}", e))?;
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

/// 隐藏悬浮待办窗口
#[tauri::command]
pub fn hide_floating_card(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("floating-card") {
        let _ = window.hide();
    }
    Ok(())
}

/// 获取悬浮窗口可见状态
#[tauri::command]
pub fn is_floating_card_visible(app: AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window("floating-card") {
        Ok(window.is_visible().unwrap_or(false))
    } else {
        Ok(false)
    }
}
