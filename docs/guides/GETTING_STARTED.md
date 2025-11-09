# Getting Started with minipg

## Introduction

minipg is a fast, modern parser generator that supports multiple target languages (Rust, Python, JavaScript, TypeScript). It's compatible with ANTLR4 grammar syntax and generates production-ready parsers with comprehensive error recovery.

## Installation

### From crates.io
```bash
cargo install minipg-cli
```

### From source
```bash
git clone https://github.com/yourusername/minipg
cd minipg
cargo install --path crates/minipg-cli
```

### Verify installation
```bash
minipg --version
```

## Your First Grammar

### Step 1: Create a Grammar File

Create `calculator.g4`:
```antlr4
grammar Calculator;

// Parser Rules
expr
    : expr ('*'|'/') expr
    | expr ('+'|'-') expr
    | NUMBER
    | '(' expr ')'
    ;

// Lexer Rules
NUMBER : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
```

### Step 2: Generate Code

#### Rust
```bash
minipg generate --input calculator.g4 --output calculator.rs --language rust
```

#### Python
```bash
minipg generate --input calculator.g4 --output calculator.py --language python
```

#### JavaScript
```bash
minipg generate --input calculator.g4 --output calculator.js --language javascript
```

#### TypeScript
```bash
minipg generate --input calculator.g4 --output calculator.ts --language typescript
```

### Step 3: Use the Generated Parser

#### Rust
```rust
use calculator::{CalculatorLexer, CalculatorParser};

fn main() {
    let input = "2 + 3 * 4";
    let mut lexer = CalculatorLexer::new(input);
    
    // Tokenize with error recovery
    let (tokens, errors) = lexer.tokenize_all();
    
    if !errors.is_empty() {
        for error in errors {
            eprintln!("Error: {}", error);
        }
    }
    
    // Parse tokens
    let parser = CalculatorParser::new(tokens);
    // ... use parser
}
```

#### Python
```python
from calculator import CalculatorLexer, CalculatorParser

input_text = "2 + 3 * 4"
lexer = CalculatorLexer(input_text)

# Tokenize with error recovery
tokens, errors = lexer.tokenize_all()

if errors:
    for error in errors:
        print(f"Error: {error}")

# Parse tokens
parser = CalculatorParser(tokens)
# ... use parser
```

#### JavaScript
```javascript
const { CalculatorLexer, CalculatorParser } = require('./calculator');

const input = "2 + 3 * 4";
const lexer = new CalculatorLexer(input);

// Tokenize with error recovery
const { tokens, errors } = lexer.tokenizeAll();

if (errors.length > 0) {
    errors.forEach(error => console.error(`Error: ${error}`));
}

// Parse tokens
const parser = new CalculatorParser(tokens);
// ... use parser
```

#### TypeScript
```typescript
import { CalculatorLexer, CalculatorParser } from './calculator';

const input = "2 + 3 * 4";
const lexer = new CalculatorLexer(input);

// Tokenize with error recovery
const { tokens, errors } = lexer.tokenizeAll();

if (errors.length > 0) {
    errors.forEach(error => console.error(`Error: ${error}`));
}

// Parse tokens
const parser = new CalculatorParser(tokens);
// ... use parser
```

## Key Features

### Error Recovery
All generated parsers include automatic error recovery:
- Skip invalid characters
- Continue parsing after errors
- Collect all errors for reporting

### Position Tracking
Every token and error includes position information:
```rust
token.position  // Character offset in input
error.position  // Where the error occurred
```

### Type Safety
- **Rust**: Full type safety with Result types
- **TypeScript**: Complete type annotations
- **Python**: Type hints (Python 3.10+)
- **JavaScript**: JSDoc comments

## Common Patterns

### Handling Errors
```rust
let (tokens, errors) = lexer.tokenize_all();

if !errors.is_empty() {
    // Report errors but continue
    for error in &errors {
        eprintln!("Line {}: {}", 
            calculate_line(error.position), 
            error.message);
    }
}

// Use tokens even if there were errors
process_tokens(tokens);
```

### Custom Error Messages
```rust
match lexer.next_token() {
    Ok(token) => process(token),
    Err(error) => {
        eprintln!("Expected: {}", error.expected.join(", "));
        eprintln!("Found: {}", error.found.unwrap_or("EOF"));
    }
}
```

## Next Steps

1. **Learn More**
   - [Grammar Syntax](../GRAMMAR_SYNTAX.md)
   - [Examples](../examples/)
   - [API Reference](../API.md)

2. **Try Examples**
   - [Calculator](../examples/01-BASIC-CALCULATOR.md)
   - [JSON Parser](../examples/02-JSON-PARSER.md)
   - [SQL Parser](../examples/05-SQL-PARSER.md)

3. **Advanced Features**
   - [ANTLR4 Compatibility](../ANTLR4_COMPATIBILITY.md) - Full ANTLR4 feature support
   - [Grammar Syntax Reference](../GRAMMAR_SYNTAX.md) - Complete syntax guide
   - [Examples Guide](../EXAMPLES_GUIDE.md) - Comprehensive examples

## Troubleshooting

### Grammar doesn't parse
- Check ANTLR4 syntax compatibility
- Verify all rules are defined
- Look for circular dependencies

### Generated code doesn't compile
- Check target language version
- Verify all dependencies are installed
- Review error messages carefully

### Performance issues
- Use DFA generation (Rust)
- Enable optimizations
- Profile your grammar

## Getting Help

- **Documentation**: [docs/INDEX.md](../INDEX.md)
- **Examples**: [examples/](../../examples/)
- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions

---

**Next**: [Migration Guide](MIGRATION_FROM_ANTLR4.md) | [Examples](../examples/)
