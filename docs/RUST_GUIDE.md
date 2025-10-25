# Rust Code Generation Guide

Complete guide to generating and using Rust parsers with minipg.

## Overview

minipg generates standalone, zero-dependency Rust parsers from ANTLR4-compatible grammars. The generated code is:

- **Standalone**: No runtime dependencies, pure Rust
- **Idiomatic**: Follows Rust conventions and best practices
- **Type-safe**: Full type safety with Result and Option
- **Performant**: Optimized with inline DFA and lookup tables
- **Well-documented**: Includes inline documentation

## Quick Start

### 1. Create a Grammar File

```antlr
// calculator.g4
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\n\r]+ -> skip;
```

### 2. Generate Rust Parser

```bash
minipg generate calculator.g4 -o src -l rust
```

This creates `src/calculator_parser.rs`.

### 3. Use in Your Project

```rust
use calculator_parser::{CalculatorLexer, CalculatorParser};

fn main() {
    let input = "1 + 2 * 3";
    let mut lexer = CalculatorLexer::new(input);
    let tokens = lexer.tokenize_all().unwrap();
    
    let mut parser = CalculatorParser::new(tokens);
    match parser.parse_expr() {
        Ok(expr) => println!("Parsed: {:?}", expr),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```

## Generated Code Structure

### Lexer

The generated lexer tokenizes input into tokens:

```rust
pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self { ... }
    
    pub fn next_token(&mut self) -> Result<Token, ParseError> { ... }
    
    pub fn tokenize_all(&mut self) -> Result<Vec<Token>, Vec<ParseError>> { ... }
}
```

### Token Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    NUMBER,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAREN,
    RPAREN,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub position: usize,
}
```

### Parser

The generated parser builds an AST from tokens:

```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { ... }
    
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> { ... }
    
    pub fn parse_term(&mut self) -> Result<Term, ParseError> { ... }
    
    pub fn parse_factor(&mut self) -> Result<Factor, ParseError> { ... }
}
```

### Error Handling

```rust
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
    pub expected: Vec<String>,
    pub found: Option<String>,
}

impl std::fmt::Display for ParseError { ... }
impl std::error::Error for ParseError { ... }
```

## Features

### 1. Type-Safe Parsing

All parsing operations return `Result<T, ParseError>`:

```rust
match parser.parse_expr() {
    Ok(expr) => { /* handle success */ },
    Err(e) => { /* handle error */ },
}
```

### 2. Error Recovery

The lexer can collect all errors instead of stopping at the first:

```rust
let mut lexer = CalculatorLexer::new(input);
match lexer.tokenize_all() {
    Ok(tokens) => { /* all tokens parsed */ },
    Err(errors) => { /* multiple errors */ },
}
```

### 3. Position Tracking

Tokens and errors include position information:

```rust
let token = lexer.next_token()?;
println!("Token at position {}: {:?}", token.position, token.kind);

let error = ParseError::new("Expected number".to_string(), 42);
println!("Error at position {}: {}", error.position, error.message);
```

### 4. Parameterized Rules

Rules with arguments and return values:

```antlr
expr[bool allowNegative]: term (('+' | '-') term)*;
factor returns [i32 value]: NUMBER;
```

Generated code:

```rust
pub fn parse_expr(&mut self, allow_negative: bool) -> Result<Expr, ParseError> { ... }

pub fn parse_factor(&mut self) -> Result<(Factor, i32), ParseError> { ... }
```

### 5. List Labels

Collect multiple values with `+=`:

```antlr
items: item+=ID (',' item+=ID)*;
```

Generated code:

```rust
pub struct Items {
    pub items: Vec<String>,
}
```

### 6. Named Actions

Insert custom code into generated parser:

```antlr
@header {
    use std::collections::HashMap;
}

@members {
    fn custom_method(&self) { ... }
}
```

## Advanced Usage

### Custom Token Handling

```rust
let mut lexer = CalculatorLexer::new(input);

loop {
    match lexer.next_token() {
        Ok(token) => {
            if token.kind == TokenKind::EOF {
                break;
            }
            println!("Token: {:?}", token);
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            // Continue to next token
        }
    }
}
```

### Custom Parser Handling

```rust
let mut parser = CalculatorParser::new(tokens);

