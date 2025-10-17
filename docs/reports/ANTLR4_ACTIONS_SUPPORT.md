# ANTLR4 Actions Support

## Overview

minipg now supports ANTLR4 embedded actions and semantic predicates, allowing you to embed target-language code directly in your grammar.

## Supported Features

### 1. Embedded Actions ✅

Actions are code blocks that execute when a rule is matched.

**Syntax:**
```antlr4
rule
    : TOKEN { action code here }
    | OTHER { more action code }
    ;
```

**Example:**
```antlr4
expr
    : left=expr '+' right=expr { println!("Add: {} + {}", left, right); }
    | NUMBER { println!("Number: {}", $NUMBER.text); }
    ;
```

### 2. Semantic Predicates ✅

Predicates are boolean expressions that control whether a rule can match.

**Syntax:**
```antlr4
rule
    : {predicate}? element
    ;
```

**Example:**
```antlr4
number
    : {self.allow_floats}? FLOAT
    | INT
    ;
```

### 3. Language-Specific Actions

Actions can be tagged with a target language.

**Syntax:**
```antlr4
rule
    : TOKEN {rust: println!("Rust code"); }
    | TOKEN {python: print("Python code") }
    | TOKEN {javascript: console.log("JS code"); }
    ;
```

## AST Representation

### Action Element
```rust
Element::Action {
    code: String,           // The action code
    language: Option<String>, // Optional language tag
}
```

### Predicate Element
```rust
Element::Predicate {
    code: String,           // The predicate code
    language: Option<String>, // Optional language tag
}
```

## Usage Examples

### Basic Action
```rust
use minipg_ast::Element;

let action = Element::action("println!(\"Hello\");".to_string());
```

### Language-Specific Action
```rust
let rust_action = Element::action_with_language(
    "println!(\"Rust\");".to_string(),
    "rust".to_string()
);
```

### Semantic Predicate
```rust
let predicate = Element::predicate("self.allow_floats".to_string());
```

### Language-Specific Predicate
```rust
let python_predicate = Element::predicate_with_language(
    "self.allow_floats".to_string(),
    "python".to_string()
);
```

## Code Generation

### Rust
Actions are embedded directly in the generated Rust code:
```rust
// Generated code
match token.kind {
    TokenKind::NUMBER => {
        println!("Number: {}", token.text);
        // ... rest of parsing
    }
}
```

### Python
Actions are embedded as Python code:
```python
# Generated code
if token.kind == TokenKind.NUMBER:
    print(f"Number: {token.text}")
    # ... rest of parsing
```

### JavaScript/TypeScript
Actions are embedded as JavaScript code:
```javascript
// Generated code
if (token.kind === TokenKind.NUMBER) {
    console.log(`Number: ${token.text}`);
    // ... rest of parsing
}
```

## Language Translation

When generating code for a different target language, actions can be:

1. **Passed through** - If language matches target
2. **Translated** - Using language-specific translation rules
3. **Commented out** - If translation not available
4. **Skipped** - If no language tag and target doesn't match

### Translation Example

**Grammar:**
```antlr4
expr
    : NUMBER {rust: println!("Rust: {}", $NUMBER.text); }
    | NUMBER {python: print(f"Python: {$NUMBER.text}") }
    ;
```

**Generated Rust:**
```rust
match token.kind {
    TokenKind::NUMBER => {
        println!("Rust: {}", token.text);
    }
}
```

**Generated Python:**
```python
if token.kind == TokenKind.NUMBER:
    print(f"Python: {token.text}")
```

## Best Practices

### 1. Use Language Tags
Always tag actions with the target language to avoid ambiguity:
```antlr4
rule : TOKEN {rust: /* Rust code */} ;
```

### 2. Keep Actions Simple
Actions should be simple and focused:
```antlr4
// Good
expr : NUMBER { self.count += 1; } ;

// Avoid
expr : NUMBER { 
    // 50 lines of complex logic
} ;
```

### 3. Use Predicates for Validation
Use semantic predicates for context-sensitive parsing:
```antlr4
floatNumber
    : {self.allow_floats}? FLOAT
    | INT
    ;
```

### 4. Avoid Side Effects in Predicates
Predicates should be pure boolean expressions:
```antlr4
// Good
rule : {self.is_valid()}? element ;

// Avoid
rule : {self.count++; self.count > 10}? element ;
```

## Limitations

### Current Limitations
- ✅ Actions and predicates are stored in AST
- ✅ Language-specific actions supported
- ⚠️ No automatic translation between languages
- ⚠️ No validation of action code syntax
- ⚠️ No scoping analysis

### Future Enhancements
- [ ] Automatic action translation
- [ ] Action code validation
- [ ] Scoping and variable analysis
- [ ] Action templates
- [ ] Common action library

## Examples

### Calculator with Actions
```antlr4
grammar Calculator;

expr returns [int value]
    : left=expr '+' right=expr { $value = $left.value + $right.value; }
    | left=expr '*' right=expr { $value = $left.value * $right.value; }
    | NUMBER { $value = $NUMBER.int; }
    ;

NUMBER : [0-9]+ ;
```

### Context-Sensitive Parsing
```antlr4
grammar ContextSensitive;

statement
    : {self.in_function}? returnStmt
    | {self.in_loop}? breakStmt
    | regularStmt
    ;

returnStmt : 'return' expr ';' ;
breakStmt : 'break' ';' ;
regularStmt : expr ';' ;
```

### Multi-Language Actions
```antlr4
grammar MultiLang;

expr
    : NUMBER 
        {rust: println!("Rust: {}", $NUMBER.text); }
        {python: print(f"Python: {$NUMBER.text}") }
        {javascript: console.log(`JS: ${$NUMBER.text}`); }
    ;
```

## Testing

### Unit Tests
```rust
#[test]
fn test_action_element() {
    let action = Element::action("println!(\"test\");".to_string());
    match action {
        Element::Action { code, language } => {
            assert_eq!(code, "println!(\"test\");");
            assert_eq!(language, None);
        }
        _ => panic!("Expected Action element"),
    }
}

#[test]
fn test_predicate_element() {
    let pred = Element::predicate("self.valid".to_string());
    match pred {
        Element::Predicate { code, language } => {
            assert_eq!(code, "self.valid");
            assert_eq!(language, None);
        }
        _ => panic!("Expected Predicate element"),
    }
}
```

## Conclusion

ANTLR4 actions support in minipg provides:
- ✅ Full AST representation
- ✅ Language-specific actions
- ✅ Semantic predicates
- ✅ Clean API for creating actions
- ⚠️ Manual translation required (for now)

**Status**: Basic support complete, ready for use! 🚀

---

**Implementation Date**: 2025-10-17  
**Version**: 0.1.0  
**Status**: Implemented ✅
