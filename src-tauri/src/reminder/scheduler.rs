use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use log::{info, warn, error};

use crate::db::reminder as reminder_db;
use crate::db::task as task_db;

/// 提醒调度器
pub struct ReminderScheduler {
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    running: Arc<Mutex<bool>>,
}

impl ReminderScheduler {
    pub fn new() -> Self {
        ReminderScheduler {
            app_handle: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// 设置 AppHandle
    pub fn set_app_handle(&self, handle: AppHandle) {
        let mut app = self.app_handle.lock().unwrap();
        *app = Some(handle);
    }

    /// 启动提醒调度器
    pub fn start(&self) {
        let running = self.running.clone();
        let app_handle = self.app_handle.clone();

        {
            let mut r = running.lock().unwrap();
            if *r {
                info!("Reminder scheduler is already running");
                return;
            }
            *r = true;
        }

        // 获取数据库路径
        let db_path = crate::db::init::get_database_path();

        // 启动时立即检查一次
        self.check_reminders(&db_path);

        // 启动后台任务
        tokio::spawn(async move {
            info!("Reminder scheduler started, checking every 60 seconds");

            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;

                {
                    let r = running.lock().unwrap();
                    if !*r {
                        break;
                    }
                }

                // 检查提醒
                let app = app_handle.lock().unwrap();
                if let Some(ref handle) = *app {
                    // 获取数据库连接
                    let conn = match Connection::open(&db_path) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Failed to open database for reminder check: {}", e);
                            continue;
                        }
                    };

                    if let Err(e) = crate::db::init::init_database(&conn) {
                        error!("Failed to initialize database for reminder check: {}", e);
                        continue;
                    }

                    match reminder_db::get_pending_reminders(&conn) {
                        Ok(reminders) => {
                            for reminder in reminders {
                                // 获取关联的任务信息
                                let task_title = task_db::get_task(&conn, &reminder.task_id)
                                    .map(|t| t.title)
                                    .unwrap_or_else(|_| "未知任务".to_string());

                                let message = reminder.message
                                    .unwrap_or_else(|| format!("任务提醒: {}", task_title));

                                // 发送系统通知
                                if let Err(e) = handle.notification()
                                    .builder()
                                    .title("SnapTask 任务提醒")
                                    .body(&message)
                                    .id(&reminder.id)
                                    .show()
                                {
                                    error!("Failed to show notification: {}", e);
                                }

                                // emit 事件到前端
                                if let Err(e) = handle.emit("reminder:fire", serde_json::json!({
                                    "id": reminder.id,
                                    "task_id": reminder.task_id,
                                    "task_title": task_title,
                                    "message": message,
                                    "reminder_time": reminder.reminder_time,
                                })) {
                                    error!("Failed to emit reminder:fire event: {}", e);
                                }

                                // 更新提醒状态为已触发
                                if let Err(e) = reminder_db::update_reminder_status(&conn, &reminder.id, "fired") {
                                    error!("Failed to update reminder status: {}", e);
                                }

                                info!("Reminder fired for task: {}", task_title);
                            }
                        }
                        Err(e) => {
                            error!("Failed to get pending reminders: {}", e);
                        }
                    }
                }
            }

            info!("Reminder scheduler stopped");
        });
    }

    /// 停止提醒调度器
    pub fn stop(&self) {
        let mut r = self.running.lock().unwrap();
        *r = false;
        info!("Reminder scheduler stop requested");
    }

    /// 检查提醒（启动时立即调用一次）
    fn check_reminders(&self, db_path: &str) {
        let conn = match Connection::open(db_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to open database for initial reminder check: {}", e);
                return;
            }
        };

        if let Err(e) = crate::db::init::init_database(&conn) {
            error!("Failed to initialize database for initial reminder check: {}", e);
            return;
        }

        match reminder_db::get_pending_reminders(&conn) {
            Ok(reminders) => {
                info!("Found {} pending reminders on startup", reminders.len());
            }
            Err(e) => {
                error!("Failed to check pending reminders on startup: {}", e);
            }
        }
    }
}

impl Default for ReminderScheduler {
    fn default() -> Self {
        Self::new()
    }
}
