use crate::lex::tokens::Token;

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
    pub fn root() -> Self {
        SyntaxTree {
            kind: SyntaxTreeKind::ProgramStart,
            children: Vec::new(),
            token: None,
        }
    }

    pub fn new(kind: SyntaxTreeKind, token: Option<Token>) -> Self {
        SyntaxTree {
            kind: kind,
            children: Vec::new(),
            token: token,
        }
    }

    pub fn add_child(&mut self, child: SyntaxTree) {
        self.children.push(child);
    }
}

