mod cmd;
mod error;
mod modules;

use clap::Parser;
pub use error::{Error, Result};
use rusqlite::Connection;

#[derive(Parser)]
#[command(version)]
#[command(about="Ping Analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command,
}

impl Cli {
    fn execute(self, conn: &Connection) -> Result<()> {
        self.command.execute(conn)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init(); // RUST_LOG=debug
    let db = Connection::open("./ping-data.db")?;
    let cli = Cli::parse();
    cli.execute(&db)
}
