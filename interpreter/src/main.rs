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

    let tokens = lex_analysis(&input);

    tokens.iter().for_each(|t| {
        println!("{:?}", t);
    });
}

fn infile_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: u hello.u");
    }
    return String::from(&args[1]);
}

#[derive(Debug, PartialEq)]
enum TokenName {
    Integer,
    Plus,
    Stdout,
    Unknown,
}

#[derive(Debug)]
struct Token {
    name: TokenName,
    value: Option<String>,
}

fn lex_analysis(input: &String) -> Vec<Token> {
    let tokens: Vec<Token> = input.split_whitespace().map(|it| to_token(it)).collect();

    let errors: Vec<String> = tokens.iter()
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
