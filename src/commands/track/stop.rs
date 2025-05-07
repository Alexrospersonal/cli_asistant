use std::error::Error;
use chrono::{DateTime, Utc};
use rusqlite::params;
use crate::commands::track::db::open_db;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let conn = open_db()?;

    let mut stmt = conn.prepare(
        "SELECT id, started_at FROM sessions WHERE ended_at IS NULL ORDER BY started_at DESC LIMIT 1"
    )?;

    let row = stmt.query_row([], |row| {
        let id: i64 = row.get(0)?;
        let started_at: String = row.get(1)?;
        Ok((id, started_at))
    });

    let (id, started_at) = match row {
        Ok(data) => data,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No active session for stoping");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    let started_dt: DateTime<Utc> = started_at.parse()?;
    let ended_dt = Utc::now();
    let duration = ended_dt.signed_duration_since(started_dt).num_seconds();

    conn.execute(
        "UPDATE sessions SET ended_at = ?1, duration = ?2 WHERE id = ?3",
        params![ended_dt.to_rfc3339(), duration, id],
    )?;

    println!(
        "🛑 Session, ID {} finished. Duration: {} sec.",
        id, duration
    );

    Ok(())
}