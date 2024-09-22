use std::error::Error;
use std::io;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use task_rustler::app::App;
use task_rustler::task_manager::{Priority, SortOrder, Task, TasksService};
use task_rustler::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let tasks = TasksService::new(&"tasks.db".to_string());
    let app = App::new();
    let mut terminal = ratatui::init();
    let res = run_app(&mut terminal, app);
    ratatui::restore();

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
