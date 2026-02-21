# Python Code Generation Guide

**minipg** generates standalone Python parsers from ANTLR4 grammars with type hints and modern Python 3.10+ features.

---

## Overview

The Python code generator produces:
- **Lexer** - Tokenizes input with pattern matching
- **Parser** - Parses tokens into AST
- **Token Types** - Enum for all token kinds
- **Error Types** - ParseError exception class
- **Type Hints** - Full type annotations (Python 3.10+)

---

## Generated Code Structure

```python
from enum import Enum
from dataclasses import dataclass
from typing import List, Optional

# Token types
class TokenKind(Enum):
    IDENTIFIER = "IDENTIFIER"
    NUMBER = "NUMBER"
    PLUS = "PLUS"
    # ... other tokens

# Token with metadata
@dataclass
class Token:
    kind: TokenKind
    value: str
    position: int

# Parse error
class ParseError(Exception):
    def __init__(self, message: str, position: int):
        self.message = message
        self.position = position
        super().__init__(f"{message} at position {position}")

# Lexer
class Lexer:
    def __init__(self, input_text: str):
        self.input = input_text
        self.position = 0
    
    def next_token(self) -> Token:
        """Get next token from input."""
        ...
    
    def tokenize_all(self) -> List[Token]:
        """Tokenize entire input."""
        ...

# Parser
class Parser:
    def __init__(self, tokens: List[Token]):
        self.tokens = tokens
        self.position = 0
    
    def parse_rule(self) -> 'AstNode':
        """Parse a rule."""
        ...
```

---

## Features

### 1. Type Hints (Python 3.10+)

Full type annotations for better IDE support:

```python
def parse_expr(self, precedence: int = 0) -> Optional[Expr]:
    """Parse expression with precedence."""
    term: Term = self.parse_term()
    if term is None:
        return None
    return Expr(term)
```

### 2. Dataclasses

Modern Python dataclasses for AST nodes:

```python
@dataclass
class BinaryOp:
    operator: str
    left: 'Expr'
    right: 'Expr'

@dataclass
class Number:
    value: int
```

### 3. Error Recovery

Exception-based error handling:

```python
def tokenize_all(self) -> List[Token]:
    """Tokenize entire input with error recovery."""
    tokens: List[Token] = []
    while self.position < len(self.input):
        try:
            token = self.next_token()
            tokens.append(token)
            if token.kind == TokenKind.EOF:
                break
        except ParseError as e:
            print(f"Lexer error: {e.message}", file=sys.stderr)
            self.position += 1  # Skip invalid character
    return tokens
```

### 4. Named Actions

Support for `@header` and `@members`:

```antlr
grammar Calculator;

@header {
    from typing import Dict
}

@members {
    symbol_table: Dict[str, int]
}

expr: term (('+' | '-') term)*;
```

Generates:

```python
# @header inserted at top
from typing import Dict

class CalculatorParser:
    def __init__(self, tokens: List[Token]):
        self.tokens = tokens
        self.position = 0
        # @members inserted here
        self.symbol_table: Dict[str, int] = {}
```

### 5. Pattern Matching (Python 3.10+)

Modern pattern matching for token types:

```python
def next_token(self) -> Token:
    match self.current_char():
        case c if c.isalpha():
            return self.read_identifier()
        case c if c.isdigit():
            return self.read_number()
        case '+':
            return Token(TokenKind.PLUS, '+', self.position)
        case _:
            raise ParseError(f"Unexpected character: {c}", self.position)
```

### 6. Lexer Modes

```antlr
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["\r\n];
STRING_END: '"' -> popMode;
```

Generates mode stack management:

```python
class Lexer:
    def __init__(self, input_text: str):
        self.input = input_text
        self.position = 0
        self.mode_stack: List[LexerMode] = [LexerMode.DEFAULT]
    
    def push_mode(self, mode: LexerMode) -> None:
        self.mode_stack.append(mode)
    
    def pop_mode(self) -> None:
        if len(self.mode_stack) > 1:
            self.mode_stack.pop()
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
minipg generate Calculator.g4 -o src/ -l python
```

### 3. Use Generated Parser

```python
from calculator import Lexer, Parser

def main():
    input_text = "2 + 3 * 4"
    
    # Tokenize
    lexer = Lexer(input_text)
    tokens = lexer.tokenize_all()
    
    # Parse
    parser = Parser(tokens)
    try:
        ast = parser.parse_expr()
        print(f"Parsed: {ast}")
    except ParseError as e:
        print(f"Parse error: {e.message}")

if __name__ == "__main__":
    main()
```

---

## Best Practices

### 1. Use Type Hints

```python
from typing import List, Optional, Union

def parse_expr(self) -> Optional[Expr]:
    """Parse expression with type safety."""
    if not self.has_tokens():
        return None
    return self.parse_term()
```

