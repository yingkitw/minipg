# Go Code Generation Guide

Complete guide to generating and using Go parsers with minipg.

## Overview

minipg generates standalone, zero-dependency Go parsers from ANTLR4-compatible grammars. The generated code is:

- **Standalone**: No runtime dependencies, pure Go
- **Idiomatic**: Follows Go conventions and best practices
- **Efficient**: Optimized for performance
- **Well-documented**: Includes comments and examples
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

### 2. Generate Go Parser

```bash
minipg generate calculator.g4 -o . -l go
```

This creates `calculator_parser.go`.

### 3. Use in Your Project

```go
package main

import (
    "fmt"
    "log"
)

func main() {
    input := "1 + 2 * 3"
    lexer := NewLexer(input)
    tokens, err := lexer.TokenizeAll()
    if err != nil {
        log.Fatal(err)
    }
    
    parser := NewParser(tokens)
    expr, err := parser.ParseExpr()
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println("Parsed:", expr)
}
```

## Generated Code Structure

### Lexer

The generated lexer tokenizes input into tokens:

```go
type Lexer struct {
    input       string
    position    int
    currentChar rune
}

func NewLexer(input string) *Lexer { ... }

func (l *Lexer) NextToken() (Token, error) { ... }

func (l *Lexer) TokenizeAll() ([]Token, error) { ... }
```

### Token Types

```go
type TokenKind string

const (
    TokenKindNUMBER TokenKind = "NUMBER"
    TokenKindPLUS   TokenKind = "PLUS"
    TokenKindMINUS  TokenKind = "MINUS"
    TokenKindSTAR   TokenKind = "STAR"
    TokenKindSLASH  TokenKind = "SLASH"
    TokenKindLPAREN TokenKind = "LPAREN"
    TokenKindRPAREN TokenKind = "RPAREN"
    TokenKindEOF    TokenKind = "EOF"
)

type Token struct {
    Kind     TokenKind
    Text     string
    Position int
}
```

### Parser

The generated parser builds an AST from tokens:

```go
type Parser struct {
    tokens   []Token
    position int
}

func NewParser(tokens []Token) *Parser { ... }

func (p *Parser) ParseExpr() (Expr, error) { ... }

func (p *Parser) ParseTerm() (Term, error) { ... }

func (p *Parser) ParseFactor() (Factor, error) { ... }
```

### Error Handling

```go
type ParseError struct {
    Message  string
    Position int
    Expected []string
    Found    *string
}

func (e *ParseError) Error() string { ... }
```

## Features

### 1. Idiomatic Go

Generated code follows Go conventions:
- PascalCase for exported types
- Error return values
- Interfaces for extensibility
- Proper error handling

### 2. Error Handling

Parsing operations return errors:

```go
expr, err := parser.ParseExpr()
if err != nil {
    if parseErr, ok := err.(*ParseError); ok {
        fmt.Printf("Parse error at position %d: %s\n", parseErr.Position, parseErr.Message)
        fmt.Printf("Expected: %v\n", parseErr.Expected)
        fmt.Printf("Found: %v\n", parseErr.Found)
    }
    return err
}
```

### 3. Error Collection

The lexer can collect all errors:

```go
lexer := NewLexer(input)
tokens, err := lexer.TokenizeAll()
if err != nil {
    fmt.Println("Lexer error:", err)
}
```

### 4. Parameterized Rules

Rules with arguments and return values:

```antlr
expr[allowNegative: bool]: term (('+' | '-') term)*;
factor returns [value: int]: NUMBER;
```

Generated code:

```go
func (p *Parser) ParseExpr(allowNegative bool) (Expr, error) { ... }

func (p *Parser) ParseFactor() (Factor, int, error) { ... }
```

### 5. List Labels

Collect multiple values with `+=`:

```antlr
items: item+=ID (',' item+=ID)*;
```

Generated code:

```go
type Items struct {
    Items []string
}
```

## Advanced Usage

### Custom Token Handling

```go
lexer := NewLexer(input)

for {
    token, err := lexer.NextToken()
    if err != nil {
        fmt.Println("Lexer error:", err)
        continue
    }
    
    if token.Kind == TokenKindEOF {
        break
    }
    
    fmt.Println("Token:", token)
}
```

### Custom Parser Handling

```go
parser := NewParser(tokens)

expr, err := parser.ParseExpr()
if err != nil {
    if parseErr, ok := err.(*ParseError); ok {
        fmt.Printf("Parse error at position %d: %s\n", parseErr.Position, parseErr.Message)
        fmt.Printf("Expected: %v\n", parseErr.Expected)
        fmt.Printf("Found: %v\n", parseErr.Found)
    }
    return err
}

// Process expression
processExpr(expr)
```

