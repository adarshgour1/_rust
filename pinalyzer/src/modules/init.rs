use super::Result;
use log::debug;
use rusqlite::Connection;

const CREATE_DEVICES_TABLE: &str = "CREATE TABLE IF NOT EXISTS devices (
id INTEGER PRIMARY KEY,
name TEXT NOT NULL,
ipaddr TEXT NOT NULL
)";

const DROP_DEVICES_TABLE: &str = "DROP TABLE IF EXISTS devices";

const CREATE_STATS_TABLE: &str = "CREATE TABLE IF NOT EXISTS stats (
time TEXT,
id INTEGER,
value INT)";

const DROP_STATS_TABLE: &str = "DROP TABLE IF EXISTS stats";

pub fn create_tables(conn: &Connection) -> Result<()> {
    debug!("creating tables");
    conn.execute(CREATE_DEVICES_TABLE, [])?;
    conn.execute(CREATE_STATS_TABLE, [])?;

    Ok(())
}
pub fn drop_tables(conn: &Connection) -> Result<()> {
    debug!("dropping tables");
    conn.execute(DROP_DEVICES_TABLE, [])?;
    conn.execute(DROP_STATS_TABLE, [])?;
    Ok(())
}
