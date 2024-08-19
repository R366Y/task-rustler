use crate::task_db;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct Tasks {
    connection: Connection,
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

impl Default for Tasks {
    fn default() -> Self {
        Self::new()
    }
}

impl Tasks {
    pub fn new() -> Self {
        Self {
            connection: task_db::create_and_return_connection("tasks.db").expect("Couldn't create or open database"),
        }
    }

    pub fn add_task(&self, description: String) {
        task_db::insert_task(&self.connection, &description).expect("Couldn't insert task")
    }

    pub fn get_task(&self, task_id: i32) -> Option<Task> {
        task_db::get_task(&self.connection, task_id).expect("Couldn't retrieve task")
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        task_db::get_all_tasks(&self.connection).expect("Couldn't retrieve tasks")
    }

    pub fn mark_completed(&mut self, task_id: i32) -> usize{
        task_db::set_task_completed(&self.connection, task_id).expect("Couldn't update database")
    }

    pub fn delete_task(&mut self, task_id: i32) -> usize{
        task_db::delete_task(&self.connection, task_id).expect("Couldn't update database")
    }

    pub fn length(&self) -> usize {
        self.get_all_tasks().len()
    }

    pub fn is_empty(&self) -> bool {
        self.get_all_tasks().is_empty()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn add_should_insert_a_task() {
//         let mut t = Tasks::new();
//         t.add_task("Hi".to_string());
//         assert_eq!(t.length(), 1);
//     }
//
//     #[test]
//     fn should_return_task_if_id_exists() {
//         let mut t = Tasks::new();
//         t.add_task("Hi".to_string());
//         let task = t.get_task(1).unwrap();
//         assert_eq!(task.id, 1);
//         assert_eq!(task.description, "Hi");
//         assert_eq!(task.completed, false);
//     }
//
//     #[test]
//     fn set_completed_should_return_ok_if_task_exists() {
//         let mut t = Tasks::new();
//         t.add_task("Hi".to_string());
//         let res = t.mark_completed(1);
//         assert_eq!(res, Ok(()));
//         let task = t.get_task(1).unwrap();
//         assert_eq!(task.completed, true);
//     }
//
//     #[test]
//     fn if_task_not_exists_set_completed_should_return_err() {
//         let mut t = Tasks::new();
//         t.add_task("Hi".to_string());
//         let res = t.mark_completed(2);
//         assert_eq!(res, Err("Cannot find task"));
//     }
//
//     #[test]
//     fn remove_task() {
//         let mut t = Tasks::new();
//         t.add_task("Hi".to_string());
//         t.add_task("Howdy".to_string());
//         t.add_task("Dear".to_uppercase());
//
//         let to_remove = t.get_task(2).unwrap();
//         assert_eq!(t.remove_task(to_remove.id), Ok(()));
//         assert_eq!(t.length(), 2);
//         assert_eq!(t.remove_task(1), Ok(()));
//         assert_eq!(t.length(), 1);
//     }
// }
