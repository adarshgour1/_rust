use clap::{Args, Parser, Subcommand};
use rusqlite::{Connection, Result};

use crate::models::task::Task;

// trait Execute {
//     fn execute(&self, conn: &Connection) -> Result<()>;
// }

#[derive(Parser)]
pub struct CmdTask {
    // this struct act as intermiated, we can't create subcommand directly on struct
    #[command(subcommand)]
    action: Action,
}

impl CmdTask {
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
            Action::Add(add) => add.execute(conn),
            Action::List(list) => list.execute(conn),
            Action::Update(update) => update.execute(conn),
        }
    }
}

#[derive(Args)]
pub struct AddTask {
    /// Task title
    #[arg(long)]
    description: String,
    /// Flag to indicate task completed or not
    #[arg(long)]
    is_done: bool,
}

impl From<&AddTask> for Task {
    fn from(value: &AddTask) -> Self {
        Self::new(value.description.to_owned(), value.is_done)
    }
}

impl AddTask {
    fn execute(&self, conn: &Connection) -> Result<()> {
        let task = Task::from(self);
        task.save(conn)
    }
}

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
    fn execute(&self, conn: &Connection) -> Result<()> {
        if self.all {
            let tasks = Task::get_all(conn)?;
            for task in tasks {
                println!("{}", task)
            }
        } else if let Some(id) = self.id {
            let task = Task::get_by_id(&conn, id)?;
            println!("{}", task);
        }

        Ok(())
    }
}

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

impl From<&UpdateTask> for Task {
    fn from(value: &UpdateTask) -> Self {
        let mut task = Self::new(value.description.to_owned(), value.is_done);
        task.set_id(value.id);
        task
    }
}

impl UpdateTask {
    fn execute(&self, conn: &Connection) -> Result<()> {
        let task = Task::from(self);
        task.update(conn)
    }
}
