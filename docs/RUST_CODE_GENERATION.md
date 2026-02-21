# Rust Code Generation Guide

**minipg** generates standalone Rust parsers from ANTLR4 grammars with no runtime dependencies.

---

## Overview

The Rust code generator produces:
- **Lexer** - Tokenizes input with optimized DFA
- **Parser** - Parses tokens into AST
- **Token Types** - Enum for all token kinds
- **Error Types** - ParseError with position tracking
- **AST Nodes** - Structured representation of parsed input

---

## Generated Code Structure

```rust
// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    Plus,
    // ... other tokens
}

// Token with metadata
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub position: usize,
}

// Parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

// Lexer
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self { ... }
    pub fn next_token(&mut self) -> Result<Token, ParseError> { ... }
    pub fn tokenize_all(&mut self) -> Vec<Token> { ... }
}

// Parser
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { ... }
    pub fn parse_rule(&mut self) -> Result<AstNode, ParseError> { ... }
}
```

---

## Features

### 1. Optimized DFA Generation

The Rust generator creates inline DFA for efficient tokenization:

```rust
// Generated DFA match
match current_char {
    'a'..='z' | 'A'..='Z' => {
        // Identifier
        while matches!(peek_char(), 'a'..='z' | 'A'..='Z' | '0'..='9') {
            advance();
        }
        TokenKind::Identifier
    }
    '0'..='9' => {
        // Number
        while matches!(peek_char(), '0'..='9') {
            advance();
        }
        TokenKind::Number
    }
    _ => return Err(ParseError::new("Unexpected character", position))
}
```

### 2. Lookup Tables

Character class matching uses const lookup tables:

```rust
const IDENTIFIER_START: [bool; 256] = [
    false, false, ..., true, // 'a'
    true, ..., true,         // 'z'
];

#[inline(always)]
fn is_identifier_start(c: char) -> bool {
    (c as u32) < 256 && IDENTIFIER_START[c as usize]
}
```

### 3. Error Recovery

Generated parsers include error recovery:

```rust
pub fn tokenize_all(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
        match self.next_token() {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Err(e) => {
                // Skip invalid character and continue
                eprintln!("Lexer error: {}", e.message);
                self.position += 1;
            }
        }
    }
    tokens
}
```

### 4. Named Actions

Support for `@header` and `@members`:

```antlr
grammar Calculator;

@header {
    use std::collections::HashMap;
}

@members {
    symbol_table: HashMap<String, i32>,
}

expr: term (('+' | '-') term)*;
```

Generates:

```rust
// @header inserted at top
use std::collections::HashMap;

pub struct CalculatorParser {
    tokens: Vec<Token>,
    position: usize,
    // @members inserted here
    symbol_table: HashMap<String, i32>,
}
```

### 5. Rule Arguments and Returns

```antlr
expr[int precedence] returns [int value]:
    term { $value = $term.value; }
    ;
```

Generates:

```rust
pub fn parse_expr(&mut self, precedence: i32) -> Result<i32, ParseError> {
    let term = self.parse_term()?;
    Ok(term)
}
```

### 6. Lexer Modes

```antlr
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["\r\n];
STRING_END: '"' -> popMode;
```

Generates mode stack management:

```rust
struct Lexer {
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }
    
    fn pop_mode(&mut self) {
        self.mode_stack.pop();
    }
}
```

---

## Usage Example

### 1. Create Grammar

```antlr
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

### 2. Generate Parser

```bash
minipg generate Calculator.g4 -o src/ -l rust
```

### 3. Use Generated Parser

```rust
use calculator::{Lexer, Parser};

fn main() {
    let input = "2 + 3 * 4".to_string();
    
    // Tokenize
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_all();
    
    // Parse
    let mut parser = Parser::new(tokens);
    match parser.parse_expr() {
        Ok(ast) => println!("Parsed: {:?}", ast),
        Err(e) => eprintln!("Parse error: {}", e.message),
    }
}
```

---

## Performance

### Optimizations

1. **Inline attributes** - Critical functions marked `#[inline]`
2. **DFA generation** - Compile-time state machine
3. **Lookup tables** - Const arrays for character classes
4. **Zero-copy** - String slices where possible
5. **Vec pre-allocation** - Capacity hints for collections

