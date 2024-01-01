use crate::lex::{Token, TokenName};

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxTreeKind {
    ProgramStart,
    Statement,
    Source,
    UnaryOp,
    Sink,
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

    fn add_child(&mut self, kind: SyntaxTreeKind, token: Option<Token>) -> usize {
        let new = SyntaxTree {
            kind: kind,
            children: Vec::new(),
            token: token,
        };
        self.children.push(new);
        return self.children.len() - 1;
    }
}

#[derive(Debug, PartialEq)]
struct SyntaxParser {
    state: SyntaxParserState,
    statement_count: usize,
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
            statement_count: 0,
        };
    }

    fn is_done(&self) -> bool {
        self.state == SyntaxParserState::Complete
    }

    fn handle_next(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match &self.state {
            SyntaxParserState::GetNextLine => self.get_next_line_or_end_program(token, tree),
            SyntaxParserState::BuildingStatement => self.append_to_statement(token, tree),
            _ => panic!("state not implemented"),
        }
    }

    fn get_next_line_or_end_program(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match token {
            None => {
                self.state = SyntaxParserState::Complete;
            },
            Some(token) => self.start_next_line(token, tree),
        }
    }

    fn start_next_line(&mut self, token: &Token, tree: &mut SyntaxTree) {
        match token.name {
            TokenName::Integer => {
                self.statement_count = tree.add_child(SyntaxTreeKind::Statement, None);
                tree.children[self.statement_count].add_child(SyntaxTreeKind::Source, Some(token.clone()));
                self.state = SyntaxParserState::BuildingStatement;
            }
            _ => {} /* noop */,
        }
    }

    fn append_to_statement(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match token {
            None => panic!("Unexpected end of file"),
            Some(token) => self.append_to_statement_concrete(token, tree),
        }
    }

    fn append_to_statement_concrete(&mut self, token: &Token, tree: &mut SyntaxTree) {
        match token.name {
            TokenName::Plus => {
                    tree.children[self.statement_count].add_child(SyntaxTreeKind::UnaryOp, Some(token.clone()));
            },
            TokenName::Stdout => {
                    tree.children[self.statement_count].add_child(SyntaxTreeKind::Sink, Some(token.clone()));
                    self.state = SyntaxParserState::GetNextLine;
            },
            _ => {
                    panic!("Unexpected token!");
            },
        }
    }
}

pub fn syntax_analysis(input: &Vec<Token>) -> SyntaxTree {
    let mut tree = SyntaxTree::root();
    let mut syntax_parser = SyntaxParser::new();
    let mut tokens = input.iter();

    while !syntax_parser.is_done() {
        let token = tokens.next();
        syntax_parser.handle_next(&token, &mut tree);
    }
    return tree;
}
