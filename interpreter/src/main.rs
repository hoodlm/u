use std::env;
use std::fs;

fn main() {
    let filename = infile_from_args();
    let input = fs::read_to_string(&filename).unwrap();

    let tokens = lex_analysis(&input);

    println!("{}", input);
    println!("{:?}", tokens);
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

fn to_token(token_value_raw: &str) -> Token {
    return Token {
        name: TokenName::Unknown,
        value: Some(token_value_raw.to_string())
    };
}
