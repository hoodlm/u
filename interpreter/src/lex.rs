use std::error::Error;
use std::fmt::{Formatter, Display};
use regex::Regex;

#[derive(Debug)]
pub enum LexError {
    UnknownToken { value: String },
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnknownToken { value } => {
                write!(f, "Unknown token: {}", value)
            },
        }
    }
}
impl Error for LexError {}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenName {
    ProgramStart,
    Float,
    Integer,
    Letter,
    Plus,
    Minus,
    Stdout,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub name: TokenName,
    pub value: Option<String>,
}

pub fn lex_analysis(input: &String) -> Result<Vec<Token>, Vec<LexError>> {
    let tokens: Vec<Token> = input.split_whitespace().map(|it| to_token(it)).collect();

    let errors: Vec<LexError> = tokens
        .iter()
        .filter(|token| token.name == TokenName::Unknown)
        .map(|token| LexError::UnknownToken {
            value: token.value.clone().expect("Internal error: Unknown token that does not have a value")
        })
        .collect();

    if errors.is_empty() {
        return Ok(tokens);
    } else {
        return Err(errors);
    }
}

fn to_token(t: &str) -> Token {
    let float_regex = Regex::new(r"^-?[0-9]+\.[0-9]+$").unwrap();
    let int_regex = Regex::new(r"^-?[0-9]+$").unwrap();
    let letter_regex = Regex::new(r"^[a-z|A-Z]$").unwrap();

    let (name, value) = match t {
        t if float_regex.is_match(t) => (TokenName::Float, Some(t.to_string())),
        t if int_regex.is_match(t) => (TokenName::Integer, Some(t.to_string())),
        "+" => (TokenName::Plus, None),
        "-" => (TokenName::Minus, None),
        "STDOUT" => (TokenName::Stdout, None),
        t if letter_regex.is_match(t) => (TokenName::Letter, Some(t.to_string())),
        _ => (TokenName::Unknown, Some(t.to_string())),
    };
    return Token { name, value };
}
