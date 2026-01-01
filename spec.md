# minipg Project Specification

## Project Overview

**Name**: minipg (Mini Parser Generator)  
**Version**: 0.1.5 (Production Ready)  
**Language**: Rust (Edition 2024)  
**License**: Apache-2.0  
**Status**: Production Ready - Editor Integration Foundation Complete

## Vision

A modern, multi-language parser generator with **incremental parsing** that generates standalone, optimized parsers from ANTLR4-compatible grammars with zero runtime dependencies. **Replacing Tree-sitter** by providing a unified solution where one ANTLR4 grammar serves both runtime parsing and editor integration.

**Key Innovation**: Incremental parsing with position tracking enables fast re-parsing for editor integration, making minipg suitable for both runtime and real-time editor use cases.

## Core Principles

1. **Incremental Parsing** - Fast re-parsing with position tracking (PRIMARY FEATURE)
2. **Editor Integration** - Complete infrastructure for replacing Tree-sitter
3. **Query Language** - Tree-sitter-compatible pattern matching for syntax highlighting
4. **Multi-Language Support** - 9 target languages (8 runtime + Tree-sitter)
5. **ANTLR4 Compatibility** - Works with existing grammars
6. **Standalone Code Generation** - No runtime dependencies
7. **Modern Architecture** - Rust 2024, trait-based design
8. **Performance** - Sub-millisecond generation, <10ms incremental edits
9. **Test-Friendly** - 147 tests with 100% pass rate
10. **DRY, KISS, SoC** - Clean, maintainable codebase

## Target Languages

### Runtime Parsers (8 languages)
1. **Rust** - Optimized with inline DFA generation
2. **Python** - Type hints and dataclasses (Python 3.10+)
3. **JavaScript** - Modern ES6+ with error recovery
4. **TypeScript** - Full type safety with interfaces and enums
5. **Go** - Idiomatic Go with interfaces and error handling
6. **Java** - Standalone .java files with proper package structure
7. **C** - Standalone .c/.h files with manual memory management
8. **C++** - Modern C++17+ with RAII and smart pointers

### Editor Integration (1 language)
9. **Tree-sitter** - Grammar.js for editor syntax highlighting and analysis

## Features

### Incremental Parsing & Editor Integration (v0.1.5)

**PRIMARY FEATURE**: Fast, position-aware parsing for editor integration

#### Position Tracking
- ✅ Byte offset tracking for every AST node
- ✅ Line and column tracking
- ✅ Range calculations (start/end positions)
- ✅ Point advancement for multiline text

#### Edit Tracking
- ✅ Insert operations with automatic position updates
- ✅ Delete operations with range handling
- ✅ Replace operations (delete + insert)
- ✅ Edit application to syntax trees

#### Incremental Re-parsing
- ✅ IncrementalParser trait for extensibility
- ✅ SyntaxTree with position information
- ✅ Foundation for subtree reuse (optimization pending)
- ✅ Target: <10ms for typical edits

#### Query Language
- ✅ S-expression syntax (Tree-sitter compatible)
- ✅ Pattern matching: `(node) @capture`
- ✅ Field matching: `(parent field: (child))`
- ✅ Wildcard patterns: `(_)`
- ✅ Capture groups with position tracking
- ✅ Multiple patterns per query
- ✅ Comment support

#### Editor Foundation
- ✅ Complete infrastructure to replace Tree-sitter
- ✅ Query-based syntax highlighting
- ✅ Example highlight queries (highlights.scm)
- ✅ 34 tests (18 incremental + 16 query)

### Grammar Support

#### ANTLR4 Compatibility
- ✅ Lexer and parser grammars
- ✅ Combined grammars
- ✅ Grammar imports and composition
- ✅ Grammar options (language, tokenVocab, superClass, etc.)
- ✅ Fragment tokens
- ✅ Token specifications

#### Rules
- ✅ Parser rules
- ✅ Lexer rules
- ✅ Rule arguments: `rule[Type name]`
- ✅ Return values: `returns [Type name]`
- ✅ Local variables: `locals [Type name]`
- ✅ Rule alternatives
- ✅ Element labels: `id=ID`
- ✅ Alternative labels: `expr # add`
- ✅ List labels: `ids+=ID`

#### Elements
- ✅ String literals: `'keyword'`
- ✅ Character classes: `[a-z0-9]`
- ✅ Negated classes: `~["\r\n]`
- ✅ Unicode escapes: `\u0000-\uFFFF`
- ✅ Quantifiers: `*`, `+`, `?`
- ✅ Non-greedy quantifiers: `.*?`, `.+?`, `.??`
- ✅ Groups: `(a | b)`
- ✅ Wildcards: `.`
- ✅ Rule references
- ✅ Token references

#### Actions & Predicates
- ✅ Embedded actions: `{code}`
- ✅ Semantic predicates: `{condition}?`
- ✅ Named actions: `@header`, `@members`, `@lexer::*`, `@parser::*`
- ✅ Action translation for target languages

