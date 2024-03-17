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
    Variable,
    Unknown,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub name: TokenName,
    pub value: String,
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
            TokenName::Variable,
            TokenName::Letter,
            TokenName::Semicolon,
            TokenName::Unknown,
        ]
    }
}

pub struct TokenMatcher {
    whitespace_regex: Regex,
    float_regex: Regex,
    integer_regex: Regex,
    letter_regex: Regex,
    repeater_regex: Regex,
    string_regex: Regex,
    variable_regex: Regex,
    plus_regex: Regex,
    minus_regex: Regex,
    stdout_regex: Regex,
    semicolon_regex: Regex,
    unknown_regex: Regex,
}

impl TokenMatcher {
    pub fn new() -> Self {
        TokenMatcher {
            whitespace_regex: Regex::new(r"^\s+").unwrap(),
            float_regex: Regex::new(r"^-?[0-9]+\.[0-9]+\s*").unwrap(),
            integer_regex: Regex::new(r"^-?[0-9]+\s*").unwrap(),
            letter_regex: Regex::new(r"^'[a-z|A-Z]'\s*").unwrap(),
            repeater_regex: Regex::new(r"^\{[0-9]+\}\s*").unwrap(),
            string_regex: Regex::new("^\"[^\"]+\"\\s*").unwrap(),
            variable_regex: Regex::new(r"^\$[a-z|A-Z|_]+\s*").unwrap(),
            plus_regex: Regex::new(r"^\+\s*").unwrap(),
            minus_regex: Regex::new(r"^-\s*").unwrap(),
            stdout_regex: Regex::new(r"^STDOUT\s*").unwrap(),
            semicolon_regex: Regex::new(r"^;").unwrap(),
            unknown_regex: Regex::new(r"^\S+\s*").unwrap(),
        }
    }

    pub fn regex(&self, token_name: &TokenName) -> &Regex {
        match token_name {
            TokenName::Whitespace => &self.whitespace_regex,
            TokenName::Float => &self.float_regex,
            TokenName::Integer => &self.integer_regex,
            TokenName::Letter => &self.letter_regex,
            TokenName::Repeater => &self.repeater_regex,
            TokenName::UString => &self.string_regex,
            TokenName::Variable => &self.variable_regex,
            TokenName::Plus => &self.plus_regex,
            TokenName::Minus => &self.minus_regex,
            TokenName::Stdout => &self.stdout_regex,
            TokenName::Semicolon => &self.semicolon_regex,
            TokenName::Unknown => &self.unknown_regex,
        }
    }

    pub fn pack_value(&self, token_name: &TokenName, value: &str) -> String {
        match token_name {
            TokenName::Whitespace => value.to_string(),
            TokenName::Float => value.trim().to_string(),
            TokenName::Integer => value.trim().to_string(),
            TokenName::Letter => value
                .trim()
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .to_string(),
            TokenName::Repeater => value
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .to_string(),
            TokenName::UString => value
                .trim()
                .trim_start_matches('\"')
                .trim_end_matches('\"')
                .to_string(),
            TokenName::Variable => value.trim().to_string(),
            TokenName::Plus => value.to_string(),
            TokenName::Minus => value.to_string(),
            TokenName::Stdout => value.to_string(),
            TokenName::Semicolon => value.to_string(),
            TokenName::Unknown => value.to_string(),
        }
    }
}
