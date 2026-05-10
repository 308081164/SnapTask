pub mod db;
pub mod screenshot;
pub mod ai;
pub mod reminder;
pub mod sync;
pub mod commands;
use std::sync::{Arc, Mutex};
use tauri::Listener;
use rusqlite::Connection;
use log::{info, error, warn};
use commands::task_commands::{DbState, create_task, get_task, update_task, delete_task, list_tasks, search_tasks, update_task_status, get_upcoming_tasks};
use commands::client_commands::{create_client, list_clients, update_client, delete_client};
use commands::project_commands::{create_project, list_projects, update_project, delete_project};
use commands::ai_commands::{analyze_screenshot, confirm_analysis, get_ai_config, update_ai_config};
use commands::reminder_commands::{create_reminder, list_reminders, update_reminder, delete_reminder};
use commands::sync_commands::{SyncEngineState, trigger_sync, get_sync_status, get_sync_config, update_sync_config};
use commands::screenshot_commands::{ScreenshotCache, trigger_screenshot, get_screenshot};
use commands::window_commands::{toggle_floating_card, show_floating_card, hide_floating_card, is_floating_card_visible};
use commands::settings_commands::{get_settings, update_settings};
use sync::engine::SyncEngine;
use reminder::scheduler::ReminderScheduler;

/// Tauri 应用入口
pub fn run() {
    // 初始化日志（使用 try_init 避免与其他 logger 冲突导致 panic）
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();
    info!("SnapTask starting...");
    
    // 初始化数据库
    let db_path = db::init::get_database_path();
    info!("Database path: {}", db_path);
    let conn = match Connection::open(&db_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to open database at {}: {}", db_path, e);
            // 尝试创建目录后重试
            if let Some(parent) = std::path::Path::new(&db_path).parent() {
                if let Err(e2) = std::fs::create_dir_all(parent) {
                    error!("Failed to create database directory: {}", e2);
                }
            }
            match Connection::open(&db_path) {
                Ok(c) => c,
                Err(e2) => {
                    error!("Failed to open database after retry: {}", e2);
                    // 使用内存数据库作为后备
                    warn!("Falling back to in-memory database");
                    Connection::open_in_memory().expect("Failed to open in-memory database")
                }
            }
        }
    };
    if let Err(e) = db::init::init_database(&conn) {
        error!("Failed to initialize database: {}", e);
    }
    info!("Database initialized successfully");
    
    // 创建同步引擎
    let sync_engine = Arc::new(SyncEngine::new());
    sync_engine.load_config(&conn);
    
    // 创建提醒调度器
    let reminder_scheduler = ReminderScheduler::new();
    
    // 创建截图缓存
    let screenshot_cache = Arc::new(Mutex::new(None::<Vec<u8>>));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(DbState(Mutex::new(conn)))
        .manage(SyncEngineState(sync_engine.clone()))
        .manage(ScreenshotCache(screenshot_cache.clone()))
        .setup(move |app| {
            info!("Tauri app setup starting...");
            let handle = app.handle().clone();
            
            // 注册全局热键
            if let Err(e) = screenshot::hotkey::register_hotkeys(&handle, screenshot_cache.clone()) {
                error!("Failed to register global hotkeys: {}", e);
            } else {
                info!("Global hotkeys registered successfully");
            }
            
            // 设置同步引擎的 AppHandle
            sync_engine.set_app_handle(handle.clone());
            
            // 设置提醒调度器的 AppHandle
            reminder_scheduler.set_app_handle(handle.clone());
            
            // 启动提醒调度器
            reminder_scheduler.start();
            
            // 启动同步引擎（如果已配置）
            {
                let db_path = db::init::get_database_path();
                match Connection::open(&db_path) {
                    Ok(conn) => {
                        if let Err(e) = db::init::init_database(&conn) {
                            error!("Failed to initialize database for sync config: {}", e);
                        }
                        let config = sync::config::SyncConfig::from_db(&conn);
                        if config.is_configured() {
                            sync_engine.start_periodic_sync();
                            info!("Sync engine started with periodic sync");
                        } else {
                            info!("Sync engine not started (not configured)");
                        }
                    }
                    Err(e) => {
                        error!("Failed to open database for sync config: {}", e);
                    }
                }
            }
            
            // 监听截屏触发事件
            app.listen("screenshot:trigger", move |_event| {
                info!("Received screenshot:trigger event");
            });
            
            // 监听截图错误事件
            app.listen("screenshot:error", move |_event| {
                info!("Received screenshot:error event");
            });
            
            info!("Tauri app setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 任务命令
            create_task,
            get_task,
            update_task,
            delete_task,
            list_tasks,
            search_tasks,
            update_task_status,
            get_upcoming_tasks,
            // 客户命令
            create_client,
            list_clients,
            update_client,
            delete_client,
            // 项目命令
            create_project,
            list_projects,
            update_project,
            delete_project,
            // AI 命令
            analyze_screenshot,
            confirm_analysis,
            get_ai_config,
            update_ai_config,
            // 提醒命令
            create_reminder,
            list_reminders,
            update_reminder,
            delete_reminder,
            // 同步命令
            trigger_sync,
            get_sync_status,
            get_sync_config,
            update_sync_config,
            // 截屏命令
            trigger_screenshot,
            get_screenshot,
            // 窗口命令
            toggle_floating_card,
            show_floating_card,
            hide_floating_card,
            is_floating_card_visible,
            // 设置命令
            get_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
