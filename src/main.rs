use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use std::error::Error;
use std::io;
use task_rustler::app::{AppContext, InputMode};
use task_rustler::command::*;
use task_rustler::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = AppContext::new(String::from("tasks.db"));
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
    mut app: AppContext,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            // Capture only press key event
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.input_mode {
                InputMode::View => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('h') => {
                        app.show_help = !app.show_help;
                    }
                    KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        }
                    }
                    _ => handle_key_event_view_mode(key.code, &mut app),
                },
                InputMode::Adding => handle_key_event_adding_mode(key.code, &mut app),
                InputMode::EditingExisting => {
                    handle_key_event_editing_existing_mode(key.code, &mut app)
                }
            }
        }
    }
}

fn handle_key_event_view_mode(key: KeyCode, app: &mut AppContext) {
    match key {
        KeyCode::Char('a') => {
            let _ = EnterAddModeCommand.execute(app);
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

fn handle_key_event_adding_mode(key: KeyCode, app: &mut AppContext) {
    match key {
        KeyCode::Enter => {
            handle_errors(AddTaskCommand, app);
            if app.error.is_none() {
                app.input_mode = InputMode::View;
            }
        }
        KeyCode::Tab => app.next_input_field(),
        KeyCode::Char(c) => app.handle_char_input(c),
        KeyCode::Backspace => app.handle_backspace(),
        KeyCode::Esc => {
            let _ = StopEditingCommand.execute(app);
        }
        _ => {}
    }
}

fn handle_key_event_editing_existing_mode(key: KeyCode, app: &mut AppContext) {
    match key {
        KeyCode::Tab => app.next_input_field(),
        KeyCode::Enter => {
            handle_errors(FinishEditingExistingTaskCommand, app);
            if app.error.is_none() {
                app.input_mode = InputMode::View;
            }
        }
        KeyCode::Char(c) => app.handle_char_input(c),
        KeyCode::Backspace => app.handle_backspace(),
        KeyCode::Esc => {
            let _ = StopEditingCommand.execute(app);
        }
        _ => {}
    }
}

fn handle_errors<T: Command>(command:T, app: &mut AppContext) {
    if let Err(e) = command.execute(app) {
        app.error= Some(e.to_string());
    } else {
        app.error = None;
    }
}
