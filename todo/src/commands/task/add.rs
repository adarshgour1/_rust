use crate::Result;
use crate::models::Task;
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct AddTask {
    /// Task title
    #[arg(long)]
    description: String,
    /// Flag to indicate task completed or not
    #[arg(long)]
    is_done: bool,
}

impl AddTask {
    pub(super) fn execute(&self, conn: &Connection) -> Result<()> {
        let task = Task {
            id: None,
            description: self.description.clone(),
            is_done: self.is_done,
        };
        task.save(conn)
    }
}
