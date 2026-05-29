use std::io::Error;

use crate::app::parser::Token;

pub struct Die {
    size: u8,
    result: u32,
}

#[derive(Default)]
pub struct Roll {
    pub input: String,
    pub dice: Vec<Die>,
    pub result: u32,
}

enum State {
    Normal,
    Add,
    Subtract,
}

pub fn roll(input: Vec<Token>) -> Result<Roll, Error> {
    let mut result: u32 = 0;
    let mut roll: Roll;
    let mut state = State::Normal;

    for token in input {
        state = match (state, token) {
            // Normal State
            (State::Normal, Token::Number(a)) => {
                result += a;
                State::Normal
            }
            (State::Normal, Token::Dice(a, b, c)) => {
                let x = 1;
                State::Normal
            }
            (State::Normal, Token::Operator(o)) => {
                if o == '-' {
                    State::Subtract
                } else {
                    State::Add
                }
            }

            (State::Add, Token::Number(a)) => {
                result += a;
                State::Normal
            }
            (State::Add, Token::Dice(a, b, c)) => {
                let x = 1;
                State::Normal
            }

            (State::Subtract, Token::Number(a)) => {
                result += a;
                State::Normal
            }
            (State::Subtract, Token::Dice(a, b, c)) => {
                let x = 1;
                State::Normal
            }

            _ => State::Normal,
        };
    }
    return Ok(roll);
}

fn roll_dice(dice: &Vec<Die>) {
    
}
