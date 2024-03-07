use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::lex::tokens::{Token, TokenName};

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

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxTreeKind {
    ProgramStart,
    Statement,
    Source,
    UnaryOp,
    EndOfLine,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxTree {
    pub kind: SyntaxTreeKind,
    pub children: Vec<SyntaxTree>,
    pub token: Option<Token>,
}

impl SyntaxTree {
    fn root() -> Self {
        SyntaxTree {
            kind: SyntaxTreeKind::ProgramStart,
            children: Vec::new(),
            token: None,
        }
    }

    fn new(kind: SyntaxTreeKind, token: Option<Token>) -> Self {
        SyntaxTree {
            kind: kind,
            children: Vec::new(),
            token: token,
        }
    }

    fn add_child(&mut self, child: SyntaxTree) {
        self.children.push(child);
    }
}

#[derive(Debug, PartialEq)]
struct SyntaxParser {
    state: SyntaxParserState,
}

#[derive(Debug, PartialEq)]
enum SyntaxParserState {
    GetNextLine,
    BuildingStatement,
    Complete,
}

impl SyntaxParser {
    fn new() -> Self {
        return SyntaxParser {
            state: SyntaxParserState::GetNextLine,
        };
    }

    fn is_done(&self) -> bool {
        self.state == SyntaxParserState::Complete
    }

    fn handle_next(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        if token.is_some() && token.unwrap().name == TokenName::Whitespace {
            // Whitespace is not syntactically significant
            return Ok(());
        }
        match &self.state {
            SyntaxParserState::GetNextLine => self.get_next_line_or_end_program(token, tree),
            SyntaxParserState::BuildingStatement => self.append_to_statement(token, tree),
            _ => panic!("Internal error: state not implemented"),
        }
    }

    fn get_next_line_or_end_program(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        match token {
            None => {
                self.state = SyntaxParserState::Complete;
                Ok(())
            },
            Some(token) => self.start_next_line(token, tree),
        }
    }

    fn start_next_line(&mut self, token: &Token, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        match token.name {
            TokenName::Letter | TokenName::Integer | TokenName::Float | TokenName::UString => {
                let mut statement = SyntaxTree::new(SyntaxTreeKind::Statement, None);
                let source = SyntaxTree::new(SyntaxTreeKind::Source, Some(token.clone()));
                statement.add_child(source);
                tree.add_child(statement);
                self.state = SyntaxParserState::BuildingStatement;
                Ok(())
            }
            _ => {
                Err(SyntaxError::UnexpectedToken { unexpected: token.clone() })
            }
        }
    }

    fn append_to_statement(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
        match token {
            None => {
                self.state = SyntaxParserState::GetNextLine;
                Err(SyntaxError::LineIncomplete)
            },
            Some(token) => {
                self.append_to_statement_concrete(token, tree)
            }
        }
    }

    fn append_to_statement_concrete(&mut self, token: &Token, tree: &mut SyntaxTree) -> Result<(), SyntaxError> {
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
                    self.state = SyntaxParserState::GetNextLine;
                    Ok(())
            },
            _ => {
                    Err(SyntaxError::UnexpectedToken { unexpected: token.clone() })
            },
        }
    }
}

pub fn syntax_analysis(input: &Vec<Token>) -> Result<SyntaxTree, Vec<SyntaxError>> {
    let mut tree = SyntaxTree::root();
    let mut syntax_parser = SyntaxParser::new();
    let mut tokens = input.iter();
    let mut syntax_errors: Vec<SyntaxError> = Vec::new();

    while !syntax_parser.is_done() {
        let token = tokens.next();
        let token_result = syntax_parser.handle_next(&token, &mut tree);
        match token_result {
            Err(error) => syntax_errors.push(error),
            Ok(_) => {},
        }
    }
    if syntax_errors.is_empty() {
        Ok(tree)
    } else {
        Err(syntax_errors)
    }
}
