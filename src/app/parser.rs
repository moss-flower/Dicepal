use crate::app::command::Command;

pub enum ParseError {
    IncorrectStructure,
    EmptyInput,
    SyntaxError(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Character(char),
    Operator(char),
    Number(u32),
    Dice(u32, char, u32),
}

#[derive(Debug, PartialEq)]
pub enum State {
    Start,
    HeldNumber(u32),
    HeldNumberAndLetter(u32, char),
    CompletedRoll,
    Invalid,
}

pub fn parse_command(input: String) -> Result<Command, ParseError> {
    let mut tokens = input.split_whitespace(); //need to change this to only split at the first white space
    let command = tokens.next().ok_or(ParseError::EmptyInput)?.to_string();
    let args = tokens.map(String::from).collect();
    Ok(Command { command, args })
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => {
                chars.next();
            }
            '+' | '-' => {
                tokens.push(Token::Operator(chars.next().unwrap()));
            }
            '0'..='9' => {
                let mut num = String::new();
                while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    num.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            'd' | 'D' => {
                tokens.push(Token::Character(chars.next().unwrap()));
            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}

pub fn parse_words(tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
    let mut state = State::Start;
    let mut words: Vec<Token> = vec![];

    for token in tokens {
        state = match (state, token) {
            // Entry or after operator reset
            (State::Start, Token::Number(n)) => State::HeldNumber(n),

            // Held Number
            (State::HeldNumber(a), Token::Character(b)) => State::HeldNumberAndLetter(a, b),
            (State::HeldNumber(a), Token::Operator(o)) => {
                words.push(Token::Number(a));
                words.push(Token::Operator(o));
                State::Start
            }

            // Held number and letter
            (State::HeldNumberAndLetter(a, b), Token::Number(n)) => {
                words.push(Token::Dice(a, b, n));
                State::CompletedRoll
            }

            // Force operator after completed roll parse.
            (State::CompletedRoll, Token::Operator(o)) => {
                words.push(Token::Operator(o));
                State::Start
            }

            _ => State::Invalid,
        };

        if state == State::Invalid {
            return Err(ParseError::SyntaxError(
                "Invalid syntax or structure in roll arguments.".into(),
            ));
        }
    }

    match state {
        State::Start => {}
        State::HeldNumber(a) => words.push(Token::Number(a)),
        State::HeldNumberAndLetter(_, _) => {
            return Err(ParseError::SyntaxError(
                "Invalid syntax, unfinished dice roll".into(),
            ));
        }
        State::CompletedRoll => {}
        _ => return Err(ParseError::SyntaxError("Invalid syntax".into())),
    }

    return Ok(words);
}
