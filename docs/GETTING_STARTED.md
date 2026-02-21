# Getting Started with minipg

**minipg** is a fast, Rust-native ANTLR4-compatible parser generator. This guide will help you get started quickly.

---

## Installation

### From Source

```bash
git clone https://github.com/yingkitw/minipg
cd minipg
cargo install --path minipg-cli
```

### Verify Installation

```bash
minipg --version
# minipg 0.2.0
```

---

## Quick Start: Calculator Example

### 1. Create a Grammar File

Create `Calculator.g4`:

```antlr
grammar Calculator;

// Parser rules
expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

// Lexer rules
NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

### 2. Generate Parser

Choose your target language:

**Rust:**
```bash
minipg generate Calculator.g4 -o output/ -l rust
```

**Python:**
```bash
minipg generate Calculator.g4 -o output/ -l python
```

**JavaScript:**
```bash
minipg generate Calculator.g4 -o output/ -l javascript
```

### 3. Use Generated Parser

**Rust:**
```rust
use calculator::{Lexer, Parser};

fn main() {
    let input = "2 + 3 * 4".to_string();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_all();
    
    let mut parser = Parser::new(tokens);
    match parser.parse_expr() {
        Ok(ast) => println!("Success: {:?}", ast),
        Err(e) => eprintln!("Error: {}", e.message),
    }
}
```

**Python:**
```python
from calculator import Lexer, Parser

input_text = "2 + 3 * 4"
lexer = Lexer(input_text)
tokens = lexer.tokenize_all()

parser = Parser(tokens)
try:
    ast = parser.parse_expr()
    print(f"Success: {ast}")
except ParseError as e:
    print(f"Error: {e.message}")
```

**JavaScript:**
```javascript
const { Lexer, Parser } = require('./calculator');

const input = '2 + 3 * 4';
const lexer = new Lexer(input);
const tokens = lexer.tokenizeAll();

const parser = new Parser(tokens);
try {
    const ast = parser.parseExpr();
    console.log('Success:', ast);
} catch (e) {
    console.error('Error:', e.message);
}
```

---

## Example Grammars

minipg includes 19 example grammars in the `examples/` directory:

### Beginner Level

1. **calculator.g4** - Simple calculator
2. **json.g4** - Basic JSON parser
3. **Expression.g4** - Expression evaluator

### Intermediate Level

4. **CompleteJSON.g4** - Full JSON specification
5. **Config.g4** - Configuration file parser
6. **YAML.g4** - YAML subset parser

### Advanced Level

7. **SQL.g4** - SQL query parser
8. **JavaSubset.g4** - Java language subset
9. **PythonSubset.g4** - Python language subset
10. **GraphQL.g4** - GraphQL query language
11. **CSS.g4** - CSS stylesheet parser
12. **Markdown.g4** - Markdown parser
13. **Protocol.g4** - Protocol buffer parser
14. **Query.g4** - Query language parser

### Feature Demonstrations

15. **ActionsExample.g4** - Embedded actions
16. **LexerModes.g4** - Lexer modes and channels
17. **RuleFeatures.g4** - Rule arguments, returns, locals
18. **test_charclass.g4** - Character classes
19. **test_simple_charclass.g4** - Simple character classes

---

## Common Commands

### Validate Grammar

```bash
minipg validate Calculator.g4
```

### Show Grammar Info

```bash
minipg info Calculator.g4
```

### Generate with Options

```bash
# Generate with visitor pattern
minipg generate Calculator.g4 -o output/ -l rust --visitor

# Generate with listener pattern
minipg generate Calculator.g4 -o output/ -l rust --listener

# Specify package name
minipg generate Calculator.g4 -o output/ -l python --package calculator_parser
```

---

## Grammar Syntax Basics

### Parser Rules

Parser rules start with lowercase:

```antlr
expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';
```

### Lexer Rules

Lexer rules start with uppercase:

```antlr
NUMBER: [0-9]+;
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
WS: [ \t\r\n]+ -> skip;
```

### Operators

- `|` - Alternative
- `*` - Zero or more
- `+` - One or more
- `?` - Optional
- `()` - Grouping

### Character Classes

```antlr
DIGIT: [0-9];
LETTER: [a-zA-Z];
ALPHANUM: [a-zA-Z0-9];
NOT_QUOTE: ~["];
```

### Lexer Commands

```antlr
WS: [ \t\r\n]+ -> skip;           // Skip whitespace
COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);  // Send to channel
STRING_START: '"' -> mode(STRING_MODE);       // Switch mode
```

---

## Advanced Features

### Rule Arguments and Returns

```antlr
expr[int precedence] returns [int value]:
    term { $value = $term.value; }
    ;
```

### Named Actions

```antlr
@header {
    use std::collections::HashMap;
}

@members {
    symbol_table: HashMap<String, i32>,
}
```

### Labels

```antlr
expr: left=term op=('+' | '-') right=term;
```

### List Labels

```antlr
argList: args+=expr (',' args+=expr)*;
```

### Lexer Modes

```antlr
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["\r\n];
STRING_END: '"' -> popMode;
```

---

## Troubleshooting

### Grammar Doesn't Parse

**Check:**
- Rule names (lowercase for parser, uppercase for lexer)
- Missing semicolons
- Unmatched parentheses or brackets
- Invalid character classes

### Generated Code Doesn't Compile

**Check:**
- Reserved keywords in rule names
- Type mismatches in actions
- Missing imports in @header

### Parser Doesn't Match Input

**Check:**
- Lexer rules (order matters - first match wins)
- Whitespace handling (add WS rule with -> skip)
- Character escaping in string literals

---

## Next Steps

1. **Read the language guides:**
   - [Rust Code Generation](RUST_CODE_GENERATION.md)
   - [Python Code Generation](PYTHON_CODE_GENERATION.md)
   - [JavaScript Code Generation](JAVASCRIPT_CODE_GENERATION.md)

2. **Explore examples:**
   - Browse `examples/` directory
   - Try generating parsers for different languages
   - Modify examples to learn

3. **Build something:**
   - Create a custom DSL
   - Parse configuration files
   - Build a query language

4. **Read advanced docs:**
   - [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md)
   - [Architecture](../ARCHITECTURE.md)
   - [User Guide](USER_GUIDE.md)

---

## Resources

- **Repository**: https://github.com/yingkitw/minipg
- **Documentation**: `docs/` directory
- **Examples**: `examples/` directory
- **Issues**: GitHub Issues

---

## Summary

minipg makes it easy to create parsers:

1. ✅ Write ANTLR4 grammar
2. ✅ Run `minipg generate`
3. ✅ Use generated parser
4. ✅ No runtime dependencies

Get started now with the examples in `examples/` directory!
