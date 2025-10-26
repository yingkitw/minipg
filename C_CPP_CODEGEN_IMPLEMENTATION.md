# C & C++ Code Generator Implementation

## Overview

Implemented C and C++ code generators for minipg, enabling parser generation for systems programming and modern C++ applications.

## What Was Implemented

### 1. C Code Generator ✅

**Features**:
- Header file generation (.h)
- Source file generation (.c)
- Token type enumeration
- Token structure with metadata
- Lexer implementation with mode support
- Parser implementation with error handling
- Memory management helpers (safe_malloc, safe_strdup)
- Manual memory cleanup functions

**Key Components**:

```c
// Token types enum
typedef enum {
    TOKEN_ID = 0,
    TOKEN_NUMBER = 1,
    TOKEN_EOF = 999,
    TOKEN_ERROR = 1000
} TokenType;

// Token structure
typedef struct {
    TokenType type;
    char *text;
    int line;
    int column;
    int length;
} Token;

// Lexer structure
typedef struct {
    const char *input;
    size_t position;
    size_t length;
    int line;
    int column;
    int mode;
} Lexer;

// Parser structure
typedef struct {
    Lexer *lexer;
    Token current_token;
    Token peek_token;
} Parser;

// Error structure
typedef struct {
    int code;
    char *message;
    int line;
    int column;
} ParseError;
```

**Memory Management**:
```c
static void* safe_malloc(size_t size);
static char* safe_strdup(const char *str);
static void free_token(Token *token);
```

**API Functions**:
```c
Lexer* lexer_new(const char *input);
void lexer_free(Lexer *lexer);
Token lexer_next_token(Lexer *lexer);
Token lexer_peek_token(Lexer *lexer);

Parser* parser_new(Lexer *lexer);
void parser_free(Parser *parser);
ParseError* parser_parse(Parser *parser);

// Rule-specific parsers
ParseError* parse_<rule_name>(Parser *parser);
```

### 2. C++ Code Generator ✅

**Features**:
- Header file generation (.hpp)
- Source file generation (.cpp)
- Modern C++ (C++17+)
- RAII principles with smart pointers
- Exception-based error handling
- Namespace support
- Class-based design
- STL containers

**Key Components**:

```cpp
namespace Grammar {

// Token type enum
enum class TokenType {
    Id,
    Number,
    Eof,
    Error
};

// Token class
class Token {
public:
    TokenType type;
    std::string text;
    int line;
    int column;
    int length;
    
    Token(TokenType t, const std::string& txt, int l, int c, int len);
};

// Lexer class
class Lexer {
private:
    std::string input;
    size_t position;
    int line;
    int column;
    int mode;
    
public:
    Lexer(const std::string& src);
    Token next_token();
    Token peek_token();
};

// Parser exception
class ParseError : public std::runtime_error {
public:
    int code;
    int line;
    int column;
    
    ParseError(int c, const std::string& msg, int l, int col);
};

// Parser class
class Parser {
private:
    std::unique_ptr<Lexer> lexer;
    Token current_token;
    Token peek_token;
    
    void advance();
    
public:
    Parser(std::unique_ptr<Lexer> lex);
    void parse();
    
    // Rule-specific parsers
    void parse_<rule_name>();
};

} // namespace
```

**Modern C++ Features**:
- `std::unique_ptr` for automatic memory management
- `std::string` for string handling
- `std::vector` for collections
- Exception handling with `std::runtime_error`
- RAII (Resource Acquisition Is Initialization)
- Namespaces for organization

## Grammar Examples

### C Example

```antlr
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

Generated C code structure:
```c
// calculator.h
#ifndef CALCULATOR_H
#define CALCULATOR_H

typedef enum {
    TOKEN_NUMBER = 0,
    TOKEN_EOF = 999,
    TOKEN_ERROR = 1000
} TokenType;

typedef struct { /* ... */ } Token;
typedef struct { /* ... */ } Lexer;
typedef struct { /* ... */ } Parser;

Lexer* lexer_new(const char *input);
Parser* parser_new(Lexer *lexer);
ParseError* parse_expr(Parser *parser);

#endif

// calculator.c
#include "calculator.h"

Lexer* lexer_new(const char *input) { /* ... */ }
Token lexer_next_token(Lexer *lexer) { /* ... */ }
Parser* parser_new(Lexer *lexer) { /* ... */ }
ParseError* parse_expr(Parser *parser) { /* ... */ }
```

### C++ Example

```antlr
grammar JsonParser;

json: value EOF;
value: object | array | STRING | NUMBER | 'true' | 'false' | 'null';
object: '{' (pair (',' pair)*)? '}';
pair: STRING ':' value;
array: '[' (value (',' value)*)? ']';

