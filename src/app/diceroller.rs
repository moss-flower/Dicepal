use std::io::Error;

use rand::RngExt;

use crate::app::parser::Token;

#[derive(Default)]
pub struct Die {
    size: u32,
    result: u32,
}
#[derive(Default)]
pub struct Dice {
    dice: Vec<Die>, 
}

#[derive(Default)]
pub struct Roll {
    pub input: String,
    pub dice: Dice,
    pub result: u32,
}

impl Roll {
    pub fn roll(&mut self) -> u32 {
        self.dice.roll();
        self.result = self.dice;
        return self.result;
    }
}

impl Die {
    pub fn new(size: u32) -> Self {
        Self { size, result: 0 }
    }
    pub fn roll(&mut self) -> u32 {
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
    fn roll(&mut self) -> u32 {
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

pub fn parse_roll(input: Vec<Token>) -> Result<Roll, Error> {
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
