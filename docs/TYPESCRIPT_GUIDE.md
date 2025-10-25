# TypeScript Code Generation Guide

Complete guide to generating and using TypeScript parsers with minipg.

## Overview

minipg generates standalone, zero-dependency TypeScript parsers from ANTLR4-compatible grammars. The generated code is:

- **Standalone**: No runtime dependencies, pure TypeScript
- **Type-Safe**: Full type safety with interfaces and enums
- **Modern**: Uses TypeScript 4.0+ features
- **Well-documented**: Includes JSDoc and type hints
- **Production-Ready**: Suitable for enterprise applications

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

### 2. Generate TypeScript Parser

```bash
minipg generate calculator.g4 -o src -l typescript
```

This creates `src/calculator_parser.ts`.

### 3. Use in Your Project

```typescript
import { CalculatorLexer, CalculatorParser } from './calculator_parser';

const input: string = "1 + 2 * 3";
const lexer: CalculatorLexer = new CalculatorLexer(input);
const tokens: Token[] = lexer.tokenizeAll();

const parser: CalculatorParser = new CalculatorParser(tokens);
const expr: Expr = parser.parseExpr();
console.log("Parsed:", expr);
```

## Generated Code Structure

### Lexer

The generated lexer tokenizes input into tokens:

```typescript
class Lexer {
    private input: string;
    private position: number;
    private currentChar: string | null;
    
    constructor(input: string) { ... }
    
    nextToken(): Token { ... }
    
    tokenizeAll(): Token[] { ... }
}
```

### Token Types

```typescript
enum TokenKind {
    NUMBER = "NUMBER",
    PLUS = "PLUS",
    MINUS = "MINUS",
    STAR = "STAR",
    SLASH = "SLASH",
    LPAREN = "LPAREN",
    RPAREN = "RPAREN",
    EOF = "EOF",
}

interface Token {
    kind: TokenKind;
    text: string;
    position: number;
}
```

### Parser

The generated parser builds an AST from tokens:

```typescript
class Parser {
    private tokens: Token[];
    private position: number;
    
    constructor(tokens: Token[]) { ... }
    
    parseExpr(): Expr { ... }
    
    parseTerm(): Term { ... }
    
    parseFactor(): Factor { ... }
}
```

### Error Handling

```typescript
class ParseError extends Error {
    public readonly position: number;
    public readonly expected: string[];
    public readonly found: string | null;
    
    constructor(
        message: string,
        position: number,
        expected: string[] = [],
        found: string | null = null
    ) { ... }
}
```

## Features

### 1. Full Type Safety

All functions have complete type signatures:

```typescript
parseExpr(): Expr { ... }

parseTerm(): Term { ... }

parseFactor(): Factor { ... }
```

### 2. Interfaces and Enums

Generated code uses TypeScript interfaces:

```typescript
interface Expr {
    term: Term;
    rest: Array<[string, Term]>;
}

interface Term {
    factor: Factor;
    rest: Array<[string, Factor]>;
}

interface Factor {
    value: number | Expr;
}
```

### 3. Exception Handling

Parsing operations throw `ParseError` on failure:

```typescript
try {
    const expr: Expr = parser.parseExpr();
} catch (e) {
    if (e instanceof ParseError) {
        console.error(`Parse error at position ${e.position}: ${e.message}`);
        console.error(`Expected: ${e.expected.join(", ")}`);
        console.error(`Found: ${e.found}`);
    }
}
```

### 4. Error Collection

The lexer can collect all errors:

```typescript
const lexer: CalculatorLexer = new CalculatorLexer(input);
try {
    const tokens: Token[] = lexer.tokenizeAll();
} catch (e) {
    if (Array.isArray(e)) {
        console.error("Multiple lexer errors:", e);
    }
}
```

### 5. Parameterized Rules

Rules with arguments and return values:

```antlr
expr[allowNegative: boolean]: term (('+' | '-') term)*;
factor returns [value: number]: NUMBER;
```

Generated code:

```typescript
parseExpr(allowNegative: boolean): Expr { ... }

parseFactor(): [Factor, number] { ... }
```

### 6. List Labels

Collect multiple values with `+=`:

```antlr
items: item+=ID (',' item+=ID)*;
```

Generated code:

```typescript
interface Items {
    items: string[];
}
```

## Advanced Usage

### Custom Token Handling

```typescript
const lexer: CalculatorLexer = new CalculatorLexer(input);

while (true) {
    try {
        const token: Token = lexer.nextToken();
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

```typescript
const parser: CalculatorParser = new CalculatorParser(tokens);

