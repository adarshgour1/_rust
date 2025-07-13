use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

// use crate::models::task::Task;

mod commands;
mod db;
mod models;

#[derive(Parser)]
#[command(version)]
#[command(about="Cli tool to manage tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Task specific operation
    Task(commands::task::CmdTask),
}

fn main() {
    let cli = Cli::parse();
    let conn = db::get_db_connection().unwrap();

    println!("{:?}", cli.execute(&conn));
}

impl Commands {
    fn execute(&self, conn: &Connection) -> Result<()> {
        match self {
            Commands::Task(task) => task.execute(conn),
        }
    }
}

impl Cli {
    fn execute(&self, conn: &Connection) -> Result<()> {
        self.commands.execute(conn)
    }
}
