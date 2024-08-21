use to_do::task_manager::{Task, TasksService};

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
        t.delete_all_tasks();
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
    fn should_return_none_if_task_is_not_found() {
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