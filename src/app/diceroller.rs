use std::io::Error;

use rand::{RngExt, rng};

use crate::app::parser::Token;

#[derive(Default)]
pub struct Dice {
    count: i32,
    size: i32,
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        let string: String = format!("{}d{}", self.count, self.size);
        string
    }
}

#[derive(Default)]
pub struct Roll {
    pub input: String,
    pub dice: Vec<Dice>,
    pub modifier: i32,
    pub result: i32,
}

impl ToString for Roll {
    fn to_string(&self) -> String {
        let string = format!("{} = {}", self.input, self.result);
        string
    }
}

impl Roll {
    pub fn execute(&mut self) -> i32 {
        let roll_result: i32 = self.dice.iter().map(|d| d.roll()).sum();
        self.result = roll_result + self.modifier;
        return self.result;
    }
}

impl Dice {
    fn new(count: i32, size: i32) -> Self {
        Self { count, size }
    }
    pub fn roll(&self) -> i32 {
        let mut result = 0;
        let mut rng = rng();

        for _ in 0..self.count - 1 {
            result += rng.random_range(0..self.size - 1);
        }
        result
    }
}

enum State {
    Normal,
    Add,
    Subtract,
}

pub fn parse_roll(input: Vec<Token>) -> Result<Roll, Error> {
    let mut roll: Roll = Roll::default();
    let mut state = State::Normal;

    for token in input {
        state = match (state, token) {
            // Normal State
            (State::Normal, Token::Number(a)) => {
                roll.modifier += a;
                State::Normal
            }
            (State::Normal, Token::Dice(a, _, c)) => {
                let dice = Dice::new(a, c);
                roll.dice.push(dice);
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
                roll.modifier += a;
                State::Normal
            }
            (State::Add, Token::Dice(a, _, c)) => {
                let dice = Dice::new(a, c);
                roll.dice.push(dice);
                State::Normal
            }

            (State::Subtract, Token::Number(a)) => {
                roll.modifier += a;
                State::Normal
            }
            (State::Subtract, Token::Dice(a, _, c)) => {
                let dice = Dice::new(a, c);
                roll.dice.push(dice);
                State::Normal
            }

            _ => State::Normal,
        };
    }
    roll.execute();
    return Ok(roll);
}
