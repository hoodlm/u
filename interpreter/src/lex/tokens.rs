use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenName {
    Whitespace,
    Float,
    Integer,
    Letter,
    UString,
    Plus,
    Minus,
    Stdout,
    Repeater,
    Unknown,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub name: TokenName,
    pub value: Option<String>,
}

impl Token {
    /// This also controls the order of precedence when parsing tokens
    pub fn all() -> Vec<TokenName> {
        vec![
            TokenName::Whitespace,
            TokenName::Repeater,
            TokenName::Float,
            TokenName::Integer,
            TokenName::Stdout,
            TokenName::UString,
            TokenName::Plus,
            TokenName::Minus,
            TokenName::Letter,
            TokenName::Semicolon,
            TokenName::Unknown,
        ]
    }

    pub fn regex(token_name: &TokenName) -> Regex {
        match token_name {
            TokenName::Whitespace => { Regex::new(r"^\s+").unwrap() },
            TokenName::Float =>      { Regex::new(r"^-?[0-9]+\.[0-9]+\s+").unwrap() },
            TokenName::Integer =>    { Regex::new(r"^-?[0-9]+\s+").unwrap() },
            TokenName::Letter =>     { Regex::new(r"^'[a-z|A-Z]'\s*").unwrap() },
            TokenName::Repeater =>   { Regex::new(r"^\{[0-9]+\}\s*").unwrap() },
            TokenName::UString =>    { Regex::new("^\"[^\"]+\"\\s*").unwrap() },
            TokenName::Plus =>       { Regex::new(r"^\+\s*").unwrap() },
            TokenName::Minus =>      { Regex::new(r"^-\s*").unwrap() },
            TokenName::Stdout =>     { Regex::new(r"^STDOUT\s*").unwrap() },
            TokenName::Semicolon =>  { Regex::new(r"^;").unwrap() },
            TokenName::Unknown =>    { Regex::new(r"^\S+\s*").unwrap() },
        }
    }

    pub fn pack_value(token_name: &TokenName, value: &str) -> Option<String> {
        match token_name {
            TokenName::Whitespace => Some(value.to_string()),
            TokenName::Float =>      Some(value.trim().to_string()),
            TokenName::Integer =>    Some(value.trim().to_string()),
            TokenName::Letter =>     Some(value.trim().trim_start_matches('\'').trim_end_matches('\'').to_string()),
            TokenName::Repeater =>   Some(value.trim().trim_start_matches('{').trim_end_matches('}').to_string()),
            TokenName::UString =>    Some(value.trim().trim_start_matches('\"').trim_end_matches('\"').to_string()),
            TokenName::Plus =>       Some(value.to_string()),
            TokenName::Minus =>      Some(value.to_string()),
            TokenName::Stdout =>     Some(value.to_string()),
            TokenName::Semicolon =>  Some(value.to_string()),
            TokenName::Unknown =>    Some(value.to_string()),
        }
    }
}
