use rusqlite::{params, Connection, Result as SqlResult};
use crate::db::models::*;

/// 创建任务
pub fn create_task(conn: &Connection, input: &CreateTaskInput) -> SqlResult<Task> {
    let id = uuid::Uuid::now_v7().to_string();
    let tags_json = input.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default());
    let source_type = input.source_type.as_deref().unwrap_or("manual");

    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at)
         VALUES (?1, ?2, ?3, 'pending', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, datetime('now'), datetime('now'))",
        params![
            id,
            input.title,
            input.description,
            input.priority.as_deref().unwrap_or("none"),
            input.client_id,
            input.project_id,
            input.deadline,
            tags_json,
            source_type,
            input.source_image,
            input.ocr_text,
            input.ai_confidence,
        ],
    )?;

    get_task(conn, &id)
}

/// 获取单个任务
pub fn get_task(conn: &Connection, id: &str) -> SqlResult<Task> {
    conn.query_row(
        "SELECT id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at
         FROM tasks WHERE id = ?1",
        params![id],
        |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                client_id: row.get(5)?,
                project_id: row.get(6)?,
                deadline: row.get(7)?,
                tags: row.get(8)?,
                source_type: row.get(9)?,
                source_image: row.get(10)?,
                ocr_text: row.get(11)?,
                ai_confidence: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                completed_at: row.get(15)?,
                archived_at: row.get(16)?,
            })
        },
    )
}

/// 更新任务
pub fn update_task(conn: &Connection, input: &UpdateTaskInput) -> SqlResult<Task> {
    let existing = get_task(conn, &input.id)?;

    let new_title = input.title.as_deref().unwrap_or(&existing.title);
    let new_description = input.description.as_ref().or(existing.description.as_ref()).cloned();
    let new_priority = input.priority.as_deref().unwrap_or(&existing.priority);
    let new_client_id = input.client_id.as_ref().or(existing.client_id.as_ref()).cloned();
    let new_project_id = input.project_id.as_ref().or(existing.project_id.as_ref()).cloned();
    let new_deadline = input.deadline.as_ref().or(existing.deadline.as_ref()).cloned();
    let new_tags = input.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default()).or(existing.tags);

    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2, priority = ?3, client_id = ?4, project_id = ?5, deadline = ?6, tags = ?7, updated_at = datetime('now')
         WHERE id = ?8",
        params![new_title, new_description, new_priority, new_client_id, new_project_id, new_deadline, new_tags, input.id],
    )?;

    // 记录变更
    if input.title.is_some() && input.title.as_deref() != Some(&existing.title) {
        add_change_record(conn, "task", &input.id, "title", Some(&existing.title), input.title.as_deref())?;
    }
    if input.priority.is_some() && input.priority.as_deref() != Some(&existing.priority) {
        add_change_record(conn, "task", &input.id, "priority", Some(&existing.priority), input.priority.as_deref())?;
    }
    if input.deadline.is_some() && input.deadline != existing.deadline {
        add_change_record(conn, "task", &input.id, "deadline", existing.deadline.as_deref(), input.deadline.as_deref())?;
    }

    get_task(conn, &input.id)
}

/// 删除任务
pub fn delete_task(conn: &Connection, id: &str) -> SqlResult<bool> {
    let rows = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(rows > 0)
}

/// 列出任务（支持筛选）
pub fn list_tasks(conn: &Connection, filter: &TaskFilter) -> SqlResult<Vec<Task>> {
    let mut sql = String::from(
        "SELECT id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at FROM tasks WHERE 1=1"
    );
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref status) = filter.status {
        sql.push_str(" AND status = ?");
        param_values.push(Box::new(status.clone()));
    }
    if let Some(ref priority) = filter.priority {
        sql.push_str(" AND priority = ?");
        param_values.push(Box::new(priority.clone()));
    }
    if let Some(ref client_id) = filter.client_id {
        sql.push_str(" AND client_id = ?");
        param_values.push(Box::new(client_id.clone()));
    }
    if let Some(ref project_id) = filter.project_id {
        sql.push_str(" AND project_id = ?");
        param_values.push(Box::new(project_id.clone()));
    }
    if let Some(ref start) = filter.deadline_start {
        sql.push_str(" AND deadline >= ?");
        param_values.push(Box::new(start.clone()));
    }
    if let Some(ref end) = filter.deadline_end {
        sql.push_str(" AND deadline <= ?");
        param_values.push(Box::new(end.clone()));
    }
    if let Some(ref search) = filter.search {
        sql.push_str(" AND (title LIKE ? OR description LIKE ?)");
        let pattern = format!("%{}%", search);
        param_values.push(Box::new(pattern.clone()));
        param_values.push(Box::new(pattern));
    }

    sql.push_str(" ORDER BY created_at DESC");

    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let tasks = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            client_id: row.get(5)?,
            project_id: row.get(6)?,
            deadline: row.get(7)?,
            tags: row.get(8)?,
            source_type: row.get(9)?,
            source_image: row.get(10)?,
            ocr_text: row.get(11)?,
            ai_confidence: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            completed_at: row.get(15)?,
            archived_at: row.get(16)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(tasks)
}

