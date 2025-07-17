use crate::{format::Formatter, modules::device::Device, Result};
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
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
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        if self.all {
            let devices = Device::get_all(conn)?;
            println!("{}", devices.table().unwrap());
        } else if let Some(id) = self.id {
            let device = Device::get_by_id(conn, id)?;
            println!("{}", device.table().unwrap());
            }
        Ok(())
    }
}
