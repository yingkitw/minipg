# TODO - minipg Development Plan

**Vision**: Modern, multi-language parser generator generating standalone, optimized code from ANTLR4-compatible grammars.

**Core Principles**:
1. ✅ Standalone Code Generation (no runtime)
2. ✅ Multi-Language Support (8 languages)
3. ✅ ANTLR4 Compatibility
4. ✅ Modern Architecture (Rust 2024)

---

## Completed ✅

### Foundation
- [x] Create workspace structure with modular crates
- [x] Consolidate to single-crate structure (removed 7,560 lines of duplicate code)
- [x] Implement core traits and error handling
- [x] Implement AST definitions with visitor pattern
- [x] Implement lexer for grammar files
- [x] Implement parser for grammar files
- [x] Implement semantic analysis
- [x] Implement basic Rust code generator
- [x] Implement CLI with clap
- [x] Add insta snapshot tests
- [x] Fix parser implementation issues
- [x] Ensure cargo build succeeds
- [x] Ensure cargo test succeeds (65 tests passing: 43 unit + 11 E2E + 11 integration)
- [x] Add comprehensive E2E tests (grammar → parser → code generation)

### Documentation
- [x] README, ARCHITECTURE, ROADMAP, DECISIONS
- [x] User Guide, Grammar Syntax Reference, API docs
- [x] ANTLR4 Compatibility plan
- [x] Multi-Language plan
- [x] Runtime Decision analysis
- [x] Comparison with antlr4rust
- [x] KNOWN_LIMITATIONS.md documenting current alpha limitations
- [x] NEXT_PHASE.md with detailed implementation plan

### Examples
- [x] Simple examples (Calculator, JSON)
- [x] Complex examples (CompleteJSON, SQL, JavaSubset, PythonSubset)

### Testing & Quality (October 2025)
- [x] **Documentation Consolidation** ✅
  - [x] Archived 15 temporary/session files to `docs/archive/`
  - [x] Created `docs/archive/INDEX.md` for historical reference
  - [x] Reduced root directory from 30+ to 14 markdown files (43% reduction)
  - [x] Updated README with archive link
  - [x] Created `docs/README.md` (documentation index)
  - [x] Created `FOLDER_STRUCTURE.md` (project organization guide)
- [x] **Property-Based Testing** ✅
  - [x] Created `tests/property_based_tests.rs` (15 tests)
  - [x] Covers grammar parsing, character classes, quantifiers, comments, etc.
  - [x] Uses proptest for random valid grammar generation
- [x] **Fuzzing Tests** ✅
  - [x] Created `tests/fuzzing_tests.rs` (18 tests)
  - [x] Tests parser robustness against malformed input
  - [x] Covers edge cases: deeply nested structures, long identifiers, etc.
  - [x] Verifies no panics on arbitrary input
- [x] **Example Grammar Tests** ✅
  - [x] Created `tests/test_all_examples.rs` (17 tests for basic examples)
  - [x] Created `tests/test_advanced_examples.rs` (48 tests for advanced examples)
  - [x] Total tests: 165 → 213 (48 new tests, 29% increase)
  - [x] 100% pass rate maintained
  - [x] All 16 example grammars covered
- [x] **Example Grammar Fixes** ✅
  - [x] Fixed Config.g4 (removed EOF reference, added token skipping)
  - [x] Fixed CompleteJSON.g4 (removed EOF reference)
  - [x] Both grammars now validate successfully
  - [x] Code generation working for all languages
- [x] **Test Coverage** ✅
  - [x] Total tests: 115+ → 213 (85% increase)
  - [x] 100% pass rate maintained
  - [x] Created `IMPROVEMENTS_SUMMARY.md` documenting changes
  - [x] Created `EXAMPLE_PROCESSING_TEST.md` (processing test results)
  - [x] Created `EXAMPLE_FIXES_APPLIED.md` (grammar fixes)
  - [x] Created `ADVANCED_EXAMPLES_TEST_REPORT.md` (advanced test coverage)

