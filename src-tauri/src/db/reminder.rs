use rusqlite::{params, Connection, Result as SqlResult};
use crate::db::models::*;

/// 创建提醒
pub fn create_reminder(conn: &Connection, input: &CreateReminderInput) -> SqlResult<Reminder> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO reminders (id, task_id, reminder_time, message, status, created_at)
         VALUES (?1, ?2, ?3, ?4, 'pending', datetime('now'))",
        params![id, input.task_id, input.reminder_time, input.message],
    )?;
    get_reminder(conn, &id)
}

/// 获取单个提醒
pub fn get_reminder(conn: &Connection, id: &str) -> SqlResult<Reminder> {
    conn.query_row(
        "SELECT id, task_id, reminder_time, message, status, created_at, fired_at FROM reminders WHERE id = ?1",
        params![id],
        |row| {
            Ok(Reminder {
                id: row.get(0)?,
                task_id: row.get(1)?,
                reminder_time: row.get(2)?,
                message: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                fired_at: row.get(6)?,
            })
        },
    )
}

/// 列出所有提醒
pub fn list_reminders(conn: &Connection) -> SqlResult<Vec<Reminder>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, reminder_time, message, status, created_at, fired_at FROM reminders ORDER BY reminder_time ASC"
    )?;

    let reminders = stmt.query_map([], |row| {
        Ok(Reminder {
            id: row.get(0)?,
            task_id: row.get(1)?,
            reminder_time: row.get(2)?,
            message: row.get(3)?,
            status: row.get(4)?,
            created_at: row.get(5)?,
            fired_at: row.get(6)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(reminders)
}

/// 更新提醒状态
pub fn update_reminder_status(conn: &Connection, id: &str, status: &str) -> SqlResult<Reminder> {
    let fired_at = if status == "fired" {
        "datetime('now')"
    } else {
        "NULL"
    };

    conn.execute(
        "UPDATE reminders SET status = ?1, fired_at = CASE WHEN ?1 = 'fired' THEN datetime('now') ELSE fired_at END WHERE id = ?2",
        params![status, id],
    )?;

    get_reminder(conn, id)
}

/// 获取待触发的提醒（reminder_time <= 当前时间 且 status = pending）
pub fn get_pending_reminders(conn: &Connection) -> SqlResult<Vec<Reminder>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, reminder_time, message, status, created_at, fired_at
         FROM reminders
         WHERE status = 'pending' AND reminder_time <= datetime('now')
         ORDER BY reminder_time ASC"
    )?;

    let reminders = stmt.query_map([], |row| {
        Ok(Reminder {
            id: row.get(0)?,
            task_id: row.get(1)?,
            reminder_time: row.get(2)?,
            message: row.get(3)?,
            status: row.get(4)?,
            created_at: row.get(5)?,
            fired_at: row.get(6)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(reminders)
}

/// 删除提醒
pub fn delete_reminder(conn: &Connection, id: &str) -> SqlResult<bool> {
    let rows = conn.execute("DELETE FROM reminders WHERE id = ?1", params![id])?;
    Ok(rows > 0)
}
