use crate::Result;
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct Delete {

}

impl Delete {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        Ok(())
    }
}