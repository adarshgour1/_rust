mod device;
mod init;

use crate::{format::Format, Result};
use clap::Subcommand;
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Command {
    Init(init::InitCommand),
    Device(device::DeviceCommand),
}

impl Command {
    pub fn execute(self, conn: &Connection, format: &Format) -> Result<()> {
        match self {
            Command::Init(subcommand) => subcommand.execute(conn),
            Command::Device(subcommand) => subcommand.execute(conn, format),
        }
    }
}
