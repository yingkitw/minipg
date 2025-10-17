# Example 1: Basic Calculator

## Level: Beginner

A simple calculator grammar demonstrating basic arithmetic operations.

## Grammar File

**File**: `examples/calculator.g4`

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

## Features Demonstrated

- ✅ Basic parser rules
- ✅ Basic lexer rules
- ✅ Operator precedence
- ✅ Parentheses grouping
- ✅ Whitespace handling

## Usage

### Generate Rust Code
```bash
minipg generate --input examples/calculator.g4 --output target/calculator.rs --language rust
```

### Generate Python Code
```bash
minipg generate --input examples/calculator.g4 --output target/calculator.py --language python
```

### Generate JavaScript Code
```bash
minipg generate --input examples/calculator.g4 --output target/calculator.js --language javascript
```

### Generate TypeScript Code
```bash
minipg generate --input examples/calculator.g4 --output target/calculator.ts --language typescript
```

## Generated Code Structure

### Rust
```rust
pub struct CalculatorLexer { ... }
pub struct CalculatorParser { ... }
pub struct ParseError { ... }
pub enum TokenKind { NUMBER, EOF }
```

### Python
```python
class CalculatorLexer: ...
class CalculatorParser: ...
class ParseError(Exception): ...
class TokenKind(Enum): ...
```

### JavaScript
```javascript
class CalculatorLexer { ... }
class CalculatorParser { ... }
class ParseError extends Error { ... }
const TokenKind = { ... };
```

### TypeScript
```typescript
export class CalculatorLexer { ... }
export class CalculatorParser { ... }
export class ParseError extends Error { ... }
export enum TokenKind { ... }
```

## Testing

### Test Input
```
2 + 3 * 4
(2 + 3) * 4
10 / 2 - 3
```

### Expected Tokens
```
NUMBER(2), PLUS, NUMBER(3), MULT, NUMBER(4), EOF
LPAREN, NUMBER(2), PLUS, NUMBER(3), RPAREN, MULT, NUMBER(4), EOF
NUMBER(10), DIV, NUMBER(2), MINUS, NUMBER(3), EOF
```

## Key Concepts

1. **Parser Rules** - Define the structure of expressions
2. **Lexer Rules** - Define tokens (NUMBER, operators)
3. **Precedence** - Order of operations
4. **Whitespace** - Automatically skipped

## Next Steps

- Try modifying the grammar to add more operators
- Add support for floating-point numbers
- Implement evaluation in actions
- Add error recovery

## Related Examples

- **Next**: [02-JSON-PARSER.md](02-JSON-PARSER.md) - More complex grammar
- **Advanced**: [05-SQL-PARSER.md](05-SQL-PARSER.md) - Real-world grammar
