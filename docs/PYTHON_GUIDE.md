# Python Code Generation Guide

Complete guide to generating and using Python parsers with minipg.

## Overview

minipg generates standalone, zero-dependency Python parsers from ANTLR4-compatible grammars. The generated code is:

- **Standalone**: No runtime dependencies, pure Python 3.10+
- **Pythonic**: Follows PEP 8 conventions and Python idioms
- **Type-hinted**: Full type hints for IDE support
- **Well-documented**: Includes docstrings and type hints
- **Dataclass-based**: Uses dataclasses for AST nodes

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

### 2. Generate Python Parser

```bash
minipg generate calculator.g4 -o . -l python
```

This creates `calculator_parser.py`.

### 3. Use in Your Project

```python
from calculator_parser import CalculatorLexer, CalculatorParser

def main():
    input_text = "1 + 2 * 3"
    lexer = CalculatorLexer(input_text)
    tokens = lexer.tokenize_all()
    
    parser = CalculatorParser(tokens)
    expr = parser.parse_expr()
    print(f"Parsed: {expr}")

if __name__ == "__main__":
    main()
```

## Generated Code Structure

### Lexer

The generated lexer tokenizes input into tokens:

```python
class Lexer:
    def __init__(self, input_text: str) -> None:
        self.input = input_text
        self.position = 0
        self.current_char = input_text[0] if input_text else None
    
    def next_token(self) -> Token:
        """Get next token from input."""
        ...
    
    def tokenize_all(self) -> list[Token]:
        """Tokenize entire input, collecting all tokens."""
        ...
```

### Token Types

```python
from enum import Enum
from dataclasses import dataclass

class TokenKind(Enum):
    NUMBER = "NUMBER"
    PLUS = "PLUS"
    MINUS = "MINUS"
    STAR = "STAR"
    SLASH = "SLASH"
    LPAREN = "LPAREN"
    RPAREN = "RPAREN"
    EOF = "EOF"

@dataclass
class Token:
    kind: TokenKind
    text: str
    position: int
```

### Parser

The generated parser builds an AST from tokens:

```python
class Parser:
    def __init__(self, tokens: list[Token]) -> None:
        self.tokens = tokens
        self.position = 0
    
    def parse_expr(self) -> Expr:
        """Parse expression rule."""
        ...
    
    def parse_term(self) -> Term:
        """Parse term rule."""
        ...
    
    def parse_factor(self) -> Factor:
        """Parse factor rule."""
        ...
```

### Error Handling

```python
@dataclass
class ParseError(Exception):
    message: str
    position: int
    expected: list[str] = None
    found: str = None
    
    def __str__(self) -> str:
        return f"Parse error at position {self.position}: {self.message}"
```

## Features

### 1. Type Hints

All functions have full type hints:

```python
def parse_expr(self) -> Expr:
    """Parse expression."""
    ...

def parse_term(self) -> Term:
    """Parse term."""
    ...
```

### 2. Exception Handling

Parsing operations raise `ParseError` on failure:

```python
try:
    expr = parser.parse_expr()
except ParseError as e:
    print(f"Parse error: {e}")
    print(f"Position: {e.position}")
    print(f"Expected: {e.expected}")
```

### 3. Error Collection

The lexer can collect all errors:

```python
lexer = CalculatorLexer(input_text)
try:
    tokens = lexer.tokenize_all()
except ParseError as e:
    print(f"Lexer error: {e}")
```

### 4. Dataclass AST Nodes

AST nodes are dataclasses:

```python
from dataclasses import dataclass

@dataclass
class Expr:
    terms: list[Term]
    operators: list[str]

@dataclass
class Term:
    factors: list[Factor]
    operators: list[str]

@dataclass
class Factor:
    value: int | Expr
```

### 5. Parameterized Rules

Rules with arguments and return values:

```antlr
expr[allowNegative: bool]: term (('+' | '-') term)*;
factor returns [value: int]: NUMBER;
```

Generated code:

```python
def parse_expr(self, allow_negative: bool) -> Expr:
    """Parse expression with optional negative numbers."""
    ...

def parse_factor(self) -> tuple[Factor, int]:
    """Parse factor and return value."""
    ...
```

### 6. List Labels

Collect multiple values with `+=`:

```antlr
items: item+=ID (',' item+=ID)*;
```

Generated code:

```python
@dataclass
class Items:
    items: list[str]
```

### 7. Named Actions

Insert custom code into generated parser:

```antlr
@header {
    from typing import Dict
}

@members {
    def custom_method(self) -> None:
        """Custom method in parser."""
        pass
}
```

## Advanced Usage

### Custom Token Handling

