`u` is a functional programming language based around unary operators.

# Intro to `u`

Examples of unary operators you're probably familiar with from other
languages are the increment and decrement operators:

```
x++
y--
```

In `u`, the increment and decrement operators are represented with a single plus `+` or a single `-`:

```
> 1 +;
2
> 99 -;
98
```

A line of `u` starts with a value (either a literal or a variable) followed by a chain of operators, and
is terminated by a semicolon. This program starts with the literal 100, increments it twice, then decrements it once:

```
> 100 + + -;
101
```

You can insert the STDOUT 'pseudo-operator' to observe the value as it's changed by each operator:

```
> 100 STDOUT + STDOUT + STDOUT -;
100
101
102
101
```

# Getting started

This repo contains a language specification and two interpreter binaries. `u` is an interpreter that runs against files on disk.
`ur` is a REPL (Read, Eval, Print, Loop) for interactively using the language.

To build from source, clone this repository, then:

```
cargo install --path ./interpreter/
```

When you're done:

```
cargo uninstall u
```

# Types and Operators

| Type         | Literal       | + (increment)               | - (decrement)                   |
|--------------|---------------|-----------------------------|---------------------------------|
| Integer      | 0             | +1                          | -1                              |
| Float        | 1.23          | +1.0                        | -1.0                            |
| Char         | 'a'           | next letter in the alphabet | previous letter in the alphabet |
| String       | "hello world" | increments each letter      | decrements each letter          |

Operations on floats are subject to system-dependent floating point rounding; e.g.

```
> 1.23 -;
0.22999999999999998
```

## Pseudo-operators

### Variable assignment

Variables are denoted by a $ followed by a variable name. Valid variable names can contain letters and underscores:

```
> 5 $five;
5
> 53 $fiftyThree;
53
> 147 $ONE_HUNDRED_FORTY_SEVEN;
147
```

You can chain operators after variable assignment:

```
> 10 $ten + $eleven + $twelve;
12
> $ten;
10
> $eleven;
11
```

Variables are assign-once (immutable):

```
> "hello" $greeting;
hello
> "Hey" $greeting;
Syntax analysis failed!
Cannot assign to this variable twice: $greeting
```

and can't be referenced unless they've been assigned:

```
> $null +;
Syntax analysis failed!
Variable not declared: $null
```

### STDOUT

Prints the current value:

```
> 5 STDOUT ++ STDOUT ++;
5
7
9
```

### Repeater

Syntactic sugar to repeat the following operator:

```
> 5 {3} +;
8
```

# More examples

Assign the literal value '2' to the variable 'y':

```
> 2 $y;
```

Add 3 to the value of 'y' and assign it to the variable 'x':

```
> $y +++ $x;
5
```

An equivalent, using `{n}` syntactic sugar to repeat the increment (+) operator three times:

```
> $y {3} + $w;
5
```

`u` supports characters, denoted by single-quotes. Characters are effectively a 26-element unary system:

```
> 'a' +;
b
> 'b' +;
c
> 'z' +;
a
> 'a' -;
z
```

Strings are effectively character arrays. Probably one of the only useful things you can do with `u` is
simple Caesar Ciphers:

```
> "Hello world" {13} +;
Uryyb jbeyq
```

# Algebraically interesting things

The repeater can be used to derive addition and subtraction:

```
> 50 {10} +;
60

> 100 {10} -;
90
```

Repeater over + can be chained to derive multiplication:

```
> 0 {5} {10} +;
50
```

With more than two factors, too. Here's eight factorial:

```
> 0 {8} {7} {6} {5} {4} {3} {2} {1} +;
40320
```

# Testing Suite

`u` comes with a language integration test suite in the [spec](./spec) directory.

The language integration tests are built with [shellspec](shellspec.info) and can be invoked
with the plain `shellspec` command.

They execute directly against the `u` binary, so you need to build the interpreter (e.g.
with `cargo build`) first to run the tests.

# FAQ

## Is `u` a real language?

Yes.

## Can I write real software with `u`?

Yes, you can use `u` to implement any program that can be expressed as a
unary operator over a finite set. If you find unary operators limiting, consider
that a flaw with the underlying laws of universe, not a flaw with `u`.

## Is it Turing complete?

Not by itself. `u` does not support boolean combinatorial logic (like AND/OR) which are a prerequisite to implement even simple automata.

## Is it done?

No, the language specification is a work-in-progress and subject to change.

