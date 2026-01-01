# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5] - 2026-01-01

### Added - Editor Integration Foundation

#### Phase 1: Incremental Parsing
- **Position Tracking**: Point, Position, Range structs for byte offset and line/column tracking
- **Edit Tracking**: Edit struct with insert, delete, replace operations
- **IncrementalParser**: Trait and default implementation for incremental re-parsing
- **SyntaxTree**: Parsed tree with position information
- **18 new tests**: All passing (100% pass rate)

#### Phase 2: Query Language
- **S-expression Syntax**: Tree-sitter-compatible query language
- **Query Parser**: Full parser for `(node) @capture` syntax
- **Pattern Matching**: QueryMatcher for finding patterns in AST
- **Capture Groups**: Named captures with position and text
- **Field Matching**: Support for `(parent field: (child))` syntax
- **Wildcard Patterns**: `(_)` matches any node
- **16 new tests**: All passing (100% pass rate)

#### Tree-sitter Code Generator
- **Grammar.js Generation**: Convert ANTLR4 to Tree-sitter format
- **Package.json**: Complete npm package configuration
- **README.md**: Documentation for generated grammars
- **Smart Case Conversion**: PascalCase → snake_case/kebab-case
- **7 new tests**: All passing (100% pass rate)

### Documentation
- `PHASE1_INCREMENTAL_PARSING.md`: Complete Phase 1 implementation guide
- `PHASE2_QUERY_LANGUAGE.md`: Complete Phase 2 implementation guide
- `TREESITTER_IMPLEMENTATION.md`: Tree-sitter generator documentation
- `docs/TREESITTER_GUIDE.md`: User guide for Tree-sitter generation
- `docs/EDITOR_INTEGRATION_STRATEGY.md`: Complete roadmap for replacing Tree-sitter
- `REPLACING_TREESITTER.md`: Vision and migration strategy
- `queries/highlights.scm`: Example syntax highlighting query
- `RELEASE_v0.1.5.md`: Comprehensive release notes

### Changed
- Updated to support **9 target languages** (added Tree-sitter)
- Total tests increased to **147** (from 106)
- Updated README, TODO, spec.md, ARCHITECTURE.md with new features
- Consolidated all phases to v0.1.5

### Fixed
- Error handling in query parser (use Error::parse instead of ParseError)
- RuleType import in query matcher (moved from core::types to ast)
- Case conversion for acronyms (JSONParser → json_parser, not j_s_o_n_parser)
- Field parsing in query patterns (attach field to child node)

### Performance
- All 147 tests pass in <1 second
- Sub-millisecond grammar parsing maintained
- <100 KB memory usage maintained

### Vision
This release lays the foundation for **completely replacing Tree-sitter** with minipg parsers for editor integration, providing a unified solution where one ANTLR4 grammar serves both runtime parsing and editor tooling.

**Next Steps**: Optimize incremental parsing, implement LSP server (Phase 3)

## [0.1.0-alpha.3] - 2025-10-17

### Major Features

#### Lexer State Machine for Character Classes ✅
- **CharClass Mode**: Context-aware lexer mode for proper `[...]` tokenization
- **Automatic Mode Entry**: Triggers after `:`, `|`, `~`, `(`, `]`, `?`, `*`, `+`
- **Escape Sequences**: Full support for `\\`, `\/`, `\n`, `\r`, `\t`, `\u0000-\uFFFF`
- **Special Character Handling**: `/`, `+`, and other chars treated as literals in CharClass mode
- **Comment Preservation**: Skips whitespace but preserves `/` for character matching

#### Non-Greedy Quantifiers ✅
- **Parsing Support**: `.*?`, `.+?`, `.??` syntax fully supported
- **AST Representation**: Added `greedy: bool` field to quantifier variants
- **Helper Methods**: `zero_or_more_non_greedy()`, `one_or_more_non_greedy()`, `optional_non_greedy()`
- **SQL.g4 Support**: All SQL grammar tests now passing

