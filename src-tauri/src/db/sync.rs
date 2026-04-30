use rusqlite::{params, Connection, Result as SqlResult};
use crate::db::models::*;

/// 记录本地变更到同步日志
pub fn log_change(
    conn: &Connection,
    entity_type: &str,
    entity_id: &str,
    operation: &str,
    data: Option<&str>,
    device_id: &str,
) -> SqlResult<String> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO sync_log (id, entity_type, entity_id, operation, data, timestamp, synced, device_id)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), 0, ?6)",
        params![id, entity_type, entity_id, operation, data, device_id],
    )?;
    Ok(id)
}

/// 获取未同步的变更
pub fn get_unsynced_changes(conn: &Connection, device_id: &str) -> SqlResult<Vec<SyncLogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, entity_type, entity_id, operation, data, timestamp, synced, device_id
         FROM sync_log
         WHERE synced = 0 AND device_id = ?1
         ORDER BY timestamp ASC"
    )?;

    let entries = stmt.query_map(params![device_id], |row| {
        Ok(SyncLogEntry {
            id: row.get(0)?,
            entity_type: row.get(1)?,
            entity_id: row.get(2)?,
            operation: row.get(3)?,
            data: row.get(4)?,
            timestamp: row.get(5)?,
            synced: row.get::<_, i32>(6)? != 0,
            device_id: row.get(7)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(entries)
}

/// 标记变更已同步
pub fn mark_as_synced(conn: &Connection, ids: &[String]) -> SqlResult<usize> {
    if ids.is_empty() {
        return Ok(0);
    }
    let placeholders: Vec<String> = ids.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect();
    let sql = format!(
        "UPDATE sync_log SET synced = 1 WHERE id IN ({})",
        placeholders.join(", ")
    );
    let params: Vec<&dyn rusqlite::types::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();
    let rows = conn.execute(&sql, params.as_slice())?;
    Ok(rows)
}

/// 应用远端变更（Last-Writer-Wins 策略）
pub fn apply_remote_changes(conn: &Connection, entries: &[SyncLogEntry]) -> SqlResult<usize> {
    let mut applied = 0;

    for entry in entries {
        // 跳过来自本设备的变更
        let local_device_id = get_device_id(conn)?;
        if entry.device_id == local_device_id {
            continue;
        }

        match (entry.entity_type.as_str(), entry.operation.as_str()) {
            ("task", "create") => {
                if let Some(ref data) = entry.data {
                    if let Ok(task) = serde_json::from_str::<Task>(data) {
                        // 检查是否已存在
                        let exists: bool = conn
                            .query_row(
                                "SELECT COUNT(*) FROM tasks WHERE id = ?1",
                                params![task.id],
                                |row| row.get(0),
                            )
                            .unwrap_or(0) > 0;

                        if !exists {
                            conn.execute(
                                "INSERT OR IGNORE INTO tasks (id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at)
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
                                params![task.id, task.title, task.description, task.status, task.priority, task.client_id, task.project_id, task.deadline, task.tags, task.source_type, task.source_image, task.ocr_text, task.ai_confidence, task.created_at, task.updated_at, task.completed_at, task.archived_at],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("task", "update") => {
                if let Some(ref data) = entry.data {
                    if let Ok(task) = serde_json::from_str::<Task>(data) {
                        // Last-Writer-Wins: 如果远端时间戳更新则覆盖
                        let local_updated: Option<String> = conn
                            .query_row(
                                "SELECT updated_at FROM tasks WHERE id = ?1",
                                params![task.id],
                                |row| row.get(0),
                            )
                            .ok();

                        let should_apply = match local_updated {
                            None => false,
                            Some(ref local) => task.updated_at > *local,
                        };

                        if should_apply {
                            conn.execute(
                                "UPDATE tasks SET title = ?1, description = ?2, status = ?3, priority = ?4, client_id = ?5, project_id = ?6, deadline = ?7, tags = ?8, source_type = ?9, source_image = ?10, ocr_text = ?11, ai_confidence = ?12, updated_at = ?13, completed_at = ?14, archived_at = ?15 WHERE id = ?16",
                                params![task.title, task.description, task.status, task.priority, task.client_id, task.project_id, task.deadline, task.tags, task.source_type, task.source_image, task.ocr_text, task.ai_confidence, task.updated_at, task.completed_at, task.archived_at, task.id],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("task", "delete") => {
                conn.execute("DELETE FROM tasks WHERE id = ?1", params![entry.entity_id])?;
                applied += 1;
            }
            ("client", "create") => {
                if let Some(ref data) = entry.data {
                    if let Ok(client) = serde_json::from_str::<Client>(data) {
                        let exists: bool = conn
                            .query_row(
                                "SELECT COUNT(*) FROM clients WHERE id = ?1",
                                params![client.id],
                                |row| row.get(0),
                            )
                            .unwrap_or(0) > 0;
                        if !exists {
                            conn.execute(
                                "INSERT OR IGNORE INTO clients (id, name, company, email, phone, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                                params![client.id, client.name, client.company, client.email, client.phone, client.notes, client.created_at, client.updated_at],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("client", "update") => {
                if let Some(ref data) = entry.data {
                    if let Ok(client) = serde_json::from_str::<Client>(data) {
                        let local_updated: Option<String> = conn
                            .query_row(
                                "SELECT updated_at FROM clients WHERE id = ?1",
                                params![client.id],
                                |row| row.get(0),
                            )
                            .ok();
                        let should_apply = match local_updated {
                            None => false,
                            Some(ref local) => client.updated_at > *local,
                        };
                        if should_apply {
                            conn.execute(
                                "UPDATE clients SET name = ?1, company = ?2, email = ?3, phone = ?4, notes = ?5, updated_at = ?6 WHERE id = ?7",
                                params![client.name, client.company, client.email, client.phone, client.notes, client.updated_at, client.id],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("client", "delete") => {
                conn.execute("DELETE FROM clients WHERE id = ?1", params![entry.entity_id])?;
                applied += 1;
            }
            ("project", "create") => {
                if let Some(ref data) = entry.data {
                    if let Ok(project) = serde_json::from_str::<Project>(data) {
                        let exists: bool = conn
                            .query_row(
                                "SELECT COUNT(*) FROM projects WHERE id = ?1",
                                params![project.id],
                                |row| row.get(0),
                            )
                            .unwrap_or(0) > 0;
                        if !exists {
                            conn.execute(
                                "INSERT OR IGNORE INTO projects (id, name, description, client_id, color, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                                params![project.id, project.name, project.description, project.client_id, project.color, project.status, project.created_at, project.updated_at],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("project", "update") => {
                if let Some(ref data) = entry.data {
                    if let Ok(project) = serde_json::from_str::<Project>(data) {
                        let local_updated: Option<String> = conn
                            .query_row(
                                "SELECT updated_at FROM projects WHERE id = ?1",
                                params![project.id],
                                |row| row.get(0),
                            )
                            .ok();
                        let should_apply = match local_updated {
                            None => false,
                            Some(ref local) => project.updated_at > *local,
                        };
                        if should_apply {
                            conn.execute(
                                "UPDATE projects SET name = ?1, description = ?2, client_id = ?3, color = ?4, status = ?5, updated_at = ?6 WHERE id = ?7",
                                params![project.name, project.description, project.client_id, project.color, project.status, project.updated_at, project.id],
                            )?;
                            applied += 1;
                        }
                    }
                }
            }
            ("project", "delete") => {
                conn.execute("DELETE FROM projects WHERE id = ?1", params![entry.entity_id])?;
                applied += 1;
            }
            _ => {
                // 未知实体类型，跳过
                log::warn!("Unknown sync entity type: {}", entry.entity_type);
            }
        }
    }

    Ok(applied)
}

/// 获取或创建设备 ID
pub fn get_device_id(conn: &Connection) -> SqlResult<String> {
    let result: SqlResult<String> = conn.query_row(
        "SELECT value FROM settings WHERE key = 'device_id'",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(id) => Ok(id),
        Err(_) => {
            let new_id = uuid::Uuid::now_v7().to_string();
            conn.execute(
                "INSERT INTO settings (key, value, updated_at) VALUES ('device_id', ?1, datetime('now'))",
                params![new_id],
            )?;
            Ok(new_id)
        }
    }
}

/// 获取设置值
pub fn get_setting(conn: &Connection, key: &str) -> SqlResult<Option<String>> {
    let result: SqlResult<String> = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    );
    match result {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// 设置值
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))",
        params![key, value],
    )?;
    Ok(())
}
