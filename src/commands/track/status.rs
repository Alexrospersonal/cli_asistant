use std::error::Error;
use chrono::{DateTime, Utc};
use rusqlite::Statement;
use crate::commands::track::db::open_db;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let conn = open_db()?;
    
    let stmt = conn.prepare("SELECT label, started_at FROM sessions WHERE ended_at IS NULL ORDER BY started_at DESC LIMIT 1")?;
    
    match get_data_from_row(stmt)? {
        None => return Ok(()),
        Some((label, started_at)) => {
            let formated_time = get_formated_time_as_string(&started_at);
            
            println!("🟢 Session is active [{label}]\
                \nStarting at: {started_at}\
                \n🕒 Duration: {formated_time}"
            );
        }
    }
    
    Ok(())
}

fn get_data_from_row(mut stmt: Statement) -> Result<Option<(String, DateTime<Utc>)>, Box<dyn Error>> {
    let row_result = stmt.query_row([], |row| {
        let label: Option<String> = row.get(0)?;
        let started_at: String = row.get(1)?;

        Ok((label, started_at))
    });

    match row_result {
        Ok((label, started_at)) => {
            let label = label.unwrap_or_else(|| "unnamed".to_string());
            let started_at: DateTime<Utc> = started_at.parse()?;
            Ok(Some((label, started_at)))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("⚠️  No active session found.");
            Ok(None)
        }
        Err(e) => Err(Box::new(e))
    }
}

fn get_formated_time_as_string(started_at: &DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(started_at);
    let total_sec = duration.num_seconds();

    let hour = total_sec / 3600;
    let minute = (total_sec % 3600) / 60;
    let seconds = total_sec % 60;

    format!("H:{hour} m:{minute} s:{seconds}")
}