use crate::lex::{Token, TokenName};

#[derive(Debug, PartialEq, Clone)]
enum SyntaxTreeKind {
    ProgramStart,
    /*
    Statement,
    Source,
    UnaryOp,
    Sink,
    */
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxTree {
    kind: SyntaxTreeKind,
    parent: Option<Box<SyntaxTreeKind>>,
    children: Vec<SyntaxTree>,
}

impl SyntaxTree {
    fn root() -> Self {
        SyntaxTree {
            kind: SyntaxTreeKind::ProgramStart,
            parent: None,
            children: Vec::new(),
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

    fn handle_next(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match &self.state {
            SyntaxParserState::GetNextLine => self.get_next_line_or_end_program(token, tree),
            SyntaxParserState::BuildingStatement => self.append_to_statement(token, tree),
            _ => panic!("state not implemented"),
        }
    }

    fn get_next_line_or_end_program(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match token {
            // None == end of file
            None => self.state = SyntaxParserState::Complete,
            Some(token) => self.start_next_line(token, tree),
        }
    }

    fn start_next_line(&mut self, token: &Token, tree: &mut SyntaxTree) {
        match token.name {
            TokenName::Integer => {
                println!(
                    "Would start new Statement with integer value {:?}",
                    token.value
                );
                println!("Transition state to BuildingStatement");
                self.state = SyntaxParserState::BuildingStatement;
            }
            _ => println!("NOOP for {:?}", token),
        }
    }

    fn append_to_statement(&mut self, token: &Option<&Token>, tree: &mut SyntaxTree) {
        match token {
            None => panic!("Unexpected end of file"),
            Some(token) => match token.name {
                TokenName::Plus => {
                    println!("Would append {:?} to current statement", token.name);
                }
                TokenName::Stdout => {
                    println!("Would append {:?} to current statement", token.name);
                    println!("Statement complete; Transition state to GetNextLine");
                    self.state = SyntaxParserState::GetNextLine;
                }
                _ => {
                    panic!("Unexpected token!");
                }
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