### 2. Handle Errors Gracefully

```python
try:
    ast = parser.parse_expr()
    process(ast)
except ParseError as e:
    print(f"Error at position {e.position}: {e.message}", file=sys.stderr)
    sys.exit(1)
```

### 3. Use Dataclasses for AST

```python
from dataclasses import dataclass

@dataclass
class Expr:
    left: 'Term'
    op: Optional[str] = None
    right: Optional['Term'] = None
```

### 4. Follow PEP 8

Generated code follows PEP 8:
- Snake_case for functions and variables
- PascalCase for classes
- 4-space indentation
- Max line length: 88 characters (Black style)

---

## Advanced Features

### Custom AST Nodes

```python
from dataclasses import dataclass
from typing import Union

@dataclass
class Number:
    value: int

@dataclass
class BinOp:
    op: str
    left: 'Expr'
    right: 'Expr'

Expr = Union[Number, BinOp]
```

### Visitor Pattern

```python
from abc import ABC, abstractmethod

class Visitor(ABC):
    @abstractmethod
    def visit_expr(self, expr: Expr) -> None:
        pass
    
    @abstractmethod
    def visit_term(self, term: Term) -> None:
        pass

class PrintVisitor(Visitor):
    def visit_expr(self, expr: Expr) -> None:
        print(f"Expression: {expr}")
```

### Context Managers

```python
from contextlib import contextmanager

@contextmanager
def parser_context(input_text: str):
    lexer = Lexer(input_text)
    tokens = lexer.tokenize_all()
    parser = Parser(tokens)
    try:
        yield parser
    finally:
        parser.cleanup()

# Usage
with parser_context("2 + 3") as parser:
    result = parser.parse_expr()
```

---

## Performance Tips

### 1. Use __slots__

```python
class Token:
    __slots__ = ('kind', 'value', 'position')
    
    def __init__(self, kind: TokenKind, value: str, position: int):
        self.kind = kind
        self.value = value
        self.position = position
```

### 2. Pre-allocate Lists

```python
def tokenize_all(self) -> List[Token]:
    tokens: List[Token] = []
    tokens.reserve(1000)  # Estimate
    # ...
```

### 3. Use String Builders

```python
from io import StringIO

def read_string(self) -> str:
    buffer = StringIO()
    while self.current_char() != '"':
        buffer.write(self.current_char())
        self.advance()
    return buffer.getvalue()
```

---

## Complete Example

```antlr
grammar JSON;

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

```python
from json_parser import Lexer, Parser, ParseError
from typing import Any

def parse_json(input_text: str) -> Any:
    """Parse JSON string."""
    lexer = Lexer(input_text)
    tokens = lexer.tokenize_all()
    parser = Parser(tokens)
    return parser.parse_json()

def main():
    json_text = '{"name": "minipg", "version": "0.2.0"}'
    try:
        value = parse_json(json_text)
        print(f"Parsed: {value}")
    except ParseError as e:
        print(f"Error: {e.message}")

if __name__ == "__main__":
    main()
```

---

## Troubleshooting

### Issue: Import Errors

**Problem**: Module not found

**Solution**: Add to PYTHONPATH:
```bash
export PYTHONPATH="${PYTHONPATH}:./src"
python main.py
```

### Issue: Type Errors

**Problem**: mypy reports type errors

**Solution**: Add type: ignore or fix types:
```python
result: Any = parser.parse_expr()  # type: ignore
```

### Issue: Slow Parsing

**Problem**: Parser is slow

**Solution**:
- Use PyPy for JIT compilation
- Profile with cProfile
- Optimize hot paths

---

## Testing

### Unit Tests

```python
import unittest
from calculator import Lexer, Parser

class TestCalculator(unittest.TestCase):
    def test_parse_number(self):
        lexer = Lexer("42")
        tokens = lexer.tokenize_all()
        parser = Parser(tokens)
        result = parser.parse_factor()
        self.assertIsNotNone(result)
    
    def test_parse_expression(self):
        lexer = Lexer("2 + 3")
        tokens = lexer.tokenize_all()
        parser = Parser(tokens)
        result = parser.parse_expr()
        self.assertIsNotNone(result)

if __name__ == "__main__":
    unittest.main()
```

---

## Summary

minipg's Python code generator produces:
- ✅ **Modern** - Python 3.10+ with type hints
- ✅ **Standalone** - No runtime dependencies
- ✅ **Idiomatic** - Follows PEP 8 and Python conventions
- ✅ **Type-safe** - Full type annotations
- ✅ **Complete** - Full ANTLR4 feature support

For more information, see:
- [User Guide](USER_GUIDE.md)
- [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md)
- [Examples](../examples/)
