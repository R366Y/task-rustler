#[cfg(test)]
mod test {
    use task_rustler::app::AppContext;
    use task_rustler::command::{AddTaskCommand, Command};

    #[test]
    fn add_task_command_test_wrong_date_format() {
        let mut app = AppContext::new(String::new());
        app.input_title = String::from("test title");
        app.input_description = String::from("test description");
        app.input_date = String::from("invalid date");
        let res = AddTaskCommand.execute(&mut app);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().to_string(), "Invalid date format, use dd-mm-yyyy")
    }

    #[test]
    fn add_task_command_test_empty_title() {
        let mut app = AppContext::new(String::new());
        app.input_title = String::from("");
        app.input_description = String::from("test description");
        app.input_date = String::from("10-12-2012");
        let res = AddTaskCommand.execute(&mut app);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().to_string(), "You must insert at least a title for the task");
    }
}
