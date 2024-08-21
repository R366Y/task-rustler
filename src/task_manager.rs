use crate::task_db::DB;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
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
        }
    }
}

impl Default for TasksService {
    fn default() -> Self {
        Self::new(&String::new())
    }
}

impl TasksService {
    pub fn new(db_path: &String) -> Self {
        Self {
            db: DB::create_and_return_connection(db_path),
            db_filename: db_path.to_string(),
        }
    }

    pub fn add_task(&self, description: String) {
        self.db.add_task(&description)
    }

    pub fn get_task(&self, task_id: i32) -> Option<Task> {
        match self.db.get_task_by_id(task_id){
            Ok(t)=> Some(t),
            Err(e) => {
                eprintln!("{e}");
                if let Some(cause) = e.source(){
                    eprintln!("Underlying cause: {:?}", cause);
                }
                None
            }
        }
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.db.get_all_tasks()
    }

    pub fn mark_completed(&self, task_id: i32) -> usize {
        self.db.set_task_completed(task_id)
    }

    pub fn delete_task(&self, task_id: i32) -> usize {
        self.db.delete_task(task_id)
    }

    pub fn length(&self) -> i64 {
        self.db.get_record_count()
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    pub fn delete_all_tasks(&self) -> usize {
        self.db.clear()
    }
}
