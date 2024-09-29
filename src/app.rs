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
#[derive(Debug)]
pub enum InputMode{
    Normal,
    Editing,
    EditingExisting,
}
#[derive(Debug)]
pub enum InputField {
    Title,
    Description,
    //sDate,
}

pub struct App {
    pub task_list: TaskList,
    pub input_title: String,
    pub input_description: String,
    pub input_mode: InputMode,
    pub input_field: InputField,
    pub tasks_service: TasksService
}

impl App {
    pub fn new() -> App {
        App {
            task_list: TaskList::new(),
            input_title: String::new(),
            input_description: String::new(),
            input_mode: InputMode::Normal,
            input_field: InputField::Title,
            tasks_service: TasksService::new(&"tasks.db".to_string()),
        }
    }

    pub fn test() -> App {
        let mut app = App::new();
        app.task_list.items = vec![
          Task{
              id:0,
              title: "Task 1 title".to_string(),
              description: "Task 1".to_string(),
              completed: true,
              priority: Priority::Low
          },
          Task{
              id:1,
              title: "Task 2 title".to_string(),
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

    pub fn next_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputField::Title => InputField::Description,
            //InputField::Description => InputField::Date,
            InputField::Description => InputField::Title,
        }
    }
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

/// Start editing a task
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

pub struct StopEditingCommand;
impl Command for StopEditingCommand {
    fn execute(&mut self, app: &mut App) {
        app.input_mode = InputMode::Normal;
        app.input_title.clear();
        app.input_description.clear();
    }
}