mod device;
mod init;
mod ping;

use crate::{Result, format::Format};
use clap::Subcommand;

#[derive(Subcommand, PartialEq)]
pub enum Command {
    Init(init::InitCommand),
    Device(device::DeviceCommand),
    Ping(ping::PingCommand),
}

impl Command {
    pub fn execute(self, format: &Format) -> Result<()> {
        match self {
            Command::Init(subcommand) => subcommand.execute(),
            Command::Device(subcommand) => subcommand.execute(format),
            Command::Ping(subcommand) => subcommand.execute(format),
        }
    }
}
