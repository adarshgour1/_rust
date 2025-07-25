use super::{DeviceFormatter, Result};
use crate::modules::Device;

use clap::Args;
use rusqlite::Connection;

#[derive(Args, PartialEq)]
pub struct Delete {
    /// Device Id
    #[arg(long)]
    id: i32,
}

impl Delete {
    pub fn execute(&self, conn: &Connection) -> Result<DeviceFormatter> {
        let device = Device::delete(conn, self.id)?;
        Ok(DeviceFormatter::One(device.into()))
    }
}
