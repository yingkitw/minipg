# Key Architectural Decisions

## Decision Summary

This document records the major architectural decisions made for minipg.

---

## Decision 1: Standalone Code Generation (No Runtime Library)

**Date**: 2025-10-17  
**Status**: ✅ FINAL DECISION  
**Context**: Should minipg generate code that depends on a runtime library (like antlr4rust) or generate standalone code?

### Decision
**Generate standalone, self-contained code with zero dependencies.**

### Rationale
1. **Zero Dependencies** - Users get a single file they can use immediately
2. **Full Transparency** - Users can read and understand all generated code
3. **Easy Customization** - Users can modify generated code as needed
4. **No Version Conflicts** - No dependency management issues
5. **Unique Differentiator** - Different from antlr4rust and ANTLR4
6. **Rust Philosophy** - Aligns with zero-cost abstractions
7. **Multi-Language Friendly** - Each language gets optimized standalone code

### Consequences
- **Positive**:
  - Easier to use and adopt
  - Better for embedding, WASM, embedded systems
  - No runtime to maintain
  - Users can inspect and learn from generated code
  
- **Negative**:
  - Larger generated files
  - Code duplication across parsers
  - Must optimize at generation time
  - Bug fixes require regeneration

### Implementation
- Generate complete parser implementation inline
- Optimize DFA as match statements at generation time
- Emit lookup tables as const arrays
- Include error recovery inline
- Make generated code readable with inline documentation

### References
- [docs/RUNTIME_DECISION.md](docs/RUNTIME_DECISION.md) - Full analysis

---

## Decision 2: Multi-Language Target Support

**Date**: 2025-10-17  
**Status**: ✅ FINAL DECISION  
**Context**: Which programming languages should minipg support?

### Decision
**Support 8 target languages: Rust, Python, JavaScript, TypeScript, Go, Java, C, C++**

### Priority Order
1. **Priority 1** (Months 1-3): Rust, Python, JavaScript, TypeScript
2. **Priority 2** (Months 4-6): Go, C, C++
3. **Priority 3** (Months 7-12): Java

### Rationale
1. **Rust** - Primary language, best architecture showcase
2. **Python** - Popular for scripting, data science
3. **JavaScript** - Web and Node.js ecosystem
4. **TypeScript** - Type-safe JavaScript
5. **Go** - Modern systems language
6. **C** - Universal, embedded systems
7. **C++** - Modern C++ with templates
8. **Java** - Enterprise applications

### Implementation Strategy
- Template-based code generation per language
- Shared optimization logic
- Language-specific idioms
- Cross-language consistency testing

### References
- [docs/MULTI_LANGUAGE_PLAN.md](docs/MULTI_LANGUAGE_PLAN.md) - Detailed plan

---

## Decision 3: Full ANTLR4 Grammar Compatibility

**Date**: 2025-10-17  
**Status**: ✅ FINAL DECISION  
**Context**: Should minipg use its own grammar format or be compatible with ANTLR4?

### Decision
**Full compatibility with ANTLR4 grammar syntax.**

### Rationale
1. **Existing Ecosystem** - 300+ grammars in grammars-v4 repository
2. **User Familiarity** - Many users already know ANTLR4
3. **Migration Path** - Easy to switch from ANTLR4 to minipg
4. **Proven Syntax** - ANTLR4 syntax is well-designed
5. **Community** - Leverage existing ANTLR4 community

### Scope
- Parse all ANTLR4 grammar syntax
- Support labels, actions, predicates
- Support lexer modes and channels
- Support grammar imports and inheritance
- Support all ANTLR4 options (where applicable)

### Differences
- **Actions**: Translate from Java-like to target language
- **Runtime**: Generate standalone code instead of runtime-dependent
- **Language Option**: Ignored (we generate for all languages)

### Implementation Phases
1. **Phase 1**: Core syntax (alternatives, quantifiers, etc.)
2. **Phase 2**: Labels and references
3. **Phase 3**: Actions and predicates
4. **Phase 4**: Advanced features (modes, channels)
5. **Phase 5**: Grammar composition (imports, inheritance)
6. **Phase 6**: Full compatibility (pass ANTLR4 test suite)

### References
- [docs/ANTLR4_COMPATIBILITY.md](docs/ANTLR4_COMPATIBILITY.md) - Compatibility plan

---

## Decision 4: Modular Crate Architecture

**Date**: 2025-10-17 (Earlier)  
**Status**: ✅ IMPLEMENTED  
**Context**: How should the codebase be organized?

### Decision
**Use workspace with 7 focused crates.**

### Crates
1. **minipg-core** - Traits, errors, diagnostics
2. **minipg-ast** - AST definitions, visitor pattern
3. **minipg-parser** - Grammar file parser
4. **minipg-analysis** - Semantic analysis
5. **minipg-codegen** - Code generation
6. **minipg-cli** - Command-line interface
7. **minipg-benchmarks** - Performance testing

