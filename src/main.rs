use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use std::error::Error;
use std::io;
use task_rustler::app::{App, InputField, InputMode};
use task_rustler::command::*;
use task_rustler::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(String::from("tasks.db"));
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
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('h') => {
                        app.show_popup = !app.show_popup;
                    }
                    KeyCode::Esc => {
                        if app.show_popup {
                            app.show_popup = false;
                        }
                    }
                    _ => handle_key_event_normal_mode(key.code, &mut app),
                },
                InputMode::Adding => handle_key_event_editing_mode(key.code, &mut app),
                InputMode::EditingExisting => {
                    handle_key_event_editing_existing_mode(key.code, &mut app)
                }
            }
        }
    }
}

fn handle_key_event_normal_mode(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('a') => {
            let _ = EnterEditModeCommand.execute(app);
        }
        KeyCode::Down => {
            app.select_next();
        }
        KeyCode::Up => {
            app.select_previous();
        }
        KeyCode::Char(' ') => {
            let _ = ToggleTaskStatusCommand.execute(app);
        }
        KeyCode::Char('m') => {
            let _ = StartEditingExistingTaskCommand.execute(app);
        }
        KeyCode::Char('p') => {
            let _ = ToggleItemPriorityCommand.execute(app);
        }
        KeyCode::Char('s') => {
            app.sort_by_priority();
        }
        KeyCode::Char('d') => {
            let _ = DeleteTaskCommand.execute(app);
        }
        _ => {}
    }
}

fn handle_key_event_editing_mode(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Enter => {
            if !app.input_title.is_empty() {
                if let Err(e) = AddTaskCommand.execute(app) {
                    app.is_error = true;
                    app.error_message = e.to_string();
                } else {
                    app.is_error = false;
                    app.error_message.clear();
                }
            }
            if !app.is_error {
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Tab => app.next_input_field(),
        KeyCode::Char(c) => match app.input_field {
            InputField::Title => app.input_title.push(c),
            InputField::Description => app.input_description.push(c),
            InputField::Date => app.input_date.push(c),
        },
        KeyCode::Backspace => app.handle_backspace(),
        KeyCode::Esc => {
            let _ = StopEditingCommand.execute(app);
        }
        _ => {}
    }
}

fn handle_key_event_editing_existing_mode(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Tab => app.next_input_field(),
        KeyCode::Enter => {
            if !app.input_title.is_empty() {
                if let Err(e) = FinishEditingExistingTaskCommand.execute(app) {
                    app.is_error = true;
                    app.error_message = e.to_string();
                } else {
                    app.is_error = false;
                    app.error_message.clear();
                }
            }
            if !app.is_error {
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Char(c) => match app.input_field {
            InputField::Title => app.input_title.push(c),
            InputField::Description => app.input_description.push(c),
            InputField::Date => app.input_date.push(c),
        },
        KeyCode::Backspace => app.handle_backspace(),
        KeyCode::Esc => {
            let _ = StopEditingCommand.execute(app);
        }
        _ => {}
    }
}
