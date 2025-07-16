mod create;
mod delete;
mod get;
mod update;

// external
use super::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct DeviceCommand {
    #[command[subcommand]]
    command: Command,
}

impl DeviceCommand {
    pub fn execute(self, conn: &Connection) -> Result<()> {
        self.command.execute(conn)
    }
}

#[derive(Subcommand)]
pub enum Command {
    Get(get::Get),
    Create(create::Create),
    Update(update::Update),
    Delete(delete::Delete),
}

impl Command {
    pub fn execute(self, conn: &Connection) -> Result<()> {
        match self {
            Self::Get(x) => x.execute(conn),
            Self::Create(x) => x.execute(conn),
            Self::Update(x) => x.execute(conn),
            Self::Delete(x) => x.execute(conn),
        }
    }
}
