use std::env;
use std::fs;

use u::syntax::syntax_analysis;
use u::lex::lex_analysis;
use u::interpret::{execute};

fn main() {
    let filename = infile_from_args();
    let input = fs::read_to_string(&filename).unwrap();

    let tokens = lex_analysis(&input);
    let ast = syntax_analysis(&tokens);
    execute(&ast);
}

fn infile_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: u hello.u");
    }
    return String::from(&args[1]);
}

