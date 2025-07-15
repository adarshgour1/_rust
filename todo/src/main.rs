use clap::Parser;
use error::Result;
use rusqlite::Connection;

mod commands;
mod db;
mod error;
mod models;

#[derive(Parser)]
#[command(version)]
#[command(about="Cli tool to manage tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: commands::Commands,
}

impl Cli {
    fn execute(&self, conn: &Connection) -> Result<()> {
        self.commands.execute(conn)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = db::get_db_connection()?;
    let result = cli.execute(&conn);
    if let Err(e) = result {
        println!("error: {e}");
    }
    Ok(())
}
