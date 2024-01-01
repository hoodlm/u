`u` is a functional programming language that supports only unary operations - functions that
take a single variable and return a single variable of the same type.

This repo contains a language specification and an interpretter.

* The language specification is a work-in-progress and subject to change.
* The interpreter is incomplete and a messy pile of spaghetti, but it's getting better.

## Language Examples:

Assign the literal value '2' to the variable 'y':

```
2 y;

--> 2
```

Add 3 to the value of 'y' and assign it to the variable 'x':

```
y + + + x;

--> 5
```

An equivalent, using `{n}` syntactic sugar to repeat the increment (+) operator three times:

```
y {3} + x;

--> 5
```

ROT13 cipher:

```
"hello world" {13} + PRINTLN;

--> "uryyb jbeyq"
```

Function declaration:

```
fn rot13: {13} +;

"hello world" rot13 PRINTLN;

--> "uryyb jbeyq"
```

# Testing

`u` comes with a language integration test suite in the [spec](./spec) directory.

The language integration tests are built with [shellspec](shellspec.info) and can be invoked
with the plain `shellspec` command.

They execute directly against the `u` binary, so you need to build the interpreter (e.g.
with `cargo build --release`) first to run the tests.

# FAQ

## Is `u` a real language?

Yes.

## Can I write real software with `u`?

Yes, you can use `u` to implement any program that can be expressed as a
series of unary operations. If you find unary operators limiting, consider
that a flaw with the underlying laws of universe, not a flaw with `u`.

