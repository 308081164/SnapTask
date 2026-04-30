use rusqlite::{params, Connection, Result as SqlResult};
use crate::db::models::*;

/// 创建客户
pub fn create_client(conn: &Connection, input: &CreateClientInput) -> SqlResult<Client> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO clients (id, name, company, email, phone, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'), datetime('now'))",
        params![id, input.name, input.company, input.email, input.phone, input.notes],
    )?;
    get_client(conn, &id)
}

/// 获取单个客户
pub fn get_client(conn: &Connection, id: &str) -> SqlResult<Client> {
    conn.query_row(
        "SELECT id, name, company, email, phone, notes, created_at, updated_at FROM clients WHERE id = ?1",
        params![id],
        |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                company: row.get(2)?,
                email: row.get(3)?,
                phone: row.get(4)?,
                notes: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
}

/// 列出所有客户
pub fn list_clients(conn: &Connection) -> SqlResult<Vec<Client>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, company, email, phone, notes, created_at, updated_at FROM clients ORDER BY name ASC"
    )?;

    let clients = stmt.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            company: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?
    .collect::<SqlResult<Vec<_>>>()?;

    Ok(clients)
}

/// 更新客户
pub fn update_client(conn: &Connection, input: &UpdateClientInput) -> SqlResult<Client> {
    let existing = get_client(conn, &input.id)?;

    let new_name = input.name.as_deref().unwrap_or(&existing.name);
    let new_company = input.company.as_ref().or(existing.company.as_ref()).cloned();
    let new_email = input.email.as_ref().or(existing.email.as_ref()).cloned();
    let new_phone = input.phone.as_ref().or(existing.phone.as_ref()).cloned();
    let new_notes = input.notes.as_ref().or(existing.notes.as_ref()).cloned();

    conn.execute(
        "UPDATE clients SET name = ?1, company = ?2, email = ?3, phone = ?4, notes = ?5, updated_at = datetime('now') WHERE id = ?6",
        params![new_name, new_company, new_email, new_phone, new_notes, input.id],
    )?;

    get_client(conn, &input.id)
}

/// 删除客户
pub fn delete_client(conn: &Connection, id: &str) -> SqlResult<bool> {
    let rows = conn.execute("DELETE FROM clients WHERE id = ?1", params![id])?;
    Ok(rows > 0)
}