### Building an AST

The parser generates AST nodes:

```go
type Expr struct {
    Term Term
    Rest []struct {
        Op   string
        Term Term
    }
}

type Term struct {
    Factor Factor
    Rest   []struct {
        Op     string
        Factor Factor
    }
}

type Factor struct {
    Value interface{} // int or *Expr
}
```

### Visitor Pattern

Implement a visitor to traverse the AST:

```go
type ExprVisitor struct{}

func (v *ExprVisitor) VisitExpr(expr Expr) int {
    result := v.VisitTerm(expr.Term)
    for _, item := range expr.Rest {
        if item.Op == "+" {
            result += v.VisitTerm(item.Term)
        } else {
            result -= v.VisitTerm(item.Term)
        }
    }
    return result
}

func (v *ExprVisitor) VisitTerm(term Term) int {
    result := v.VisitFactor(term.Factor)
    for _, item := range term.Rest {
        if item.Op == "*" {
            result *= v.VisitFactor(item.Factor)
        } else {
            result /= v.VisitFactor(item.Factor)
        }
    }
    return result
}

func (v *ExprVisitor) VisitFactor(factor Factor) int {
    switch val := factor.Value.(type) {
    case int:
        return val
    case *Expr:
        return v.VisitExpr(*val)
    default:
        return 0
    }
}

// Usage
visitor := &ExprVisitor{}
result := visitor.VisitExpr(expr)
fmt.Println("Result:", result)
```

## Performance

### Optimization Tips

1. **Reuse lexer/parser**: Create once, reuse for multiple parses
2. **Use goroutines**: Parallelize parsing of multiple inputs
3. **Profile your code**: Use pprof to find bottlenecks

### Benchmarking

```go
import "testing"

func BenchmarkParser(b *testing.B) {
    input := "1 + 2 * 3"
    
    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        lexer := NewLexer(input)
        tokens, _ := lexer.TokenizeAll()
        parser := NewParser(tokens)
        parser.ParseExpr()
    }
}
```

Run with:
```bash
go test -bench=. -benchmem
```

## Best Practices

### 1. Error Handling

Always handle errors properly:

```go
// Good
expr, err := parser.ParseExpr()
if err != nil {
    return err
}

// Avoid
expr, _ := parser.ParseExpr()  // Ignores error
```

### 2. Defer Cleanup

Use defer for resource cleanup:

```go
func parseFile(filename string) error {
    file, err := os.Open(filename)
    if err != nil {
        return err
    }
    defer file.Close()
    
    // Parse file
    return nil
}
```

### 3. Testing

Write tests for your parsers:

```go
func TestSimpleExpr(t *testing.T) {
    lexer := NewLexer("1 + 2")
    tokens, err := lexer.TokenizeAll()
    if err != nil {
        t.Fatal(err)
    }
    
    parser := NewParser(tokens)
    expr, err := parser.ParseExpr()
    if err != nil {
        t.Fatal(err)
    }
    
    if expr == nil {
        t.Error("Expected non-nil expression")
    }
}

func TestComplexExpr(t *testing.T) {
    lexer := NewLexer("1 + 2 * 3")
    tokens, err := lexer.TokenizeAll()
    if err != nil {
        t.Fatal(err)
    }
    
    parser := NewParser(tokens)
    expr, err := parser.ParseExpr()
    if err != nil {
        t.Fatal(err)
    }
    
    if expr == nil {
        t.Error("Expected non-nil expression")
    }
}
```

### 4. Documentation

Document your code:

```go
// ParseExpr parses an expression rule.
// Returns the parsed expression or an error if parsing fails.
func (p *Parser) ParseExpr() (Expr, error) {
    // Implementation
}
```

## Troubleshooting

### Issue: "undefined: Lexer"

**Cause**: Parser file not imported or in wrong package

**Solution**: Check import path and package name
```go
import "path/to/parser"

lexer := parser.NewLexer(input)
```

### Issue: "ParseError" returned unexpectedly

**Cause**: Grammar doesn't match input

**Solution**: Check grammar and input
```bash
minipg validate grammar.g4
```

### Issue: Compilation errors

**Cause**: Go version mismatch

**Solution**: Update Go to 1.16+
```bash
go version
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
- [TypeScript Guide](TYPESCRIPT_GUIDE.md) - TypeScript code generation
- [KNOWN_LIMITATIONS.md](../KNOWN_LIMITATIONS.md) - Current limitations

---

**Last Updated**: October 25, 2025  
**Version**: v0.1.0-alpha.3  
**Status**: Production-ready  
**Go Version**: 1.16+
