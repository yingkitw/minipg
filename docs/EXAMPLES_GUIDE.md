# minipg Examples Guide

This guide provides a comprehensive overview of all example grammars included with minipg and demonstrates how to use them to learn the parser generator.

## Quick Start

Generate a parser from any example:

```bash
minipg generate examples/calculator.g4 -o output/ -l rust
minipg generate examples/json.g4 -o output/ -l python
minipg generate examples/GraphQL.g4 -o output/ -l typescript
```

## Example Categories

### 1. Simple Examples (Beginner)

#### calculator.g4 (229 bytes)
**Purpose**: Learn basic grammar structure

**Features**:
- Arithmetic operators (+, -, *, /)
- Parentheses for grouping
- Integer numbers
- Simple expression hierarchy

**Key Concepts**:
- Rule definitions
- Terminals and non-terminals
- Operator precedence

**Use Case**: Perfect for first-time users to understand grammar basics

---

#### json.g4 (303 bytes)
**Purpose**: Learn handling multiple value types

**Features**:
- Strings, numbers, booleans, null
- Arrays and objects
- Nested structures

**Key Concepts**:
- Recursive rules
- Alternative patterns
- Lexer vs parser rules

**Use Case**: Understand how to parse structured data

---

### 2. Intermediate Examples

#### Expression.g4 (1.2 KB)
**Purpose**: Master operator precedence and complex expressions

**Features**:
- Logical operators (||, &&)
- Comparison operators (==, !=, <, >, <=, >=)
- Arithmetic operators (+, -, *, /, %)
- Unary operators (!, -, +)
- Function calls and array access
- Array and object literals

**Key Concepts**:
- Operator precedence through rule hierarchy
- Postfix operations
- Primary expressions

**Use Case**: Build expression evaluators, formula parsers, scripting languages

---

#### Config.g4 (1.1 KB)
**Purpose**: Learn configuration file parsing