#### Lexer Features
- ✅ Lexer commands: `-> skip`, `-> channel(NAME)`, `-> mode(NAME)`
- ✅ Lexer modes with mode stack (push/pop)
- ✅ Channels for token routing
- ✅ Fragment tokens
- ✅ Character class optimization

### Code Generation

#### Common Features (All Languages)
- ✅ Standalone parsers (no runtime dependencies)
- ✅ Error recovery and reporting
- ✅ Token type definitions
- ✅ Lexer implementation
- ✅ Parser implementation
- ✅ Error types (ParseError)
- ✅ Documentation comments
- ✅ Named action insertion
- ✅ Lexer mode stack management
- ✅ Channel routing
- ✅ Action code generation

#### Language-Specific Features

**Rust**:
- Inline DFA generation
- `#[inline]` attributes for performance
- `Debug` derives
- Idiomatic error handling with `Result`

**Python**:
- Type hints (Python 3.10+)
- Dataclasses for AST nodes
- PEP 8 compliant
- Exception-based error handling

**JavaScript**:
- ES6+ classes
- Modern syntax (const/let)
- JSDoc documentation
- Works in Node.js and browsers

**TypeScript**:
- Full type safety
- Interfaces and enums
- Type guards
- Strict mode compatible

**Go**:
- Idiomatic Go style
- Interface-based design
- Error interface implementation
- Receiver methods

**Java**:
- Proper package structure
- Exception handling
- JavaDoc comments
- Standalone .java files

**C**:
- Manual memory management
- Header/source separation
- Safe memory helpers
- Struct-based design

**C++**:
- Modern C++17+
- RAII principles
- Smart pointers (std::unique_ptr)
- Exception handling
- Namespace support

**Tree-sitter**:
- Grammar.js generation
- npm package.json
- README.md documentation
- Editor integration support
- Smart case conversion

### Testing

#### Test Coverage
- **Total Tests**: 193+ tests
- **Pass Rate**: 100%
- **Coverage Types**:
  - Unit tests (core functionality)
  - Integration tests (full pipeline)
  - Analysis tests (semantic analysis)
  - Codegen tests (all languages)
  - Compatibility tests (ANTLR4 features)
  - Feature tests (advanced features)
  - Example tests (real-world grammars)

#### Test Strategy
- Property-based testing with proptest
- Fuzzing tests for robustness
- Snapshot testing with insta
- E2E tests from grammar to parser
- Real-world grammar validation

### Performance

#### Benchmarks
- **Code Generation**: Sub-millisecond for typical grammars
- **Memory Usage**: <100 KB
- **Scaling**: Linear O(n) with grammar complexity
- **Build Time**: Fast compilation with Rust

#### Optimizations
- Inline DFA generation (Rust)
- Const lookup tables for character classes
- Optimized tokenization
- Minimal allocations

### CLI

#### Commands
```bash
# Generate parser
minipg generate <grammar.g4> -o <output> -l <language>

# Validate grammar
minipg validate <grammar.g4>

# Show grammar info
minipg info <grammar.g4>
```

#### Options
- `-o, --output`: Output directory
- `-l, --target-language`: Target language (rust, python, js, ts, go, java, c, cpp, treesitter)
- `-p, --package`: Package name
- `--visitor`: Generate visitor pattern
- `--listener`: Generate listener pattern

### MCP Server

Model Context Protocol server for AI integration:
- **Tools**:
  - `parse_grammar`: Parse grammar text into AST
  - `generate_parser`: Generate parser code
  - `validate_grammar`: Validate and get diagnostics
- Enables AI assistants to interact with minipg programmatically

## Architecture

### Module Structure

```
minipg/
├── src/
│   ├── core/           # Core traits, types, errors
│   ├── ast/            # Abstract Syntax Tree
│   ├── parser/         # Grammar file parsing
│   ├── analysis/       # Semantic analysis
│   ├── codegen/        # Code generation
│   │   ├── rust.rs
│   │   ├── python.rs
│   │   ├── javascript.rs
│   │   ├── typescript.rs
│   │   ├── go.rs
│   │   ├── java.rs
│   │   ├── c.rs
│   │   ├── cpp.rs
│   │   ├── treesitter.rs  # NEW
│   │   └── registry.rs
│   ├── cli/            # Command-line interface
│   └── mcp/            # MCP server
├── tests/              # Integration tests
├── examples/           # Example grammars
└── docs/               # Documentation
```

### Design Patterns

- **Trait-based abstraction**: Core capabilities as traits
- **Visitor pattern**: AST traversal
- **Registry pattern**: Language generator registration
- **Builder pattern**: DFA and lookup table construction
- **Strategy pattern**: Language-specific code generation

### Data Flow

```
Grammar File (.g4)
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST (Grammar)
    ↓
[Semantic Analyzer] → Validated AST + Diagnostics
    ↓
[Code Generator] → Source Code (target language)
    ↓
Output Files
```

## Dependencies

