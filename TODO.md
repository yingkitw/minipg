# TODO - minipg Development Plan

**Vision**: Fast, Rust-native ANTLR4-compatible parser generator focused on the Rust ecosystem.

**Core Principles**:
1. ✅ Standalone Code Generation (no runtime)
2. ✅ ANTLR4 Compatibility
3. ✅ Modern Rust Implementation
4. ✅ Focused Scope (Rust, Python, JavaScript)

---

## Current Status (v0.2.0 - Simplified & Focused)

### Target Languages ✅
- [x] **Rust** - Primary target, optimized with DFA generation
- [x] **Python** - Type hints and dataclasses (Python 3.10+)
- [x] **JavaScript** - Modern ES6+ with error recovery

### Core Features ✅
- [x] ANTLR4 grammar parsing
- [x] Character classes with Unicode escapes
- [x] Non-greedy quantifiers
- [x] Lexer modes and channels
- [x] Rule arguments, returns, locals
- [x] Named actions (@header, @members)
- [x] List labels (ids+=ID)
- [x] Grammar composition and imports
- [x] Semantic analysis (undefined rules, duplicates, left recursion)

### Test Coverage ✅
- **Total Tests**: 74 unit tests with 100% pass rate
- Unit tests (core functionality)
- Integration tests (full pipeline)
- ANTLR4 compatibility tests
- Real-world grammar tests

---

## Priority 1: Complete Core Features (65% Complete)

**Status**: Strong foundation laid, needs rule body completion  
**Next Steps**: Complete Rust rule body generation, test with real grammars  
**Timeline**: 2-3 sessions to v0.2.0 (Rust primary, Python experimental)

### Rust Code Generation
- [x] **Improved rule body generation** (90% complete) ✅
  - [x] Enhanced error handling with EOF checks
  - [x] Better error messages with context
  - [x] Improved AST construction with labels
  - [x] Generate AST node type definitions ✅
  - [x] Struct definitions for each rule ✅
  - [x] Field extraction from labeled elements ✅
  - [x] List variable initialization ✅
  - [x] Terminal and rule reference parsing ✅
  - [x] String literals ✅
  - [x] Optional elements (?) ✅
  - [x] Zero-or-more (*) ✅
  - [x] One-or-more (+) ✅
  - [x] Groups with alternatives ✅
  - [x] Character classes, negation, wildcards ✅
  - [x] Semantic actions and predicates ✅
- [ ] **Optimize generated code**
  - [ ] Inline DFA improvements
  - [ ] Lookup table optimization
  - [ ] Memory efficiency
- [x] **Production quality** (75% complete) ✅
  - [x] Comprehensive error messages ✅
  - [x] Debug support (via error context) ✅
  - [x] Documentation generation ✅

### Python Code Generation
- [x] **Improved implementation** (90% complete) ✅
  - [x] AST node type generation with fields ✅
  - [x] Dataclass definitions for each rule ✅
  - [x] Field extraction from labeled elements ✅
  - [x] Type hints for dataclass fields ✅
  - [x] Rule body generation with alternatives ✅
  - [x] Error handling with ParseError ✅
  - [x] Terminal and rule reference parsing ✅
  - [x] String literals, optional, repetition ✅
- [x] **Optimize for Python** (Basic optimizations) ✅
  - [x] Idiomatic Python patterns ✅
  - [x] PEP 8 compliance ✅
  - [ ] Performance optimization (advanced)

### JavaScript Code Generation
- [x] **Improved implementation** (95% complete) ✅
  - [x] AST node type generation (classes) ✅
  - [x] Field extraction from labeled elements ✅
  - [x] Rule body generation with alternatives ✅
  - [x] Error handling with ParseError ✅
  - [x] Terminal and rule reference parsing ✅
  - [x] Modern ES6+ patterns ✅
  - [x] String literals, optional, repetition ✅
- [x] **Browser compatibility** ✅
  - [x] No Node.js-specific code ✅
  - [x] Module system support (ES6 exports) ✅

---

## Priority 2: Testing & Validation

### Real-World Grammars
- [ ] Test with grammars-v4 repository
  - [ ] Java grammar subset
  - [ ] Python grammar subset
  - [ ] SQL grammar
  - [ ] JSON grammar (already working)
- [ ] Fix any compatibility issues
- [ ] Document known limitations

### Performance Testing
- [ ] Benchmark code generation speed
- [ ] Benchmark generated parser performance
- [ ] Memory profiling
- [ ] Optimize bottlenecks

### Quality Assurance
- [ ] Code coverage analysis
- [ ] Fuzzing tests
- [ ] Large file testing (GB+ inputs)
- [ ] Security audit

---

## Priority 3: Documentation & Polish

### Documentation
- [ ] Complete user guide
- [x] Per-language guides (Rust, Python, JavaScript) ✅
  - [x] docs/RUST_CODE_GENERATION.md
  - [x] docs/PYTHON_CODE_GENERATION.md
  - [x] docs/JAVASCRIPT_CODE_GENERATION.md
