# minipg Features

**Version**: 0.1.0-alpha.3  
**Last Updated**: October 17, 2025

## ✅ Fully Implemented Features

### Core Parser Generator
- [x] **ANTLR4-Compatible Grammar Parsing**
  - Lexer and parser rules
  - Alternatives and sequences
  - Quantifiers: `?`, `*`, `+`
  - Non-greedy quantifiers: `??`, `*?`, `+?`
  - Grouping with parentheses
  - Labels for elements and alternatives

### Character Classes
- [x] **Advanced Character Class Support**
  - Simple ranges: `[a-z]`, `[0-9]`
  - Multiple ranges: `[a-zA-Z0-9]`
  - Negated classes: `~["\r\n]`
  - Unicode escapes: `\u0000-\uFFFF`
  - Escape sequences: `\\`, `\/`, `\n`, `\r`, `\t`
  - Special characters in classes: `[+\-*/]`
  - Context-aware lexer state machine

### Rule Features
- [x] **Parameterized Rules** (All 4 languages)
  - Rule arguments: `rule[int x, String name]`
  - Return values: `returns [Value result]`
  - Multiple returns: `returns [Value v, int code]`
  - Local variables: `locals [int temp, String buffer]`
  - Type inference for untyped parameters
  - Full documentation generation

### Lexer Features
- [x] **Lexer Commands** (Parsing only)
  - `-> skip` - Skip tokens
  - `-> channel(NAME)` - Channel routing
  - `-> mode(NAME)` - Mode switching
  - `-> type(TYPE)` - Type changing
  - `-> pushMode(NAME)` - Push mode
  - `-> popMode` - Pop mode
  - `-> more` - Continue token
  - **Note**: Commands are parsed and stored in AST; code generation pending

- [x] **Fragment Rules**
  - Reusable lexer components
  - Not emitted as tokens
  - Can be referenced by other lexer rules

### Code Generation

#### Rust Target ✅
- Standalone parsers with no runtime dependencies
- Inline DFA generation for efficiency
- Const lookup tables for O(1) character classification
- Full error recovery with position tracking
- Parameterized rules with proper types
- Documentation comments
- `#[inline]` attributes for performance
- `Result<T, ParseError>` error handling

#### Python Target ✅
- Python 3.10+ with type hints
- Dataclasses for AST nodes
- PEP 8 compliant code
- Full error recovery
- Parameterized rules with type annotations
- Docstrings with Args/Returns sections
- `tokenize_all()` method for error collection

#### JavaScript Target ✅
- Modern ES6+ syntax
- Classes with proper encapsulation
- JSDoc documentation
- Error recovery with ParseError class
- Parameterized rules with JSDoc types
- Works in Node.js and browsers
- `tokenizeAll()` method

#### TypeScript Target ✅
- Full type safety with interfaces
- Typed error handling
- Export all public types
- Parameterized rules with TypeScript types
- Private/public modifiers
- Enum for token kinds
- `tokenizeAll()` with typed returns

### Testing
- [x] **99 tests passing** (0 ignored)
  - 48 unit tests
  - 11 E2E tests
  - 9 integration tests
  - 6 parameterized rule tests
  - 25 other tests
- [x] **Real-World Grammar Support**
  - CompleteJSON.g4 (RFC 8259) - All 5 tests passing
  - SQL.g4 - All 4 tests passing
- [x] **Snapshot Testing** with insta
- [x] **Cross-Language Consistency** tests

### Performance
- [x] **Sub-Millisecond Code Generation**
  - Rust: ~2 µs per rule
  - Python: ~5 µs per rule
  - JavaScript: ~4 µs per rule
  - TypeScript: ~6 µs per rule
- [x] **Linear Scaling** O(n) with grammar complexity
- [x] **Low Memory** <100 KB for large grammars
- [x] **Comprehensive Benchmarks** with criterion

### Documentation
- [x] **User Documentation**
  - Getting Started tutorial
  - Migration guide from ANTLR4
  - Grammar syntax reference
  - Example documentation (3 levels)
- [x] **Technical Documentation**
  - Architecture documentation
  - API documentation
  - Performance reports
  - Known limitations
