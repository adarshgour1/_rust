mod collect;
mod get;
use crate::format::{Format, Formatter};
use crate::{Result, modules::ping::PingStats};
use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use rusqlite::{Connection, OpenFlags};

#[derive(Parser, PartialEq)]
pub struct PingCommand {
    #[command[subcommand]]
    command: Command,
}

impl PingCommand {
    pub fn execute(self, format: &Format) -> Result<()> {
        let conn = Connection::open_with_flags(
            crate::DB_FILE,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI,
        )?;

        self.command.execute(&conn, format)?;
        Ok(())
    }
}

#[derive(Subcommand, PartialEq)]
pub enum Command {
    Collect(collect::Collect),
    Get(get::Get),
}

impl Command {
    pub fn execute(self, conn: &Connection, format: &Format) -> Result<()> {
        match self {
            Self::Collect(x) => x.execute(conn),
            Self::Get(x) => x.execute(conn, format),
        }
    }
}

#[derive(Debug)]
pub enum PingDataFormatter {
    One(PingStats),
    Many(Vec<PingStats>),
}

impl Formatter for PingDataFormatter {
    fn json(&self) -> crate::format::Result {
        match self {
            Self::One(data) => Ok(serde_json::to_string(data)?),
            Self::Many(data) => Ok(serde_json::to_string(data)?),
        }
    }

    fn table(&self) -> crate::format::Result {
        let mut table = Table::new();
        table.set_titles(row![
            "timestamp",
            "device_id",
            "transmitted",
            "received",
            "loss (%)",
            "min",
            "max",
            "avg"
        ]);
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        match self {
            Self::One(ping_stats) => {
                table.add_row(row![
                    ping_stats.timestamp,
                    ping_stats.device_id,
                    ping_stats.transmitted,
                    ping_stats.received,
                    format!("{:.2}", ping_stats.loss * 100.0),
                    ping_stats.min.map_or("N/A".to_string(), |v| v.to_string()),
                    ping_stats.max.map_or("N/A".to_string(), |v| v.to_string()),
                    ping_stats.avg.map_or("N/A".to_string(), |v| v.to_string()),
                ]);
            }
            Self::Many(ping_stats) => {
                for s in ping_stats.iter() {
                    table.add_row(row![
                        s.timestamp,
                        s.device_id,
                        s.transmitted,
                        s.received,
                        format!("{:.2}", s.loss * 100.0),
                        s.min.map_or("N/A".to_string(), |v| v.to_string()),
                        s.max.map_or("N/A".to_string(), |v| v.to_string()),
                        s.avg.map_or("N/A".to_string(), |v| v.to_string()),
                    ]);
                }
            }
        }
        Ok(table.to_string())
    }
}
