use std::net::Ipv4Addr;

use rusqlite::{Connection, params};
use serde::Serialize;

use super::{Result};


// Represents a network device entry in the database
#[derive(Debug, Serialize)]
pub struct Device {
    pub id: Option<i32>,   // Device ID (primary key, optional for new devices)
    pub name: String,      // Device name
    pub ipaddr: String,    // Device IP address as string
}

impl Device {
    // Save a new device to the database and return the inserted device
    pub fn save(&self, conn: &Connection) -> Result<Device> {
        let ip: Ipv4Addr = self.ipaddr.parse()?;

        let device = conn.query_one(
            "INSERT INTO devices(name, ipaddr) VALUES (?1, ?2) RETURNING id, name, ipaddr",
            params![self.name, ip.to_string()],
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

    // Delete a device by ID and return the deleted device
    pub fn delete(conn: &Connection, id: i32) -> Result<Device> {
        let device = conn.query_one(
            "DELETE FROM devices WHERE id = ?1 RETURNING id, name, ipaddr",
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

    // Update an existing device and return the updated device
    pub fn update(&self, conn: &Connection) -> Result<Device> {
        let ip: Ipv4Addr = self.ipaddr.parse()?;

        let device = conn.query_one(
            "UPDATE devices set name=?1, ipaddr=?2 where id = ?3 RETURNING id, name, ipaddr",
            params![self.name, ip.to_string(), self.id],
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