---

## Month 1 (Current) - Rust Optimization & Foundation

### Priority 1: Rust Code Generation Optimization 🔥
- [x] **Implement inline DFA generation** ✅
  - [x] Generate DFA as match statements at compile time
  - [x] Optimize state transitions
  - [x] Add tests for DFA generation
  - [x] Created `dfa.rs` module with DfaBuilder
  - [x] Integrated DFA into Rust code generator
  - [x] All 68 tests passing
- [x] **Add const lookup tables** ✅
  - [x] Character class lookup tables (256-byte ASCII table)
  - [x] Token type tables (token name to TokenKind)
  - [x] Optimize for common patterns
  - [x] Created `lookup_table.rs` module with LookupTableBuilder
  - [x] Generated inline helper functions (match_char_fast, is_in_range)
  - [x] All 72 tests passing (added 4 new tests)
- [x] **Improve generated code quality** ✅
  - [x] Make code readable with comments
  - [x] Follow Rust idioms (Result, Option, iterators)
  - [x] Add inline documentation
  - [x] Generated lexer now has proper documentation
- [x] **Implement error recovery in generated code** ✅
  - [x] ParseError type with context (message, position, expected, found)
  - [x] Result<Token, ParseError> for error handling
  - [x] tokenize_all() method for error collection
  - [x] Error recovery: skip invalid characters and continue
  - [x] Position tracking in tokens and errors
  - [x] Display and Error trait implementations
  - [x] 4 comprehensive error recovery tests

### Priority 2: Testing & Validation
- [x] Test generated code compiles ✅
- [x] Test generated parsers work correctly ✅
- [x] Add end-to-end tests with complex grammars ✅
- [x] Added 24 new comprehensive tests ✅
- [x] Cross-language testing (Rust, Python, JavaScript) ✅
- [x] Error recovery testing (4 comprehensive tests) ✅
- [x] Validate against CompleteJSON.g4 ✅
- [x] Validate against SQL.g4 ✅
- [x] All 4 languages generate code successfully ✅
- [x] Created GRAMMAR_VALIDATION.md report ✅

### Priority 3: CI/CD Setup
- [x] Set up GitHub Actions ✅
- [x] Add cargo build workflow ✅
- [x] Add cargo test workflow ✅
- [x] Add cargo clippy workflow ✅
- [x] Add cargo fmt check ✅
- [x] Add release workflow ✅
- [x] Add documentation workflow ✅
- [x] Add coverage workflow ✅

---

## Current Status (v0.1.0-alpha.4 Ready) 🎯

### Recent Accomplishments (October 25, 2025)
- [x] **Project cleanup** - Reduced root directory by 43% ✅
- [x] **213 total tests** (up from 165, 29% increase) ✅
- [x] **Advanced example tests** - 48 new tests for 8 advanced grammars ✅
- [x] **Example grammar fixes** - Config.g4 and CompleteJSON.g4 now valid ✅
- [x] **Comprehensive documentation** - Test reports and guides created ✅
- [x] **Published to crates.io** as v0.1.0-alpha.3 ✅
- [x] **Consolidated to single crate** (removed 7,560 lines) ✅
- [x] **5 target languages** (Rust, Python, JS, TS, Go) ✅
- [x] **E2E test suite** covering full pipeline ✅
- [x] **CharClass AST support** with negation ✅
- [x] **CompleteJSON.g4 fully supported** - all 5 tests passing ✅
- [x] **SQL.g4 fully supported** - all 4 tests passing ✅
- [x] **Lexer state machine** for character classes ✅
- [x] **Non-greedy quantifiers** (`.*?`, `.+?`, `.??`) ✅
- [x] **Lexer commands** parsed and stored in AST ✅
- [x] **Go code generator** - 5th target language ✅
- [x] **List labels** fully implemented (`ids+=ID`) ✅
- [x] **Named actions** parsing (`@header`, `@members`) ✅
- [x] **Named actions code generation** (ALL 5 languages!) ✅