#### Lexer Commands ✅
- **AST Support**: New `LexerCommand` enum with Skip, Channel, Mode, Type, PushMode, PopMode, More
- **Parser Integration**: Properly parses `-> skip`, `-> channel(NAME)`, etc.
- **Storage**: Commands stored in `Alternative.lexer_command` field

### Test Suite Improvements
- **99 tests passing** (up from 65 in alpha.1)
- **0 tests ignored** (down from 9 in alpha.1)
- **CompleteJSON.g4**: All 5 integration tests passing
- **SQL.g4**: All 4 integration tests passing
- **New test coverage**: Character classes, Unicode escapes, non-greedy quantifiers

### Bug Fixes
- Fixed character class parsing for complex patterns like `["\\\u0000-\u001F]`
- Fixed tokenization of special characters in character classes
- Fixed comment handling in character class mode
- Fixed CharClass mode entry/exit logic
- Updated all pattern matches to handle new `greedy` field

### Changed
- **Element enum**: Added `greedy` field to `Optional`, `ZeroOrMore`, `OneOrMore` variants
- **Alternative struct**: Added `lexer_command` field
- **Visitor patterns**: Updated to handle new fields with `..` patterns

### Performance
- No performance regression
- All benchmarks still passing
- Code generation remains sub-millisecond

### Documentation
- Created CHANGELOG_v0.1.0-alpha.2.md with detailed changes
- Updated TODO.md with progress and new test counts
- Updated README.md (pending)

## [0.1.0-alpha.1] - 2025-10-17

### Project Structure
- **Consolidated to single crate**: Simplified from multi-crate workspace to single package for easier publishing and installation
- **Modular organization**: Core, AST, Parser, Analysis, and Codegen modules within single crate

### Added
- Initial release of minipg parser generator
- **Multi-language code generation**:
  - Rust code generator with optimizations
  - Python code generator with type hints
  - JavaScript code generator (ES6+)
  - TypeScript code generator with full type safety
- **Core features**:
  - ANTLR4-compatible grammar parsing
  - Inline DFA generation for efficient tokenization
  - Const lookup tables for O(1) character classification
  - Comprehensive error recovery with position tracking
  - Element and alternative label support
  - Embedded actions and semantic predicates
- **Performance**:
  - Sub-millisecond code generation for typical grammars
  - Linear O(n) scaling with grammar complexity
  - Low memory footprint (<100 KB)
  - Comprehensive benchmarks with criterion
- **Testing**:
  - 51 tests passing (9 ignored due to alpha limitations)
  - Unit, integration, and end-to-end tests
  - Grammar validation tests
  - Error recovery tests
  - Cross-language consistency tests
  - Comprehensive coverage of core types, tokens, and AST
- **Documentation**:
  - Getting started tutorial
  - Migration guide from ANTLR4
  - API documentation
  - Example documentation (3 difficulty levels)
  - Performance reports
  - Architecture documentation
- **CI/CD**:
  - GitHub Actions workflows
  - Multi-platform builds (Linux, macOS, Windows)
  - Automated testing and linting
  - Code coverage reporting
  - Release automation
- **Examples**:
  - Calculator grammar (beginner)
  - JSON parser (intermediate)
  - Complete JSON (RFC 8259)
  - SQL parser (advanced)
  - Java subset
  - Python subset

### Performance
- Code generation: 1.87-21.6 µs depending on language
- Memory usage: <100 KB for large grammars
- Scaling: ~2 µs per rule (linear)

### Compatibility
- Rust: 1.85+
- Python: 3.10+
- Node.js: 14+
- TypeScript: 4.5+

## [0.0.1] - 2025-10-01

### Added
- Project initialization
- Basic project structure
- Core trait definitions

---

[Unreleased]: https://github.com/yingkitw/minipg/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yingkitw/minipg/releases/tag/v0.1.0
[0.0.1]: https://github.com/yingkitw/minipg/releases/tag/v0.0.1