### Runtime Dependencies
- `serde` - Serialization
- `serde_json` - JSON support
- `thiserror` - Error handling
- `clap` - CLI argument parsing
- `tracing` - Logging
- `rmcp` - MCP server

### Development Dependencies
- `insta` - Snapshot testing
- `proptest` - Property-based testing
- `criterion` - Benchmarking
- `tempfile` - Temporary files for tests

## Quality Standards

### Code Quality
- ✅ No warnings in production code
- ✅ Clippy clean
- ✅ Rustfmt formatted
- ✅ Documentation for public APIs
- ✅ Comprehensive error messages

### Testing Requirements
- ✅ 100% test pass rate
- ✅ All features tested
- ✅ Real-world grammar validation
- ✅ Property-based tests for robustness
- ✅ Fuzzing tests for edge cases

### Documentation Requirements
- ✅ README with examples
- ✅ Architecture documentation
- ✅ Per-language guides
- ✅ API documentation
- ✅ Example grammars

## Release Process

### Version Numbering
- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes

### Release Checklist
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Run full test suite
4. Update documentation
5. Build and test locally
6. Publish to crates.io
7. Create GitHub release
8. Update README badges

## Future Enhancements

### Editor Integration (Replace Tree-sitter)

**Primary Goal**: Enable minipg parsers to completely replace Tree-sitter for editor integration

#### Phase 1: Incremental Parsing (v0.1.5) ✅ COMPLETE
- [x] Position-aware AST with byte offsets and line/column tracking ✅
- [x] Edit tracking and incremental re-parsing ✅
- [x] IncrementalParser trait and implementation ✅
- [x] SyntaxTree with position information ✅
- [x] 18 comprehensive tests ✅
- [ ] Subtree reuse for unchanged regions (optimization pending)
- [ ] Performance target: <10ms for typical edits (optimization pending)

#### Phase 2: Query Language (v0.1.5) ✅ COMPLETE
- [x] S-expression query syntax (Tree-sitter compatible) ✅
- [x] Pattern matching engine for syntax highlighting ✅
- [x] Capture groups for extracting nodes ✅
- [x] Query files (highlights.scm, etc.) ✅
- [x] Field matching (field: syntax) ✅
- [x] Wildcard patterns (_) ✅
- [x] 16 comprehensive tests ✅

#### Phase 3: Language Server Protocol (v0.1.6)
- [ ] LSP server implementation (`minipg-lsp`)
- [ ] Semantic tokens (syntax highlighting)
- [ ] Folding ranges (code folding)
- [ ] Document symbols (outline)
- [ ] Diagnostics (errors/warnings)
- [ ] Navigation (go to definition, find references)

#### Phase 4: Editor Extensions (v0.1.6)
- [ ] VS Code extension with LSP client
- [ ] Neovim plugin with Tree-sitter-compatible API
- [ ] Emacs mode with lsp-mode integration
- [ ] Multi-editor testing and optimization

**Benefits**:
- ✅ Single ANTLR4 grammar for runtime AND editor
- ✅ No grammar conversion needed
- ✅ Access to 1000+ existing ANTLR4 grammars
- ✅ Consistent parsing everywhere
- ✅ Full ANTLR4 features (actions, modes, etc.)

**See**: `docs/EDITOR_INTEGRATION_STRATEGY.md` for complete roadmap

### Other Planned Features
- [ ] Visitor/listener generation for all languages
- [ ] Tree construction and AST generation
- [ ] Error recovery improvements
- [ ] Performance optimizations
- [ ] More target languages (Swift, Kotlin, etc.)
- [ ] Grammar debugging tools
- [ ] Online playground

### Research Areas
- [ ] Novel parsing algorithms
- [ ] Grammar optimization techniques
- [ ] Automatic grammar inference
- [ ] Machine learning integration

## Success Metrics

### Technical Metrics
- **Test Coverage**: 100% pass rate maintained
- **Performance**: Sub-millisecond generation
- **Memory**: <100 KB usage
- **Build Time**: <30 seconds clean build

### User Metrics
- **Downloads**: Track crates.io downloads
- **Issues**: Response time <48 hours
- **Documentation**: Complete and up-to-date
- **Examples**: 20+ real-world grammars

## Maintenance

### Regular Tasks
- Update dependencies monthly
- Review and merge PRs weekly
- Triage issues weekly
- Update documentation as needed
- Run security audits quarterly

### Long-term Goals
- Maintain 100% test pass rate
- Keep build warnings at zero
- Improve performance continuously
- Expand language support
- Build community around project

## License

Apache-2.0

## Contact

- **Repository**: https://github.com/yingkitw/minipg
- **Issues**: https://github.com/yingkitw/minipg/issues
- **Crates.io**: https://crates.io/crates/minipg
- **Documentation**: https://docs.rs/minipg

---

**Last Updated**: January 1, 2026  
**Version**: 0.1.5 (Development)  
**Status**: Production Ready with Tree-sitter Support
