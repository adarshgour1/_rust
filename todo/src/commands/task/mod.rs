mod add;
mod list;
mod update;

use self::add::AddTask;
use self::list::ListTask;
use self::update::UpdateTask;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::Result;

#[derive(Parser)]
pub struct Task {
    // we can't create subcommand directly on struct
    #[command(subcommand)]
    action: Action,
}

impl Task {
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        self.action.execute(conn)
    }
}

#[derive(Subcommand)]
pub enum Action {
    /// Add a task
    Add(AddTask),
    /// List task
    List(ListTask),
    /// Update a task
    Update(UpdateTask),
}

impl Action {
    fn execute(&self, conn: &Connection) -> Result<()> {
        match self {
            Action::Add(subcmd) => subcmd.execute(conn),
            Action::List(subcmd) => subcmd.execute(conn),
            Action::Update(subcmd) => subcmd.execute(conn),
        }
    }
}
