use crate::lex::tokens::TokenName;
use crate::syntax::tree::{SyntaxTree, SyntaxTreeKind};
use std::char::from_digit;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
enum UValue {
    Integer(i64),
    Float(f64),
    Letter(char),
    UString(String),
}

impl Display for UValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UValue::Integer(int) => write!(f, "{}", int),
            UValue::Float(float) => write!(f, "{}", float),
            UValue::Letter(c) => write!(f, "{}", c),
            UValue::UString(s) => write!(f, "{}", s),
        }
    }
}

impl Add<i32> for &UValue {
    type Output = UValue;
    fn add(self, other: i32) -> Self::Output {
        match self {
            UValue::Integer(int) => UValue::Integer(int + other as i64),
            UValue::Float(float) => UValue::Float(float + other as f64),
            UValue::Letter(c) => UValue::Letter(char_add(c, other)),
            UValue::UString(s) => UValue::UString(s.chars().map(|c| char_add(&c, other)).collect()),
        }
    }
}

impl Sub<i32> for &UValue {
    type Output = UValue;
    fn sub(self, other: i32) -> Self::Output {
        match self {
            UValue::Integer(int) => UValue::Integer(int - other as i64),
            UValue::Float(float) => UValue::Float(float - other as f64),
            UValue::Letter(c) => UValue::Letter(char_sub(c, other)),
            UValue::UString(s) => UValue::UString(s.chars().map(|c| char_sub(&c, other)).collect()),
        }
    }
}

fn char_sub(c: &char, other: i32) -> char {
    if c.is_whitespace() {
        return c.clone();
    }
    let n: i32 = (c.to_digit(36).unwrap() - 10).try_into().unwrap();
    let decremented = (n - other).rem_euclid(26) + 10;
    let mut result_char = from_digit(decremented.try_into().unwrap(), 36).unwrap();
    if c.is_ascii_uppercase() {
        result_char = result_char.to_ascii_uppercase();
    }
    return result_char;
}

fn char_add(c: &char, other: i32) -> char {
    if c.is_whitespace() {
        return c.clone();
    }
    let n = c.to_digit(36).unwrap() - 10;
    let incremented = (n + other as u32) % 26;
    let mut result_char = from_digit(incremented + 10, 36).unwrap();
    if c.is_ascii_uppercase() {
        result_char = result_char.to_ascii_uppercase();
    }
    return result_char;
}

pub struct UInterpreter {
    variable_table: HashMap<String, UValue>,
}

impl UInterpreter {
    pub fn new() -> Self {
        UInterpreter {
            variable_table: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: &SyntaxTree) {
        assert!(
            program.kind == SyntaxTreeKind::ProgramStart,
            "program SyntaxTree passed to execute must be of type ProgramStart"
        );

        program.children.iter().for_each(|line| match &line.kind {
            SyntaxTreeKind::Statement => {
                self.exec_statement(&line);
            }
            other => {
                let msg = format!(
                    "Syntax subtree directly below ProgramStart is an unexpected kind: {:?}",
                    other
                );
                panic!("{}", msg);
            }
        });
    }

    pub fn exec_statement(&mut self, statement: &SyntaxTree) {
        self.prevalidate_statement(&statement);
        let source_value = self.get_source_value(&statement.children[0]);

        let mut result = source_value;

        let operators = &statement.children[1..(statement.children.len() - 1)];

        operators.iter().for_each(|item| match &item.kind {
            _ => {
                result = self.apply_operator(&result, item);
            }
        });
    }

    fn prevalidate_statement(&self, statement: &SyntaxTree) {
        assert!(
            statement.kind == SyntaxTreeKind::Statement,
            "SyntaxTree passed to exec_statement must be of type Statement"
        );
        let children = &statement.children;
        assert!(
            children.len() >= 2,
            "Statement syntaxtreenode should have had at least two child nodes"
        );

        let source = &children[0];
        let sink = &children[children.len() - 1];
        assert!(
            source.kind == SyntaxTreeKind::Source,
            "Statement syntaxTree's first child should be a Source"
        );
        assert!(
            sink.kind == SyntaxTreeKind::EndOfLine,
            "Statement syntaxTree's last node should be a EndOfLine"
        );
    }

    fn get_source_value(&self, source_node: &SyntaxTree) -> UValue {
        assert!(
            source_node.kind == SyntaxTreeKind::Source,
            "SyntaxTree passed to get_source_value must be of type Source"
        );
        match &source_node.token {
            None => panic!(
                "Source nodes should always have a token: {:?}",
                &source_node
            ),
            Some(t) => match t.name {
                TokenName::Integer => {
                    let val: i64 = t.value.parse().expect("Malformed integer value");
                    return UValue::Integer(val);
                }
                TokenName::Float => {
                    let val: f64 = t.value.parse().expect("Malformed float value");
                    return UValue::Float(val);
                }
                TokenName::Letter => {
                    let val: char = t.value.chars().collect::<Vec<char>>()[0].clone();
                    return UValue::Letter(val);
                }
                TokenName::UString => {
                    let val: String = t.value.to_string();
                    return UValue::UString(val);
                }
                TokenName::Variable => {
                    let val = self.variable_table.get(&t.value);
                    if val.is_none() {
                        panic!("Internal error: variable {:?} not found in table (this should have been caught sooner as a syntax error!",
                        t);
                    }
                    return val.unwrap().clone();
                }
                _ => {
                    panic!("Unexpected token in Source node: {:?}", t);
                }
            },
        };
    }

    fn apply_operator(&mut self, input: &UValue, operator: &SyntaxTree) -> UValue {
        match operator.kind {
            SyntaxTreeKind::RepeatedUnaryOp => {
                if operator.children.len() != 1 {
                    panic!(
                        "RepeatedUnaryOp subtree should have exactly one child: {:?}",
                        operator
                    );
                }
                let repeated_operator = &operator.children[0];
                let repeat_count: u32 = operator
                    .token
                    .clone()
                    .expect("Internal error: RepeatedUnaryOp node should have a token")
                    .value
                    .parse()
                    .expect("Internal error: Failed to parse u32 from repeater token");
                let mut result = input.clone();
                for _ in 0..repeat_count {
                    result = self.apply_operator(&result, &repeated_operator);
                }
                result
            }
            SyntaxTreeKind::UnaryOp => {
                match &operator.token {
                    None => panic!("UnaryOp nodes should always have a token: {:?}", &operator),
                    Some(token) => match token.name {
                        TokenName::Plus => {
                            return input + 1;
                        }
                        TokenName::Minus => {
                            return input - 1;
                        }
                        TokenName::Stdout => {
                            println!("{}", input);
                            return input.clone();
                        }
                        TokenName::Variable => {
                            self.variable_table.insert(token.value.clone(), input.clone());
                            return input.clone();
                        }
                        _ => {
                            panic!("Unexpected token in UnaryOp node: {:?}", token);
                        }
                    },
                };
            }
            _ => {
                panic!("SyntaxTree passed to get_source_value must be of type UnaryOp");
            }
        }
    }
}
