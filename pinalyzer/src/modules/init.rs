use super::Result;
use log::debug;
use rusqlite::Connection;

const CREATE_DEVICES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    ipaddr TEXT NOT NULL
)
"#;

const DROP_DEVICES_TABLE: &str = "DROP TABLE IF EXISTS devices";

const CREATE_STATS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS stats (
    time TEXT,
    id INTEGER,
    value INT
)
"#;

const DROP_STATS_TABLE: &str = "DROP TABLE IF EXISTS stats";


const CREATE_PING_DATA_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS ping_data (
    id INTEGER PRIMARY KEY,
    device_id INTEGER NOT NULL,
    timestamp TEXT NOT NULL,
    transmitted INTEGER,
    received INTEGER,
    loss REAL,
    min REAL,
    max REAL,
    avg REAL,
    FOREIGN KEY (device_id) REFERENCES devices(id)
)
"#;

const DROP_PING_DATA_TABLE: &str = "DROP TABLE IF EXISTS ping_data";

pub fn create_tables(conn: &Connection) -> Result<()> {
    debug!("creating tables");
    conn.execute(CREATE_DEVICES_TABLE, [])?;
    conn.execute(CREATE_STATS_TABLE, [])?;
    conn.execute(CREATE_PING_DATA_TABLE, [])?;

    Ok(())
}
pub fn drop_tables(conn: &Connection) -> Result<()> {
    debug!("dropping tables");
    conn.execute(DROP_DEVICES_TABLE, [])?;
    conn.execute(DROP_STATS_TABLE, [])?;
    conn.execute(DROP_PING_DATA_TABLE, [])?;
    Ok(())
}
