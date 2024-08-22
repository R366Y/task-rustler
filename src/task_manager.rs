use crate::task_db::DB;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Priority {
    High = 1,
    Medium = 2,
    Low = 3,
}

impl Priority {
    pub fn from_u8(value: u8) -> Option<Priority> {
        match value {
            1 => Some(Priority::High),
            2 => Some(Priority::Medium),
            3 => Some(Priority::Low),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
}

#[derive(Debug)]
pub struct TasksService {
    db: DB,
    db_filename: String,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            priority: Priority::Medium,
        }
    }
}

impl Default for TasksService {
    fn default() -> Self {
        Self::new(&String::new())
    }
}

impl TasksService {
    /// Create a new database
    /// `db_path` is the path to the database file. If it doesn't exist it is going to be created.
    /// If db_path is an empty string __""__, an in memory instance of database is going to be
    /// created instead.
    pub fn new(db_path: &String) -> Self {
        Self {
            db: DB::create_and_return_connection(db_path),
            db_filename: db_path.to_string(),
        }
    }

    /// Add a new task
    pub fn add_task(&self, description: String) {
        self.db.add_task(&description)
    }

    /// Get a task with `task_id`. Returns an Option containing the task or None
    /// if it couldn't find the task.
    pub fn get_task(&self, task_id: i32) -> Option<Task> {
        match self.db.get_task_by_id(task_id) {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("{e}");
                if let Some(cause) = e.source() {
                    eprintln!("Underlying cause: {:?}", cause);
                }
                None
            }
        }
    }

    /// Returns all the tasks present in the database
    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.db.get_all_tasks()
    }

    /// Mark a task completed with `task_id` number
    pub fn mark_completed(&self, task_id: i32) -> usize {
        self.db.set_task_completed(task_id)
    }

    /// Delete a task with `task_id` number
    pub fn delete_task(&self, task_id: i32) -> usize {
        self.db.delete_task(task_id)
    }

    /// Number of tasks present in the database
    pub fn length(&self) -> i64 {
        self.db.get_record_count()
    }

    /// Check if there are no tasks at all
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Clean the database, delete all tasks
    pub fn delete_all_tasks(&self) -> usize {
        self.db.clear()
    }
}
