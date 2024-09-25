use ratatui::widgets::ListState;
use crate::command::Command;
use crate::task::{Priority, Task};
use crate::task_manager::TasksService;

pub struct TaskList {
    pub items: Vec<Task>,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            items: vec![],
            state: ListState::default()
        }
    }
}

pub enum InputMode{
    Normal,
    Editing,
    EditingExisting,
}

pub struct App {
    pub task_list: TaskList,
    pub input: String,
    pub input_mode: InputMode,
    pub tasks_service: TasksService
}

impl App {
    pub fn new() -> App {
        App {
            task_list: TaskList::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            tasks_service: TasksService::new(&"tasks.db".to_string()),
        }
    }

    pub fn test() -> App {
        let mut app = App::new();
        app.task_list.items = vec![
          Task{
              id:0,
              description: "Task 1".to_string(),
              completed: true,
              priority: Priority::Low
          },
          Task{
              id:1,
              description: "Task 2".to_string(),
              completed: false,
              priority: Priority::High
          },
        ];
        app
    }

    pub fn sort_by_priority(&mut self) {
        self.task_list.items.sort_by(|a, b| b.priority.cmp(&a.priority))
    }

    pub fn select_none(&mut self) {
        self.task_list.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.task_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.task_list.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.task_list.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.task_list.state.select_last();
    }

    pub fn refresh_task_list(&mut self){
        self.task_list.items = self.tasks_service.get_all_tasks()
    }
}


/// Add a new task
pub struct AddTaskCommand;
impl Command for AddTaskCommand {
    fn execute(&mut self, app: &mut App) {
        app.tasks_service.add_task_with_priority(app.input.drain(..).collect(), Priority::Low);
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

pub struct StartEditingTaskCommand;
impl Command for StartEditingTaskCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            app.input = app.task_list.items[index].description.clone();
            app.input_mode = InputMode::EditingExisting;
        }
    }
}

pub struct FinishEditingTaskCommand;
impl Command for FinishEditingTaskCommand {
    fn execute(&mut self, app: &mut App) {
        if let Some(index) = app.task_list.state.selected() {
            app.task_list.items[index].description = app.input.drain(..).collect();
            app.tasks_service.update_description(app.task_list.items[index].id, app.task_list.items[index].description.as_str())
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