/// 全文搜索任务
pub fn search_tasks(conn: &Connection, query: &str, limit: i64) -> SqlResult<Vec<Task>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at
         FROM tasks
         WHERE title LIKE ?1 OR description LIKE ?1 OR ocr_text LIKE ?1
         ORDER BY created_at DESC
         LIMIT ?2"
    )?;

    let tasks = stmt.query_map(params![pattern, limit], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            client_id: row.get(5)?,
            project_id: row.get(6)?,
            deadline: row.get(7)?,
            tags: row.get(8)?,
            source_type: row.get(9)?,
            source_image: row.get(10)?,
            ocr_text: row.get(11)?,
            ai_confidence: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            completed_at: row.get(15)?,
            archived_at: row.get(16)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(tasks)
}

/// 更新任务状态（带状态转换验证）
pub fn update_task_status(conn: &Connection, task_id: &str, new_status: &str) -> SqlResult<Task> {
    let task = get_task(conn, task_id)?;
    let current = TaskStatus::from_str_value(&task.status);
    let target = TaskStatus::from_str_value(new_status);

    // 验证状态转换
    let valid = matches!(
        (&current, &target),
        (TaskStatus::Pending, TaskStatus::InProgress)
            | (TaskStatus::Pending, TaskStatus::Archived)
            | (TaskStatus::InProgress, TaskStatus::Completed)
            | (TaskStatus::InProgress, TaskStatus::Pending)
            | (TaskStatus::Completed, TaskStatus::Archived)
            | (TaskStatus::Completed, TaskStatus::InProgress)
            | (TaskStatus::Archived, TaskStatus::Pending)
            | (TaskStatus::Archived, TaskStatus::InProgress)
    );

    if !valid {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let _completed_at = if target == TaskStatus::Completed {
        Some("datetime('now')".to_string())
    } else {
        task.completed_at.clone()
    };

    let _archived_at = if target == TaskStatus::Archived {
        Some("datetime('now')".to_string())
    } else {
        task.archived_at.clone()
    };

    conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = datetime('now'), completed_at = CASE WHEN ?1 = 'completed' THEN datetime('now') ELSE completed_at END, archived_at = CASE WHEN ?1 = 'archived' THEN datetime('now') ELSE archived_at END WHERE id = ?2",
        params![target.as_str(), task_id],
    )?;

    add_change_record(
        conn,
        "task",
        task_id,
        "status",
        Some(&task.status),
        Some(target.as_str()),
    )?;

    get_task(conn, task_id)
}

/// 获取指定时间范围内的任务（用于提醒）
pub fn get_tasks_by_deadline(conn: &Connection, start: &str, end: &str) -> SqlResult<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at
         FROM tasks
         WHERE deadline >= ?1 AND deadline <= ?2 AND status != 'completed' AND status != 'archived'
         ORDER BY deadline ASC"
    )?;

    let tasks = stmt.query_map(params![start, end], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            client_id: row.get(5)?,
            project_id: row.get(6)?,
            deadline: row.get(7)?,
            tags: row.get(8)?,
            source_type: row.get(9)?,
            source_image: row.get(10)?,
            ocr_text: row.get(11)?,
            ai_confidence: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            completed_at: row.get(15)?,
            archived_at: row.get(16)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(tasks)
}

/// 添加变更记录
pub fn add_change_record(
    conn: &Connection,
    entity_type: &str,
    entity_id: &str,
    field_name: &str,
    old_value: Option<&str>,
    new_value: Option<&str>,
) -> SqlResult<()> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO change_records (id, entity_type, entity_id, field_name, old_value, new_value, changed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))",
        params![id, entity_type, entity_id, field_name, old_value, new_value],
    )?;
    Ok(())
}

/// 获取即将到期的任务（未来 N 天内）
pub fn get_upcoming_tasks(conn: &Connection, days: i64) -> SqlResult<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, status, priority, client_id, project_id, deadline, tags, source_type, source_image, ocr_text, ai_confidence, created_at, updated_at, completed_at, archived_at
         FROM tasks
         WHERE deadline >= datetime('now') AND deadline <= datetime('now', '+' || ?1 || ' days')
         AND status NOT IN ('completed', 'archived')
         ORDER BY deadline ASC, priority DESC
         LIMIT 50"
    )?;

    let tasks = stmt.query_map(params![days], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            client_id: row.get(5)?,
            project_id: row.get(6)?,
            deadline: row.get(7)?,
            tags: row.get(8)?,
            source_type: row.get(9)?,
            source_image: row.get(10)?,
            ocr_text: row.get(11)?,
            ai_confidence: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            completed_at: row.get(15)?,
            archived_at: row.get(16)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(tasks)
}