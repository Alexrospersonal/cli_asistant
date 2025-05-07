use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn get_db_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("cli.db");
    path
}

pub fn open_db() -> Result<Connection> {
    let path = get_db_path();
    let conn = Connection::open(path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            label       TEXT,
            started_at  TEXT NOT NULL,
            ended_at    TEXT,
            duration    INTEGER
        )", 
        [],
    )?;
    Ok(())
}