**Features**:
- Section headers [section]
- Key-value pairs
- Multiple value types (strings, numbers, booleans, arrays)
- Comments (both ; and # styles)

**Key Concepts**:
- Lexer commands (-> skip)
- Multiple token types
- Flexible parsing

**Use Case**: Parse INI, TOML-like configuration files

---

#### YAML.g4 (1.0 KB)
**Purpose**: Handle nested structures and multiple formats

**Features**:
- Mappings (key-value pairs)
- Sequences (lists)
- Multiple scalar types
- Nested structures

**Key Concepts**:
- Complex nesting
- Multiple scalar representations
- Flexible value types

**Use Case**: Parse YAML, configuration languages, data formats

---

#### CompleteJSON.g4 (909 bytes)
**Purpose**: RFC 8259 compliant JSON parsing

**Features**:
- Escape sequences in strings
- Unicode support
- Scientific notation for numbers
- Proper whitespace handling

**Key Concepts**:
- Unicode character classes
- Escape sequence handling
- Exact specification compliance

**Use Case**: Production-ready JSON parser

---

### 3. Advanced Examples

#### SQL.g4 (2.0 KB)
**Purpose**: Parse complex database queries

**Features**:
- SELECT with WHERE, ORDER BY, GROUP BY, LIMIT
- INSERT, UPDATE, DELETE statements
- CREATE TABLE with column definitions
- Joins and subqueries
- Operators and conditions

**Key Concepts**:
- Complex statement structures
- Multiple statement types
- Operator precedence in queries
- Nested expressions

**Use Case**: Build database query parsers, SQL validators

---

#### CSS.g4 (1.8 KB)
**Purpose**: Parse stylesheets

**Features**:
- Selectors (element, class, ID, universal)
- Pseudo-classes and pseudo-elements
- Declarations and properties
- Media queries
- Keyframes and font-face

**Key Concepts**:
- Selector combinators
- Pseudo-element syntax
- At-rules (@media, @keyframes)
- Nested structures

**Use Case**: CSS preprocessors, style validators, theme builders

---

#### GraphQL.g4 (2.5 KB)
**Purpose**: Parse GraphQL schema definitions

**Features**:
- Schema definitions with operation types
- Type definitions (scalar, object, interface, union, enum, input)
- Field definitions with arguments
- Directives and directive locations
- Complex type system

**Key Concepts**:
- Complex type hierarchies
- Multiple definition types
- Directive specifications
- Advanced language features

**Use Case**: GraphQL schema validators, API generators

---

#### Markdown.g4 (1.6 KB)
**Purpose**: Parse Markdown documents

**Features**:
- Headings, paragraphs, code blocks
- Lists and blockquotes
- Inline formatting (bold, italic, code)
- Links and images
- Comments

**Key Concepts**:
- Text processing
- Special character handling
- Inline vs block elements
- Nested structures

**Use Case**: Markdown processors, documentation generators

---

#### Query.g4 (2.3 KB)
**Purpose**: Parse advanced query languages

**Features**:
- SELECT with complex joins
- INSERT, UPDATE, DELETE
- CREATE TABLE
- Subqueries and aggregations
- Complex expressions

**Key Concepts**:
- Advanced statement structures
- Join operations
- Subquery handling
- Expression nesting

**Use Case**: Query builders, database tools, DSL creation

---

#### Protocol.g4 (2.1 KB)
**Purpose**: Parse Protocol Buffer-like definitions

**Features**:
- Message definitions with fields
- Enum definitions
- Service definitions with RPC methods
- Nested messages and enums
- Field options and modifiers

**Key Concepts**:
- Message structure definitions
- Nested type definitions
- Service specifications
- Field modifiers and options

**Use Case**: Protocol definition parsers, API schema generators

---

### 4. Real-World Examples

#### JavaSubset.g4 (4.8 KB)
**Purpose**: Parse a subset of Java

**Features**:
- Package and import declarations
- Class and interface declarations
- Fields, methods, constructors
- Control flow statements
- Expressions and operators
- Comments

**Key Concepts**:
- Programming language syntax
- Complex statement structures
- Method and field definitions
- Type specifications

**Use Case**: Java parsers, code analyzers, refactoring tools

---

#### PythonSubset.g4 (4.9 KB)
**Purpose**: Parse a subset of Python 3

**Features**:
- Import statements
- Function and class definitions
- Control flow (if, while, for, try)
- Expressions and operators
- List, dict, tuple literals
- String literals (including triple-quoted)

**Key Concepts**:
- Indentation handling
- Multiple string types
- Complex control flow
- Python-specific syntax

**Use Case**: Python parsers, code analysis, AST generation

---

## Learning Recommendations

### For Beginners
1. Start with `calculator.g4` - Understand basic structure
2. Move to `json.g4` - Learn multiple value types
3. Try `Expression.g4` - Master operator precedence
4. Experiment with `Config.g4` - Real-world format

### For Intermediate Users
1. Study `CompleteJSON.g4` - Production-quality parsing
2. Explore `CSS.g4` - Selector syntax and media queries
3. Examine `YAML.g4` - Complex nesting patterns
4. Review `SQL.g4` - Multiple statement types

### For Advanced Users
1. Analyze `GraphQL.g4` - Type system design
2. Study `Protocol.g4` - Message definitions
3. Examine `Markdown.g4` - Text processing
4. Review `Query.g4` - Advanced query structures
5. Explore `JavaSubset.g4` and `PythonSubset.g4` - Language design

## Feature Coverage by Example

| Feature | Example | Complexity |
|---------|---------|-----------|
| Basic operators | calculator.g4 | Beginner |
| Nested structures | json.g4 | Beginner |
| Operator precedence | Expression.g4 | Intermediate |
| Multiple statement types | SQL.g4 | Advanced |
| Selectors & pseudo-classes | CSS.g4 | Advanced |
| Type system | GraphQL.g4 | Advanced |
| Directives | GraphQL.g4 | Advanced |
| Nested definitions | Protocol.g4 | Advanced |
| Text processing | Markdown.g4 | Advanced |
| Language syntax | JavaSubset.g4 | Advanced |

## Testing Your Understanding

For each example, try these exercises:

1. **Modify the grammar**: Add new features (e.g., add comments to calculator.g4)
2. **Generate different languages**: Create parsers in Python, JavaScript, TypeScript, Go
3. **Test edge cases**: Try parsing complex valid inputs
4. **Combine features**: Mix features from different examples

## Code Generation

Generate parsers in multiple languages:

```bash
# Rust (optimized with DFA)
minipg generate examples/GraphQL.g4 -o output/ -l rust

# Python (with type hints)
minipg generate examples/GraphQL.g4 -o output/ -l python

# JavaScript (ES6+)
minipg generate examples/GraphQL.g4 -o output/ -l javascript

# TypeScript (fully typed)
minipg generate examples/GraphQL.g4 -o output/ -l typescript

# Go (idiomatic)
minipg generate examples/GraphQL.g4 -o output/ -l go
```

## Performance Notes

- **Simple examples** (calculator, json): Sub-millisecond generation
- **Intermediate examples** (CSS, Config): Millisecond generation
- **Complex examples** (GraphQL, Protocol): Few milliseconds generation
- **Real-world examples** (JavaSubset, PythonSubset): Sub-second generation

All examples generate optimized, standalone parsers with no runtime dependencies.

## Next Steps

1. Study the generated code to understand parser structure
2. Modify examples to add new features
3. Create your own domain-specific language
4. Integrate generated parsers into your projects

## Resources

- [User Guide](USER_GUIDE.md) - Complete usage guide
- [Grammar Syntax Reference](GRAMMAR_SYNTAX.md) - Detailed syntax
- [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md) - Feature support
- [API Documentation](API.md) - Library API reference

---

**Last Updated**: October 25, 2025  
**Examples**: 16 grammars covering beginner to advanced topics  
**Status**: All examples validated and tested
