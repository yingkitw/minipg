# JavaScript Code Generation Guide

Complete guide to generating and using JavaScript parsers with minipg.

## Overview

minipg generates standalone, zero-dependency JavaScript parsers from ANTLR4-compatible grammars. The generated code is:

- **Standalone**: No runtime dependencies, pure JavaScript (ES6+)
- **Modern**: Uses classes, const/let, arrow functions
- **Works Everywhere**: Node.js and browsers
- **Well-documented**: Includes JSDoc comments
- **Type-friendly**: Works well with TypeScript

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

### 2. Generate JavaScript Parser

```bash
minipg generate calculator.g4 -o . -l javascript
```

This creates `calculator_parser.js`.

### 3. Use in Your Project

**Node.js:**
```javascript
const { CalculatorLexer, CalculatorParser } = require('./calculator_parser');

const input = "1 + 2 * 3";
const lexer = new CalculatorLexer(input);
const tokens = lexer.tokenizeAll();

const parser = new CalculatorParser(tokens);
const expr = parser.parseExpr();
console.log("Parsed:", expr);
```

**Browser:**
```html
<script src="calculator_parser.js"></script>
<script>
    const { CalculatorLexer, CalculatorParser } = window;
    
    const input = "1 + 2 * 3";
    const lexer = new CalculatorLexer(input);
    const tokens = lexer.tokenizeAll();
    
    const parser = new CalculatorParser(tokens);
    const expr = parser.parseExpr();
    console.log("Parsed:", expr);
</script>
```

## Generated Code Structure

### Lexer

The generated lexer tokenizes input into tokens:

```javascript
class Lexer {
    constructor(input) {
        this.input = input;
        this.position = 0;
        this.currentChar = input[0] || null;
    }
    
    nextToken() { ... }
    
    tokenizeAll() { ... }
}
```

### Token Types

```javascript
const TokenKind = {
    NUMBER: "NUMBER",
    PLUS: "PLUS",
    MINUS: "MINUS",
    STAR: "STAR",
    SLASH: "SLASH",
    LPAREN: "LPAREN",
    RPAREN: "RPAREN",
    EOF: "EOF",
};

class Token {
    constructor(kind, text, position) {
        this.kind = kind;
        this.text = text;
        this.position = position;
    }
}
```

### Parser

The generated parser builds an AST from tokens:

```javascript
class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
    }
    
    parseExpr() { ... }
    
    parseTerm() { ... }
    
    parseFactor() { ... }
}
```

### Error Handling

```javascript
class ParseError extends Error {
    constructor(message, position, expected = [], found = null) {
        super(message);
        this.name = "ParseError";
        this.position = position;
        this.expected = expected;
        this.found = found;
    }
}
```

## Features

### 1. Modern JavaScript

Generated code uses modern JavaScript features:
- ES6 classes
- const/let
- Arrow functions
- Template literals
- Destructuring

### 2. Error Handling

Parsing operations throw `ParseError` on failure:

```javascript
try {
    const expr = parser.parseExpr();
} catch (e) {
    if (e instanceof ParseError) {
        console.error(`Parse error at position ${e.position}: ${e.message}`);
        console.error(`Expected: ${e.expected.join(", ")}`);
        console.error(`Found: ${e.found}`);
    }
}
```

### 3. Error Collection

The lexer can collect all errors:

```javascript
const lexer = new CalculatorLexer(input);
try {
    const tokens = lexer.tokenizeAll();
} catch (e) {
    if (Array.isArray(e)) {
        console.error("Multiple lexer errors:", e);
    }
}
```

### 4. Parameterized Rules

Rules with arguments and return values:

```antlr
expr[allowNegative]: term (('+' | '-') term)*;
factor returns [value]: NUMBER;
```

Generated code:

```javascript
parseExpr(allowNegative) { ... }

parseFactor() { ... }  // returns [factor, value]
```

### 5. List Labels

Collect multiple values with `+=`:

```antlr
items: item+=ID (',' item+=ID)*;
```

Generated code:

```javascript
class Items {
    constructor(items) {
        this.items = items;
    }
}
```

## Advanced Usage

### Custom Token Handling

```javascript
const lexer = new CalculatorLexer(input);

while (true) {
    try {
        const token = lexer.nextToken();
        if (token.kind === TokenKind.EOF) {
            break;
        }
        console.log("Token:", token);
    } catch (e) {
        console.error("Lexer error:", e);
        // Continue to next token
    }
}
```

