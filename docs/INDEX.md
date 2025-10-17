# Documentation Index

## Quick Start

- **[README.md](../README.md)** - Project overview and quick start
- **[USER_GUIDE.md](USER_GUIDE.md)** - Comprehensive user guide
- **[PROJECT_STRUCTURE.md](../PROJECT_STRUCTURE.md)** - Project organization

## Guides

- **[GRAMMAR_SYNTAX.md](GRAMMAR_SYNTAX.md)** - ANTLR4 grammar syntax
- **[API.md](API.md)** - API documentation
- **[ANTLR4_COMPATIBILITY.md](ANTLR4_COMPATIBILITY.md)** - ANTLR4 compatibility
- **[COMPARISON_SUMMARY.md](COMPARISON_SUMMARY.md)** - vs. other tools

## Examples

### Beginner
- **[01-BASIC-CALCULATOR.md](examples/01-BASIC-CALCULATOR.md)** - Simple arithmetic calculator
  - Basic parser and lexer rules
  - Operator precedence
  - ~20 lines of grammar

### Intermediate
- **[02-JSON-PARSER.md](examples/02-JSON-PARSER.md)** - Complete JSON parser
  - Recursive structures
  - Fragment rules
  - Unicode support
  - ~80 lines of grammar

### Advanced
- **[05-SQL-PARSER.md](examples/05-SQL-PARSER.md)** - SQL query parser
  - Multiple statement types
  - Optional clauses
  - Case-insensitive keywords
  - ~140 lines of grammar

## Reports

### Performance
- **[PERFORMANCE_REPORT.md](reports/PERFORMANCE_REPORT.md)** - Benchmark results
  - Sub-millisecond generation
  - Linear scaling O(n)
  - ~2 µs per rule

### Optimization
- **[RUST_OPTIMIZATION_REPORT.md](reports/RUST_OPTIMIZATION_REPORT.md)** - Rust optimizations
  - Inline attributes
  - DFA generation
  - Lookup tables

### Testing
- **[TEST_COVERAGE_REPORT.md](reports/TEST_COVERAGE_REPORT.md)** - Test coverage
  - 101 tests (100% passing)
  - Unit, integration, and benchmark tests

### Features
- **[ANTLR4_ACTIONS_SUPPORT.md](reports/ANTLR4_ACTIONS_SUPPORT.md)** - Actions and predicates
  - Embedded actions `{...}`
  - Semantic predicates `{...}?`
  - Language-specific actions

- **[GRAMMAR_VALIDATION.md](reports/GRAMMAR_VALIDATION.md)** - Grammar validation
  - CompleteJSON.g4 ✅
  - SQL.g4 ✅
  - All 6 example grammars validated

## Development

- **[TODO.md](../TODO.md)** - Development roadmap
- **[ROADMAP.md](../ROADMAP.md)** - Feature roadmap
- **[ARCHITECTURE.md](../ARCHITECTURE.md)** - System architecture
- **[DECISIONS.md](../DECISIONS.md)** - Design decisions

## Session Reports

- **[COMPLETE_FINAL_SUMMARY.md](reports/COMPLETE_FINAL_SUMMARY.md)** - Final session summary
  - 13 features completed
  - 101 tests passing
  - 4 languages supported

## By Topic

### Getting Started
1. [README.md](../README.md) - Start here
2. [USER_GUIDE.md](USER_GUIDE.md) - Detailed guide
3. [01-BASIC-CALCULATOR.md](examples/01-BASIC-CALCULATOR.md) - First example

### Grammar Writing
1. [GRAMMAR_SYNTAX.md](GRAMMAR_SYNTAX.md) - Syntax reference
2. [ANTLR4_COMPATIBILITY.md](ANTLR4_COMPATIBILITY.md) - ANTLR4 features
3. [Examples](examples/) - Real-world grammars

### Code Generation
1. [API.md](API.md) - API reference
2. [Language-specific guides](#) - Per-language docs
3. [RUST_OPTIMIZATION_REPORT.md](reports/RUST_OPTIMIZATION_REPORT.md) - Optimizations

### Performance
1. [PERFORMANCE_REPORT.md](reports/PERFORMANCE_REPORT.md) - Benchmarks
2. [Comparison](COMPARISON_SUMMARY.md) - vs. other tools

### Advanced Features
1. [ANTLR4_ACTIONS_SUPPORT.md](reports/ANTLR4_ACTIONS_SUPPORT.md) - Actions
2. [Error Recovery](#) - Error handling
3. [Custom Generators](#) - Extending minipg

## By Language

### Rust
- [Rust Code Generation](#)
- [Rust Optimization](reports/RUST_OPTIMIZATION_REPORT.md)
- [Rust Examples](#)

### Python
- [Python Code Generation](#)
- [Python Examples](#)

### JavaScript
- [JavaScript Code Generation](#)
- [JavaScript Examples](#)

### TypeScript
- [TypeScript Code Generation](#)
- [TypeScript Examples](#)

## Statistics

- **Documentation Files**: 30+
- **Example Grammars**: 6
- **Test Files**: 15+
- **Benchmark Suites**: 6
- **Lines of Documentation**: ~10,000+

## Contributing

- [CONTRIBUTING.md](#) - Contribution guidelines
- [CODE_OF_CONDUCT.md](#) - Code of conduct
- [DEVELOPMENT.md](#) - Development setup

---

**Last Updated**: 2025-10-17  
**Version**: 0.1.0  
**Status**: Complete ✅
