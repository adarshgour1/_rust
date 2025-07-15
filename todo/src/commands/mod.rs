mod task;

//------------------------------------- Command ------------------------------------

use crate::Result;
use clap::Subcommand;
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Commands {
    /// Task specific operation
    Task(task::Task),
}

impl Commands {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        match self {
            Commands::Task(subcmd) => subcmd.execute(conn),
        }
    }
}