### Rationale
- Clear separation of concerns
- Easy to test in isolation
- Reusable components
- Better maintainability
- Follows Rust best practices

---

## Decision 5: Trait-Based Design

**Date**: 2025-10-17 (Earlier)  
**Status**: ✅ IMPLEMENTED  
**Context**: How should core capabilities be abstracted?

### Decision
**Define core capabilities as traits.**

### Key Traits
```rust
pub trait GrammarParser {
    type Output;
    fn parse_file(&self, path: &Path) -> Result<Self::Output>;
    fn parse_string(&self, source: &str, filename: &str) -> Result<Self::Output>;
}

pub trait SemanticAnalyzer {
    type Input;
    type Output;
    fn analyze(&self, input: &Self::Input) -> Result<Self::Output>;
    fn diagnostics(&self) -> &[Diagnostic];
}

pub trait CodeGenerator {
    type Input;
    type Config;
    fn generate(&self, input: &Self::Input, config: &Self::Config) -> Result<String>;
    fn target_language(&self) -> &str;
}
```

### Rationale
- Extensible design
- Test-friendly (easy to mock)
- Multiple implementations possible
- Clear contracts
- Idiomatic Rust

---

## Decision 6: Modern Testing Approach

**Date**: 2025-10-17 (Earlier)  
**Status**: ✅ IMPLEMENTED  
**Context**: What testing strategy should we use?

### Decision
**Use snapshot testing, property-based testing, and benchmarks.**

### Tools
- **insta** - Snapshot testing for regression prevention
- **proptest** - Property-based testing for edge cases
- **criterion** - Performance benchmarking

### Rationale
- Catch regressions early
- Test edge cases automatically
- Track performance over time
- Modern Rust testing practices

---

## Decision 7: Complex Grammar Examples

**Date**: 2025-10-17  
**Status**: ✅ IMPLEMENTED  
**Context**: What example grammars should we provide?

### Decision
**Provide both simple and complex real-world grammar examples.**

### Examples Created
1. **Simple**: Calculator, Basic JSON
2. **Complex**: Complete JSON (RFC 8259), SQL, Java Subset, Python Subset

### Future Examples
- Full C/C++ grammar
- Full Java grammar
- Full Python 3 grammar
- GraphQL, Protocol Buffers
- XML, Markdown

### Rationale
- Demonstrate capabilities
- Test complex features
- Provide learning resources
- Validate ANTLR4 compatibility

---

## Decision 8: Rust Edition 2024

**Date**: 2025-10-17 (Earlier)  
**Status**: ✅ IMPLEMENTED  
**Context**: Which Rust edition should we use?

### Decision
**Use Rust 2024 edition.**

### Rationale
- Latest language features
- Better diagnostics
- Modern idioms
- Future-proof
- Differentiator from antlr4rust (2018)

---

## Decision 9: No Unsafe Code (Initially)

**Date**: 2025-10-17 (Earlier)  
**Status**: ✅ IMPLEMENTED  
**Context**: Should we use unsafe code for optimization?

### Decision
**Start with pure safe Rust. Add unsafe only if needed for performance.**

### Rationale
- Easier to audit and maintain
- Safer codebase
- Rust's safe abstractions are fast enough
- Can add unsafe later if profiling shows need

### Future Consideration
- May add unsafe for specific hot paths
- Only after profiling shows bottleneck
- Must be well-documented and justified

---

## Decision 10: Documentation-First Approach

**Date**: 2025-10-17  
**Status**: ✅ IMPLEMENTED  
**Context**: How much documentation should we have?

### Decision
**Comprehensive documentation from the start.**

### Documentation Created
- README.md
- ARCHITECTURE.md
- TODO.md
- ROADMAP.md
- User Guide
- Grammar Syntax Reference
- API Documentation
- ANTLR4 Compatibility Guide
- Multi-Language Plan
- Runtime Decision Analysis
- Comparison with antlr4rust

### Rationale
- Better onboarding
- Easier contribution
- Clear vision
- Professional project
- Differentiator from other projects

---

## Summary

These decisions establish minipg as a **modern, multi-language parser generator** that:

1. ✅ Generates **standalone code** (no runtime dependencies)
2. ✅ Supports **8 target languages**
3. ✅ Is **fully compatible with ANTLR4** grammars
4. ✅ Has a **modular, trait-based architecture**
5. ✅ Uses **modern Rust practices** (2024 edition, safe code)
6. ✅ Has **comprehensive documentation**
7. ✅ Provides **complex real-world examples**
8. ✅ Uses **modern testing approaches**

This positions minipg as a compelling alternative to ANTLR4 and antlr4rust, with unique advantages in code generation quality, multi-language support, and ease of use.

---

**Last Updated**: 2025-10-17  
**Version**: 0.1.0
