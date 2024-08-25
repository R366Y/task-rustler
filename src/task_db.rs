use crate::task_manager::{Priority, Task};
use anyhow::{Context, Result};
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct DB {
    connection: Connection,
}

impl DB {
    /// Create and return a connection to a database located at path
    /// if path is an empty string creates and in memory db instance
    pub fn create_and_return_connection(path: &str) -> DB {
        let conn: Connection = if path.is_empty() {
            Connection::open_in_memory().unwrap()
        } else {
            Connection::open(path).unwrap()
        };
        let mut db = DB { connection: conn };
        db.init();
        db
    }

    fn init(&mut self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL,
            priority INTEGER NOT NULL
        )",
                [],
            )
            .unwrap();
    }

    pub fn add_task(&self, description: &str) {
        self.connection
            .execute(
                "INSERT INTO tasks (description, completed, priority) VALUES (?1, 0, 2)",
                params![description.trim()],
            )
            .unwrap();
    }

    pub fn add_task_with_priority(&self, description: &str, priority: Priority) {
        self.connection
            .execute(
                "INSERT INTO tasks (description, completed, priority) VALUES (?1, 0, ?2)",
                params![description.trim(), priority as u8],
            )
            .unwrap();
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, description, completed, priority FROM tasks")
            .unwrap();
        let task_row_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    completed: row.get(2)?,
                    priority: Priority::from_u8(row.get(3)?).unwrap(),
                })
            })
            .unwrap();
        let mut tasks = Vec::new();
        for task in task_row_iter {
            tasks.push(task.unwrap());
        }
        tasks
    }

    pub fn get_task_by_id(&self, task_id: i32) -> Result<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, description, completed, priority FROM tasks where id = ?1")?;
        stmt.query_row(params![task_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get(2)?,
                priority: Priority::from_u8(row.get(3)?).unwrap(),
            })
        })
        .with_context(|| format!("Couldn't get task at index {task_id}"))
    }

    pub fn get_all_task_by_highest_priority(&self) -> Vec<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, description, completed, priority FROM tasks order by priority desc")
            .unwrap();
        let task_row_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    completed: row.get(2)?,
                    priority: Priority::from_u8(row.get(3)?).unwrap(),
                })
            })
            .unwrap();
        let mut tasks = Vec::new();
        for task in task_row_iter {
            tasks.push(task.unwrap());
        }
        tasks
    }

    pub fn get_all_task_by_lowest_priority(&self) -> Vec<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, description, completed, priority FROM tasks order by priority asc")
            .unwrap();
        let task_row_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    completed: row.get(2)?,
                    priority: Priority::from_u8(row.get(3)?).unwrap(),
                })
            })
            .unwrap();
        let mut tasks = Vec::new();
        for task in task_row_iter {
            tasks.push(task.unwrap());
        }
        tasks
    }

    pub fn set_task_completed(&self, task_id: i32) -> usize {
        self.connection
            .execute(
                "UPDATE tasks SET completed = 1 WHERE id = ?1",
                params![task_id],
            )
            .unwrap()
    }

    pub fn update_task_priority(&self, task_id: i32, priority: Priority) -> usize{
        self.connection
            .execute(
                "UPDATE tasks SET priority = ?2 WHERE id = ?1",
                params![task_id, priority as u8],
            )
            .unwrap()
    }

    pub fn delete_task(&self, task_id: i32) -> usize {
        self.connection
            .execute("delete from tasks where id = ?1", params![task_id])
            .unwrap()
    }

    pub fn get_record_count(&self) -> i64 {
        let query = "SELECT count(*) FROM tasks";
        self.connection.query_row(query, [], |r| r.get(0)).unwrap()
    }

    pub fn clear(&self) -> usize {
        self.connection.execute("DELETE FROM tasks", []).unwrap()
    }
}
