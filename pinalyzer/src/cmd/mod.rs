mod device;
mod init;
mod ping;
mod error;

use crate::format::Format;
use clap::Subcommand;

pub use error::{Error, Result};

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
