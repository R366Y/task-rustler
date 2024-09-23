use ratatui::widgets::ListState;
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

    /// Toggle completed for selected task status
    pub fn toggle_item_status(&mut self) {
        if let Some(index) = self.task_list.state.selected(){
            let item = &mut self.task_list.items[index];
            item.completed = match item.completed {
                true => false,
                false => true,
            }
        };
    }

    /// Cycle through priorities
    pub fn toggle_item_priority(&mut self) {
        if let Some(index) = self.task_list.state.selected() {
            let item = &mut self.task_list.items[index];
            item.priority = item.priority.next();
        }
    }

    /// Copy task description into App.input and set the ui to InputMode::EditingExisting
    pub fn start_editing_exisiting(&mut self) {
        if let Some(index) = self.task_list.state.selected() {
            self.input = self.task_list.items[index].description.clone();
            self.input_mode = InputMode::EditingExisting;
        }
    }

    /// Copy new description into the selected item and set the ui to InputMode::Normal
    pub fn finish_editing_existing(&mut self) {
        if let Some(index) = self.task_list.state.selected() {
            self.task_list.items[index].description = self.input.drain(..).collect();
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn sort_by_priority(&mut self) {
        self.task_list.items.sort_by(|a, b| b.priority.cmp(&a.priority))
    }

    pub fn delete_item(&mut self) {
        if let Some(index) = self.task_list.state.selected() {
            self.task_list.items.remove(index);
        }
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
}