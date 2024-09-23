use std::error::Error;
use std::io;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use task_rustler::app::App;
use task_rustler::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::test();
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
