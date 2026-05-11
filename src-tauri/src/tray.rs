use tauri::{
    AppHandle, Manager,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};
use log::{info, error};

pub fn setup_system_tray(app: &AppHandle) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    info!("Setting up system tray...");
    
    // 创建菜单项
    let quit_item = MenuItem::with_id(app, "quit", "退出 (E)", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "显示窗口 (S)", true, None::<&str>)?;
    
    // 创建菜单
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
    
    // 构建托盘图标
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("SnapTask - AI 智能任务管理")
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "quit" => {
                    info!("Quit menu item clicked, exiting application...");
                    app.exit(0);
                }
                "show" => {
                    info!("Show menu item clicked, showing main window...");
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
                    info!("Tray icon left-clicked");
                    if let Some(window) = tray.app_handle().get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                TrayIconEvent::Click { button: MouseButton::Right, button_state: MouseButtonState::Up, .. } => {
                    info!("Tray icon right-clicked");
                    // 右键点击会显示菜单（由 TrayIconBuilder 自动处理）
                }
                _ => {}
            }
        })
        .build(app)?;
    
    info!("System tray setup completed");
    Ok(tray)
}

pub fn cleanup_system_tray(tray: &TrayIcon) {
    info!("Cleaning up system tray...");
    // TrayIcon 会自动清理
}
