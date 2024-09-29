use crate::app::{App, InputField, InputMode};
use crate::task::Task;

pub trait Command {
    fn execute(&mut self, app: &mut App);
}

pub struct EnterEditModeCommand;

impl Command for EnterEditModeCommand {
    fn execute(&mut self, app: &mut App) {
        app.input_mode = InputMode::Editing;
        app.input_field = InputField::Title;
    }
}

/// Add a new task
pub struct AddTaskCommand;

impl Command for AddTaskCommand {
    fn execute(&mut self, app: &mut App) {
        let mut t = Task::new();
        t.title = app.input_title.drain(..).collect();
        t.description = app.input_description.drain(..).collect();
        app.tasks_service.add_new_task(&t);
        app.refresh_task_list();
    }
}

/// Toggle completed for selected task status
pub struct ToggleTaskStatusCommand;

impl Command for ToggleTaskStatusCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected(){
            let item = &mut app.task_list.items[index];
            item.completed = match item.completed {
                true => false,
                false => true,
            };
            let _ = app.tasks_service.toggle_task_status(item.id, item.completed);
        };
    }
}

/// Switch between priorities
pub struct ToggleItemPriorityCommand;

impl Command for ToggleItemPriorityCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            let item = &mut app.task_list.items[index];
            item.priority = item.priority.next();
            app.tasks_service.change_priority(item.id, &item.priority);
        }
    }
}

/// Start editing a task, move cursor to Title input field
/// and set InputMode equal to InputMode::EditingExisting
pub struct StartEditingExistingTaskCommand;

impl Command for StartEditingExistingTaskCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            app.input_title = app.task_list.items[index].title.clone();
            app.input_description = app.task_list.items[index].description.clone();
            app.input_mode = InputMode::EditingExisting;
            app.input_field = InputField::Title;
        }
    }
}

/// Finish editing an existing task, set InputMode back to Normal
pub struct FinishEditingExistingTaskCommand;

impl Command for FinishEditingExistingTaskCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            app.task_list.items[index].title = app.input_title.drain(..).collect();
            app.task_list.items[index].description = app.input_description.drain(..).collect();
            app.tasks_service.update_task(&app.task_list.items[index])
        }
        app.input_mode = InputMode::Normal;
    }
}

pub struct DeleteTaskCommand;

impl Command for DeleteTaskCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            app.tasks_service.delete_task(app.task_list.items[index].id);
            app.task_list.items.remove(index);
        }
    }
}

/// Stop adding or editing the current task, clear the input fields and
/// set InputMode back to Normal
pub struct StopEditingCommand;

impl Command for StopEditingCommand {
    fn execute(&mut self, app: &mut App) {
        app.input_mode = InputMode::Normal;
        app.input_title.clear();
        app.input_description.clear();
    }
}