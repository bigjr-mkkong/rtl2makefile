## RTL2Makefile – Compile RTL Design into Makefile and Simulate with GNU Make
### Introduction

This project leverages GNU Make to detect file dependencies and modification times, and uses this behavior to compile RTL designs into Makefile rules.

To keep things simple and minimal, the input is not written in Verilog, but instead in a Lisp-like DSL tailored for this purpose.

An example of this idea is included in the example/ directory — a simple up-counter circuit that increments by 1.

More details can be found in this post: [Makesim](https://bigjr-mkkong.github.io/2025/07/22/makesim.html)
### Language Specification

This compiler does not support full Lisp, but it borrows several core ideas from it.
#### Circuit Definition

Each circuit begins with a let block, followed by a list of variable definitions:
```
(let ((reg a b c) (wire d e f)) ...)
```

Only two types are currently supported: `reg` and `wire`, and Constants are not supported yet

#### Circuit Description

After the let block, the circuit logic starts with circ, followed by one or more expressions:
```
    (circ (conn expressions) ...)
```

#### Connection Syntax
```
(conn a1 a2)
```
This connects a2 to a1, like:
```
a2 ---> a1
```

Note: both a1 and a2 can be reg or wire, but both cannot be reg, due to internal design constraints.

#### Arithmetic and Logic Operators

The following operators are supported:

Arithmetic: `+, -, *, /`

Logic: `&, |, ^ (bitwise XOR), ! (bitflip)`

Each arithmetic or logical operation produces a temporary hidden wire as its output.

Nested expressions are also supported:
```
(+ a (+ b c))
```
This adds the values on wires a, b, and c.
#### Example: Counter Circuit

Here's a full example that generates a simple counter:
```
(let ((reg a) (wire b c))
  (circ
    (conn a (+ b c))
    (conn b a)))
```
This connects the output of `(+ b c)` to `a`, and then routes a back into b on the next cycle — effectively building a 1-step incrementing counter.
