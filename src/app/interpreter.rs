use std::{
    fmt::{self},
    io::Error,
};

use crate::app::{
    AppState,
    command::Command,
    diceroller,
    interpreter::InterpreterError::DownstreamError,
    parser::{self, ParseError},
};

pub enum InterpreterError {
    UnknownCommand(String),
    DownstreamError,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::UnknownCommand(arg) => write!(f, "{}", arg),
            InterpreterError::DownstreamError => write!(f, "Error"),
        }
    }
}

impl From<ParseError> for InterpreterError {
    fn from(value: ParseError) -> Self {
        DownstreamError
    }
}
impl From<Error> for InterpreterError {
    fn from(value: Error) -> Self {
        InterpreterError::DownstreamError
    }
}

pub fn execute(command: Command, app_state: &mut AppState) -> Result<(), InterpreterError> {
    match command.command.as_str() {
        "exit" | "quit" | "q" => {
            app_state.should_quit = true;
            Ok(())
        }
        "roll" => {
            let string = command.args.join("");
            let tokens = parser::tokenize(string.as_str());
            let words = parser::parse_words(tokens)?;
            app_state.roll_history.push(diceroller::parse_roll(words)?);
            Ok(())
        }

        _ => Err(InterpreterError::UnknownCommand(command.command)),
    }
}
