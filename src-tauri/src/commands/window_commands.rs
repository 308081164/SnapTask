use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// 打开悬浮待办窗口
#[tauri::command]
pub fn toggle_floating_card(app: AppHandle) -> Result<bool, String> {
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
        // 窗口不存在，创建新窗口
        let window = WebviewWindowBuilder::new(
            &app,
            label,
            WebviewUrl::App("/#/floating".into()),
        )
        .title("SnapTask - 悬浮待办")
        .inner_size(340.0, 500.0)
        .min_inner_size(340.0, 200.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .position(50.0, 50.0)
        .build()
        .map_err(|e| format!("Failed to create floating window: {}", e))?;
        let _ = window.show();
        let _ = window.set_focus();
        Ok(true)
    }
}

/// 显示悬浮待办窗口
#[tauri::command]
pub fn show_floating_card(app: AppHandle) -> Result<(), String> {
    let label = "floating-card";
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let window = WebviewWindowBuilder::new(
            &app,
            label,
            WebviewUrl::App("/#/floating".into()),
        )
        .title("SnapTask - 悬浮待办")
        .inner_size(340.0, 500.0)
        .min_inner_size(340.0, 200.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .position(50.0, 50.0)
        .build()
        .map_err(|e| format!("Failed to create floating window: {}", e))?;
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
