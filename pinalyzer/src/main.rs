mod cmd;
mod error;
mod format;
mod modules;
mod plugin;

use clap::Parser;
pub use error::{Error, Result};
use log::debug;

use crate::format::{Format, Formatter};

const DB_FILE: &str = "ping-data.db";

#[derive(Parser)]
#[command(version)]
#[command(about = "Ping Analyzer")]
#[command(long_about = r#"
Ping Analyzer

It is used to ping multiple ips at same time and collect the data in sqlite db.
Several mathamatical tools can be run to analyze the collected data."#)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command,

    #[arg(long, short, global = true, value_enum, default_value = "table")]
    format: Format,
}

impl Cli {
    fn execute(self) -> Result<()> {
        self.command.execute(&self.format)?;
        Ok(())
    }
}

fn main() {
    env_logger::init(); // RUST_LOG=debug

    let cli = Cli::parse();
    let format = cli.format.clone();

    if let Err(e) = cli.execute() {
        debug!("{:?}", e);
        println!("{}", e.parse(&format).unwrap())
    }
}
