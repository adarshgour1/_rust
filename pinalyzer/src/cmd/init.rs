use super::Result;
use clap::Parser;
use rusqlite::Connection;

use crate::modules::init::{create_tables, drop_tables};

#[derive(Parser)]
pub struct InitCommand {
    /// It will delete entire database and recreate it
    #[arg(long)]
    recreate: bool,
}

impl InitCommand {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        if self.recreate {
            drop_tables(conn)?;
        }
        create_tables(conn)
    }
}
