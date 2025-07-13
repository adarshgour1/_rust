use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Task {
    id: Option<i32>,
    description: String,
    is_done: bool,
}

impl Task {
    pub fn new(description: String, is_done: bool) -> Self {
        Self {
            id: None,
            description,
            is_done,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn save(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO task (description, is_done) VALUES  (?1, ?2)",
            params![&self.description, &self.is_done],
        )?;
        Ok(())
    }
    pub fn update(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "UPDATE task set description = ?1, is_done = ?2 WHERE id = ?3",
            params![self.description, self.is_done, self.id],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i32) -> Result<Self> {
        conn.query_one(
            "SELECT id, description, is_done FROM task WHERE id = ?1",
            [&id],
            |row| {
                Ok(Self {
                    id: Some(row.get(0)?),
                    description: row.get(1)?,
                    is_done: row.get(2)?,
                })
            },
        )
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Self>> {
        let mut statement = conn.prepare("SELECT id, description, is_done from task")?;

        let task_iter = statement.query_map([], |row| {
            Ok(Self {
                id: Some(row.get(0)?),
                description: row.get(1)?,
                is_done: row.get(2)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.id {
            write!(f, "id: {}, ", id)?;
        }
        write!(
            f,
            "description: {}, is_done: {}",
            self.description, self.is_done
        )
    }
}
