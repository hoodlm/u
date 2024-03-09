use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;

use crate::lex::tokens::{Token, TokenName};
use crate::syntax::tree::{SyntaxTree, SyntaxTreeKind};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken { unexpected: Token, message: String },
    LineIncomplete,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedToken { unexpected, message } => {
                write!(f, "Unexpected token: {:?}, {}", unexpected, message)
            },
            SyntaxError::LineIncomplete => {
                write!(f, "Expected more tokens before end of line")
            },
        }
    }
}
impl Error for SyntaxError {}

pub trait SyntaxParser {
    fn parse(&mut self, tokens: &mut Peekable<Iter<'_, Token>>) -> Result<SyntaxTree, Vec<SyntaxError>>;
}

#[derive(Debug, PartialEq)]
pub struct ProgramParser;

impl SyntaxParser for ProgramParser {
    fn parse(&mut self, tokens: &mut Peekable<Iter<'_, Token>>) -> Result<SyntaxTree, Vec<SyntaxError>> {
        let mut tree = SyntaxTree::root();
        let mut syntax_errors: Vec<SyntaxError> = Vec::new();

        loop {
            // kinda hacky, skip over whitespace
            while let Some(TokenName::Whitespace) = tokens.peek().and_then(|it| Some(it.name)) {
                tokens.next();
            }
            if let None = tokens.peek() {
                break;
            }
            let mut sp = StatementParser{};
            let result = sp.parse(tokens);
            match result {
                Ok(subtree) => tree.add_child(subtree),
                Err(errors) => {
                    errors.iter().for_each(|e| { syntax_errors.push(e.clone()) });
                },
            }
        }

        if syntax_errors.is_empty() {
            Ok(tree)
        } else {
            Err(syntax_errors)
        }
    }
}

impl ProgramParser {
    pub fn new() -> Self {
        return ProgramParser {};
    }
}

#[derive(Debug, PartialEq)]
pub struct StatementParser {
}


impl SyntaxParser for StatementParser {
    fn parse(&mut self, tokens: &mut Peekable<Iter<'_, Token>>) -> Result<SyntaxTree, Vec<SyntaxError>> {
        let mut statement = SyntaxTree::new(SyntaxTreeKind::Statement, None);
        let mut errors: Vec<SyntaxError> = Vec::new();

        let source_token = tokens.next().expect("Internal error: SyntaxParser.parse called with an empty token iterator");
        match source_token.name {
            TokenName::Letter | TokenName::Integer | TokenName::Float | TokenName::UString => {
                let source = SyntaxTree::new(SyntaxTreeKind::Source, Some(source_token.clone()));
                statement.add_child(source);
            }
            _ => {
                errors.push(
                    SyntaxError::UnexpectedToken {
                        unexpected: source_token.clone(),
                        message: String::from("StatementParser: expected statement to start with Letter/Integer/Float/UString"),
                    }
                )
            }
        }

        let mut line_completed = false;
        while let Some(token) = tokens.next() {
            match token.name {
                TokenName::Plus | TokenName::Minus | TokenName::Stdout => {
                        let op = SyntaxTree::new(SyntaxTreeKind::UnaryOp, Some(token.clone()));
                        statement.add_child(op);
                },
                TokenName::Semicolon => {
                        let end = SyntaxTree::new(SyntaxTreeKind::EndOfLine, None);
                        statement.add_child(end);
                        line_completed = true;
                        break;
                },
                _ => {
                    errors.push(
                        SyntaxError::UnexpectedToken {
                            unexpected: token.clone(),
                            message: String::from("StatementParser: expected UnaryOperator or Semicolon"),
                        }
                    )
                },
            }
        }
        if !line_completed {
            errors.push(SyntaxError::LineIncomplete);
        }
        if errors.is_empty() {
            Ok(statement)
        } else {
            Err(errors)
        }
    }
}
