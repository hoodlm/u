use crate::lex::tokens::{Token, TokenName, TokenMatcher};
use std::error::Error;
use std::fmt::{Display, Formatter};

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
            }
        }
    }
}
impl Error for LexError {}

pub struct LexAnalyzer {
    token_matcher: TokenMatcher,
}

impl LexAnalyzer {
    pub fn new() -> Self {
        LexAnalyzer {
            token_matcher: TokenMatcher::new(),
        }
    }

    pub fn lex_analysis(&self, input: &String) -> Result<Vec<Token>, Vec<LexError>> {
        let tokens = self.collect_tokens(input);
        let errors: Vec<LexError> = tokens
            .iter()
            .filter(|token| token.name == TokenName::Unknown)
            .map(|token| LexError::UnknownToken {
                value: token.value.clone(),
            })
            .collect();

        if errors.is_empty() {
            return Ok(tokens);
        } else {
            return Err(errors);
        }
    }

    fn collect_tokens(&self, input: &String) -> Vec<Token> {
        let mut tokens = Vec::new();
        if input.is_empty() {
            return tokens;
        }
        Token::all().iter().find(|token_kind| {
            let regex = self.token_matcher.regex(token_kind);
            let token_match = regex.find(input);
            if token_match.is_some() {
                tokens.push(Token {
                    name: **token_kind,
                    value: self.token_matcher.pack_value(token_kind, token_match.unwrap().as_str()),
                });
                let skip_index = token_match.unwrap().end();
                let remaining_input = &input[skip_index..].to_string();
                let mut more_tokens = self.collect_tokens(remaining_input);
                tokens.append(&mut more_tokens);
                true
            } else {
                false
            }
        });
        return tokens;
    }
}
