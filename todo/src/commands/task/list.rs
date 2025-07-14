use crate::Result;
use crate::models::Task;
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct ListTask {
    /// Task id to list
    #[arg(long, group = "list")]
    id: Option<i32>,
    /// List all task
    #[arg(long, group = "list")]
    all: bool,
}

impl ListTask {
    pub(super) fn execute(&self, conn: &Connection) -> Result<()> {
        if self.all {
            let tasks = Task::get_all(conn)?;
            for task in tasks {
                println!("{task}")
            }
        } else if let Some(id) = self.id {
            let task = Task::get_by_id(conn, id)?;
            println!("{task}");
        }

        Ok(())
    }
}
