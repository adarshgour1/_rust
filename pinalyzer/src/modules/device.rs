use rusqlite::{Connection, params};

use crate::Result;

#[derive(Debug)]
pub struct Device {
    pub id: Option<i32>,
    pub name: String,
    pub ipaddr: String,
}

impl Device {
    pub fn save(&self, conn: &Connection) -> Result<Device> {
        let device = conn.query_one(
            "INSERT INTO devices(name, ipaddr) VALUES (?1, ?2) RETURNING id, name, ipaddr",
            params![self.name, self.ipaddr],
            |row| {
                Ok(Self {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    ipaddr: row.get(2)?,
                })
            },
        );

        Ok(device?)
    }

    pub fn get_by_id(conn: &Connection, id: i32) -> Result<Device> {
        let device = conn.query_one(
            "SELECT id, name, ipaddr FROM devices WHERE id = ?1",
            params![id],
            |row| {
                Ok(Self {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    ipaddr: row.get(2)?,
                })
            },
        );

        Ok(device?)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Device>> {
        let mut statement = conn.prepare("SELECT id, name, ipaddr FROM devices")?;

        let device_iter = statement.query_map([], |row| {
            Ok(Self {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                ipaddr: row.get(2)?,
            })
        })?;

        let mut devices = Vec::new();
        for d in device_iter {
            devices.push(d?);
        }
        Ok(devices)
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
