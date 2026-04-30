use rusqlite::{params, Connection, Result as SqlResult};
use crate::db::models::*;

/// 创建项目
pub fn create_project(conn: &Connection, input: &CreateProjectInput) -> SqlResult<Project> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO projects (id, name, description, client_id, color, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 'active', datetime('now'), datetime('now'))",
        params![id, input.name, input.description, input.client_id, input.color],
    )?;
    get_project(conn, &id)
}

/// 获取单个项目
pub fn get_project(conn: &Connection, id: &str) -> SqlResult<Project> {
    conn.query_row(
        "SELECT id, name, description, client_id, color, status, created_at, updated_at FROM projects WHERE id = ?1",
        params![id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                client_id: row.get(3)?,
                color: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
}

/// 列出所有项目
pub fn list_projects(conn: &Connection) -> SqlResult<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, client_id, color, status, created_at, updated_at FROM projects ORDER BY name ASC"
    )?;

    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            client_id: row.get(3)?,
            color: row.get(4)?,
            status: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(projects)
}

/// 更新项目
pub fn update_project(conn: &Connection, input: &UpdateProjectInput) -> SqlResult<Project> {
    let existing = get_project(conn, &input.id)?;

    let new_name = input.name.as_deref().unwrap_or(&existing.name);
    let new_description = input.description.as_ref().or(existing.description.as_ref()).cloned();
    let new_client_id = input.client_id.as_ref().or(existing.client_id.as_ref()).cloned();
    let new_color = input.color.as_ref().or(existing.color.as_ref()).cloned();
    let new_status = input.status.as_deref().unwrap_or(&existing.status);

    conn.execute(
        "UPDATE projects SET name = ?1, description = ?2, client_id = ?3, color = ?4, status = ?5, updated_at = datetime('now') WHERE id = ?6",
        params![new_name, new_description, new_client_id, new_color, new_status, input.id],
    )?;

    get_project(conn, &input.id)
}

/// 删除项目
pub fn delete_project(conn: &Connection, id: &str) -> SqlResult<bool> {
    let rows = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    Ok(rows > 0)
}
