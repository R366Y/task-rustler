#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use task_rustler::date::{TaskDate, DATE_FORMAT};
    use task_rustler::task::{Priority, Task};
    use task_rustler::export::export_tasks_to_icalendar;

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
        println!("{}", calendar);
    }
}