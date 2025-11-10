use rusqlite::{Connection, Result, params};
use crate::screenshot::SSData;
use chrono::{Utc, DateTime};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("screenshots.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
             id INTEGER PRIMARY KEY,
             file TEXT NOT NULL,
             content TEXT NOT NULL,
             created_at TEXT NOT NULL
         )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_ss_data(conn: &Connection, ss_data: &SSData) -> Result<usize> {
    let created_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO screenshots (file, content, created_at) VALUES (?1, ?2, ?3)",
        &[&ss_data.file, &ss_data.content, &created_at],
    )
}

pub fn query_ss_data(conn: &Connection, query: &str) -> Result<Vec<SSData>> {
    let mut stmt = conn.prepare("SELECT id, file, content, created_at FROM screenshots WHERE content LIKE ?1")?;
    let mut rows = stmt.query(params![format!("%{}%", query)])?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        let created_at_str: String = row.get(3)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str).unwrap().with_timezone(&Utc);
        results.push(SSData {
            id: Some(row.get(0)?),
            file: row.get(1)?,
            content: row.get(2)?,
            created_at: Some(created_at),
        });
    }
    Ok(results)
}