use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;

use crate::lex::tokens::{Token, TokenName};
use crate::syntax::tree::{SyntaxTree, SyntaxTreeKind};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken { unexpected: Token, message: String },
    UndeclaredVariable { var_name: String },
    AssignedVariableTwice { var_name: String },
    LineIncomplete,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedToken {
                unexpected,
                message,
            } => {
                write!(f, "Unexpected token: {:?}, {}", unexpected, message)
            }
            SyntaxError::LineIncomplete => {
                write!(f, "Expected more tokens before end of line")
            }
            SyntaxError::UndeclaredVariable { var_name } => {
                write!(f, "Variable not declared: {}", var_name)
            }
            SyntaxError::AssignedVariableTwice { var_name } => {
                write!(f, "Cannot assign to this variable twice: {}", var_name)
            }
        }
    }
}
impl Error for SyntaxError {}

pub trait SyntaxParser {
    fn parse(
        &mut self,
        variables_in_scope: &mut HashSet<String>,
        tokens: &mut Peekable<Iter<'_, Token>>,
    ) -> Result<SyntaxTree, Vec<SyntaxError>>;
}

#[derive(Debug, PartialEq)]
pub struct ProgramParser;

impl SyntaxParser for ProgramParser {
    fn parse(
        &mut self,
        variables_in_scope: &mut HashSet<String>,
        tokens: &mut Peekable<Iter<'_, Token>>,
    ) -> Result<SyntaxTree, Vec<SyntaxError>> {
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
            let mut sp = StatementParser {};
            let result = sp.parse(variables_in_scope, tokens);
            match result {
                Ok(subtree) => tree.add_child(subtree),
                Err(errors) => {
                    errors.iter().for_each(|e| syntax_errors.push(e.clone()));
                }
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
        ProgramParser {}
    }
}

#[derive(Debug, PartialEq)]
pub struct StatementParser;

impl SyntaxParser for StatementParser {
    fn parse(
        &mut self,
        variables_in_scope: &mut HashSet<String>,
        tokens: &mut Peekable<Iter<'_, Token>>,
    ) -> Result<SyntaxTree, Vec<SyntaxError>> {
        let mut statement = SyntaxTree::new(SyntaxTreeKind::Statement, None);
        let mut errors: Vec<SyntaxError> = Vec::new();

        let source_token = tokens
            .next()
            .expect("Internal error: SyntaxParser.parse called with an empty token iterator");
        match source_token.name {
            TokenName::Letter | TokenName::Integer | TokenName::Float | TokenName::UString => {
                let source = SyntaxTree::new(SyntaxTreeKind::Source, Some(source_token.clone()));
                statement.add_child(source);
            }
            TokenName::Variable => {
                if variables_in_scope.contains(&source_token.value) {
                    let source = SyntaxTree::new(SyntaxTreeKind::Source, Some(source_token.clone()));
                    statement.add_child(source);
                } else {
                    errors.push(
                        SyntaxError::UndeclaredVariable { var_name: source_token.value.clone() }
                    )
                }
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
                }
                TokenName::Variable => {
                    if variables_in_scope.contains(&token.value) {
                        errors.push(
                            SyntaxError::AssignedVariableTwice { var_name: token.value.to_string() }
                        )
                    } else {
                        variables_in_scope.insert(token.value.to_string());
                        let op = SyntaxTree::new(SyntaxTreeKind::UnaryOp, Some(token.clone()));
                        statement.add_child(op);
                    }
                }
                TokenName::Repeater => match RepeaterParser::new(token.clone()).parse(variables_in_scope, tokens) {
                    Err(repeater_errors) => {
                        repeater_errors.iter().for_each(|e| errors.push(e.clone()));
                    }
                    Ok(subtree) => {
                        statement.add_child(subtree);
                    }
                },
                TokenName::Semicolon => {
                    let end = SyntaxTree::new(SyntaxTreeKind::EndOfLine, None);
                    statement.add_child(end);
                    line_completed = true;
                    break;
                }
                _ => errors.push(SyntaxError::UnexpectedToken {
                    unexpected: token.clone(),
                    message: String::from("StatementParser: expected UnaryOperator or Semicolon"),
                }),
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

#[derive(Debug, PartialEq)]
pub struct RepeaterParser {
    token: Token,
}

impl RepeaterParser {
    pub fn new(token: Token) -> Self {
        return RepeaterParser { token };
    }
}

impl SyntaxParser for RepeaterParser {
    fn parse(
        &mut self,
        variables_in_scope: &mut HashSet<String>,
        tokens: &mut Peekable<Iter<'_, Token>>,
    ) -> Result<SyntaxTree, Vec<SyntaxError>> {
        let mut subtree =
            SyntaxTree::new(SyntaxTreeKind::RepeatedUnaryOp, Some(self.token.clone()));
        let mut errors: Vec<SyntaxError> = Vec::new();

        let operator = tokens
            .next()
            .expect("Internal error: RepeaterParser.parse called with an empty token iterator");
        match operator.name {
            TokenName::Repeater => {
                let nested_repeater_result = RepeaterParser::new(operator.clone()).parse(variables_in_scope, tokens);
                match nested_repeater_result {
                    Ok(nested_repeater) => subtree.add_child(nested_repeater),
                    Err(suberrors) => suberrors.iter().for_each(|e| errors.push(e.clone())),
                }
            }
            TokenName::Plus | TokenName::Minus | TokenName::Stdout => {
                let op = SyntaxTree::new(SyntaxTreeKind::UnaryOp, Some(operator.clone()));
                subtree.add_child(op);
            }
            _ => errors.push(SyntaxError::UnexpectedToken {
                unexpected: operator.clone(),
                message: String::from("RepeaterParser: expected UnaryOp"),
            }),
        }

        if errors.is_empty() {
            Ok(subtree)
        } else {
            Err(errors)
        }
    }
}