try {
    const expr: Expr = parser.parseExpr();
    // Process expression
} catch (e) {
    if (e instanceof ParseError) {
        console.error(`Parse error at position ${e.position}: ${e.message}`);
        console.error(`Expected: ${e.expected}`);
        console.error(`Found: ${e.found}`);
    }
}
```

### Building an AST

The parser generates AST nodes with full type safety:

```typescript
interface Expr {
    term: Term;
    rest: Array<[string, Term]>;
}

interface Term {
    factor: Factor;
    rest: Array<[string, Factor]>;
}

interface Factor {
    value: number | Expr;
}
```

### Visitor Pattern

Implement a typed visitor to traverse the AST:

```typescript
class ExprVisitor {
    visitExpr(expr: Expr): number {
        let result: number = this.visitTerm(expr.term);
        for (const [op, term] of expr.rest) {
            if (op === '+') {
                result += this.visitTerm(term);
            } else {
                result -= this.visitTerm(term);
            }
        }
        return result;
    }
    
    visitTerm(term: Term): number {
        let result: number = this.visitFactor(term.factor);
        for (const [op, factor] of term.rest) {
            if (op === '*') {
                result *= this.visitFactor(factor);
            } else {
                result /= this.visitFactor(factor);
            }
        }
        return result;
    }
    
    visitFactor(factor: Factor): number {
        if (typeof factor.value === 'number') {
            return factor.value;
        } else {
            return this.visitExpr(factor.value);
        }
    }
}

// Usage
const visitor: ExprVisitor = new ExprVisitor();
const result: number = visitor.visitExpr(expr);
console.log("Result:", result);
```

## Performance

### Optimization Tips

1. **Reuse lexer/parser**: Create once, reuse for multiple parses
2. **Use const**: Helps TypeScript compiler optimize
3. **Enable strict mode**: Catches more errors at compile time

### Benchmarking

```typescript
const start: number = performance.now();
for (let i: number = 0; i < 1000; i++) {
    const lexer: CalculatorLexer = new CalculatorLexer(input);
    const tokens: Token[] = lexer.tokenizeAll();
    const parser: CalculatorParser = new CalculatorParser(tokens);
    parser.parseExpr();
}
const elapsed: number = performance.now() - start;
console.log(`Time per parse: ${elapsed / 1000}ms`);
```

## Best Practices

### 1. Type Annotations

Always use type annotations:

```typescript
// Good
const expr: Expr = parser.parseExpr();

// Avoid
const expr = parser.parseExpr();  // Type inferred
```

### 2. Error Handling

Always handle errors properly:

```typescript
// Good
try {
    const expr: Expr = parser.parseExpr();
    process(expr);
} catch (e) {
    if (e instanceof ParseError) {
        return null;
    }
    throw e;
}

// Avoid
const expr: Expr = parser.parseExpr();  // May throw
```

### 3. Strict Mode

Enable TypeScript strict mode:

```json
{
    "compilerOptions": {
        "strict": true,
        "strictNullChecks": true,
        "strictFunctionTypes": true
    }
}
```

### 4. Testing

Write typed tests for your parsers:

```typescript
import * as assert from 'assert';

function testSimpleExpr(): void {
    const lexer: CalculatorLexer = new CalculatorLexer("1 + 2");
    const tokens: Token[] = lexer.tokenizeAll();
    const parser: CalculatorParser = new CalculatorParser(tokens);
    const expr: Expr = parser.parseExpr();
    assert(expr !== null);
}

function testComplexExpr(): void {
    const lexer: CalculatorLexer = new CalculatorLexer("1 + 2 * 3");
    const tokens: Token[] = lexer.tokenizeAll();
    const parser: CalculatorParser = new CalculatorParser(tokens);
    const expr: Expr = parser.parseExpr();
    assert(expr !== null);
}

testSimpleExpr();
testComplexExpr();
console.log("All tests passed!");
```

## Troubleshooting

### Issue: "Cannot find module"

**Cause**: Parser file not in correct location

**Solution**: Check file path and import statement
```typescript
import { Lexer, Parser } from './path/to/parser';
```

### Issue: "ParseError" thrown unexpectedly

**Cause**: Grammar doesn't match input

**Solution**: Check grammar and input
```bash
minipg validate grammar.g4
```

### Issue: Type errors in generated code

**Cause**: TypeScript version mismatch

**Solution**: Update TypeScript to 4.0+
```bash
npm install --save-dev typescript@latest
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
- [Python Guide](PYTHON_GUIDE.md) - Python code generation
- [JavaScript Guide](JAVASCRIPT_GUIDE.md) - JavaScript code generation
- [KNOWN_LIMITATIONS.md](../KNOWN_LIMITATIONS.md) - Current limitations

---

**Last Updated**: October 25, 2025  
**Version**: v0.1.0-alpha.3  
**Status**: Production-ready  
**TypeScript Version**: 4.0+
