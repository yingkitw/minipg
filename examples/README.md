# minipg Examples

This directory contains example grammars demonstrating various features of minipg.

## Available Examples

### Simple Examples

#### calculator.g4
A simple arithmetic expression calculator supporting:
- Addition and subtraction
- Multiplication and division
- Parentheses for grouping
- Integer numbers

#### json.g4
A basic JSON parser supporting:
- Strings
- Numbers
- Booleans (true/false)
- Null values

### Complex Examples (ANTLR4 Compatible)

#### CompleteJSON.g4
Complete JSON grammar based on RFC 8259:
- Objects with key-value pairs
- Arrays
- Strings with escape sequences and Unicode
- Numbers (integers, floats, scientific notation)
- Booleans and null
- Proper whitespace handling

#### SQL.g4
SQL query language subset:
- SELECT statements with WHERE, ORDER BY, LIMIT
- INSERT statements
- UPDATE statements
- DELETE statements
- Operators and conditions
- Comments (line and block)

#### JavaSubset.g4
Simplified Java language grammar:
- Package and import declarations
- Class and interface declarations
- Fields, methods, and constructors
- Statements (if, while, for, return, etc.)
- Expressions with operators
- Literals and identifiers
- Comments

#### PythonSubset.g4
Simplified Python 3 grammar:
- Import statements
- Function and class definitions
- Control flow (if, while, for, try)
- Expressions and operators
- List, dict, and tuple literals
- String literals (including triple-quoted)
- Indentation handling (simplified)

### Modern Language Examples

#### GraphQL.g4
GraphQL Schema Definition Language:
- Schema definitions with operation types
- Type definitions (scalar, object, interface, union, enum, input)
- Field definitions with arguments
- Directives and directive locations
- Complex type system with generics
- Demonstrates: complex grammar with multiple rule types

#### Expression.g4
Expression language with operator precedence:
- Logical operators (||, &&)
- Comparison operators (==, !=, <, >, <=, >=)
- Arithmetic operators (+, -, *, /, %)
- Unary operators (!, -, +)
- Function calls and array/object access
- Array and object literals
- Demonstrates: operator precedence, function calls, variables

#### Query.g4
SQL-like query language:
- SELECT with joins, WHERE, GROUP BY, ORDER BY
- INSERT, UPDATE, DELETE statements
- CREATE TABLE with column definitions
- Subqueries and complex expressions
- Demonstrates: complex statements, joins, aggregations

#### CSS.g4
CSS stylesheet language:
- Selectors (element, class, ID, universal)
- Pseudo-classes and pseudo-elements
- Declarations and properties
- Media queries and keyframes
- Font-face definitions
- Demonstrates: selector syntax, nested rules, media queries

#### YAML.g4
YAML configuration language:
- Mappings (key-value pairs)
- Sequences (lists)
- Multiple scalar types (strings, numbers, booleans)
- Nested structures
- Demonstrates: indentation-sensitive parsing, multiple string types

#### Markdown.g4
Markdown document format:
- Headings, paragraphs, code blocks
- Lists and blockquotes
- Inline formatting (bold, italic, code)
- Links and images
- Demonstrates: text processing, special characters, nested structures

#### Config.g4
Configuration file format (INI-like):
- Section headers
- Key-value pairs
- Multiple value types (strings, numbers, booleans, arrays)
- Comments
- Demonstrates: sections, key-value pairs, various value types

#### Protocol.g4
Protocol Buffer-like definition language:
- Message definitions with fields
- Enum definitions
- Service definitions with RPC methods
- Nested messages and enums
- Field options and modifiers
- Demonstrates: message definitions, nested structures, options

## Running Examples

1. Generate the parser:
   ```bash
   minipg generate <example>.g4 -o output/
   ```

2. The generated code will be in `output/<example>_parser.rs`

3. Use the generated parser in your Rust project

## Learning Path

### Beginner
1. `calculator.g4` - Basic grammar structure with operators
2. `json.g4` - Handling multiple value types
3. `Expression.g4` - Operator precedence and function calls

### Intermediate
4. `CompleteJSON.g4` - Full RFC-compliant JSON with escape sequences
5. `Config.g4` - Configuration files with sections
6. `YAML.g4` - Nested structures and multiple formats

### Advanced
7. `SQL.g4` - Complex statements with joins and subqueries
8. `CSS.g4` - Selectors, pseudo-classes, and media queries
9. `GraphQL.g4` - Type systems and complex definitions
10. `Protocol.g4` - Message definitions and nested structures
11. `Markdown.g4` - Text processing with inline formatting
12. `Query.g4` - Advanced query language with aggregations

### Real-World
- `JavaSubset.g4` - Programming language subset
- `PythonSubset.g4` - Another programming language
- Create your own domain-specific language!

## See Also

- [User Guide](../docs/USER_GUIDE.md)
- [Grammar Syntax Reference](../docs/GRAMMAR_SYNTAX.md)
- [API Documentation](../docs/API.md)
