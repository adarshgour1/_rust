use rusqlite::{Connection, params};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PingStats {
    pub timestamp: String, // RFC 3339 and ISO 8601 date and time string such as 1996-12-19T16:39:57-08:00
    pub device_id: i32,
    pub transmitted: u32,
    pub received: u32,
    pub loss: f32,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub avg: Option<f64>,
}

impl PingStats {
    pub fn new(
        timestamp: String,
        device_id: i32,
        transmitted: u32,
        received: u32,
        loss: f32,
        min: Option<f64>,
        max: Option<f64>,
        avg: Option<f64>,
    ) -> Self {
        PingStats {
            timestamp,
            device_id,
            transmitted,
            received,
            loss,
            min,
            max,
            avg,
        }
    }

    pub fn save(&self, conn: &Connection) -> crate::Result<()> {
        conn.execute(
            "INSERT INTO ping_data (timestamp, device_id, transmitted, received, loss, min, max, avg) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                self.timestamp,
                self.device_id,
                self.transmitted,
                self.received,
                self.loss,
                self.min,
                self.max,
                self.avg
            ],
        )?;
        Ok(())
    }

    pub fn get(conn: &Connection, device_id: i32, limit: usize) -> crate::Result<Vec<PingStats>> {
        let mut stmt = conn.prepare("SELECT timestamp, device_id, transmitted, received, loss, min, max, avg FROM ping_data WHERE device_id = ? ORDER BY timestamp DESC LIMIT ?")?;
        let rows = stmt.query_map(params![device_id, limit], |row| {
            Ok(PingStats {
                timestamp: row.get(0)?,
                device_id: row.get(1)?,
                transmitted: row.get(2)?,
                received: row.get(3)?,
                loss: row.get(4)?,
                min: row.get(5)?,
                max: row.get(6)?,
                avg: row.get(7)?,
            })
        })?;

        let mut stats = Vec::new();
        for row in rows {
            stats.push(row?);
        }
        Ok(stats)
    }
}

impl std::fmt::Display for PingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "timestamp: {}, Device ID: {}, Transmitted: {}, Received: {}, Loss: {:.2}%, Min: {}, Max: {}, Avg: {}",
            self.timestamp,
            self.device_id,
            self.transmitted,
            self.received,
            self.loss * 100.0,
            self.min.map(|v| v.to_string()).unwrap_or("N/A".to_string()),
            self.max.map(|v| v.to_string()).unwrap_or("N/A".to_string()),
            self.avg.map(|v| v.to_string()).unwrap_or("N/A".to_string())
        )
    }
}
