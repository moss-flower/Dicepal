use std::io::Error;

use rand::{RngExt, rng};

use crate::app::parser::Token;

pub struct Die {
    size: u32,
    result: u32,
}
pub struct Dice {
    dice: Vec<Die>,
}

#[derive(Default)]
pub struct Roll {
    pub input: String,
    pub dice: Vec<Dice>,
    pub result: u32,
}

impl Die {
    fn new(size: u32) -> Self {
        Self { size, result: 0 }
    }
    fn roll(&mut self) -> u32 {
        let mut rng = rand::rng();
        let result = rng.random_range(1..self.size);
        self.result = result;
        return result;
    }
}

impl Dice {
    fn new(dice: Vec<Die>) -> Self {
        Self { dice }
    }
    fn roll(mut self) -> u32 {
        let mut result: u32 = 0;
        for mut die in self.dice {
            die.roll();
            result += &die.result;
        }
        result
    }
}

enum State {
    Normal,
    Add,
    Subtract,
}

pub fn roll(input: Vec<Token>) -> Result<Roll, Error> {
    let mut roll: Roll = Roll::default();
    let mut state = State::Normal;

    for token in input {
        state = match (state, token) {
            // Normal State
            (State::Normal, Token::Number(a)) => {
                roll.result += a;
                State::Normal
            }
            (State::Normal, Token::Dice(a, _, c)) => {
                let dice = create_and_roll_dice(a, c);
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
                roll.result += a;
                State::Normal
            }
            (State::Add, Token::Dice(a, _, c)) => {
                let dice = create_and_roll_dice(a, c);
                roll.dice.push(dice);
                State::Normal
            }

            (State::Subtract, Token::Number(a)) => {
                roll.result += a;
                State::Normal
            }
            (State::Subtract, Token::Dice(a, _, c)) => {
                let dice = create_and_roll_dice(a, c);
                roll.dice.push(dice);
                State::Normal
            }

            _ => State::Normal,
        };
    }
    return Ok(roll);
}

fn create_and_roll_dice(count: u32, size: u32) -> Dice {
    let mut dice: Vec<Die> = vec![];
    for _ in 0..count {
        let mut die = Die::new(size);
        die.roll();
        dice.push(die);
    }
    Dice::new(dice)
}
