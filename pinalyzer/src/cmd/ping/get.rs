use crate::{
    cmd::ping::PingDataFormatter,
    format::{Format, Formatter},
};
use clap::Args;
use rusqlite::Connection;
use super::{Result};

#[derive(Debug, Args, PartialEq)]
pub struct Get {
    /// deivce id
    #[arg(long)]
    id: i32,

    /// limit the number of results
    #[arg(long, default_value_t = 10)]
    limit: usize,
}

impl Get {
    pub fn execute(&self, conn: &Connection, format: &Format) -> Result<()> {
        let ping_stats = crate::modules::ping::PingStats::get(&conn, self.id, self.limit)?;

        let formatter = PingDataFormatter::Many(ping_stats);
        println!("{}", formatter.parse(format)?);

        Ok(())
    }
}
