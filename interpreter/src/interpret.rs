use crate::syntax::{SyntaxTree, SyntaxTreeKind};
use crate::lex::{TokenName};
use std::fmt::{Formatter, Display};
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
enum UValue {
    Integer(i64),
    Float(f64),
}

impl Display for UValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UValue::Integer(int) => write!(f, "{}", int),
            UValue::Float(float) => write!(f, "{}", float),
        }
    }
}

impl Add<i32> for &UValue {
    type Output = UValue;
    fn add(self, other: i32) -> Self::Output {
        match self {
            UValue::Integer(int) => UValue::Integer(int + other as i64),
            UValue::Float(float) => UValue::Float(float + other as f64),
        }
    }
}

impl Sub<i32> for &UValue {
    type Output = UValue;
    fn sub(self, other: i32) -> Self::Output {
        match self {
            UValue::Integer(int) => UValue::Integer(int - other as i64),
            UValue::Float(float) => UValue::Float(float - other as f64),
        }
    }
}

pub fn execute(program: &SyntaxTree) {
    assert!(
        program.kind == SyntaxTreeKind::ProgramStart,
        "program SyntaxTree passed to execute must be of type ProgramStart"
    );

    program.children.iter().for_each(|line| match &line.kind {
        SyntaxTreeKind::Statement => {
            exec_statement(&line);
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

fn exec_statement(statement: &SyntaxTree) {
    let (source, sink) = prevalidate_statement(&statement);
    let source_value = get_source_value(&source);

    let mut result = source_value;

    let operators = &statement.children[1..(statement.children.len() - 1)];

    operators.iter().for_each(|item| match &item.kind {
        _ => {
            result = apply_operator(&result, item);
        }
    });

    apply_result_to_sink(result, &sink);
}

fn prevalidate_statement(statement: &SyntaxTree) -> (&SyntaxTree, &SyntaxTree) {
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
        sink.kind == SyntaxTreeKind::Sink,
        "Statement syntaxTree's last node should be a Sink"
    );
    return (source, sink);
}

fn get_source_value(source_node: &SyntaxTree) -> UValue {
    assert!(
        source_node.kind == SyntaxTreeKind::Source,
        "SyntaxTree passed to get_source_value must be of type Source"
    );
    match &source_node.token {
        None => panic!("Source nodes should always have a token: {:?}", &source_node),
        Some(t) => {
            match t.name {
                TokenName::Integer => {
                    let val: i64 = t.value.clone().unwrap().parse().expect("Malformed integer value");
                    return UValue::Integer(val);
                },
                TokenName::Float => {
                    let val: f64 = t.value.clone().unwrap().parse().expect("Malformed float value");
                    return UValue::Float(val);
                },
                _ => {
                    panic!("Unexpected token in Source node: {:?}", t);
                }
            }
        }
    };
}

fn apply_operator(input: &UValue, operator: &SyntaxTree) -> UValue {
    assert!(
        operator.kind == SyntaxTreeKind::UnaryOp,
        "SyntaxTree passed to get_source_value must be of type UnaryOp"
    );
    match &operator.token {
        None => panic!("UnaryOp nodes should always have a token: {:?}", &operator),
        Some(t) => {
            match t.name {
                TokenName::Plus => {
                    return input + 1;
                },
                TokenName::Minus => {
                    return input - 1;
                },
                _ => {
                    panic!("Unexpected token in UnaryOp node: {:?}", t);
                }
            }
        }
    };
}

fn apply_result_to_sink(result: UValue, sink: &SyntaxTree) {
    assert!(
        sink.kind == SyntaxTreeKind::Sink,
        "SyntaxTree passed to get_source_value must be of type Sink"
    );
    match &sink.token {
        None => panic!("Sink nodes should always have a token: {:?}", &sink),
        Some(t) => {
            match t.name {
                TokenName::Stdout => {
                    println!("{}", result);
                }
                _ => {
                    panic!("Unexpected token in Sink node: {:?}", t);
                }
            }
        }
    };
}

