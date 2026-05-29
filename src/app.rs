mod command;
mod errors;
mod parser;

pub struct AppState {
    pub should_quit: bool,
}

pub struct App {
    pub app_state: AppState,
}
