mod create;
mod delete;
mod get;
mod update;

use crate::format::{Format, Formatter};

// external
use super::Result;
use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;

// ------------------------------- command section ------------------------------------
#[derive(Parser, PartialEq)]
pub struct DeviceCommand {
    #[command[subcommand]]
    command: Command,
}

impl DeviceCommand {
    pub fn execute(self, format: &Format) -> Result<()> {
        let conn = Connection::open_with_flags(
            crate::DB_FILE,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI,
        )?;

        let formatter = self.command.execute(&conn)?;
        println!("{}", formatter.parse(format)?);
        Ok(())
    }
}

#[derive(Subcommand, PartialEq)]
pub enum Command {
    Get(get::Get),
    Create(create::Create),
    Update(update::Update),
    Delete(delete::Delete),
}

impl Command {
    pub fn execute(self, conn: &Connection) -> Result<DeviceFormatter> {
        match self {
            Self::Get(x) => x.execute(conn),
            Self::Create(x) => x.execute(conn),
            Self::Update(x) => x.execute(conn),
            Self::Delete(x) => x.execute(conn),
        }
    }
}

//----------------------------------- DeviceFormatter ----------------------------------------------------
#[derive(Debug, Serialize)]
pub struct Device {
    id: String,
    name: String,
    ipaddr: String,
}

impl From<crate::modules::device::Device> for Device {
    fn from(value: crate::modules::device::Device) -> Self {
        let id = if let Some(id) = value.id {
            format!("{id}")
        } else {
            String::new()
        };
        Self {
            id,
            name: value.name,
            ipaddr: value.ipaddr,
        }
    }
}

#[derive(Debug)]
pub enum DeviceFormatter {
    One(Device),
    Many(Vec<Device>),
}

impl Formatter for DeviceFormatter {
    fn json(&self) -> crate::format::Result {
        match self {
            Self::One(device) => Ok(serde_json::to_string(device)?),
            Self::Many(devices) => Ok(serde_json::to_string(devices)?),
        }
    }

    fn table(&self) -> crate::format::Result {
        let mut table = Table::new();
        table.set_titles(row!["id", "name", "ipaddr"]);
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        match self {
            Self::One(device) => {
                table.add_row(row![device.id, device.name, device.ipaddr,]);
            }
            Self::Many(devices) => {
                for d in devices.iter() {
                    table.add_row(row![d.id, d.name, d.ipaddr,]);
                }
            }
        }
        Ok(table.to_string())
    }
}
