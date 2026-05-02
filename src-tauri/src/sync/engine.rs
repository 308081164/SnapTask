use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{AppHandle, Emitter};
use log::{info, error};
use crate::db::sync as sync_db;
use crate::db::models::SyncLogEntry;
use crate::sync::config::SyncConfig;

/// 同步状态
#[derive(Debug, Clone, serde::Serialize)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub last_sync_at: Option<String>,
    pub last_error: Option<String>,
    pub pending_changes: u64,
    pub is_online: bool,
}

/// 同步引擎
pub struct SyncEngine {
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    running: Arc<Mutex<bool>>,
    status: Arc<Mutex<SyncStatus>>,
    config: Arc<Mutex<SyncConfig>>,
}

impl SyncEngine {
    pub fn new() -> Self {
        SyncEngine {
            app_handle: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            status: Arc::new(Mutex::new(SyncStatus {
                is_syncing: false,
                last_sync_at: None,
                last_error: None,
                pending_changes: 0,
                is_online: true,
            })),
            config: Arc::new(Mutex::new(SyncConfig::default())),
        }
    }

    /// 设置 AppHandle
    pub fn set_app_handle(&self, handle: AppHandle) {
        let mut app = self.app_handle.lock().unwrap();
        *app = Some(handle);
    }

    /// 加载配置
    pub fn load_config(&self, conn: &Connection) {
        let config = SyncConfig::from_db(conn);
        let mut cfg = self.config.lock().unwrap();
        *cfg = config;
    }

    /// 获取当前同步状态
    pub fn get_status(&self) -> SyncStatus {
        let status = self.status.lock().unwrap();
        status.clone()
    }

