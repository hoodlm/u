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

