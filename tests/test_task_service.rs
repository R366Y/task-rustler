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
                description: "First task".to_string(),
                completed: false,
                priority: Priority::Low,
            },
            Task {
                id: 2,
                description: "Second task".to_string(),
                completed: false,
                priority: Priority::Medium,
            },
            Task {
                id: 3,
                description: "Third task".to_string(),
                completed: false,
                priority: Priority::High,
            },
        ];
        for t in tasks_to_add {
            tasks.add_task_with_priority(t.description, t.priority);
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

    #[test]
    fn get_all_the_task_sorted_by_highest_priority() {
        let t = setup();
        let tasks = t.get_all_tasks_sorted(SortOrder::High);
        assert_eq!(
            tasks[0],
            Task {
                id: 3,
                description: "Third task".to_string(),
                completed: false,
                priority: Priority::High,
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
                description: "First task".to_string(),
                completed: false,
                priority: Priority::Low,
            }
        );
    }
}
