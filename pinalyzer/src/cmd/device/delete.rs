use crate::{Result, modules::device::Device};
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct Delete {
    /// Device Id
    #[arg(long)]
    id: i32,
}

impl Delete {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        let device = Device::delete(conn, self.id)?;
        println!("{device}");
        Ok(())
    }
}
