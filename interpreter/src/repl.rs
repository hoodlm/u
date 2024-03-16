use std::collections::HashSet;

use u::interpret::{UInterpreter, UValue};
use u::lex::lex_analysis;
use u::syntax::parser::{ProgramParser, SyntaxParser};

fn main() {
    banner();

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut variables = HashSet::new();
    let mut syntax_analyzer = ProgramParser::new();
    let mut interpreter = UInterpreter::new();

    loop {
        let program = rl.readline("> ").unwrap();
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
}

fn banner() {
    println!("u!");
    println!("type 'exit' to close");
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
