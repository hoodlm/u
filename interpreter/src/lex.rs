use std::error::Error;
use std::fmt::{Formatter, Display};
use crate::lex::tokens::{Token, TokenName};

pub mod tokens;

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

pub fn lex_analysis(input: &String) -> Result<Vec<Token>, Vec<LexError>> {
    let tokens = collect_tokens(input);
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
    let whitespace_regex = Token::regex(TokenName::Whitespace);
    let float_regex = Token::regex(TokenName::Float);
    let int_regex = Token::regex(TokenName::Integer);
    let plus_regex = Token::regex(TokenName::Plus);
    let minus_regex = Token::regex(TokenName::Minus);
    let stdout_regex = Token::regex(TokenName::Stdout);
    let letter_regex = Token::regex(TokenName::Letter);
    let semicolon_regex = Token::regex(TokenName::Semicolon);
    let unknown_token_regex = Token::regex(TokenName::Unknown);

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
                value: Some(float_mat.unwrap().as_str().trim().to_string())
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
                value: Some(int_mat.unwrap().as_str().trim().to_string())
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
                value: Some(letter_mat.unwrap().as_str().trim().to_string())
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

    let unknown_token_mat = unknown_token_regex.find(input);
    if unknown_token_mat.is_some() {
        tokens.push(
            Token {
                name: TokenName::Unknown,
                value: Some(unknown_token_mat.unwrap().as_str().trim().to_string())
            }
        );
        let skip_index = unknown_token_mat.unwrap().end();
        let remaining_input = &input[skip_index..].to_string();
        let mut more_tokens = collect_tokens(remaining_input);
        tokens.append(&mut more_tokens);
        return tokens;
    }

    panic!("Unexpected: remaining input '{}'", input);
}

