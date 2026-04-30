use rusqlite::{Connection, Result as SqlResult};
use log::info;

/// 当前数据库 schema 版本
const SCHEMA_VERSION: i32 = 1;

/// 初始化数据库，创建所有表和索引
pub fn init_database(conn: &Connection) -> SqlResult<()> {
    // 启用 WAL 模式提升并发性能
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    // 创建版本追踪表
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        );"
    )?;

    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < SCHEMA_VERSION {
        info!("Database schema version {}, migrating to {}", current_version, SCHEMA_VERSION);
        migrate_database(conn, current_version, SCHEMA_VERSION)?;
    } else {
        info!("Database schema is up to date at version {}", current_version);
    }

    Ok(())
}

/// 数据库迁移逻辑
fn migrate_database(conn: &Connection, from: i32, to: i32) -> SqlResult<()> {
    if from < 1 {
        migrate_v0_to_v1(conn)?;
    }
    // 未来版本迁移可以在这里添加
    // if from < 2 { migrate_v1_to_v2(conn)?; }

    conn.execute(
        "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
        [to],
    )?;
    Ok(())
}

/// 初始 schema 创建 (v0 -> v1)
fn migrate_v0_to_v1(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "
        -- 客户表
        CREATE TABLE IF NOT EXISTS clients (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            company TEXT,
            email TEXT,
            phone TEXT,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- 项目表
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            client_id TEXT,
            color TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL
        );

        -- 任务表
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'pending',
            priority TEXT NOT NULL DEFAULT 'none',
            client_id TEXT,
            project_id TEXT,
            deadline TEXT,
            tags TEXT,
            source_type TEXT NOT NULL DEFAULT 'manual',
            source_image TEXT,
            ocr_text TEXT,
            ai_confidence REAL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            completed_at TEXT,
            archived_at TEXT,
            FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
        );

        -- 提醒表
        CREATE TABLE IF NOT EXISTS reminders (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            reminder_time TEXT NOT NULL,
            message TEXT,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            fired_at TEXT,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        );

        -- 变更记录表
        CREATE TABLE IF NOT EXISTS change_records (
            id TEXT PRIMARY KEY,
            entity_type TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            field_name TEXT NOT NULL,
            old_value TEXT,
            new_value TEXT,
            changed_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- 同步日志表
        CREATE TABLE IF NOT EXISTS sync_log (
            id TEXT PRIMARY KEY,
            entity_type TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            operation TEXT NOT NULL,
            data TEXT,
            timestamp TEXT NOT NULL DEFAULT (datetime('now')),
            synced INTEGER NOT NULL DEFAULT 0,
            device_id TEXT NOT NULL
        );

        -- 设置表
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- 索引
        CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
        CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
        CREATE INDEX IF NOT EXISTS idx_tasks_client_id ON tasks(client_id);
        CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
        CREATE INDEX IF NOT EXISTS idx_tasks_deadline ON tasks(deadline);
        CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
        CREATE INDEX IF NOT EXISTS idx_tasks_title ON tasks(title);
        CREATE INDEX IF NOT EXISTS idx_tasks_description ON tasks(description);

        CREATE INDEX IF NOT EXISTS idx_reminders_task_id ON reminders(task_id);
        CREATE INDEX IF NOT EXISTS idx_reminders_status ON reminders(status);
        CREATE INDEX IF NOT EXISTS idx_reminders_reminder_time ON reminders(reminder_time);

        CREATE INDEX IF NOT EXISTS idx_change_records_entity ON change_records(entity_type, entity_id);
        CREATE INDEX IF NOT EXISTS idx_change_records_changed_at ON change_records(changed_at);

        CREATE INDEX IF NOT EXISTS idx_sync_log_synced ON sync_log(synced);
        CREATE INDEX IF NOT EXISTS idx_sync_log_entity ON sync_log(entity_type, entity_id);
        CREATE INDEX IF NOT EXISTS idx_sync_log_timestamp ON sync_log(timestamp);

        CREATE INDEX IF NOT EXISTS idx_projects_client_id ON projects(client_id);
        CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
        ",
    )?;

    Ok(())
}

/// 获取数据库文件路径
pub fn get_database_path() -> String {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("snaptask");

    // 确保目录存在
    std::fs::create_dir_all(&data_dir).ok();

    data_dir.join("snaptask.db").to_string_lossy().to_string()
}
