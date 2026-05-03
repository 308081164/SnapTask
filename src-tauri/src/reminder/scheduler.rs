use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;
use log::{info, error};
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

    /// 启动定期检查（在独立线程中运行 Tokio runtime，避免在 Tauri setup 上下文中调用 tokio::spawn）
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
        let db_path = crate::db::init::get_database_path();
        self.check_reminders(&db_path);

        // 在独立线程中创建 Tokio runtime，因为 Tauri 2.x 的 setup 闭包不在 Tokio runtime 上下文中
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    error!("Failed to create Tokio runtime for reminder scheduler: {}", e);
                    return;
                }
            };
            rt.block_on(async move {
                info!("Reminder scheduler started, checking every 60 seconds");
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                    {
                        let r = running.lock().unwrap();
                        if !*r {
                            break;
                        }
                    }
                    let app = app_handle.lock().unwrap();
                    if let Some(ref handle) = *app {
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
                                    let task_title = task_db::get_task(&conn, &reminder.task_id)
                                        .map(|t| t.title)
                                        .unwrap_or_else(|_| "\u{672a}\u{77e5}\u{4efb}\u{52a1}".to_string());
                                    let message = reminder.message
                                        .unwrap_or_else(|| format!("\u{4efb}\u{52a1}\u{63d0}\u{9192}: {}", task_title));
                                    let notification_id: i32 = reminder.id.as_bytes().iter()
                                        .fold(0i32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as i32));
                                    if let Err(e) = handle.notification()
                                        .builder()
                                        .title("SnapTask \u{4efb}\u{52a1}\u{63d0}\u{9192}")
                                        .body(&message)
                                        .id(notification_id)
                                        .show()
                                    {
                                        error!("Failed to show notification: {}", e);
                                    }
                                    if let Err(e) = handle.emit("reminder:fire", serde_json::json!({
                                        "id": reminder.id,
                                        "task_id": reminder.task_id,
                                        "task_title": task_title,
                                        "message": message,
                                        "reminder_time": reminder.reminder_time,
                                    })) {
                                        error!("Failed to emit reminder:fire event: {}", e);
                                    }
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
        });
    }

    pub fn stop(&self) {
        let mut r = self.running.lock().unwrap();
        *r = false;
        info!("Reminder scheduler stop requested");
    }

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
        };
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