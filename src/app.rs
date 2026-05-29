use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{command::CommandBar, parser::ParseError};

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
    pub command_bar: CommandBar,
    pub status_message: String,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                self.status_message.clear();
                let cmd = match parser::parse_command(&self.command_bar.text_field) {
                    Ok(cmd) => cmd,
                    Err(ParseError::EmptyInput) => return,
                    _ => return,
                };

                if let Err(e) = interpreter::execute(cmd, &mut self.app_state) {
                    self.status_message = e.to_string();
                };
                self.command_bar.text_field.clear();
                self.command_bar.location_pointer = 0;
                return;
            }
            KeyCode::Char(char) => {
                self.command_bar.append(char);
                return;
            }
            KeyCode::Backspace => {
                self.command_bar.backspace();
                return;
            }
            KeyCode::Esc => {
                self.is_in_writing_mode = false;
                self.command_bar.reset();
                return;
            }
            _ => return,
        }
    }
}
