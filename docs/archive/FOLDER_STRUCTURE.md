# minipg Project Folder Structure

## Root Directory

### Core Files
- **README.md** - Project overview and quick start
- **Cargo.toml** - Rust project manifest
- **Cargo.lock** - Dependency lock file
- **LICENSE-APACHE** - Apache 2.0 license

### Documentation (Root)
- **ARCHITECTURE.md** - System design and architecture
- **TODO.md** - Development tasks and roadmap
- **ROADMAP.md** - Long-term project vision
- **PROGRESS.md** - Development progress tracking
- **CHANGELOG.md** - Version history
- **DECISIONS.md** - Design decisions
- **FEATURES.md** - Feature list
- **KNOWN_LIMITATIONS.md** - Current limitations
- **PROJECT_STRUCTURE.md** - Project organization
- **PUBLISHING.md** - Publishing guide
- **COMPARISON_WITH_ANTLR4RUST.md** - ANTLR4 comparison
- **COMPARISON_WITH_PEST.md** - Pest comparison

### Project Files
- **minipg.code-workspace** - VS Code workspace configuration
- **minipg.png** - Project mascot image

---

## `/src` - Source Code

Main Rust source code organized by module:

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── cli.rs               # CLI argument parsing
├── commands.rs          # CLI command implementations
├── core/                # Core types and traits
│   ├── mod.rs
│   ├── error.rs         # Error types
│   ├── traits.rs        # Core traits (GrammarParser, CodeGenerator, etc.)
│   └── types.rs         # Common types
├── ast/                 # Abstract Syntax Tree
│   ├── mod.rs
│   ├── grammar.rs       # Grammar AST node
│   ├── rule.rs          # Rule definitions
│   ├── element.rs       # Grammar elements
│   └── visitor.rs       # Visitor pattern
├── parser/              # Grammar file parser
│   ├── mod.rs
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # Parser
│   ├── token.rs         # Token definitions
│   └── error_recovery.rs # Error recovery
├── analysis/            # Semantic analysis
│   ├── mod.rs
│   ├── semantic.rs      # Semantic analyzer
│   ├── validator.rs     # Grammar validator
│   ├── reachability.rs  # Reachability analysis
│   ├── left_recursion.rs # Left recursion detection
│   ├── first_follow.rs  # First/Follow sets
│   └── ambiguity.rs     # Ambiguity detection
└── codegen/             # Code generation
    ├── mod.rs
    ├── rust.rs          # Rust code generator
    ├── python.rs        # Python code generator
    ├── javascript.rs    # JavaScript code generator
    ├── typescript.rs    # TypeScript code generator
    ├── go.rs            # Go code generator
    ├── dfa.rs           # DFA builder
    ├── lookup_table.rs  # Lookup table generation
    ├── template.rs      # Template engine
    └── visitor_gen.rs   # Visitor/Listener generation
