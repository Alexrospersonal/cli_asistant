use std::error::Error;
use crate::commands::track::db::open_db;
use chrono::Utc;
use rusqlite::params;

pub async fn run(label: Option<String>) -> Result<(), Box<dyn Error>> {
    let conn = open_db()?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sessions WHERE ended_at IS NULL")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;

    if count > 0 {
        println!("You already have active session. Before stop it use: (`track stop`).");
        return Ok(());
    }

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO sessions (label, started_at) VALUES (?1, ?2)",
        params![label, now]
    )?;

    println!("Session is running: {}", now);
    Ok(())
}