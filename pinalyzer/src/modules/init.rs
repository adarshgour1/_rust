use super::Result;
use log::debug;
use rusqlite::Connection;


// SQL statement to create the 'devices' table if it doesn't exist
const CREATE_DEVICES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    ipaddr TEXT NOT NULL
)
"#;


// SQL statement to drop the 'devices' table if it exists
const DROP_DEVICES_TABLE: &str = "DROP TABLE IF EXISTS devices";


// SQL statement to create the 'stats' table if it doesn't exist
const CREATE_STATS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS stats (
    time TEXT,
    id INTEGER,
    value INT
)
"#;


// SQL statement to drop the 'stats' table if it exists
const DROP_STATS_TABLE: &str = "DROP TABLE IF EXISTS stats";



// SQL statement to create the 'ping_data' table for storing ping statistics
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


// SQL statement to drop the 'ping_data' table if it exists
const DROP_PING_DATA_TABLE: &str = "DROP TABLE IF EXISTS ping_data";


// Create all necessary tables in the database
pub fn create_tables(conn: &Connection) -> Result<()> {
    debug!("creating tables");
    conn.execute(CREATE_DEVICES_TABLE, [])?; // Create devices table
    conn.execute(CREATE_STATS_TABLE, [])?;   // Create stats table
    conn.execute(CREATE_PING_DATA_TABLE, [])?; // Create ping_data table
    Ok(())
}

// Drop all tables from the database
pub fn drop_tables(conn: &Connection) -> Result<()> {
    debug!("dropping tables");
    conn.execute(DROP_DEVICES_TABLE, [])?;      // Drop devices table
    conn.execute(DROP_STATS_TABLE, [])?;        // Drop stats table
    conn.execute(DROP_PING_DATA_TABLE, [])?;    // Drop ping_data table
    Ok(())
}
