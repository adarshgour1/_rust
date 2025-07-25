
// Import submodules for commands, error handling, formatting, data models, and plugins
mod cmd;
mod error;
mod format;
mod modules;
mod plugin;

use clap::Parser;
pub use error::{Error, Result};
use log::debug;

use crate::format::{Format, Formatter};


// Path to the SQLite database file
const DB_FILE: &str = "ping-data.db";


// Command-line interface definition using clap
#[derive(Parser)]
#[command(version)]
#[command(about = "Ping Analyzer")]
#[command(long_about = r#"
Ping Analyzer

It is used to ping multiple ips at same time and collect the data in sqlite db.
Several mathamatical tools can be run to analyze the collected data."#)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command, // The subcommand to execute

    #[arg(long, short, global = true, value_enum, default_value = "table")]
    format: Format, // Output format (table or json)
}


impl Cli {
    // Execute the selected subcommand with the chosen output format
    fn execute(self) -> Result<()> {
        self.command.execute(&self.format)?;
        Ok(())
    }
}


fn main() {
    env_logger::init(); // Initialize logger (set RUST_LOG=debug for debug output)

    let cli = Cli::parse(); // Parse command-line arguments
    let format = cli.format.clone();

    // Execute the CLI and handle errors with formatted output
    if let Err(e) = cli.execute() {
        debug!("{:?}", e);
        println!("{}", e.parse(&format).unwrap())
    }
}
