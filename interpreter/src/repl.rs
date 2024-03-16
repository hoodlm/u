use std::io;
use std::io::BufRead;
use std::io::Write;
use std::collections::HashSet;

use u::interpret::{UInterpreter, UValue};
use u::lex::lex_analysis;
use u::syntax::parser::{ProgramParser, SyntaxParser};

fn main() -> Result<(), std::io::Error> {
    banner();

    let stdin = io::stdin();
    let mut variables = HashSet::new();
    let mut syntax_analyzer = ProgramParser::new();
    let mut interpreter = UInterpreter::new();

    loop {
        let program = read(&stdin)?;
        if program.trim() == "exit" {
            break;
        }
        let result = eval(program, &mut syntax_analyzer, &mut variables, &mut interpreter);
        match result {
            Ok(output) => {
                if let Some(output) = output {
                    println!("{}", output)
                }
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
    Ok(())
}

fn banner() {
    println!("u!");
    println!("type 'exit' to close");
}

fn read(stdin: &io::Stdin) -> Result<String, std::io::Error> {
    print!("> ");
    let _ = io::stdout().flush();
    let mut input_buffer = String::new();
    stdin.lock().read_line(&mut input_buffer)?;
    Ok(input_buffer)
}

fn eval(
    input: String,
    syntax_analyzer: &mut ProgramParser,
    variables: &mut HashSet<String>,
    interpreter: &mut UInterpreter
    ) -> Result<Option<UValue>, ()> {
    let lex_result = lex_analysis(&input);
    let tokens = match lex_result {
        Ok(tokens) => tokens,
        Err(errors) => {
            eprintln!("Lexical analysis failed!");
            errors.iter().for_each(|msg| {
                eprintln!("{}", msg);
            });
            return Err(());
        }
    };
    let mut token_iter = tokens.iter().peekable();
    let syntax_result = syntax_analyzer.parse(variables, &mut token_iter);
    let ast = match syntax_result {
        Ok(ast) => ast,
        Err(errors) => {
            eprintln!("Syntax analysis failed!");
            errors.iter().for_each(|msg| {
                eprintln!("{}", msg);
            });
            return Err(());
        }
    };
    let result = interpreter.execute(&ast)?;
    Ok(result)
}
