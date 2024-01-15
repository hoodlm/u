use std::env;
use std::fs;
use std::process::ExitCode;

use u::syntax::syntax_analysis;
use u::lex::lex_analysis;
use u::interpret::{execute};

fn main() -> ExitCode {
    let filename = infile_from_args();
    let input = fs::read_to_string(&filename).unwrap();

    let lex_result = lex_analysis(&input);
    let tokens = match lex_result {
        Ok(tokens) => tokens,
        Err(errors) => {
            errors.iter().for_each(|msg| {
                eprintln!("Lexical analysis failed!");
                eprintln!("{}", msg);
            });
            return ExitCode::FAILURE;
        },
    };
    let ast = syntax_analysis(&tokens);
    execute(&ast);
    return ExitCode::SUCCESS;
}

fn infile_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: u hello.u");
    }
    return String::from(&args[1]);
}

