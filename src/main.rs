mod app;
mod ui;

use crossterm::event::Event;

use crossterm::event;
use std::io::Result;

use crate::app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;
        match event::read()? {
            Event::Key(key) => app.update(key),
            _ => {}
        }

        if app.app_state.should_quit {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