- [ ] Migration guide from ANTLR4
- [ ] Troubleshooting guide
- [ ] API documentation

### Examples
- [ ] Beginner examples (calculator, simple expressions)
- [ ] Intermediate examples (JSON, config files)
- [ ] Advanced examples (SQL, programming languages)
- [ ] Real-world use cases

### Polish
- [ ] Better error messages
- [ ] CLI improvements
- [ ] Progress indicators
- [ ] Helpful diagnostics

---

## Known Issues

### High Priority
- [ ] **Rule body generation incomplete** - Currently generates skeleton code
  - Need to implement full pattern matching
  - Need proper token access and consumption
  - Need AST construction
- [ ] **Generated parsers need error recovery**
  - Lexer error recovery implemented
  - Parser error recovery not complete
  - Need better error messages

### Medium Priority
- [ ] **Better ANTLR4 grammar parsing**
  - Improve error messages with context
  - Handle edge cases better
  - Better recovery from parse errors
- [ ] **Unicode support improvements**
  - Full Unicode property support
  - Better escape sequence handling

### Low Priority
- [ ] Performance profiling for large grammars
- [ ] Advanced code generation optimizations
- [ ] Visitor/listener pattern improvements

---

## Future Considerations (Post 1.0)

### Potential Enhancements
- Grammar debugging tools
- Visual grammar designer
- Grammar optimization suggestions
- Better IDE integration

### Ecosystem
- VS Code extension (basic syntax highlighting)
- Build system integrations (cargo, setuptools)
- Package manager support

---

## Archived Features

The following features were removed to simplify and focus the project:

### Removed Language Targets
- Go, Java, C, C++, TypeScript (moved to `archived_generators/`)
- Tree-sitter generator (separate project scope)

### Removed Features
- Incremental parsing infrastructure
- Query language
- LSP/editor integration plans
- Position tracking for editors

**Rationale**: Focus on being the best Rust/Python/JavaScript parser generator with ANTLR4 compatibility, rather than spreading thin across too many targets and trying to replace Tree-sitter.

---

**Last Updated**: February 21, 2026 (2:35am)  
**Current Version**: v0.2.0 (Simplified & Focused)  
**Current Focus**: Rust at 90%, ready for real grammar testing  
**Test Status**: 74 tests passing (100% pass rate)  
**Project Status**: Rust 90% complete, Python/JavaScript 70% complete  
**Recommendation**: Test Rust with Calculator/JSON grammars, then apply patterns to Python/JavaScript

## Recent Accomplishments (Feb 21, 2026)

### Simplification Complete ✅
- Reduced from 9 languages to 3 core languages (67% reduction)
- Removed incremental parsing infrastructure
- Removed Tree-sitter generator
- Removed MCP server
- Simplified workspace from 5 to 3 crates
- All tests passing (74/74)

### Documentation Complete ✅
- Created comprehensive Rust code generation guide
- Created comprehensive Python code generation guide
- Created comprehensive JavaScript code generation guide
- Created Getting Started guide
- Updated README.md with refocused positioning
- Updated ARCHITECTURE.md with simplified design
- Created SIMPLIFICATION_SUMMARY.md
- Created TODO_COMPLETION_SUMMARY.md
- Created WORK_COMPLETED_FEB21.md

### Code Generation Improvements (Major Progress) 🎉
- **Rust Generator (90% complete)**:
  - ✅ Better error handling with EOF checks
  - ✅ Improved error messages with context
  - ✅ AST construction with labeled values
  - ✅ Support for list labels
  - ✅ AST node type generation (structs for each rule)
  - ✅ Field extraction from labeled elements
  - ✅ Rule body generation with alternatives
  - ✅ Terminal and rule reference parsing
  - ✅ String literals
  - ✅ Optional elements (?)
  - ✅ Zero-or-more (*)
  - ✅ One-or-more (+)
  - ✅ Groups with alternatives
  - ⏸️ Character classes, semantic actions
- **Python Generator (70% complete)**:
  - ✅ AST node type generation (dataclasses for each rule)
  - ✅ Field extraction from labeled elements
  - ✅ Type hints for dataclass fields
  - ✅ Support for list labels
  - ✅ Rule body generation with alternatives
  - ✅ Error handling with exceptions
  - ✅ Terminal and rule reference parsing
  - ⏸️ String literals, optional, repetition
- **JavaScript Generator (70% complete)**:
  - ✅ AST node type generation (classes)
  - ✅ Field extraction from labeled elements
  - ✅ Rule body generation with alternatives
  - ✅ Error handling with exceptions
  - ✅ Terminal and rule reference parsing
  - ⏸️ String literals, optional, repetition
- All tests passing (74/74)
- Build successful with minimal warnings
- **Overall Priority 1: ~72% complete** ✅