### Completed for v0.1.0-alpha.2
- [x] **Lexer state machine** for character class tokenization ✅
  - [x] Enter CharClass mode after `:`, `|`, `~`, `(`, `]`, `?`, `*`, `+`
  - [x] Handle escape sequences in character classes (`\\`, `\u0000`, etc.)
  - [x] Treat special characters as literals in CharClass mode (`/`, `+`, etc.)
  - [x] Skip whitespace but not comments in CharClass mode
- [x] Fix all 9 ignored integration tests ✅
  - [x] CompleteJSON.g4 - all 5 tests passing ✅
  - [x] SQL.g4 - all 4 tests passing ✅
- [x] Add support for lexer commands (`-> skip`, `-> channel(...)`) ✅
- [x] Implement non-greedy quantifiers (`.*?`, `.+?`, `.??`) ✅
- [x] Implement list labels (`ids+=ID`) ✅
- [x] Implement named actions (`@header`, `@members`) ✅
- [x] Prepare v0.1.0-alpha.3 release ✅
- [x] Published v0.1.0-alpha.3 to crates.io ✅

### Next Priorities (See NEXT_PHASE.md for details)
1. **Phase 1**: Advanced ANTLR4 Syntax ✅ (Mostly Complete)
   - [x] Character class improvements (lexer state machine) ✅
   - [x] Rule arguments & return values ✅
   - [x] Non-greedy quantifiers ✅
   - [x] Grammar imports ✅
   - [x] Grammar options ✅
   - [x] List labels (fully implemented) ✅
   - [x] Named actions (@header, @members) ✅
   - [ ] Lexer modes & channels (code generation)
   - [ ] Action code generation
2. **Phase 2**: Go Target Language ✅ (COMPLETE!)
   - [x] Create Go code generator ✅
   - [x] Idiomatic Go code ✅
   - [x] Support parameterized rules ✅
   - [x] Tests passing ✅
3. **Phase 3**: Code Generation Enhancements ✅ (COMPLETE!)
   - [x] Named actions code generation (insert into generated code) ✅
   - [ ] Lexer mode code generation (future)
   - [ ] Action translation for target languages (future)
4. **Phase 4**: C Target Language (Future)
5. **Phase 5**: C++ Target Language (Future)

---

## Month 2 - Multi-Language Foundation

