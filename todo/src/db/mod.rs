use rusqlite::{Connection, Result};

pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open("./test.db")?;
    // println!("testing-db: {}", conn.is_autocommit());

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            is_done BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}
