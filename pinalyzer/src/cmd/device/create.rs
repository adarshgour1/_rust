use crate::{cmd::device::DeviceFormatter, modules::device::Device, Result};
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct Create {
    /// device name
    #[arg(long)]
    name: String,

    /// ipv4 address
    #[arg(long)]
    ip: String,
}

impl Create {
    pub fn execute(self, conn: &Connection) -> Result<DeviceFormatter> {
        let device: Device = self.into();
        let device = device.save(conn)?;
        Ok(DeviceFormatter::One(device.into()))
    }
}

impl From<Create> for Device {
    fn from(value: Create) -> Self {
        Self {
            id: None,
            name: value.name,
            ipaddr: value.ip,
        }
    }
}
