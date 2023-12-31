`u` is a functional programming language that only supports unary operations - functions that
take a single variable and return a single variable of the same type.

A `u` statement is read left-to-right, composed of the following:

## Examples:

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
y {3}+ x;

--> 5
```

ROT13 cipher:

```
"hello world" {13}+ PRINTLN;

--> "uryyb jbeyq"
```

Function declaration:

```
fn rot13: {13}+;

"hello world" rot13 PRINTLN;

--> "uryyb jbeyq"
```

## Supported datatypes

`u` currently supports integers and chars, as well as integer arrays and char arrays (or strings).

Type coercion and conversion isn't supported.

## Other semantics

* Variable and function assignments are immutable and assign-once

