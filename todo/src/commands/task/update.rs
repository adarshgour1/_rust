use crate::Result;
use crate::models::Task;
use clap::Args;
use rusqlite::Connection;

#[derive(Args)]
pub struct UpdateTask {
    /// Id of the task
    #[arg(long)]
    id: i32,
    /// Task title
    #[arg(long)]
    description: String,
    /// Flag to indicate task completed or not
    #[arg(long)]
    is_done: bool,
}

impl UpdateTask {
    pub(super) fn execute(&self, conn: &Connection) -> Result<()> {
        let task = Task {
            id: Some(self.id),
            description: self.description.clone(),
            is_done: self.is_done,
        };
        task.update(conn)
    }
}
