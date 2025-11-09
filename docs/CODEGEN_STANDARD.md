# Code Generator Interface Standardization

## Overview

This document defines the standard interface and patterns that all code generators should follow to ensure consistency and maintainability.

## Standard Structure

All code generators should follow this structure:

```rust
pub struct {Language}CodeGenerator;

impl {Language}CodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    // Helper methods (private)
    fn generate_header(&self, grammar: &Grammar) -> String { ... }
    fn generate_token_types(&self, grammar: &Grammar) -> String { ... }
    fn generate_error_type(&self, grammar: &Grammar) -> String { ... }
    fn generate_lexer(&self, grammar: &Grammar) -> String { ... }
    fn generate_parser(&self, grammar: &Grammar) -> String { ... }
    fn generate_rule_method(&self, rule: &Rule) -> String { ... }
}

impl CodeGenerator for {Language}CodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, grammar: &Grammar, config: &CodeGenConfig) -> Result<String> {
        // Standard generation order:
        // 1. Header/package declarations
        // 2. Named actions (@header)
        // 3. Error types
        // 4. Token types
        // 5. Token structure/class
        // 6. AST types (if applicable)
        // 7. Lexer implementation
        // 8. Parser implementation
        // 9. Named actions (@members) integration
    }
    
    fn target_language(&self) -> &'static str {
        "{language}"
    }
}
```

## Required Components

### 1. Header/Package Declaration
- Include generation comment
- Package/module declaration (if applicable)
- Standard imports/dependencies
- Insert `@header` named action if present

### 2. Error Type
- Standard fields: message, position, expected, found
- Language-specific error handling (exceptions, Result types, etc.)
- Display/string representation

### 3. Token Types
- Enum or equivalent for token kinds
- Include all lexer rules (excluding fragments)
- Include EOF token
- Include Error token (if applicable)

### 4. Token Structure/Class
- Standard fields: kind/type, text, position
- Optional: line, column, length
- String representation method

### 5. Lexer Implementation
- Constructor/initializer
- `next_token()` / `NextToken()` method
- `tokenize_all()` / `TokenizeAll()` method (for error collection)
- Mode support (if applicable)
- Channel support (if applicable)
- Error recovery

### 6. Parser Implementation
- Constructor/initializer
- Rule parsing methods (one per parser rule)
- Support for rule arguments, returns, locals
- Support for labels (element and list)
- Error handling

### 7. Named Actions Integration
- `@header` - Inserted at top of file
- `@members` - Inserted in lexer/parser class/struct
- `@lexer::header` - Lexer-specific header
- `@parser::header` - Parser-specific header
- `@lexer::members` - Lexer-specific members
- `@parser::members` - Parser-specific members

## Standard Method Patterns

### Rule Method Generation

All generators should generate rule methods with:

1. **Documentation** - Rule description, arguments, returns
2. **Signature** - Properly typed parameters and return values
3. **Local Variables** - Declare locals if present
4. **Implementation** - Actual parsing logic (or TODO placeholder)

### Error Handling

- Use language-appropriate error types
- Include position information
- Include expected tokens list
- Include found token (if applicable)

### Tokenization

- Skip whitespace before tokenization
- Check for EOF
- Use DFA or pattern matching
- Error recovery: skip invalid characters
- Continue tokenizing after errors

## Language-Specific Conventions

### Rust
- Use `Result<T>` for error handling
- Use `snake_case` for identifiers
- Use `PascalCase` for types
- Use `#[derive(...)]` for common traits

### Python
- Use exceptions for error handling
- Use `snake_case` for identifiers
- Use `PascalCase` for classes
- Use type hints (Python 3.10+)
- Use `@dataclass` for data structures

### JavaScript/TypeScript
- Use exceptions for error handling
- Use `camelCase` for identifiers
- Use `PascalCase` for classes
- Use JSDoc/TSDoc comments
- Use classes for structures

### Go
- Use `error` interface for error handling
- Use `PascalCase` for exported identifiers
- Use `camelCase` for unexported identifiers
- Use receivers for methods
- Use constructor functions

### C/C++
- Use error codes or exceptions (C++)
- Use `snake_case` for identifiers
- Use `PascalCase` or `UPPER_CASE` for types
- Manual memory management (C)
- RAII and smart pointers (C++)

### Java
- Use exceptions for error handling
- Use `camelCase` for identifiers
- Use `PascalCase` for classes
- Use packages for organization
- Use proper access modifiers

## Common Utilities

Use `src/codegen/common.rs` helpers:

- `extract_token_types()` - Get token types from grammar
- `extract_parser_rules()` - Get parser rules
- `get_named_action()` - Get named action code
- `format_identifier()` - Format identifiers per language
- `format_type_name()` - Format type names per language

## Testing Requirements

All generators should have tests that verify:

1. Basic grammar generation
2. Token type generation
3. Error type generation
4. Lexer generation
5. Parser generation
6. Named actions integration
7. Rule features (arguments, returns, locals)
8. Labels support
9. Mode/channel support (if applicable)

## Checklist for New Generators

- [ ] Implement `CodeGenerator` trait
- [ ] Generate header/package declaration
- [ ] Generate error type
- [ ] Generate token types
- [ ] Generate token structure/class
- [ ] Generate lexer implementation
- [ ] Generate parser implementation
- [ ] Support named actions (@header, @members)
- [ ] Support rule features (arguments, returns, locals)
- [ ] Support labels (element and list)
- [ ] Support modes/channels (if applicable)
- [ ] Use common utilities where applicable
- [ ] Add comprehensive tests
- [ ] Follow language conventions
- [ ] Register in LanguageRegistry

---

**Last Updated**: 2025-01-XX
**Status**: Standardization Guide