STRING: '"' (~["\\\r\n] | '\\' .)* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
WS: [ \t\r\n]+ -> skip;
```

Generated C++ code structure:
```cpp
// json_parser.hpp
#ifndef JSON_PARSER_HPP
#define JSON_PARSER_HPP

namespace JsonParser {

enum class TokenType {
    String,
    Number,
    Eof,
    Error
};

class Token {
public:
    TokenType type;
    std::string text;
    // ...
};

class Lexer {
public:
    Lexer(const std::string& src);
    Token next_token();
    // ...
};

class Parser {
public:
    Parser(std::unique_ptr<Lexer> lex);
    void parse();
    void parse_json();
    void parse_value();
    // ...
};

} // namespace

#endif

// json_parser.cpp
#include "json_parser.hpp"

namespace JsonParser {

Lexer::Lexer(const std::string& src) { /* ... */ }
Token Lexer::next_token() { /* ... */ }
Parser::Parser(std::unique_ptr<Lexer> lex) { /* ... */ }
void Parser::parse() { /* ... */ }
void Parser::parse_json() { /* ... */ }

} // namespace
```

## Files Created

### Source Code
- `src/codegen/c.rs` - C code generator (350+ lines)
- `src/codegen/cpp.rs` - C++ code generator (350+ lines)

### Tests
- Built-in tests in both generators (7 tests each)

### Documentation
- `C_CPP_CODEGEN_IMPLEMENTATION.md` - This file

## Test Coverage

### C Code Generator Tests (7 tests)
- `test_c_codegen_basic` - Basic code generation
- `test_c_codegen_token_types` - Token type enumeration
- `test_c_codegen_structures` - Structure definitions
- `test_c_codegen_functions` - Function declarations
- `test_c_codegen_memory_helpers` - Memory management
- `test_c_codegen_target_language` - Target language identification
- `test_c_codegen_with_completejson` - CompleteJSON.g4 support

### C++ Code Generator Tests (7 tests)
- `test_cpp_codegen_basic` - Basic code generation
- `test_cpp_codegen_classes` - Class definitions
- `test_cpp_codegen_namespace` - Namespace support
- `test_cpp_codegen_methods` - Method declarations
- `test_cpp_codegen_smart_pointers` - Smart pointer usage
- `test_cpp_codegen_target_language` - Target language identification
- `test_cpp_codegen_with_completejson` - CompleteJSON.g4 support

**Test Results**: ✅ All 14 tests passing

## Features Verified

### C Code Generator ✅
- Header file generation (.h)
- Source file generation (.c)
- Token type enumeration
- Structure definitions
- Function declarations and implementations
- Memory management helpers
- Lexer implementation
- Parser implementation
- Error handling
- Mode support

### C++ Code Generator ✅
- Header file generation (.hpp)
- Source file generation (.cpp)
- Enum class definitions
- Class definitions with methods
- Smart pointer usage (std::unique_ptr)
- Exception handling (std::runtime_error)
- Namespace support
- STL containers (std::string, std::vector)
- RAII principles
- Modern C++ (C++17+)

## Architecture

### C Code Generator
- Procedural design with struct-based state
- Manual memory management
- C89/C99 compatible
- Portable across platforms
- Minimal dependencies

### C++ Code Generator
- Object-oriented design with classes
- Automatic memory management with smart pointers
- C++17 features
- STL integration
- Exception-based error handling

## Integration

Both generators are integrated into the main code generator dispatcher:

```rust
match target_language {
    "c" => CCodeGenerator::new().generate(...),
    "cpp" | "c++" => CppCodeGenerator::new().generate(...),
    // ...
}
```

## Status

✅ **Complete** - C and C++ code generators fully implemented and tested

### What Was Implemented
- C code generator with manual memory management
- C++ code generator with modern C++ features
- Comprehensive test suites (14 tests total)
- Support for all grammar features
- CompleteJSON.g4 verification

### Features Verified
✅ Header and source file generation
✅ Token type enumeration
✅ Structure/class definitions
✅ Function/method declarations
✅ Memory management (C) and RAII (C++)
✅ Error handling
✅ Mode support
✅ All grammar rule types

## Next Steps

1. **Grammar Composition** - Support for grammar imports
2. **VS Code Extension** - Syntax highlighting and validation
3. **Java Generator** - Add Java code generation
4. **Beta Release** - Prepare v0.1.0-beta.1

## References

- [src/codegen/c.rs](src/codegen/c.rs) - C code generator implementation
- [src/codegen/cpp.rs](src/codegen/cpp.rs) - C++ code generator implementation
- [src/codegen/mod.rs](src/codegen/mod.rs) - Code generator dispatcher