```

---

## `/tests` - Integration Tests

Comprehensive test suite:

```
tests/
├── test_all_examples.rs              # Example grammar tests (17 tests)
├── property_based_tests.rs           # Property-based tests (15 tests)
├── fuzzing_tests.rs                  # Fuzzing tests (18 tests)
├── integration_test.rs               # Basic integration tests
├── integration_grammar_tests.rs      # Grammar integration tests
├── end_to_end_test.rs                # E2E tests
├── e2e_simple_pipeline.rs            # Simple pipeline tests
├── parser_advanced_features.rs       # Parser feature tests
├── codegen_parameterized_rules.rs    # Parameterized rule tests
├── test_completejson_line.rs         # CompleteJSON tests
├── test_list_labels.rs               # List label tests
├── test_named_actions.rs             # Named action tests
├── test_named_actions_codegen.rs     # Named action codegen tests
├── test_unicode_escape.rs            # Unicode escape tests
└── debug_charclass.rs                # Character class debug tests
```

**Total**: 165 tests (49 unit + 34 integration + 15 property-based + 18 fuzzing + 1 doc)

---

## `/examples` - Example Grammars

16 example grammars demonstrating minipg capabilities:

```
examples/
├── README.md                    # Examples documentation
├── calculator.g4                # Simple arithmetic (beginner)
├── json.g4                      # Simple JSON (beginner)
├── Expression.g4                # Operator precedence (intermediate)
├── Config.g4                    # Configuration files (intermediate)
├── YAML.g4                      # YAML format (intermediate)
├── CompleteJSON.g4              # RFC 8259 JSON (intermediate)
├── GraphQL.g4                   # GraphQL schema (advanced)
├── Query.g4                     # SQL-like queries (advanced)
├── CSS.g4                       # Stylesheets (advanced)
├── Markdown.g4                  # Markdown documents (advanced)
├── Protocol.g4                  # Protocol buffers (advanced)
├── SQL.g4                       # SQL language (advanced)
├── JavaSubset.g4                # Java subset (advanced)
├── PythonSubset.g4              # Python subset (advanced)
├── test_charclass.g4            # Character class tests
└── test_simple_charclass.g4     # Simple character class tests
```

---

## `/docs` - Documentation

Organized documentation:

```
docs/
├── README.md                    # Documentation index
├── USER_GUIDE.md                # Complete user guide
├── GRAMMAR_SYNTAX.md            # Grammar syntax reference
├── API.md                       # API documentation
├── EXAMPLES_GUIDE.md            # Examples guide with learning path
├── ANTLR4_COMPATIBILITY.md      # ANTLR4 compatibility details
├── MULTI_LANGUAGE_PLAN.md       # Multi-language support roadmap
├── RUNTIME_DECISION.md          # Runtime design decisions
├── COMPARISON_SUMMARY.md        # Feature comparison summary
├── INDEX.md                     # Documentation index
├── _config.yml                  # Jekyll configuration
├── index.html                   # Landing page
├── sessions/                    # Development sessions
│   ├── SESSION_SUMMARY_OCT25.md
│   ├── IMPROVEMENTS_SUMMARY.md
│   ├── EXAMPLES_ADDED.md
│   └── EXAMPLE_TESTS_ADDED.md
├── archive/                     # Historical documentation
│   ├── INDEX.md
│   ├── ALPHA_RELEASE_NOTES.md
│   ├── RELEASE_NOTES_v0.1.0-alpha.3.md
│   ├── CHANGELOG_v0.1.0-alpha.2.md
│   ├── FINAL_SESSION_REPORT.md
│   ├── EXTENDED_SESSION_SUMMARY.md
│   ├── SESSION_SUMMARY.md
│   ├── FINAL_DEVELOPMENT_SESSION.md
│   ├── COMPLETE_IMPLEMENTATION_SUMMARY.md
│   ├── COMPLETION_SUMMARY.md
│   ├── IMPLEMENTATION_COMPLETE.md
│   ├── DOCUMENTATION_UPDATE_SUMMARY.md
│   ├── LEXER_MODES_IMPLEMENTATION_GUIDE.md
│   ├── NEXT_PHASE.md
│   ├── ORGANIZATION_COMPLETE.md
│   ├── READY_FOR_RELEASE.md
│   └── [17 historical reports]
├── guides/                      # Additional guides
│   └── [guide files]
├── examples/                    # Example documentation
│   ├── 01-BASIC-CALCULATOR.md
│   ├── 02-JSON-PARSER.md
│   └── 05-SQL-PARSER.md
└── [other documentation files]
```

---

## `/benches` - Benchmarks

Performance benchmarks:

```
benches/
├── codegen_bench.rs            # Code generation benchmarks
└── [other benchmark files]
```

---

## `/.github` - GitHub Configuration

CI/CD and GitHub configuration:

```
.github/
├── workflows/                  # GitHub Actions workflows
│   ├── build.yml
│   ├── test.yml
│   ├── clippy.yml
│   ├── fmt.yml
│   ├── release.yml
│   ├── docs.yml
│   └── coverage.yml
└── [other GitHub files]
```

---

## `/target` - Build Output

Generated during build (not tracked):

```
target/
├── debug/                      # Debug builds
├── release/                    # Release builds
└── [build artifacts]
```

---

## Summary

### Key Directories
- **src/** - Source code (modular, organized by feature)
- **tests/** - 165 comprehensive tests
- **examples/** - 16 example grammars
- **docs/** - Complete documentation
- **benches/** - Performance benchmarks

### File Organization
- **Root**: Core documentation and project files
- **docs/**: User-facing documentation
- **docs/archive/**: Historical documentation
- **docs/sessions/**: Development session reports
- **src/**: Modular Rust source code
- **tests/**: Comprehensive test suite
- **examples/**: Learning examples

### Statistics
- **Source Files**: 30+ modules
- **Test Files**: 14 test suites
- **Example Grammars**: 16 examples
- **Documentation Files**: 40+ files
- **Total Tests**: 165 passing

---

**Last Updated**: October 25, 2025  
**Status**: Clean and organized ✅