### Custom Parser Handling

```javascript
const parser = new CalculatorParser(tokens);

try {
    const expr = parser.parseExpr();
    // Process expression
} catch (e) {
    console.error(`Parse error at position ${e.position}: ${e.message}`);
    console.error(`Expected: ${e.expected}`);
    console.error(`Found: ${e.found}`);
}
```

### Building an AST

The parser generates AST nodes:

```javascript
class Expr {
    constructor(term, rest) {
        this.term = term;
        this.rest = rest;  // Array of [op, term] pairs
    }
}

class Term {
    constructor(factor, rest) {
        this.factor = factor;
        this.rest = rest;  // Array of [op, factor] pairs
    }
}

class Factor {
    constructor(value) {
        this.value = value;  // number or Expr
    }
}
```

### Visitor Pattern

Implement a visitor to traverse the AST:

```javascript
class ExprVisitor {
    visitExpr(expr) {
        let result = this.visitTerm(expr.term);
        for (const [op, term] of expr.rest) {
            if (op === '+') {
                result += this.visitTerm(term);
            } else {
                result -= this.visitTerm(term);
            }
        }
        return result;
    }
    
    visitTerm(term) {
        let result = this.visitFactor(term.factor);
        for (const [op, factor] of term.rest) {
            if (op === '*') {
                result *= this.visitFactor(factor);
            } else {
                result /= this.visitFactor(factor);
            }
        }
        return result;
    }
    
    visitFactor(factor) {
        if (typeof factor.value === 'number') {
            return factor.value;
        } else {
            return this.visitExpr(factor.value);
        }
    }
}

// Usage
const visitor = new ExprVisitor();
const result = visitor.visitExpr(expr);
console.log("Result:", result);
```

## Performance

### Optimization Tips

1. **Reuse lexer/parser**: Create once, reuse for multiple parses
2. **Minimize allocations**: Avoid creating unnecessary objects
3. **Use const**: Helps JavaScript engines optimize

### Benchmarking

```javascript
const start = performance.now();
for (let i = 0; i < 1000; i++) {
    const lexer = new CalculatorLexer(input);
    const tokens = lexer.tokenizeAll();
    const parser = new CalculatorParser(tokens);
    parser.parseExpr();
}
const elapsed = performance.now() - start;
console.log(`Time per parse: ${elapsed / 1000}ms`);
```

## Best Practices

### 1. Error Handling

Always handle errors properly:

```javascript
// Good
try {
    const expr = parser.parseExpr();
    process(expr);
} catch (e) {
    return null;
}

// Avoid
const expr = parser.parseExpr();  // May throw
```

### 2. Type Checking

Use JSDoc for better IDE support:

```javascript
/**
 * Parse expression.
 * @returns {Expr} The parsed expression
 * @throws {ParseError} If parsing fails
 */
parseExpr() { ... }
```

### 3. Testing

Write tests for your parsers:

```javascript
const assert = require('assert');

function testSimpleExpr() {
    const lexer = new CalculatorLexer("1 + 2");
    const tokens = lexer.tokenizeAll();
    const parser = new CalculatorParser(tokens);
    const expr = parser.parseExpr();
    assert(expr !== null);
}

function testComplexExpr() {
    const lexer = new CalculatorLexer("1 + 2 * 3");
    const tokens = lexer.tokenizeAll();
    const parser = new CalculatorParser(tokens);
    const expr = parser.parseExpr();
    assert(expr !== null);
}

testSimpleExpr();
testComplexExpr();
console.log("All tests passed!");
```

## Troubleshooting

### Issue: "Cannot find module"

**Cause**: Parser file not in correct location

**Solution**: Check file path and require statement
```javascript
const { Lexer, Parser } = require('./path/to/parser');
```

### Issue: "ParseError" thrown unexpectedly

**Cause**: Grammar doesn't match input

**Solution**: Check grammar and input
```bash
minipg validate grammar.g4
```

### Issue: AST nodes are undefined

**Cause**: Parser rule not implemented

**Solution**: Check grammar for all rules defined

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
- [Python Guide](PYTHON_GUIDE.md) - Python code generation
- [KNOWN_LIMITATIONS.md](../KNOWN_LIMITATIONS.md) - Current limitations

---

**Last Updated**: October 25, 2025  
**Version**: v0.1.0-alpha.3  
**Status**: Production-ready  
**Environment**: Node.js and browsers
