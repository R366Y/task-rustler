use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::crossterm::event;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use task_rustler::app::{AddTaskCommand, App, DeleteTaskCommand, FinishEditingTaskCommand, InputMode, StartEditingTaskCommand, ToggleItemPriorityCommand, ToggleTaskStatusCommand};
use task_rustler::command::Command;
use task_rustler::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.refresh_task_list();
    let mut terminal = ratatui::init();
    let res = run_app(&mut terminal, app);
    ratatui::restore();

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Down => {
                        app.select_next();
                    }
                    KeyCode::Up => {
                        app.select_previous();
                    }
                    KeyCode::Char(' ') => {
                        ToggleTaskStatusCommand.execute(&mut app);
                    }
                    KeyCode::Char('m') => {
                        StartEditingTaskCommand.execute(&mut app);
                    }
                    KeyCode::Char('p') => {
                        ToggleItemPriorityCommand.execute(&mut app);
                    }
                    KeyCode::Char('s') => {
                        app.sort_by_priority();
                    }
                    KeyCode::Char('d') => {
                        DeleteTaskCommand.execute(&mut app);
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            AddTaskCommand.execute(&mut app);
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::EditingExisting => match key.code {
                    KeyCode::Enter => {
                        FinishEditingTaskCommand.execute(&mut app);
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}
