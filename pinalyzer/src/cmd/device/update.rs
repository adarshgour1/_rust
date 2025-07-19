use crate::{Result, cmd::device::DeviceFormatter, modules::device::Device};
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct Update {
    /// device id
    #[arg(long)]
    id: i32,

    /// device name
    #[arg(long)]
    name: String,

    /// ipv4 address
    #[arg(long)]
    ip: String,
}

impl Update {
    pub fn execute(self, conn: &Connection) -> Result<DeviceFormatter> {
        let new_device: Device = self.into();
        let device = new_device.update(conn)?;
        Ok(DeviceFormatter::One(device.into()))
    }
}

impl From<Update> for Device {
    fn from(value: Update) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            ipaddr: value.ip,
        }
    }
}
