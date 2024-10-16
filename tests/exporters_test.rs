#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use task_rustler::date::{TaskDate, DATE_FORMAT};
    use task_rustler::task::{Priority, Task};
    use task_rustler::export::export_tasks_to_icalendar;
    use icalendar::{Component, DatePerhapsTime, TodoStatus};

    #[test]
    fn tasks_to_icalendar(){
        let task1 = Task{
            id: 0,
            title: "First task".to_string(),
            description: "Task n 1".to_string(),
            completed: false,
            priority: Priority::Low,
            date: TaskDate(Some(NaiveDate::parse_from_str("15-10-2024", DATE_FORMAT).unwrap()))
        };

        let task2 = Task{
            id: 1,
            title: "Second task".to_string(),
            description: "Task n 2".to_string(),
            completed: true,
            priority: Priority::High,
            date: TaskDate(None)
        };

        let tasks = vec![task1, task2];
        let calendar = export_tasks_to_icalendar("Task Rustler", &tasks );
        assert_eq!(calendar.get_name().unwrap(), "Task Rustler");

        let todo1 = calendar.components[0].as_todo().unwrap();
        assert_eq!(todo1.get_summary().unwrap(), "First task");
        assert_eq!(todo1.get_description().unwrap(), "Task n 1");
        assert_eq!(todo1.get_priority().unwrap(), 3);
        assert_eq!(todo1.get_end().unwrap(), DatePerhapsTime::Date(NaiveDate::parse_from_str("15-10-2024", DATE_FORMAT).unwrap()));
        assert_eq!(todo1.get_status().unwrap(), TodoStatus::NeedsAction);

        let todo2 = calendar.components[1].as_todo().unwrap();
        assert_eq!(todo2.get_summary().unwrap(), "Second task");
        assert_eq!(todo2.get_description().unwrap(), "Task n 2");
        assert_eq!(todo2.get_priority().unwrap(), 1);
        assert_eq!(todo2.get_end(), None);
        assert_eq!(todo2.get_status().unwrap(), TodoStatus::Completed);
    }
}