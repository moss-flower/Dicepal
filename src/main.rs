mod app;
mod ui;

use crossterm::event::Event;

use crossterm::event;
use log::{error, info, warn};
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::io::Result;

use crate::app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();

    let log_file = File::create("app.log").expect("Failed to create log file");

    WriteLogger::init(log::LevelFilter::Info, Config::default(), log_file)
        .expect("Failed to Initialize logger");

    info!("The application has successfully started.");
    warn!("This is a warning message.");
    error!("Something went wrong!");

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
