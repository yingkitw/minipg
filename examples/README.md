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

## Running Examples

1. Generate the parser:
   ```bash
   minipg generate <example>.g4 -o output/
   ```

2. The generated code will be in `output/<example>_parser.rs`

3. Use the generated parser in your Rust project

## Learning Path

1. Start with `calculator.g4` - Basic grammar structure
2. Try `json.g4` - More complex value types
3. Create your own grammar!

## See Also

- [User Guide](../docs/USER_GUIDE.md)
- [Grammar Syntax Reference](../docs/GRAMMAR_SYNTAX.md)
- [API Documentation](../docs/API.md)
