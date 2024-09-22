use ratatui::widgets::ListState;
use crate::task_manager::{Priority, Task};

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
}

impl App {
    pub fn new() -> App {
        App {
            task_list: TaskList::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
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
}