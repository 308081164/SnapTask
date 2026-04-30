use serde::{Deserialize, Serialize};

/// 同步配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub server_url: String,
    pub api_key: String,
    pub device_id: String,
    pub sync_interval_secs: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        SyncConfig {
            server_url: String::new(),
            api_key: String::new(),
            device_id: uuid::Uuid::now_v7().to_string(),
            sync_interval_secs: 30,
        }
    }
}

impl SyncConfig {
    /// 从数据库读取同步配置
    pub fn from_db(conn: &rusqlite::Connection) -> Self {
        let mut config = SyncConfig::default();

        // 读取 device_id
        if let Ok(id) = crate::db::sync::get_device_id(conn) {
            config.device_id = id;
        }

        // 读取 server_url
        if let Some(url) = crate::db::sync::get_setting(conn, "sync_server_url").ok().flatten() {
            config.server_url = url;
        }

        // 读取 api_key
        if let Some(key) = crate::db::sync::get_setting(conn, "sync_api_key").ok().flatten() {
            config.api_key = key;
        }

        // 读取 sync_interval
        if let Some(interval) = crate::db::sync::get_setting(conn, "sync_interval_secs").ok().flatten() {
            if let Ok(secs) = interval.parse::<u64>() {
                config.sync_interval_secs = secs;
            }
        }

        config
    }

    /// 保存同步配置到数据库
    pub fn save_to_db(&self, conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        crate::db::sync::set_setting(conn, "sync_server_url", &self.server_url)?;
        crate::db::sync::set_setting(conn, "sync_api_key", &self.api_key)?;
        crate::db::sync::set_setting(conn, "sync_interval_secs", &self.sync_interval_secs.to_string())?;
        // device_id 已在 get_device_id 中自动持久化
        Ok(())
    }

    /// 检查同步是否已配置
    pub fn is_configured(&self) -> bool {
        !self.server_url.is_empty() && !self.api_key.is_empty()
    }
}
