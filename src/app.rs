use crossterm::event::KeyEvent;

mod command;
mod interpreter;
mod parser;

#[derive(Default)]
pub struct AppState {
    pub should_quit: bool,
}

#[derive(Default)]
pub struct App {
    pub app_state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, key: KeyEvent) {}
}
