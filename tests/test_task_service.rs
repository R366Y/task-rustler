#[cfg(test)]
mod test {
    use task_rustler::task::{Priority, Task};
    use task_rustler::task_manager::SortOrder;
    use task_rustler::task_manager::TasksService;

    fn setup() -> TasksService {
        let tasks = TasksService::default();
        let tasks_to_add = vec![
            Task {
                id: 1,
                title: "My first task title".to_string(),
                description: "First task".to_string(),
                completed: false,
                priority: Priority::Low,
                date: None,
            },
            Task {
                id: 2,
                title: "My second task title".to_string(),
                description: "Second task".to_string(),
                completed: false,
                priority: Priority::Medium,
                date: None,
            },
            Task {
                id: 3,
                title: "My third task title".to_string(),
                description: "Third task".to_string(),
                completed: false,
                priority: Priority::High,
                date: None,
            },
        ];
        for t in tasks_to_add {
            tasks.add_new_task(&t)
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
        t.add_new_task(&Task::default());
        let task = t.get_task(4).unwrap();
        assert_eq!(task.id, 4);
        assert_eq!(task.description, "Test task description");
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
        let num_tasks_completed = t.toggle_task_status(1, true);
        assert_eq!(num_tasks_completed, 1);
        let num_tasks_completed = t.toggle_task_status(100, true);
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

    #[test]
    fn get_all_the_task_sorted_by_highest_priority() {
        let t = setup();
        let tasks = t.get_all_tasks_sorted(SortOrder::High);
        assert_eq!(
            tasks[0],
            Task {
                id: 3,
                title: "My third task title".to_string(),
                description: "Third task".to_string(),
                completed: false,
                priority: Priority::High,
                date: None,
            }
        );
    }

    #[test]
    fn get_all_the_task_sorted_by_lowest_priority() {
        let t = setup();
        let tasks = t.get_all_tasks_sorted(SortOrder::Low);
        assert_eq!(
            tasks[0],
            Task {
                id: 1,
                title: "My first task title".to_string(),
                description: "First task".to_string(),
                completed: false,
                priority: Priority::Low,
                date: None,
            }
        );
    }
}
