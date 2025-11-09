# Implementation Guide

This document consolidates implementation details for various features in minipg.

## Table of Contents

1. [Action Translation](#action-translation)
2. [Rule Features](#rule-features)
3. [Lexer Modes & Channels](#lexer-modes--channels)
4. [Grammar Composition](#grammar-composition)
5. [Code Generators](#code-generators)
   - [Go](#go-code-generator)
   - [C & C++](#c--c-code-generators)
   - [Java](#java-code-generator)
6. [ANTLR4 Compatibility](#antlr4-compatibility)

---

## Action Translation

### Overview

Action translation enables embedded actions and semantic predicates in grammar rules to be translated to target languages.

### Implementation

**Module**: `src/codegen/actions.rs`

**Key Functions**:
- `translate_action_element()` - Translates action/predicate elements
- `generate_predicate_for_language()` - Generates semantic predicates
- `generate_action_for_language()` - Generates action code wrappers

**Supported Languages**: Rust, Python, JavaScript, TypeScript, Go

**Translation Patterns**:
- Rust → Python: `Vec::new()` → `[]`, `.push()` → `.append()`
- Rust → JavaScript: `self.` → `this.`, `.len()` → `.length`
- Rust → Go: `self.` → `l.`, `Vec::new()` → `make([]Token, 0)`

See `docs/archive/sessions/ACTION_TRANSLATION_IMPLEMENTATION.md` for details.

---

## Rule Features

### Overview

Rule features allow grammar rules to be parameterized with arguments, return values, and local variables, following ANTLR4 conventions.

### Implementation

**AST Support**: Already exists in `src/ast/rule.rs`
- `RuleArg` - Rule arguments
- `RuleReturn` - Return values
- `RuleLocal` - Local variables

**Parser Support**: `src/parser/parser.rs`
- `parse_rule_arguments()` - Parses `rule[Type name]`
- `parse_rule_returns()` - Parses `returns [Type name]`
- `parse_rule_locals()` - Parses `locals [Type name]`

**Code Generation**: All 8 languages support rule features

**Example**:
```antlr
rule[int x, String name] returns [Value result]
locals [int temp]
    : ID { temp = x + 1; }
    ;
```

See `docs/archive/sessions/RULE_FEATURES_IMPLEMENTATION.md` for details.

---

## Lexer Modes & Channels

### Overview

Lexer modes enable context-sensitive lexing for complex grammars. Channels allow tokens to be routed to different processing streams.

### Implementation

**Mode Declaration**: `mode NAME;`
- Parsed in `src/parser/parser.rs`
- Stored in `Grammar.lexer_modes` HashMap

**Lexer Commands**:
- `-> skip` - Skip token
- `-> channel(NAME)` - Route to channel
- `-> mode(NAME)` - Switch mode
- `-> pushMode(NAME)` - Push mode onto stack
- `-> popMode` - Pop mode from stack
- `-> more` - Continue matching
- `-> type(TYPE)` - Set token type

**Code Generation**: `src/codegen/modes.rs`
- Mode stack management for all languages
- Channel routing for all languages

**Example**:
```antlr
mode DEFAULT_MODE;
QUOTE: '"' -> mode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["];
END_QUOTE: '"' -> popMode;
```

See `docs/archive/sessions/LEXER_MODES_IMPLEMENTATION.md` for details.

---

## Grammar Composition

### Overview

Grammar composition enables modular grammar design through imports and rule resolution.

### Implementation

**GrammarComposer**: `src/analysis/composition.rs`

**Features**:
- Recursive import resolution
- Grammar caching
- Configurable search paths
- Circular import detection
- Rule conflict detection

**Syntax**:
```antlr
grammar MyGrammar;
import Base;
import Common;
```

**Usage**:
```rust
let mut composer = GrammarComposer::new();
composer.add_search_path("grammars/");
composer.resolve_imports(&mut grammar)?;
```

See `docs/archive/sessions/GRAMMAR_COMPOSITION_IMPLEMENTATION.md` for details.

---

## Code Generators

### Go Code Generator

**Module**: `src/codegen/go.rs`

**Features**:
- Idiomatic Go patterns (error interface, receivers, constructors)
- Package structure with imports
- Token types with String() method
- ParseError implementing error interface
- Lexer with NextToken() and TokenizeAll()
- Parser with rule parsing methods

**Example Output**:
```go
package mygrammar

type TokenKind int

const (
    TokenID TokenKind = iota
    TokenNUMBER
)

type Token struct {
    Kind     TokenKind
    Text     string
    Position Position
}
```

See `docs/archive/sessions/GO_CODEGEN_IMPLEMENTATION.md` for details.

### C & C++ Code Generators

**Modules**: `src/codegen/c.rs`, `src/codegen/cpp.rs`

**C Features**:
- Header (.h) and source (.c) files
- Manual memory management helpers
- Token enumeration and structures
- Lexer and parser implementations

**C++ Features**:
- Modern C++17+ (RAII, smart pointers)
- std::unique_ptr for memory management
- Exception-based error handling
- Namespace support

**Example C Output**:
```c
typedef enum {
    TOKEN_ID = 0,
    TOKEN_NUMBER = 1
} TokenType;

typedef struct {
    TokenType type;
    char *text;
    int line;
    int column;
} Token;
```

See `docs/archive/sessions/C_CPP_CODEGEN_IMPLEMENTATION.md` for details.

### Java Code Generator

**Module**: `src/codegen/java.rs`

**Features**:
- Proper package structure (grammar.lexer, grammar.parser)
- Token enum and Token class
- Lexer and Parser classes
- ParseException for error handling
- Mode support

**Example Output**:
```java
package mygrammar.lexer;

public enum TokenType {
    ID, NUMBER, EOF
}

public class Token {
    public TokenType type;
    public String text;
    public int line;
    public int column;
}
```

See `docs/archive/sessions/JAVA_CODEGEN_IMPLEMENTATION.md` for details.

---

## ANTLR4 Compatibility

### Overview

minipg aims for high compatibility with ANTLR4 grammar syntax and features.

### Supported Features

✅ **Grammar Structure**
- Grammar types (lexer, parser, combined)
- Grammar options
- Grammar imports
- Named actions (@header, @members)

✅ **Rule Features**
- Rule arguments
- Return values
- Local variables
- Element labels (id=ID)
- List labels (ids+=ID)

✅ **Lexer Features**
- Character classes with Unicode escapes
- Non-greedy quantifiers (.*?, .+?, .??)
- Lexer commands (skip, channel, mode, etc.)
- Lexer modes
- Fragment tokens

✅ **Parser Features**
- Alternatives
- Quantifiers (*, +, ?)
- Groups
- Actions and semantic predicates

### Test Coverage

- 20+ ANTLR4 compatibility tests
- 14 grammars-v4 repository tests
- 22 ANTLR4 test suite patterns
- Real-world grammar testing (Java, Python, SQL, GraphQL)

See `docs/ANTLR4_COMPATIBILITY.md` for full details.

---

## Adding New Features

### Guidelines

1. **AST First**: Add AST support in `src/ast/`
2. **Parser Support**: Add parsing logic in `src/parser/`
3. **Analysis**: Add semantic analysis in `src/analysis/`
4. **Code Generation**: Add code generation in `src/codegen/`
5. **Tests**: Add comprehensive tests
6. **Documentation**: Update this guide

### Code Generator Checklist

When adding a new code generator:

- [ ] Implement `CodeGenerator` trait
- [ ] Generate token types
- [ ] Generate error types
- [ ] Generate lexer implementation
- [ ] Generate parser implementation
- [ ] Support mode/channel handling
- [ ] Support action translation
- [ ] Add tests
- [ ] Add language guide

---

**Last Updated**: 2025-01-XX
**Status**: Consolidated from individual implementation guides

