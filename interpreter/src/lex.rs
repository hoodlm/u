use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenName {
    ProgramStart,
    Integer,
    Plus,
    Stdout,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub name: TokenName,
    pub value: Option<String>,
}

pub fn lex_analysis(input: &String) -> Vec<Token> {
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
