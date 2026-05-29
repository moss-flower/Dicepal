use std::fmt;

use crate::app::{AppState, command::Command};

pub enum InterpreterError {
    UnknownCommand(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::UnknownCommand(arg) => write!(f, "{}", arg),
        }
    }
}

pub fn execute(command: Command, app_state: &mut AppState) -> Result<(), InterpreterError> {
    match command.command.as_str() {
        "exit" | "quit" | "q" => {
            app_state.should_quit = true;
            Ok(())
        }
        "roll" => Ok(()),
        _ => Err(InterpreterError::UnknownCommand(command.command)),
    }
}