### Rust Target Completion
- [x] Complete standalone Rust generation ✅
- [x] Optimize for performance ✅
- [x] Add inline attributes (#[inline], #[inline(always)]) ✅
- [x] Add Debug derives ✅
- [x] Pass all integration tests ✅
- [ ] Zero-copy parsing with lifetimes (future enhancement)

### Python Target (Priority 1)
- [x] Create PythonCodeGenerator trait implementation ✅
- [x] Generate standalone .py files ✅
- [x] Type hints (Python 3.10+) ✅
- [x] Dataclasses for AST nodes and error types ✅
- [x] PEP 8 compliant code ✅
- [x] Error recovery with ParseError exception ✅
- [x] tokenize_all() method for error collection ✅
- [x] Test with CompleteJSON.g4 ✅

### JavaScript Target (Priority 1)
- [x] Create JavaScriptCodeGenerator trait implementation ✅
- [x] Generate standalone .js files (ES6+) ✅
- [x] Modern JavaScript (classes, const/let) ✅
- [x] Works in Node.js and browsers ✅
- [x] Error recovery with ParseError class ✅
- [x] tokenizeAll() method for error collection ✅
- [x] JSDoc documentation ✅
- [x] Test with CompleteJSON.g4 ✅

### ANTLR4 Parser Enhancement
- [x] Parse element labels: `left=expr` ✅
- [x] Parse alternative labels: `expr # add` ✅
- [x] Update AST to support labels ✅
- [x] Element.with_label() method ✅
- [x] Alternative.with_label() method ✅
- [x] Parse rule arguments: `rule[int x, String name]` ✅
- [x] Parse return values: `returns [Value result]` ✅
- [x] Parse local variables: `locals [int temp]` ✅
- [x] Parse list labels: `ids+=ID` ✅
- [x] Parse named actions: `@header {...}` ✅

### CI/CD & Publishing
- [x] Add badges to README ✅
- [x] Create release workflow ✅
- [x] Create CHANGELOG.md ✅
- [x] Add crate metadata ✅
- [x] Add pig mascot to README ✅
- [x] Update version to 0.1.0-alpha.1 ✅
- [x] Add LICENSE file (Apache-2.0) ✅
- [x] Create PUBLISHING.md guide ✅
- [ ] Publish to crates.io (ready, awaiting publication)
- [ ] Set up docs.rs (automatic after publication)

---

## Month 3 - TypeScript & ANTLR4 Actions

### TypeScript Target (Priority 1)
- [x] Create TypeScriptCodeGenerator trait implementation ✅
- [x] Generate standalone .ts files ✅
- [x] Full type safety with interfaces and enums ✅
- [x] Error recovery with typed ParseError class ✅
- [x] tokenizeAll() with typed return values ✅
- [x] Export all public types ✅
- [x] Private/public modifiers ✅
- [x] Test with CompleteJSON.g4 ✅

### Go Target (Priority 1) ✅ COMPLETE!
- [x] Create GoCodeGenerator trait implementation ✅
- [x] Generate standalone .go files ✅
- [x] Idiomatic Go code (PascalCase, interfaces) ✅
- [x] Error handling with error interface ✅
- [x] TokenizeAll() method ✅
- [x] Support parameterized rules ✅
- [x] Package structure with imports ✅
- [x] Test with basic grammar ✅

### ANTLR4 Actions Support
- [x] Parse embedded actions: `{...}` ✅
- [x] Parse semantic predicates: `{...}?` ✅
- [x] Add Action element to AST ✅
- [x] Add Predicate element to AST ✅
- [x] Language-specific actions: `{rust: ...}` ✅
- [x] Helper methods (action(), predicate()) ✅
- [x] Comprehensive documentation ✅
- [ ] Implement action translation for Rust (future)
- [ ] Implement action translation for Python (future)
- [ ] Implement action translation for JavaScript/TypeScript (future)

### Performance Baseline
- [x] Benchmark parser generation time ✅
- [x] Benchmark all 4 language generators ✅
- [x] Benchmark grammar complexity scaling ✅
- [x] Identify bottlenecks ✅
- [x] Create performance report ✅
- [x] Compare with ANTLR4 ✅
- [ ] Benchmark generated parser performance (future)
- [ ] Compare with hand-written parsers (future)

### Documentation
- [x] Getting started tutorial ✅
- [x] Migration guide from ANTLR4 ✅
- [x] Documentation index (docs/INDEX.md) ✅
- [x] Example documentation (3 levels) ✅
- [x] Project structure guide ✅
- [ ] Per-language guides (Rust, Python, JS, TS) (future)
- [ ] Troubleshooting guide (future)

---

## Month 4 - Go & Advanced ANTLR4

### Go Target (Priority 2)
- [ ] Create GoCodeGenerator trait implementation
- [ ] Generate standalone .go files
- [ ] Idiomatic Go (errors, interfaces)
- [ ] Test with CompleteJSON.g4

### ANTLR4 Advanced Features
- [ ] Rule arguments: `rule[int x]`
- [ ] Return values: `returns [Type]`
- [ ] Local variables: `locals [Type]`
- [ ] Update AST to support these features
- [ ] Generate code for all targets

### Lexer Modes & Channels
- [ ] Parse lexer modes: `mode NAME;`
- [ ] Parse lexer channels: `-> channel(NAME)`
- [ ] Parse lexer commands: `-> more`, `-> type(TYPE)`
- [ ] Implement in code generation

### VS Code Extension (Basic)
- [ ] Syntax highlighting for .g4 files
- [ ] Basic grammar validation
- [ ] Integration with minipg CLI

---

## Month 5-6 - C/C++ & Grammar Composition

### C Target (Priority 2)
- [ ] Create CCodeGenerator trait implementation
- [ ] Generate standalone .c/.h files
- [ ] Manual memory management helpers
- [ ] Test with CompleteJSON.g4

### C++ Target (Priority 2)
- [ ] Create CppCodeGenerator trait implementation
- [ ] Generate standalone .cpp/.hpp files
- [ ] Modern C++ (C++17+, RAII, smart pointers)
- [ ] Test with CompleteJSON.g4

### Grammar Composition
- [ ] Parse grammar imports: `import X;`
- [ ] Parse grammar inheritance
- [ ] Resolve imported rules
- [ ] Handle token vocabularies
- [ ] Support grammar options

### Beta Release
- [ ] All Priority 1 & 2 languages complete
- [ ] Advanced ANTLR4 features working
- [ ] Comprehensive testing
- [ ] Documentation complete
- [ ] Publish beta to crates.io

---

## Month 7-12 - Java & Production Ready

### Java Target (Priority 3)
- [ ] Create JavaCodeGenerator trait implementation
- [ ] Generate standalone .java files
- [ ] Proper package structure
- [ ] Test with CompleteJSON.g4

### Full ANTLR4 Compatibility
- [ ] Named actions (@header, @members)
- [ ] Token specifications
- [ ] All ANTLR4 options
- [ ] Test with grammars-v4 repository
- [ ] Pass ANTLR4 test suite

### Real-World Grammars
- [ ] Full Java grammar
- [ ] Full Python 3 grammar
- [ ] Full C/C++ grammar
- [ ] Full SQL grammar
- [ ] GraphQL grammar

### Production Hardening
- [ ] Fuzz testing
- [ ] Large file testing (GB+ inputs)
- [ ] Memory profiling
- [ ] Performance optimization
- [ ] Security audit

### Ecosystem
- [ ] VS Code extension (advanced features)
- [ ] Language Server Protocol (LSP)
- [ ] Online playground
- [ ] Package manager integrations

### 1.0 Release
- [ ] All 8 languages fully supported
- [ ] 100% ANTLR4 compatibility
- [ ] Production-ready quality
- [ ] Comprehensive documentation
- [ ] Active community

---

## Ongoing Tasks

### Code Quality
- [ ] Keep cargo build passing
- [ ] Keep cargo test passing (maintain 100% pass rate)
- [ ] Run cargo clippy regularly
- [ ] Run cargo fmt regularly
- [ ] Update dependencies

### Documentation
- [ ] Keep README.md updated
- [ ] Keep TODO.md updated
- [ ] Update examples as features are added
- [ ] Add inline code documentation

### Testing
- [ ] Add tests for new features
- [ ] Maintain test coverage
- [ ] Add regression tests for bugs
- [ ] Update snapshot tests

---

## Known Issues

### High Priority
- Code generator produces skeleton code (needs optimization)
- Generated code needs error recovery
- Need more robust ANTLR4 grammar parsing

### Medium Priority
- Parser needs better error messages
- Need to handle more grammar edge cases
- Unicode character class support incomplete

### Low Priority
- Performance not yet optimized
- No incremental parsing support

---

## Future Considerations (Post 1.0)

### Advanced Features
- Incremental parsing
- Grammar debugging tools
- Visual grammar designer
- AI-assisted grammar generation

### Ecosystem Growth
- IDE plugins (IntelliJ, Emacs, Vim)
- Build system integrations
- Cloud-based grammar services
- Enterprise support

### Research
- Novel parsing algorithms
- Grammar optimization techniques
- Automatic grammar inference
- Machine learning integration

---

**Last Updated**: 2025-10-25  
**Current Version**: v0.1.0-alpha.3 (Published to crates.io)  
**Current Focus**: Documentation & Per-Language Guides  
**Next Milestone**: Beta Release v0.1.0-beta.1  
**Test Status**: 213/213 passing (100% pass rate)  
**Project Status**: Production-ready for alpha release
