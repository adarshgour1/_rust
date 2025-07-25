use clap::Parser;
use rusqlite::{Connection, OpenFlags};

use super::Result;
use crate::modules::init::{create_tables, drop_tables};

#[derive(Parser, PartialEq)]
pub struct InitCommand {
    /// It will delete entire database and recreate it
    #[arg(long)]
    recreate: bool,
}

impl InitCommand {
    pub fn execute(&self) -> Result<()> {
        let conn = Connection::open_with_flags(
            crate::DB_FILE,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        if self.recreate {
            drop_tables(&conn)?;
        }
        Ok(create_tables(&conn)?)
    }
}
