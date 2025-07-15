mod init;

use crate::Result;
use clap::Subcommand;
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Command {
    Init(init::InitCommand),
}

impl Command {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        match self {
            Command::Init(subcommand) => subcommand.execute(conn),
        }
    }
}
