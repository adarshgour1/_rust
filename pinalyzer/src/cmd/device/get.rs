use crate::{Result, cmd::device::DeviceFormatter, modules::device::Device};
use clap::Args;
use rusqlite::Connection;

#[derive(Args, PartialEq)]
#[group(required = true, multiple = false)]
pub struct Get {
    /// Device Id
    #[arg(long, group = "list")]
    id: Option<i32>,

    /// Use to get all devices
    #[arg(long, group = "list")]
    all: bool,
}

impl Get {
    pub fn execute(&self, conn: &Connection) -> Result<DeviceFormatter> {
        let formatter = if let Some(id) = self.id {
            let device = Device::get_by_id(conn, id)?;
            DeviceFormatter::One(device.into())
        } else {
            let devices = Device::get_all(conn)?;
            DeviceFormatter::Many(devices.into_iter().map(Device::into).collect())
        };
        Ok(formatter)
    }
}
