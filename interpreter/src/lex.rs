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
    Whitespace,
    Float,
    Integer,
    Letter,
    Plus,
    Minus,
    Stdout,
    Unknown,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub name: TokenName,
    pub value: Option<String>,
}

pub fn lex_analysis(input: &String) -> Result<Vec<Token>, Vec<LexError>> {
    // let tokens: Vec<Token> = input.split_whitespace().map(|it| to_token(it)).collect();
    let tokens = collect_tokens(input);
    // eprintln!("DEBUG: TOKENS: {:?}", tokens);

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

fn collect_tokens(input: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    if input.is_empty() {
        return tokens;
    }

    let whitespace_regex = Regex::new(r"^\s+").unwrap();
    let float_regex = Regex::new(r"^-?[0-9]+\.[0-9]+").unwrap();
    let int_regex = Regex::new(r"^-?[0-9]+").unwrap();
    let plus_regex = Regex::new(r"^\+").unwrap();
    let minus_regex = Regex::new(r"^-").unwrap();
    let stdout_regex = Regex::new(r"^STDOUT").unwrap();
    let letter_regex = Regex::new(r"^[a-z|A-Z]").unwrap();
    let semicolon_regex = Regex::new(r"^;").unwrap();

    let whitespace_mat = whitespace_regex.find(input);
    if whitespace_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Whitespace,
                value: Some(whitespace_mat.unwrap().as_str().to_string())
            }
        );
        let skip_index = whitespace_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let float_mat = float_regex.find(input);
    if float_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Float,
                value: Some(float_mat.unwrap().as_str().to_string())
            }
        );
        let skip_index = float_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let int_mat = int_regex.find(input);
    if int_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Integer,
                value: Some(int_mat.unwrap().as_str().to_string())
            }
        );
        let skip_index = int_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let plus_mat = plus_regex.find(input);
    if plus_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Plus,
                value: None
            }
        );
        let skip_index = plus_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let minus_mat = minus_regex.find(input);
    if minus_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Minus,
                value: None
            }
        );
        let skip_index = minus_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let stdout_mat = stdout_regex.find(input);
    if stdout_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Stdout,
                value: None
            }
        );
        let skip_index = stdout_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let letter_mat = letter_regex.find(input);
    if letter_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Letter,
                value: Some(letter_mat.unwrap().as_str().to_string())
            }
        );
        let skip_index = letter_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    let semicolon_mat = semicolon_regex.find(input);
    if semicolon_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Semicolon,
                value: None
            }
        );
        let skip_index = semicolon_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    panic!("Unexpected: remaining input '{}'", input);
}

