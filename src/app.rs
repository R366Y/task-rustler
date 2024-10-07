use crate::task::Task;
use crate::task_manager::TasksService;
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
    Normal,
    Adding,
    EditingExisting,
}
#[derive(Debug)]
pub enum InputField {
    Title,
    Description,
    Date,
}

pub struct App {
    pub task_list: TaskList,
    pub input_title: String,
    pub input_description: String,
    pub input_date: String,
    pub input_mode: InputMode,
    pub input_field: InputField,
    pub tasks_service: TasksService,
    pub show_popup: bool,
    pub error_message: String,
    pub is_error: bool,
}

impl App {
    pub fn new(db_path: String) -> App {
        App {
            task_list: TaskList::new(),
            input_title: String::new(),
            input_description: String::new(),
            input_date: String::new(),
            input_mode: InputMode::Normal,
            input_field: InputField::Title,
            tasks_service: TasksService::new(db_path),
            show_popup: false,
            error_message: String::new(),
            is_error: false,
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
        self.task_list.items = self.tasks_service.get_all_tasks()
    }

    pub fn next_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputField::Title => InputField::Description,
            InputField::Description => InputField::Date,
            InputField::Date => InputField::Title,
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.input_field {
            InputField::Title => {
                self.input_title.pop();
            }
            InputField::Description => {
                self.input_description.pop();
            }
            InputField::Date => {
                self.input_date.pop();
            }
        }
    }
}
