use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenName {
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

impl Token {
    pub fn regex(token_name: TokenName) -> Regex {
        match token_name {
            TokenName::Whitespace => { Regex::new(r"^\s+").unwrap() },
            TokenName::Float =>      { Regex::new(r"^-?[0-9]+\.[0-9]+\s+").unwrap() },
            TokenName::Integer =>    { Regex::new(r"^-?[0-9]+\s+").unwrap() },
            TokenName::Letter =>     { Regex::new(r"^[a-z|A-Z]\s*").unwrap() },
            TokenName::Plus =>       { Regex::new(r"^\+\s*").unwrap() },
            TokenName::Minus =>      { Regex::new(r"^-\s*").unwrap() },
            TokenName::Stdout =>     { Regex::new(r"^STDOUT\s*").unwrap() },
            TokenName::Semicolon =>  { Regex::new(r"^;").unwrap() },
            TokenName::Unknown =>    { Regex::new(r"^\S+\s*").unwrap() },
        }
    }
}
