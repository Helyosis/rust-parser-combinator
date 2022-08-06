# Rust-parser-combinator

[![Cargo Build & Test](https://github.com/Helyosis/rust-parser-combinator/actions/workflows/rust.yml/badge.svg)](https://github.com/Helyosis/rust-parser-combinator/actions/workflows/rust.yml)

This is a toy project that I made to learn Rust. The code is not perfect, nor is it efficient but it's mostly readable and easy to understand.
This implement a type of parsers called Parser Combinator, it is an alternative approach to parsing that combines small, simple parsers to form more complex parsers. Compared to other knowns (to me) approaches, it does not require the lexing step of parsing, doing both the lexing and the parsing in one go.
There is also the added benefit of forming a tree structure on-the-go, enabling developpers to form the AST naturally, just by following the specifications.

## Example
You can see an example project in the test/ folder at the root. Because it is also used as integration testing, there are test cases that you can refer to.

## Documentation
There is currently no documentation but the functions (and their tests) should be (hopefully) self-explanatory.