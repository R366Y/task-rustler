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
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> TasksService {
        let tasks = TasksService::default();
        let tasks_to_add = vec![
            Task::new(1, "First task".to_string()),
            Task::new(2, "Second task".to_string()),
            Task::new(3, "Third task".to_string()),
        ];
        for t in tasks_to_add {
            tasks.add_task(t.description);
        }
        tasks
    }

    fn teardown(t: &TasksService) {
        t.db.clear();
    }

    #[test]
    fn get_all_tasks() {
        let t = setup();
        assert_eq!(t.length(), 3);
        teardown(&t);
    }
    #[test]
    fn should_return_task_if_id_exists() {
        let t = setup();
        t.add_task("Hi".to_string());
        let task = t.get_task(4).unwrap();
        assert_eq!(task.id, 4);
        assert_eq!(task.description, "Hi");
        assert_eq!(task.completed, false);
    }
    #[test]
    fn should_return_an_error_if_task_is_not_found() {
        let t = setup();
        let task = t.get_task(100);
        assert_eq!(task.is_none(), true);

    }
    #[test]
    fn set_completed_should_return_1_if_task_exists_0_otherwise() {
        let t = setup();
        let num_tasks_completed = t.mark_completed(1);
        assert_eq!(num_tasks_completed, 1);
        let num_tasks_completed = t.mark_completed(100);
        assert_eq!(num_tasks_completed, 0);
    }

    #[test]
    fn delete_task_should_return_1_if_task_exists_0_otherwise() {
        let t = setup();
        let num_task_removed = t.delete_task(2);
        assert_eq!(num_task_removed, 1);
        let num_task_removed = t.delete_task(100);
        assert_eq!(num_task_removed, 0);
    }
}
