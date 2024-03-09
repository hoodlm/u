use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::lex::tokens::{Token, TokenName};
use crate::syntax::tree::{SyntaxTree, SyntaxTreeKind};

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken { unexpected: Token },
    LineIncomplete,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedToken { unexpected } => {
                write!(f, "Unexpected token: {:?}", unexpected)
            },
            SyntaxError::LineIncomplete => {
                write!(f, "Expected more tokens before end of line")
            },
        }
    }
}
impl Error for SyntaxError {}

pub trait SyntaxParser {
    fn parse(&mut self, tokens: &mut dyn Iterator::<Item = &Token>) -> Result<SyntaxTree, Vec<SyntaxError>>;
}

#[derive(Debug, PartialEq)]
pub struct ProgramParser {
    state: ProgramParserState,
}

#[derive(Debug, PartialEq)]
enum ProgramParserState {
    GetNextLine,
    BuildingStatement,
}

impl SyntaxParser for ProgramParser {
    fn parse(&mut self, tokens: &mut dyn Iterator<Item = &Token>) -> Result<SyntaxTree, Vec<SyntaxError>> {
        let mut tree = SyntaxTree::root();
        let mut syntax_errors: Vec<SyntaxError> = Vec::new();

        tokens.for_each(|token| {
            let token_result = self.handle_next(token, &mut tree);
            match token_result {
                Err(error) => syntax_errors.push(error),
                Ok(_) => {},
            }
        });

        if self.state != ProgramParserState::GetNextLine {
            syntax_errors.push(SyntaxError::LineIncomplete {});
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
        return ProgramParser {
            state: ProgramParserState::GetNextLine,
        };
    }

    fn handle_next(&mut self, token: &Token, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        if token.name == TokenName::Whitespace {
            // Whitespace is not syntactically significant
            return Ok(());
        }
        match &self.state {
            ProgramParserState::GetNextLine => self.start_next_line(token, tree),
            ProgramParserState::BuildingStatement => self.append_to_statement(token, tree),
        }
    }

    fn start_next_line(&mut self, token: &Token, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        match token.name {
            TokenName::Letter | TokenName::Integer | TokenName::Float | TokenName::UString => {
                let mut statement = SyntaxTree::new(SyntaxTreeKind::Statement, None);
                let source = SyntaxTree::new(SyntaxTreeKind::Source, Some(token.clone()));
                statement.add_child(source);
                tree.add_child(statement);
                self.state = ProgramParserState::BuildingStatement;
                Ok(())
            }
            _ => {
                Err(SyntaxError::UnexpectedToken { unexpected: token.clone() })
            }
        }
    }

    fn append_to_statement(&mut self, token: &Token, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        match token.name {
            TokenName::Plus | TokenName::Minus | TokenName::Stdout => {
                    let op = SyntaxTree::new(SyntaxTreeKind::UnaryOp, Some(token.clone()));
                    let tip = tree.children.iter_mut().last()
                        .expect("Internal error: Should not enter append_to_statement_concrete() if tree has no children");
                    tip.add_child(op);
                    Ok(())
            },
            TokenName::Semicolon => {
                    let end = SyntaxTree::new(SyntaxTreeKind::EndOfLine, None);
                    let tip = tree.children.iter_mut().last()
                        .expect("Internal error: Should not enter append_to_statement_concrete() if tree has no children");
                    tip.add_child(end);
                    self.state = ProgramParserState::GetNextLine;
                    Ok(())
            },
            _ => {
                    Err(SyntaxError::UnexpectedToken { unexpected: token.clone() })
            },
        }
    }
}

