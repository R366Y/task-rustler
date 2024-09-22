use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::task_db::DB;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {

    pub fn next(&self) -> Self {
        match self {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Low,
        }
    }

    pub fn from_u8(value: u8) -> Option<Priority> {
        match value {
            0 => Some(Priority::Low),
            1 => Some(Priority::Medium),
            2 => Some(Priority::High),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
        }
    }
}

impl From<&str> for Priority {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "L" => Priority::Low,
            "M" => Priority::Medium,
            "H" => Priority::High,
            _ => unreachable!("Invalid priority",),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "L"),
            Priority::Medium => write!(f, "M"),
            Priority::High => write!(f, "H"),
        }
    }
}

impl PartialOrd<Self> for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
}

pub enum SortOrder {
    High,
    Low,
}

#[allow(dead_code)]
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

    pub fn add_task_with_priority(&self, description: String, priority: Priority) {
        self.db.add_task_with_priority(&description, priority)
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

    /// Return all the tasks sorted by `sort`
    pub fn get_all_tasks_sorted(&self, sort:SortOrder) -> Vec<Task> {
        match sort {
            SortOrder::High => self.db.get_all_task_by_highest_priority(),
            SortOrder::Low => self.db.get_all_task_by_lowest_priority(),
        }
    }

    /// Mark a task completed with `task_id` number
    pub fn mark_completed(&self, task_id: i32) -> usize {
        self.db.set_task_completed(task_id)
    }

    /// Change priority of the task
    pub fn change_priortiy(&self, task_id: i32, priority: Priority) -> usize {
        self.db.update_task_priority(task_id, priority)
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
