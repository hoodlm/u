use std::env;
use std::fs;

use regex::Regex;

fn main() {
    println!("\n==== u ====");
    let filename = infile_from_args();
    let input = fs::read_to_string(&filename).unwrap();

    println!("Interpreting {}", filename);
    println!("{}", input);
    println!("...");

    println!("Starting lexical analysis: ");
    let tokens = lex_analysis(&input);

    tokens.iter().for_each(|t| {
        println!("{:?}", t);
    });
    println!("...");

    println!("Starting syntax analysis: ");
    let ast = syntax_analysis(&tokens);
    println!("{:?}", ast);
}

fn infile_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: u hello.u");
    }
    return String::from(&args[1]);
}

#[derive(Debug, PartialEq, Clone)]
enum TokenName {
    ProgramStart,
    Integer,
    Plus,
    Stdout,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
    name: TokenName,
    value: Option<String>,
}

fn lex_analysis(input: &String) -> Vec<Token> {
    let tokens: Vec<Token> = input.split_whitespace().map(|it| to_token(it)).collect();

    let errors: Vec<String> = tokens
        .iter()
        .filter(|token| token.name == TokenName::Unknown)
        .map(|token| format!("Unknown token {:?}", token.value))
        .collect();

    if errors.is_empty() {
        return tokens;
    } else {
        let message = errors.join("\n");
        panic!("{}", message);
    }
}

fn to_token(t: &str) -> Token {
    let int_regex = Regex::new(r"[0-9]+").unwrap();

    let (name, value) = match t {
        t if int_regex.is_match(t) => (TokenName::Integer, Some(t.to_string())),
        "+" => (TokenName::Plus, None),
        "STDOUT" => (TokenName::Stdout, None),
        _ => (TokenName::Unknown, Some(t.to_string())),
    };
    return Token { name, value };
}

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
struct SyntaxTree {
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

fn syntax_analysis(input: &Vec<Token>) -> SyntaxTree {
    let mut tree = SyntaxTree::root();
    let mut syntax_parser = SyntaxParser::new();
    let mut tokens = input.iter();

    while !syntax_parser.is_done() {
        let token = tokens.next();
        syntax_parser.handle_next(&token, &mut tree);
    }
    return tree;
}
