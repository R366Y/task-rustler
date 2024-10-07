#[cfg(test)]
mod test {
    use task_rustler::app::App;
    use task_rustler::command::{AddTaskCommand, Command};

    #[test]
    fn add_task_command_test() {
        let mut app = App::new(String::new());
        app.input_title = String::from("test title");
        app.input_description = String::from("test description");
        app.input_date = String::from("invalid date");
        let res = AddTaskCommand.execute(&mut app);
        assert!(res.is_err());
    }
}
