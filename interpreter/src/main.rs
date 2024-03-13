use std::env;
use std::fs;
use std::process::ExitCode;

use u::interpret::{UInterpreter};
use u::lex::lex_analysis;
use u::syntax::parser::{ProgramParser, SyntaxParser};

fn main() -> ExitCode {
    let filename = infile_from_args();
    let input = fs::read_to_string(&filename).unwrap();

    let lex_result = lex_analysis(&input);
    let tokens = match lex_result {
        Ok(tokens) => tokens,
        Err(errors) => {
            eprintln!("Lexical analysis failed!");
            errors.iter().for_each(|msg| {
                eprintln!("{}", msg);
            });
            return ExitCode::FAILURE;
        }
    };
    let mut token_iter = tokens.iter().peekable();
    let syntax_result = ProgramParser::new().parse(&mut token_iter);
    let ast = match syntax_result {
        Ok(ast) => ast,
        Err(errors) => {
            eprintln!("Syntax analysis failed!");
            errors.iter().for_each(|msg| {
                eprintln!("{}", msg);
            });
            return ExitCode::FAILURE;
        }
    };
    UInterpreter::new().execute(&ast);
    return ExitCode::SUCCESS;
}

fn infile_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: u hello.u");
    }
    return String::from(&args[1]);
}
