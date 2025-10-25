# Action Translation Guide

## Overview

minipg supports **embedded actions** and **semantic predicates** in ANTLR4 grammars. Actions allow you to embed code that executes during parsing, and semantic predicates allow you to add conditions to parsing decisions.

The action translation system automatically converts action code between different target languages, making it easy to write grammars that work across multiple platforms.

## Action Types

### 1. Embedded Actions

Embedded actions are code blocks that execute during parsing:

```antlr
rule
  : element1 { /* action code here */ } element2
  ;
```

### 2. Semantic Predicates

Semantic predicates are conditions that must be true for parsing to continue:

```antlr
rule
  : element {condition}?
  ;
```

## Language-Specific Actions

You can specify actions for specific languages using language prefixes:

```antlr
rule
  : element
    {rust: self.values.insert(key, value);}
    {python: self.values[key] = value}
    {javascript: this.values[key] = value;}
  ;
```

## Automatic Translation

If you write actions in Rust, minipg can automatically translate them to other languages:

### Rust → Python

| Rust | Python |
|------|--------|
| `self.method()` | `self.method()` |
| `Vec::new()` | `[]` |
| `HashMap::new()` | `{}` |
| `.push()` | `.append()` |
| `.to_string()` | `str()` |
| `true` / `false` | `True` / `False` |

### Rust → JavaScript/TypeScript

| Rust | JavaScript |
|------|-----------|
| `self.method()` | `this.method()` |
| `Vec::new()` | `[]` |
| `HashMap::new()` | `{}` |
| `.to_string()` | `.toString()` |
| `.len()` | `.length` |

### Rust → Go

| Rust | Go |
|------|-----|
| `self.method()` | `l.method()` |
| `Vec::new()` | `make([]Token, 0)` |
| `HashMap::new()` | `make(map[string]Token)` |
| `.push()` | `append()` |

## Semantic Predicates

Semantic predicates are generated with language-specific syntax:

### Rust
```rust
if !(condition) {
    return Err(ParseError::new("Predicate failed".to_string(), self.position));
}
```

### Python
```python
if not (condition):
    raise ParseError("Predicate failed", self.position)
```

### JavaScript/TypeScript
```javascript
if (!(condition)) {
    throw new ParseError("Predicate failed", this.position);
}
```

### Go
```go
if !(condition) {
    return nil, &ParseError{Message: "Predicate failed", Position: l.position}
}
```

## Example: Calculator with Actions

```antlr
grammar Calculator;

@members {
  // Rust: values: HashMap<String, i32>
  // Python: values = {}
  // JavaScript: this.values = {};
}

program
  : statement+ EOF
  ;

statement
  : ID '=' expr=expression
    {
      // Store the result
      // Rust: self.values.insert(ID.to_string(), expr);
      // Python: self.values[ID] = expr
      // JavaScript: this.values[ID] = expr;
    }
  | expression
  ;

expression
  : term (('+' | '-') term)*
  ;

term
  : factor (('*' | '/') factor)*
  ;

factor
  : NUMBER
  | '(' expression ')'
  | ID
  ;

ID: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

## Best Practices

1. **Keep Actions Simple**: Complex logic should be in separate functions
2. **Use Language-Specific Actions**: When possible, write language-specific actions for better performance
3. **Handle Errors**: Always check for errors in actions
4. **Test Translations**: Test generated code in each target language
5. **Document Actions**: Add comments explaining what actions do

## Limitations

- Action translation is best-effort; complex Rust code may not translate perfectly
- Some language-specific idioms may not translate automatically
- For complex transformations, consider writing language-specific actions

## See Also

- [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md) - Full ANTLR4 feature support
- [Grammar Syntax Reference](GRAMMAR_SYNTAX.md) - Complete grammar syntax
- [Examples](../examples/) - Example grammars with actions
