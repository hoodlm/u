use std::env;
use std::fs;

use u::syntax::syntax_analysis;
use u::lex::lex_analysis;

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
