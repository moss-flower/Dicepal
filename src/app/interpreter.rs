use crate::app::{AppState, command::Command};

pub enum InterpreterError {
    UnknownCommand(String),
}

pub fn execute(command: Command, app_state: &mut AppState) -> Result<(), InterpreterError> {
    match command.command.as_str() {
        "exit" => {
            app_state.should_quit = true;
            Ok(())
        }
        "roll" => Ok(()),
        _ => Err(InterpreterError::UnknownCommand(command.command)),
    }
}