### Benchmarks

Typical performance (1000-line grammar):
- Code generation: **<1ms**
- Generated parser: **~10µs per token**
- Memory usage: **<100 KB**

---

## Best Practices

### 1. Use Inline DFA

Enable DFA optimization for better performance:

```rust
// Automatically generated with minipg
// No configuration needed
```

### 2. Handle Errors Gracefully

```rust
match parser.parse_expr() {
    Ok(ast) => process(ast),
    Err(e) => {
        eprintln!("Error at position {}: {}", e.position, e.message);
        // Recover or exit
    }
}
```

### 3. Reuse Parsers

```rust
// Create once, reuse multiple times
let mut parser = Parser::new(tokens);
for input in inputs {
    parser.reset(input);
    parser.parse_expr()?;
}
```

### 4. Use Type Aliases

```rust
type Result<T> = std::result::Result<T, ParseError>;

pub fn parse_expr(&mut self) -> Result<AstNode> {
    // ...
}
```

---

## Advanced Features

### Custom AST Nodes

Define your own AST types:

```rust
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    BinOp { op: char, left: Box<Expr>, right: Box<Expr> },
}
```

### Visitor Pattern

Implement visitors for AST traversal:

```rust
trait Visitor {
    fn visit_expr(&mut self, expr: &Expr);
    fn visit_term(&mut self, term: &Term);
}
```

### Error Recovery

Implement panic mode recovery:

```rust
fn synchronize(&mut self) {
    while self.position < self.tokens.len() {
        if matches!(self.current_token().kind, TokenKind::Semicolon) {
            self.advance();
            return;
        }
        self.advance();
    }
}
```

---

## Troubleshooting

### Issue: Compilation Errors

**Problem**: Generated code doesn't compile

**Solution**: Check grammar for:
- Invalid Rust keywords in rule names
- Type mismatches in actions
- Missing imports in @header

### Issue: Slow Parsing

**Problem**: Parser is slower than expected

**Solution**:
- Enable release mode: `cargo build --release`
- Profile with `cargo flamegraph`
- Check for excessive backtracking

### Issue: Memory Usage

**Problem**: High memory consumption

**Solution**:
- Use string slices instead of owned strings
- Implement streaming parsing
- Clear AST nodes after processing

---

## Complete Example

```antlr
grammar JSON;

@header {
    use std::collections::HashMap;
}

json: value;

value
    : object
    | array
    | STRING
    | NUMBER
    | 'true'
    | 'false'
    | 'null'
    ;

object: '{' pair (',' pair)* '}' | '{' '}';
pair: STRING ':' value;

array: '[' value (',' value)* ']' | '[' ']';

STRING: '"' (~["\r\n])* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
WS: [ \t\r\n]+ -> skip;
```

Generated usage:

```rust
use json::{Lexer, Parser};

fn parse_json(input: &str) -> Result<JsonValue, ParseError> {
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize_all();
    let mut parser = Parser::new(tokens);
    parser.parse_json()
}

fn main() {
    let json = r#"{"name": "minipg", "version": "0.2.0"}"#;
    match parse_json(json) {
        Ok(value) => println!("Parsed: {:?}", value),
        Err(e) => eprintln!("Error: {}", e.message),
    }
}
```

---

## Summary

minipg's Rust code generator produces:
- ✅ **Fast** - Optimized DFA and lookup tables
- ✅ **Standalone** - No runtime dependencies
- ✅ **Safe** - Rust's type system and error handling
- ✅ **Idiomatic** - Follows Rust conventions
- ✅ **Complete** - Full ANTLR4 feature support

For more information, see:
- [User Guide](USER_GUIDE.md)
- [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md)
- [Examples](../examples/)
