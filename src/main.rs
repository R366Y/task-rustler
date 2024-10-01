use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use std::error::Error;
use std::io;
use task_rustler::app::{App, InputField, InputMode};
use task_rustler::command::*;
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
            // Capture only press key event
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        EnterEditModeCommand.execute(&mut app);
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
                        StartEditingExistingTaskCommand.execute(&mut app);
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
                        if !app.input_description.is_empty() {
                            AddTaskCommand.execute(&mut app);
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Tab => app.next_input_field(),
                    KeyCode::Char(c) => match app.input_field {
                        InputField::Title => app.input_title.push(c),
                        InputField::Description => app.input_description.push(c),
                        InputField::Date => app.input_date.push(c),
                    },
                    KeyCode::Backspace => match app.input_field {
                        InputField::Title => {
                            app.input_title.pop();
                        }
                        InputField::Description => {
                            app.input_description.pop();
                        }
                        InputField::Date => {
                            app.input_date.pop();
                        }
                    },
                    KeyCode::Esc => {
                        StopEditingCommand.execute(&mut app);
                    }
                    _ => {}
                },
                InputMode::EditingExisting => match key.code {
                    KeyCode::Tab => app.next_input_field(),
                    KeyCode::Enter => {
                        FinishEditingExistingTaskCommand.execute(&mut app);
                    }
                    KeyCode::Char(c) => match app.input_field {
                        InputField::Title => app.input_title.push(c),
                        InputField::Description => app.input_description.push(c),
                        InputField::Date => app.input_date.push(c),
                    },
                    KeyCode::Backspace => match app.input_field {
                        InputField::Title => {
                            app.input_title.pop();
                        }
                        InputField::Description => {
                            app.input_description.pop();
                        }
                        InputField::Date => {
                            app.input_date.pop();
                        }
                    },
                    KeyCode::Esc => {
                        StopEditingCommand.execute(&mut app);
                    }
                    _ => {}
                },
            }
        }
    }
}