```python
lexer = CalculatorLexer(input_text)

while True:
    try:
        token = lexer.next_token()
        if token.kind == TokenKind.EOF:
            break
        print(f"Token: {token}")
    except ParseError as e:
        print(f"Lexer error: {e}")
        # Continue to next token
```

### Custom Parser Handling

```python
parser = CalculatorParser(tokens)

try:
    expr = parser.parse_expr()
    # Process expression
except ParseError as e:
    print(f"Parse error at position {e.position}: {e.message}")
    print(f"Expected: {e.expected}")
    print(f"Found: {e.found}")
```

### Building an AST

The parser generates AST nodes:

```python
from dataclasses import dataclass
from typing import Union

@dataclass
class Expr:
    term: Term
    rest: list[tuple[str, Term]]

@dataclass
class Term:
    factor: Factor
    rest: list[tuple[str, Factor]]

@dataclass
class Factor:
    value: Union[int, Expr]
```

### Visitor Pattern

Implement a visitor to traverse the AST:

```python
class ExprVisitor:
    def visit_expr(self, expr: Expr) -> int:
        result = self.visit_term(expr.term)
        for op, term in expr.rest:
            if op == '+':
                result += self.visit_term(term)
            else:
                result -= self.visit_term(term)
        return result
    
    def visit_term(self, term: Term) -> int:
        result = self.visit_factor(term.factor)
        for op, factor in term.rest:
            if op == '*':
                result *= self.visit_factor(factor)
            else:
                result //= self.visit_factor(factor)
        return result
    
    def visit_factor(self, factor: Factor) -> int:
        if isinstance(factor.value, int):
            return factor.value
        else:
            return self.visit_expr(factor.value)

# Usage
visitor = ExprVisitor()
result = visitor.visit_expr(expr)
print(f"Result: {result}")
```

## Performance

### Optimization Tips

1. **Reuse lexer/parser**: Create once, reuse for multiple parses
2. **Use type hints**: Helps Python optimize with Cython
3. **Profile your code**: Use `cProfile` to find bottlenecks

### Benchmarking

```python
import time

start = time.time()
for _ in range(1000):
    lexer = CalculatorLexer(input_text)
    tokens = lexer.tokenize_all()
    parser = CalculatorParser(tokens)
    parser.parse_expr()
elapsed = time.time() - start
print(f"Time per parse: {elapsed / 1000:.6f}s")
```

## Best Practices

### 1. Error Handling

Always handle exceptions properly:

```python
# Good
try:
    expr = parser.parse_expr()
except ParseError as e:
    return None

# Avoid
expr = parser.parse_expr()  # May raise exception
```

### 2. Type Hints

Use type hints for better IDE support:

```python
def process_expr(expr: Expr) -> int:
    """Process expression and return result."""
    ...
```

### 3. Docstrings

Document your code:

```python
def parse_expr(self) -> Expr:
    """Parse expression rule.
    
    Returns:
        Expr: The parsed expression.
        
    Raises:
        ParseError: If parsing fails.
    """
    ...
```

### 4. Testing

Write tests for your parsers:

```python
import unittest

class TestCalculatorParser(unittest.TestCase):
    def test_simple_expr(self):
        lexer = CalculatorLexer("1 + 2")
        tokens = lexer.tokenize_all()
        parser = CalculatorParser(tokens)
        expr = parser.parse_expr()
        self.assertIsNotNone(expr)
    
    def test_complex_expr(self):
        lexer = CalculatorLexer("1 + 2 * 3")
        tokens = lexer.tokenize_all()
        parser = CalculatorParser(tokens)
        expr = parser.parse_expr()
        self.assertIsNotNone(expr)

if __name__ == "__main__":
    unittest.main()
```

## Troubleshooting

### Issue: "ModuleNotFoundError"

**Cause**: Generated file not in Python path

**Solution**: Add directory to Python path
```python
import sys
sys.path.insert(0, '.')
from calculator_parser import CalculatorLexer, CalculatorParser
```

### Issue: "ParseError" raised unexpectedly

**Cause**: Grammar doesn't match input

**Solution**: Check grammar and input
```bash
minipg validate grammar.g4
```

### Issue: Type hints not working

**Cause**: Python version < 3.10

**Solution**: Update Python or use string annotations
```python
from __future__ import annotations
```

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
- [Rust Guide](RUST_GUIDE.md) - Rust code generation
- [KNOWN_LIMITATIONS.md](../KNOWN_LIMITATIONS.md) - Current limitations

---

**Last Updated**: October 25, 2025  
**Version**: v0.1.0-alpha.3  
**Status**: Production-ready  
**Python Version**: 3.10+
