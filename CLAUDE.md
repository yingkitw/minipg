# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

minipg is a blazingly fast parser generator written in Rust (edition 2024) with ANTLR4 compatibility and incremental parsing capabilities. It generates standalone parsers for 9 target languages without runtime dependencies, and provides infrastructure to replace Tree-sitter in editor tooling.

**Current Version**: 0.1.5 (Production Ready)

## Development Commands

### Build and Test
```bash
cargo build                          # Build the project
cargo build --release               # Build optimized release
cargo test --all                    # Run all tests (147+ tests, 100% pass rate)
cargo test --release                # Run tests in release mode
cargo bench                         # Run benchmarks
cargo install --path .              # Install locally
```

### Run Specific Tests
```bash
cargo test <test_name>              # Run specific test
cargo test --package minipg <module>::tests  # Run module tests
cargo test -- --nocapture           # Show print output
```

### Enable Logging
```bash
RUST_LOG=info cargo run -- generate grammar.g4
RUST_LOG=debug cargo test           # Debug logs during tests
```

### CLI Usage
```bash
# Generate parsers (9 target languages)
minipg generate grammar.g4 -o output/ -l rust
minipg generate grammar.g4 -o output/ -l python
minipg generate grammar.g4 -o output/ -l javascript
minipg generate grammar.g4 -o output/ -l typescript
minipg generate grammar.g4 -o output/ -l go
minipg generate grammar.g4 -o output/ -l java
minipg generate grammar.g4 -o output/ -l c
minipg generate grammar.g4 -o output/ -l cpp
minipg generate grammar.g4 -o output/ -l treesitter

# Validate and inspect
minipg validate grammar.g4
minipg info grammar.g4
```

## Architecture

minipg follows a **modular single-crate architecture** with clear separation of concerns through a pipeline model:

### Core Modules

- **core** - Foundation: Error types, diagnostics, traits (`GrammarParser`, `SemanticAnalyzer`, `CodeGenerator`), common types
- **ast** - Abstract Syntax Tree: Grammar, Rule, Element, Alternative definitions with visitor pattern
- **parser** - Grammar file parser: Lexer with context-aware modes, Parser with 40+ token types
- **analysis** - Semantic analysis: Undefined/duplicate rule detection, left recursion detection, reachability analysis
- **codegen** - Code generation for 9 target languages with language-specific generators, template engine, DFA builder
- **cli** - Command-line interface: generate, validate, info commands
- **incremental** - NEW in v0.1.5: Position tracking, edit tracking, incremental re-parsing for editor integration
- **query** - NEW in v0.1.5: Tree-sitter-compatible S-expression queries, pattern matching with capture groups
- **mcp** - Model Context Protocol server for AI assistant integration

### Processing Pipelines

**Traditional Pipeline (Code Generation)**:
```
Grammar File → [Lexer] → Tokens → [Parser] → AST → [Semantic Analysis] → Validated AST → [Code Generator] → Output Files
```

**Incremental Pipeline (Editor Integration)**:
```
Source Code → [IncrementalParser] → SyntaxTree (with positions) → [Edit Applied] → Updated SyntaxTree → [QueryMatcher] → Pattern Matches
```

### Key Architectural Patterns

1. **Trait-Based Design** - Core capabilities defined as traits (`GrammarParser`, `SemanticAnalyzer`, `CodeGenerator`, `IncrementalParser`)
2. **Visitor Pattern** - AST traversal through `AstVisitor` and `AstVisitorMut` for flexible tree walking
3. **Pipeline Processing** - Two main pipelines: traditional (code generation) and incremental (editor integration)
4. **Error Handling** - Unified error types with rich diagnostics using `thiserror` and `anyhow`
5. **Test-Friendly** - All components designed for isolated testing with comprehensive test coverage

### Multi-Language Support (9 Languages)

All code generators support:
- Parameterized rules (arguments, returns, locals)
- Named actions (`@header`, `@members`)
- List labels (`ids+=ID`)
- Non-greedy quantifiers, character classes with Unicode
- Lexer modes & channels, actions

Language-specific generators in `codegen/`:
- **Rust** - Optimized with inline DFA generation
- **Python** - Type hints and dataclasses (3.10+)
- **JavaScript** - Modern ES6+ with error recovery
- **TypeScript** - Full type safety with interfaces
- **Go** - Idiomatic with interfaces
- **Java** - Proper package structure
- **C** - Manual memory management
- **C++** - Modern C++17+ with RAII
- **Tree-sitter** - grammar.js for editor integration

### ANTLR4 Compatibility Highlights

Supported features (comprehensive):
- Advanced character classes with Unicode escapes (`\u0000-\uFFFF`)
- Non-greedy quantifiers (`.*?`, `.+?`, `.??`)
- Lexer modes and channels with push/pop operations
- Rule arguments, returns, and local variables
- Named actions (`@header`, `@members`) with code generation
- List labels (`ids+=ID`)
- Grammar imports and composition
- Lexer commands (`-> skip`, `-> channel(NAME)`, `-> mode(NAME)`)

## Testing Strategy

Test suite with **147+ tests at 100% pass rate**:

- **106 unit tests** - Core functionality and parsing
- **19 integration tests** - Full pipeline (parse → analyze → generate)
- **21 analysis tests** - Semantic analysis, ambiguity detection
- **21 codegen tests** - Multi-language code generation
- **19 compatibility tests** - ANTLR4 feature compatibility
- **13 feature tests** - Advanced grammar features
- **9 example tests** - Real-world grammar examples
- **18 incremental parsing tests** - Position tracking, edit tracking
- **16 query language tests** - Pattern matching

When adding features:
1. Write unit tests for isolated components
2. Add integration tests for full pipeline validation
3. Test code generation for all 9 target languages
4. Verify ANTLR4 compatibility with real-world grammars

## Important Notes

### Code Conventions
- Prefix unused variables with underscore (e.g., `_unused_var`) to suppress Rust warnings
- Use `thiserror` for error types with proper context
- Use `anyhow` for application-level error handling
- Leverage serde for serialization (AST is serializable)

### Module Organization
- Single crate with modular `src/` structure
- Each module has focused responsibility
- Traits defined in `core/` for extensibility
- Language-specific code in `codegen/<language>/`

### Incremental Parsing (v0.1.5+)
- Position tracking: `Point` (byte offset, line, column), `Range` (start, end)
- Edit tracking: Insert, delete, replace operations with point advancement
- Target: <10ms incremental re-parsing for editor use cases

### Query Language (v0.1.5+)
- Tree-sitter-compatible S-expression syntax
- Pattern matching with capture groups (`@name`)
- Supports node types, field matching, wildcards

### Example Grammars
19+ example grammars in `examples/`:
- Simple: calculator, JSON
- Intermediate: Expression, Config, YAML
- Advanced: GraphQL, SQL, Java, Python

Use these as reference when testing new features.

## Documentation Files

- `README.md` - Project overview and comparison with ANTLR4/Pest
- `ARCHITECTURE.md` - Detailed system architecture
- `CHANGELOG.md` - Version history
- `TODO.md` - Development roadmap
- `docs/USER_GUIDE.md` - Complete user guide
- `docs/GRAMMAR_SYNTAX.md` - Grammar syntax reference
- `docs/API.md` - Library API reference
- `docs/ANTLR4_COMPATIBILITY.md` - ANTLR4 compatibility guide

## Performance Targets

- Code generation: **sub-millisecond** for typical grammars
- Incremental re-parsing: **<10ms** for typical edits
- Memory usage: **<100 KB** for code generation