- [x] **Examples**
  - Calculator (beginner)
  - JSON parser (intermediate)
  - CompleteJSON (RFC 8259)
  - SQL parser (advanced)
  - Java subset
  - Python subset

### CI/CD
- [x] **GitHub Actions**
  - Multi-platform builds (Linux, macOS, Windows)
  - Automated testing
  - Cargo clippy linting
  - Code coverage reporting
  - Release automation
  - Documentation deployment

## 🚧 Partially Implemented

### Lexer Commands Code Generation
- **Status**: AST support complete, code generation pending
- **Parsed**: All 7 command types
- **Stored**: In `Alternative.lexer_command` field
- **TODO**: Generate actual skip/channel/mode logic in lexers

### Actions & Predicates
- **Status**: AST support complete, code generation pending
- **Parsed**: `{...}` actions and `{...}?` predicates
- **Stored**: In `Element::Action` and `Element::Predicate`
- **TODO**: Language-specific action translation

## 📋 Planned Features

### Phase 1: Lexer Command Implementation (Next)
- [ ] Generate skip logic in lexers
- [ ] Implement channel support
- [ ] Implement mode switching
- [ ] Add tests for lexer commands

### Phase 2: Additional Target Languages
- [ ] Go code generator
- [ ] C code generator
- [ ] C++ code generator
- [ ] Java code generator

### Phase 3: Advanced ANTLR4 Features
- [ ] Grammar imports: `import X;`
- [ ] Grammar inheritance
- [ ] Token vocabularies
- [ ] Named actions: `@header`, `@members`
- [ ] Grammar options

### Phase 4: Tooling & Ecosystem
- [ ] VS Code extension
- [ ] Language Server Protocol (LSP)
- [ ] Online playground
- [ ] Package manager integrations

### Phase 5: Production Hardening
- [ ] Fuzz testing
- [ ] Large file testing (GB+ inputs)
- [ ] Memory profiling
- [ ] Security audit
- [ ] Full ANTLR4 test suite compatibility

## 📊 Feature Comparison

| Feature | minipg | ANTLR4 |
|---------|--------|--------|
| Code Generation Speed | ⚡ Sub-millisecond | ~100ms |
| Runtime Dependency | ✅ None | ❌ Required |
| Character Classes | ✅ Full | ✅ Full |
| Non-Greedy Quantifiers | ✅ Yes | ✅ Yes |
| Parameterized Rules | ✅ Yes | ✅ Yes |
| Lexer Commands (parsing) | ✅ Yes | ✅ Yes |
| Lexer Commands (codegen) | 🚧 Partial | ✅ Yes |
| Multi-Language Support | ✅ 4 languages | ✅ 10+ languages |
| Grammar Imports | ❌ No | ✅ Yes |
| Named Actions | ❌ No | ✅ Yes |

## 🎯 Current Capabilities

### What You Can Do Now
1. **Parse Complex Grammars**: CompleteJSON, SQL, and more
2. **Generate Production Code**: Rust, Python, JS, TS
3. **Use Advanced Features**: Non-greedy quantifiers, character classes, parameterized rules
4. **Get Fast Results**: Sub-millisecond code generation
5. **No Runtime Dependencies**: Generated code is standalone
6. **Full Error Recovery**: Detailed error messages with positions

### What's Coming Next
1. **Lexer Command Code Generation**: Make `-> skip` actually work
2. **More Languages**: Go, C, C++, Java
3. **Grammar Composition**: Import and extend grammars
4. **IDE Support**: VS Code extension with syntax highlighting
5. **Production Ready**: Fuzz testing, performance optimization

## 📝 Notes

- **Alpha Status**: Feature-complete for core use cases
- **Production Ready**: For Rust, Python, JS, TS targets
- **Active Development**: New features added regularly
- **Community Driven**: Contributions welcome!

---

For detailed implementation status, see [TODO.md](TODO.md).  
For roadmap and timeline, see [ROADMAP.md](ROADMAP.md).  
For migration from ANTLR4, see [docs/guides/MIGRATION_FROM_ANTLR4.md](docs/guides/MIGRATION_FROM_ANTLR4.md).
