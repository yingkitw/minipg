# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
  - 32 tests passing (9 ignored due to alpha limitations)
  - Unit, integration, and end-to-end tests
  - Grammar validation tests
  - Error recovery tests
  - Cross-language consistency tests
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
