mod cmd;
mod error;
mod format;
mod modules;

use clap::Parser;
pub use error::{Error, Result};
use rusqlite::Connection;

use crate::format::Format;

#[derive(Parser)]
#[command(version)]
#[command(about="Ping Analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command,

    #[arg(long, short, global = true, value_enum, default_value = "table")]
    format: Format,
}

impl Cli {
    fn execute(self, conn: &Connection) -> Result<()> {
        self.command.execute(conn, &self.format)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init(); // RUST_LOG=debug
    let db = Connection::open("./ping-data.db")?;
    let cli = Cli::parse();
    cli.execute(&db)
}
