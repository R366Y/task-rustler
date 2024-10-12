use crate::task::Task;
use crate::task_manager::{SortOrder, TasksService};
use ratatui::widgets::ListState;

pub struct TaskList {
    pub items: Vec<Task>,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            items: vec![],
            state: ListState::default(),
        }
    }
}
#[derive(Debug)]
pub enum InputMode {
    View,
    Adding,
    EditingExisting,
}
#[derive(Debug)]
pub enum InputFieldType {
    Title,
    Description,
    Date,
}

pub struct AppContext {
    pub task_list: TaskList,
    pub input_title: String,
    pub input_description: String,
    pub input_date: String,
    pub input_mode: InputMode,
    pub input_field: InputFieldType,
    pub tasks_service: TasksService,
    pub show_help: bool,
    pub error: Option<String>,
}

impl AppContext {
    pub fn new(db_path: String) -> AppContext {
        AppContext {
            task_list: TaskList::new(),
            input_title: String::new(),
            input_description: String::new(),
            input_date: String::new(),
            input_mode: InputMode::View,
            input_field: InputFieldType::Title,
            tasks_service: TasksService::new(db_path),
            show_help: false,
            error: None,
        }
    }

    pub fn sort_by_priority(&mut self) {
        self.task_list
            .items
            .sort_by(|a, b| b.priority.cmp(&a.priority))
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

    pub fn refresh_task_list(&mut self) {
        self.task_list.items = self.tasks_service.get_all_tasks_sorted(SortOrder::High);
    }

    pub fn next_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputFieldType::Title => InputFieldType::Description,
            InputFieldType::Description => InputFieldType::Date,
            InputFieldType::Date => InputFieldType::Title,
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.input_field {
            InputFieldType::Title => {
                self.input_title.pop();
            }
            InputFieldType::Description => {
                self.input_description.pop();
            }
            InputFieldType::Date => {
                self.input_date.pop();
            }
        }
    }

    pub fn handle_char_input(&mut self, c: char) {
        match self.input_field {
            InputFieldType::Title => self.input_title.push(c),
            InputFieldType::Description => self.input_description.push(c),
            InputFieldType::Date => self.input_date.push(c),
        }
    }
}
