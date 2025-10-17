# minipg User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Grammar Syntax](#grammar-syntax)
5. [Command Line Interface](#command-line-interface)
6. [Code Generation](#code-generation)
7. [Examples](#examples)
8. [Troubleshooting](#troubleshooting)

## Introduction

minipg is a modern parser generator written in Rust, inspired by ANTLR4. It generates efficient parsers from grammar specifications, supporting multiple target languages (currently Rust, with more planned).

### Key Features

- **Modular Architecture**: Clean separation of concerns
- **Trait-Based Design**: Extensible and test-friendly
- **Rich Diagnostics**: Helpful error messages with location information
- **Visitor/Listener Patterns**: Multiple ways to traverse parse trees
- **Snapshot Testing**: Comprehensive test coverage

## Installation

### From Source

```bash
git clone https://github.com/yourusername/minipg
cd minipg
cargo install --path crates/minipg-cli
```

### Verify Installation

```bash
minipg --version
```

## Quick Start

### 1. Create a Grammar File

Create a file named `calculator.g4`:

```
grammar parser Calculator;

expr: term;
term: factor;
factor: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
```

### 2. Generate Parser

```bash
minipg generate calculator.g4 -o output/
```

### 3. Use Generated Code

The generated parser will be in `output/calculator_parser.rs`.

## Grammar Syntax

### Grammar Declaration

```
grammar <type> <name>;
```

Types:
- `parser` - Parser grammar
- `lexer` - Lexer grammar
- `combined` - Combined parser and lexer (default if type omitted)

### Rules

#### Parser Rules

Parser rules start with lowercase letters:

```
expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';
```

#### Lexer Rules

Lexer rules start with uppercase letters:

```
NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
WS: SPACE+;
SPACE: ' ' | '\t' | '\r' | '\n';
```

#### Fragment Rules

Fragment rules are reusable components (not tokens themselves):

```
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
```

### Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `\|` | Alternative | `'a' \| 'b'` |
| `?` | Optional | `expr?` |
| `*` | Zero or more | `expr*` |
| `+` | One or more | `expr+` |
| `()` | Grouping | `('a' \| 'b')+` |
| `~` | Negation | `~'a'` |
| `.` | Any character | `.` |

### String Literals

```
'+' | '-' | '*' | '/'
"hello" | "world"
```

### Comments

```
// Line comment

/* Block
   comment */
```

### Options

```
grammar parser MyGrammar;

options {
    language = rust;
}
```

### Imports

```
grammar parser MyGrammar;

import CommonRules;
```

## Command Line Interface

### Generate Command

Generate a parser from a grammar file:

```bash
minipg generate <grammar-file> [OPTIONS]
```

Options:
- `-o, --output <DIR>` - Output directory (default: current directory)
- `-l, --target-language <LANG>` - Target language (default: rust)
- `-p, --package <NAME>` - Package name for generated code
- `--visitor` - Generate visitor pattern
- `--listener` - Generate listener pattern (default: true)

Example:
```bash
minipg generate grammar.g4 \
  --output ./generated \
  --target-language rust \
  --visitor \
  --package my_parser
```

### Validate Command

Validate a grammar file without generating code:

```bash
minipg validate <grammar-file>
```

This checks for:
- Syntax errors
- Undefined rules
- Duplicate rules
- Left recursion
- Unreachable rules

### Info Command

Display information about a grammar:

```bash
minipg info <grammar-file>
```

Shows:
- Grammar name and type
- Number of rules
- Parser vs lexer rules
- Options and imports

## Code Generation

### Generated Files

For a grammar named `Calculator`, minipg generates:

- `calculator_parser.rs` - Complete parser implementation

### Generated Components

1. **Token Types**
   ```rust
   pub enum TokenKind {
       NUMBER,
       PLUS,
       MINUS,
       // ...
       Eof,
   }
   ```

2. **AST Types**
   ```rust
   pub enum AstNode {
       Expr(Box<Expr>),
       Term(Box<Term>),
       // ...
   }
   ```

3. **Lexer**
   ```rust
   pub struct CalculatorLexer {
       // ...
   }
   ```

4. **Parser**
   ```rust
   pub struct CalculatorParser {
       // ...
   }
   ```

5. **Visitor** (if `--visitor` flag used)
   ```rust
   pub trait Visitor<T> {
       fn visit_expr(&mut self, node: &Expr) -> T;
       fn visit_term(&mut self, node: &Term) -> T;
       // ...
   }
   ```

6. **Listener** (if `--listener` flag used)
   ```rust
   pub trait Listener {
       fn enter_expr(&mut self, node: &Expr) {}
       fn exit_expr(&mut self, node: &Expr) {}
       // ...
   }
   ```

### Using Generated Code

```rust
use calculator_parser::{CalculatorLexer, CalculatorParser};

fn main() {
    let input = "123 + 456";
    let mut lexer = CalculatorLexer::new(input);
    let tokens = lexer.tokenize();
    
    let mut parser = CalculatorParser::new(tokens);
    let ast = parser.parse_expr().unwrap();
    
    println!("{:?}", ast);
}
```

## Examples

### Simple Calculator

See `examples/calculator.g4` for a complete calculator grammar.

### JSON Parser

```
grammar parser JSON;

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

STRING: '"' CHAR* '"';
NUMBER: DIGIT+ ('.' DIGIT+)?;

fragment CHAR: ~["\\\r\n];
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
```

## Troubleshooting

### Common Errors

#### "undefined rule: X"

The rule `X` is referenced but not defined. Add the rule definition:

```
X: /* definition */;
```

#### "duplicate rule definition: X"

A rule is defined multiple times. Remove duplicate definitions.

#### "direct left recursion in rule 'X'"

The rule has direct left recursion. Rewrite to eliminate it:

```
// Before (left-recursive)
expr: expr '+' term | term;

// After (right-recursive)
expr: term ('+' term)*;
```

#### "unreachable rule: X"

The rule is never referenced. Either use it or remove it.

### Debug Mode

Enable verbose logging:

```bash
RUST_LOG=debug minipg generate grammar.g4
```

### Getting Help

```bash
minipg --help
minipg generate --help
```

## Next Steps

- Read the [Grammar Syntax Reference](GRAMMAR_SYNTAX.md)
- Explore [API Documentation](API.md)
- Check out more [Examples](../examples/)
- Join our community discussions

## License

MIT OR Apache-2.0