// Parse with custom error handling
match parser.parse_expr() {
    Ok(expr) => {
        // Process expression
    }
    Err(e) => {
        eprintln!("Parse error at position {}: {}", e.position, e.message);
        eprintln!("Expected: {:?}", e.expected);
        eprintln!("Found: {:?}", e.found);
    }
}
```

### Building an AST

The parser generates AST nodes for each rule:

```rust
pub enum Expr {
    Term(Term),
    Add(Box<Expr>, Box<Term>),
    Sub(Box<Expr>, Box<Term>),
}

pub enum Term {
    Factor(Factor),
    Mul(Box<Term>, Box<Factor>),
    Div(Box<Term>, Box<Factor>),
}

pub enum Factor {
    Number(i32),
    Expr(Box<Expr>),
}
```

## Performance

### Optimizations

The generated code includes several optimizations:

1. **Inline DFA**: State machines compiled to match statements
2. **Lookup Tables**: Character class matching with 256-byte tables
3. **Inline Attributes**: Hot functions marked with `#[inline]`
4. **Zero-Copy**: No unnecessary allocations

### Benchmarking

```rust
use std::time::Instant;

let start = Instant::now();
for _ in 0..1000 {
    let mut lexer = CalculatorLexer::new(input);
    let tokens = lexer.tokenize_all()?;
    let mut parser = CalculatorParser::new(tokens);
    parser.parse_expr()?;
}
let elapsed = start.elapsed();
println!("Time per parse: {:?}", elapsed / 1000);
```

## Best Practices

### 1. Error Handling

Always handle errors properly:

```rust
// Good
match parser.parse_expr() {
    Ok(expr) => process(expr),
    Err(e) => return Err(e),
}

// Avoid
let expr = parser.parse_expr().unwrap(); // Panics on error
```

### 2. Reuse Lexer/Parser

Create once, reuse for multiple parses:

```rust
let mut lexer = CalculatorLexer::new(input);
let tokens = lexer.tokenize_all()?;

let mut parser = CalculatorParser::new(tokens);
let expr = parser.parse_expr()?;
```

### 3. Document Custom Code

If using named actions, document the custom code:

```antlr
@members {
    /// Validates the expression tree
    fn validate(&self, expr: &Expr) -> bool {
        // Implementation
    }
}
```

### 4. Test Generated Code

Write tests for your generated parsers:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expr() {
        let mut lexer = CalculatorLexer::new("1 + 2");
        let tokens = lexer.tokenize_all().unwrap();
        let mut parser = CalculatorParser::new(tokens);
        assert!(parser.parse_expr().is_ok());
    }
}
```

## Troubleshooting

### Issue: "No such file or directory"

**Cause**: Output directory doesn't exist

**Solution**: Create the directory first
```bash
mkdir -p src
minipg generate grammar.g4 -o src -l rust
```

### Issue: "Undefined rule"

**Cause**: Grammar references undefined rules or tokens

**Solution**: Check grammar for typos and ensure all rules are defined
```bash
minipg validate grammar.g4
```

### Issue: "Ambiguous alternatives"

**Cause**: Grammar has ambiguous parsing paths

**Solution**: Restructure grammar to remove ambiguity or add semantic predicates

### Issue: Generated code doesn't compile

**Cause**: Grammar uses unsupported features

**Solution**: Check KNOWN_LIMITATIONS.md for unsupported features

## Examples

See `examples/` directory for complete examples:

- `calculator.g4` - Simple arithmetic expressions
- `json.g4` - JSON parser
- `CompleteJSON.g4` - RFC 8259 compliant JSON
- `SQL.g4` - SQL language subset

## See Also

- [User Guide](USER_GUIDE.md) - General minipg usage
- [Grammar Syntax Reference](GRAMMAR_SYNTAX.md) - ANTLR4 syntax
- [API Documentation](API.md) - Detailed API reference
- [KNOWN_LIMITATIONS.md](../KNOWN_LIMITATIONS.md) - Current limitations

---

**Last Updated**: October 25, 2025  
**Version**: v0.1.0-alpha.3  
**Status**: Production-ready
