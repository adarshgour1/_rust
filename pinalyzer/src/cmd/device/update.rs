use crate::Result;
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct Update {

}

impl Update {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        Ok(())
    }
}