    /// 启动定期同步
    pub fn start_periodic_sync(&self) {
        let running = self.running.clone();
        let app_handle = self.app_handle.clone();
        let config = self.config.clone();
        let status = self.status.clone();
        {
            let mut r = running.lock().unwrap();
            if *r {
                info!("Sync engine is already running");
                return;
            }
            *r = true;
        }
        let db_path = crate::db::init::get_database_path();
        tokio::spawn(async move {
            info!("Sync engine started");
            loop {
                // 读取配置（MutexGuard 在块结束时释放）
                let interval = {
                    let cfg = config.lock().unwrap();
                    cfg.sync_interval_secs
                };
                tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
                {
                    let r = running.lock().unwrap();
                    if !*r {
                        break;
                    }
                }
                // 获取 app handle（MutexGuard 在块结束时释放）
                let handle = {
                    let app = app_handle.lock().unwrap();
                    app.clone()
                };
                let Some(handle) = handle else {
                    continue;
                };
                // 获取配置（MutexGuard 在块结束时释放）
                let cfg = {
                    let c = config.lock().unwrap();
                    c.clone()
                };
                if !cfg.is_configured() {
                    continue;
                }
                // 更新状态
                {
                    let mut st = status.lock().unwrap();
                    st.is_syncing = true;
                    st.is_online = true;
                }
                // emit 同步开始事件
                let _ = handle.emit("sync:status", serde_json::json!({
                    "status": "syncing",
                    "message": "正在同步..."
                }));

                // 使用 spawn_blocking 执行同步数据库 + HTTP 操作
                let cfg_clone = cfg.clone();
                let status_clone = status.clone();
                let handle_clone = handle.clone();
                let db_path_clone = db_path.clone();

                let sync_result = tokio::task::spawn_blocking(move || {
                    // 在阻塞线程中执行所有同步操作
                    let conn = match Connection::open(&db_path_clone) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Failed to open database for sync: {}", e);
                            let mut st = status_clone.lock().unwrap();
                            st.is_syncing = false;
                            st.last_error = Some(format!("数据库连接失败: {}", e));
                            st.is_online = false;
                            return Err("数据库连接失败".to_string());
                        }
                    };
                    if let Err(e) = crate::db::init::init_database(&conn) {
                        error!("Failed to initialize database for sync: {}", e);
                        return Err("数据库初始化失败".to_string());
                    }

                    // 推送本地变更（同步 HTTP）
                    match push_changes_sync(&conn, &cfg_clone) {
                        Ok(count) => {
                            info!("Pushed {} changes to server", count);
                        }
                        Err(e) => {
                            error!("Failed to push changes: {}", e);
                            let mut st = status_clone.lock().unwrap();
                            st.last_error = Some(format!("推送失败: {}", e));
                            st.is_online = false;
                        }
                    }

                    // 拉取远端变更（同步 HTTP）
                    match pull_changes_sync(&conn, &cfg_clone) {
                        Ok(count) => {
                            info!("Applied {} remote changes", count);
                        }
                        Err(e) => {
                            error!("Failed to pull changes: {}", e);
                            let mut st = status_clone.lock().unwrap();
                            st.last_error = Some(format!("拉取失败: {}", e));
                        }
                    }

                    // 更新状态
                    {
                        let mut st = status_clone.lock().unwrap();
                        st.is_syncing = false;
                        st.last_sync_at = Some(chrono::Utc::now().to_rfc3339());
                        st.is_online = true;
                        let device_id = sync_db::get_device_id(&conn).unwrap_or_default();
                        st.pending_changes = sync_db::get_unsynced_changes(&conn, &device_id)
                            .map(|c| c.len() as u64)
                            .unwrap_or(0);
                    }

                    // 返回最终状态用于 emit
                    let st = status_clone.lock().unwrap().clone();
                    Ok(st)
                }).await;

                match sync_result {
                    Ok(Ok(st)) => {
                        let _ = handle_clone.emit("sync:status", serde_json::json!({
                            "status": "completed",
                            "message": "同步完成",
                            "last_sync_at": st.last_sync_at,
                            "pending_changes": st.pending_changes,
                        }));
                    }
                    Ok(Err(e)) => {
                        error!("Sync failed: {}", e);
                    }
                    Err(e) => {
                        error!("Sync task panicked: {}", e);
                    }
                }
            }
            info!("Sync engine stopped");
        });
    }

    /// 停止同步引擎
    pub fn stop(&self) {
        let mut r = self.running.lock().unwrap();
        *r = false;
        info!("Sync engine stop requested");
    }

    /// 手动触发一次同步
    pub async fn trigger_sync(&self) -> Result<(), String> {
        let db_path = crate::db::init::get_database_path();
        let cfg = {
            let c = self.config.lock().unwrap();
            c.clone()
        };
        if !cfg.is_configured() {
            return Err("同步未配置，请先设置服务器地址和 API Key".to_string());
        }

        // 使用 spawn_blocking 执行同步数据库 + HTTP 操作
        let status = self.status.clone();
        let app_handle = self.app_handle.clone();

        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(&db_path)
                .map_err(|e| format!("数据库连接失败: {}", e))?;
            crate::db::init::init_database(&conn)
                .map_err(|e| format!("数据库初始化失败: {}", e))?;

            // 推送
            push_changes_sync(&conn, &cfg)
                .map_err(|e| format!("推送失败: {}", e))?;
            // 拉取
            pull_changes_sync(&conn, &cfg)
                .map_err(|e| format!("拉取失败: {}", e))?;

            // 更新状态
            {
                let mut st = status.lock().unwrap();
                st.last_sync_at = Some(chrono::Utc::now().to_rfc3339());
                st.last_error = None;
                st.is_online = true;
                let device_id = sync_db::get_device_id(&conn).unwrap_or_default();
                st.pending_changes = sync_db::get_unsynced_changes(&conn, &device_id)
                    .map(|c| c.len() as u64)
                    .unwrap_or(0);
            }

            // emit 事件
            if let Ok(app) = app_handle.lock() {
                if let Some(ref handle) = *app {
                    let st = status.lock().unwrap();
                    let _ = handle.emit("sync:status", serde_json::json!({
                        "status": "completed",
                        "message": "同步完成",
                        "last_sync_at": st.last_sync_at,
                        "pending_changes": st.pending_changes,
                    }));
                }
            }
            Ok::<(), String>(())
        }).await.map_err(|e| format!("同步任务执行失败: {}", e))??;

        Ok(())
    }
}

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 推送本地变更到云端（同步版本，在 spawn_blocking 中调用）
fn push_changes_sync(conn: &Connection, config: &SyncConfig) -> Result<usize, String> {
    let device_id = sync_db::get_device_id(conn)
        .map_err(|e| format!("Failed to get device_id: {}", e))?;
    let unsynced = sync_db::get_unsynced_changes(conn, &device_id)
        .map_err(|e| format!("Failed to get unsynced changes: {}", e))?;
    if unsynced.is_empty() {
        return Ok(0);
    }

    // 在同步上下文中使用阻塞 HTTP 客户端
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("{}/api/sync/push", config.server_url.trim_end_matches('/'));
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("X-Device-ID", &device_id)
        .json(&unsynced)
        .send()
        .map_err(|e| format!("Push request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("Push failed (HTTP {}): {}", status, body));
    }

    // 标记已同步
    let ids: Vec<String> = unsynced.iter().map(|e| e.id.clone()).collect();
    let synced_count = sync_db::mark_as_synced(conn, &ids)
        .map_err(|e| format!("Failed to mark as synced: {}", e))?;
    Ok(synced_count)
}

/// 从云端拉取变更（同步版本，在 spawn_blocking 中调用）
fn pull_changes_sync(conn: &Connection, config: &SyncConfig) -> Result<usize, String> {
    let device_id = sync_db::get_device_id(conn)
        .map_err(|e| format!("Failed to get device_id: {}", e))?;

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("{}/api/sync/pull?device_id={}", config.server_url.trim_end_matches('/'), device_id);
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()
        .map_err(|e| format!("Pull request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("Pull failed (HTTP {}): {}", status, body));
    }

    let remote_entries: Vec<SyncLogEntry> = response.json()
        .map_err(|e| format!("Failed to parse pull response: {}", e))?;

    if remote_entries.is_empty() {
        return Ok(0);
    }

    let applied = sync_db::apply_remote_changes(conn, &remote_entries)
        .map_err(|e| format!("Failed to apply remote changes: {}", e))?;
    Ok(applied)
}