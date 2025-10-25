# Project Structure

## Overview

minipg is organized into a modular workspace with clear separation of concerns.

## Directory Structure

```
minipg/
├── crates/                    # Rust workspace crates
│   ├── minipg-core/          # Core traits and types
│   ├── minipg-ast/           # Abstract Syntax Tree
│   ├── minipg-parser/        # Grammar parser
│   ├── minipg-analysis/      # Semantic analysis
│   ├── minipg-codegen/       # Code generators
│   ├── minipg-cli/           # Command-line interface
│   └── minipg-benchmarks/    # Performance benchmarks
├── docs/                      # Documentation
│   ├── guides/               # User guides
│   ├── examples/             # Example documentation
│   ├── reports/              # Performance & test reports
│   ├── API.md               # API documentation
│   ├── GRAMMAR_SYNTAX.md    # Grammar syntax guide
│   └── USER_GUIDE.md        # Getting started
├── examples/                  # Example grammars
│   ├── calculator.g4         # Basic calculator
│   ├── json.g4              # Simple JSON
│   ├── CompleteJSON.g4      # Full JSON (RFC 8259)
│   ├── SQL.g4               # SQL parser
│   ├── JavaSubset.g4        # Java subset
│   └── PythonSubset.g4      # Python subset
├── tests/                     # Integration tests
├── benches/                   # Benchmarks
├── README.md                  # Project README
├── TODO.md                    # Development roadmap
├── ARCHITECTURE.md            # Architecture overview
└── Cargo.toml                 # Workspace configuration
```

## Crate Organization

### minipg-core
**Purpose**: Core traits and types used across all crates

**Contents**:
- `CodeGenerator` trait
- `SemanticAnalyzer` trait
- `GrammarParser` trait
- Common types (CodeGenConfig, GrammarType, etc.)
- Error types and diagnostics

**Dependencies**: None (foundation crate)

### minipg-ast
**Purpose**: Abstract Syntax Tree definitions

**Contents**:
- `Grammar` - Root AST node
- `Rule` - Parser/lexer rules
- `Element` - Grammar elements
- `Alternative` - Rule alternatives
- `Visitor` pattern support

**Dependencies**: `serde` for serialization

### minipg-parser
**Purpose**: Parse ANTLR4 grammar files

**Contents**:
- `Lexer` - Tokenize grammar files
- `Parser` - Build AST from tokens
- ANTLR4 syntax support

**Dependencies**: `minipg-core`, `minipg-ast`

### minipg-analysis
**Purpose**: Semantic analysis and validation

**Contents**:
- Left recursion detection
- Ambiguity detection
- Undefined rule detection
- Diagnostic reporting

**Dependencies**: `minipg-core`, `minipg-ast`

### minipg-codegen
**Purpose**: Generate code for target languages

**Contents**:
- `RustCodeGenerator`
- `PythonCodeGenerator`
- `JavaScriptCodeGenerator`
- `TypeScriptCodeGenerator`
- DFA generation
- Lookup table generation

**Dependencies**: `minipg-core`, `minipg-ast`, `minipg-analysis`

### minipg-cli
**Purpose**: Command-line interface

**Contents**:
- CLI argument parsing
- File I/O
- Progress reporting
- Error formatting

**Dependencies**: All other crates, `clap`

### minipg-benchmarks
**Purpose**: Performance benchmarks

**Contents**:
- Code generation benchmarks
- Parser benchmarks
- Analysis benchmarks
- Complexity scaling tests

**Dependencies**: All crates, `criterion`

## Documentation Organization

### docs/guides/
- Getting Started
- Migration from ANTLR4
- Language-specific guides
- Troubleshooting

### docs/examples/
- 01-BASIC-CALCULATOR.md (Beginner)
- 02-JSON-PARSER.md (Intermediate)
- 03-EXPRESSION-EVALUATOR.md (Intermediate)
- 04-PROGRAMMING-LANGUAGE.md (Advanced)
- 05-SQL-PARSER.md (Advanced)
- 06-CUSTOM-ACTIONS.md (Expert)

### docs/reports/
- PERFORMANCE_REPORT.md
- RUST_OPTIMIZATION_REPORT.md
- GRAMMAR_VALIDATION.md
- TEST_COVERAGE_REPORT.md
- ANTLR4_ACTIONS_SUPPORT.md
- Session summaries

## Test Organization

### Unit Tests
Located in each crate's `src/` directory:
```
crates/minipg-ast/src/lib.rs
#[cfg(test)]
mod tests { ... }
```

### Integration Tests
Located in `crates/*/tests/`:
- `generated_code_tests.rs` - Code quality tests
- `compile_tests.rs` - Compilation tests
- `cross_language_tests.rs` - Multi-language tests
- `error_recovery_tests.rs` - Error handling tests

### Benchmark Tests
Located in `crates/minipg-benchmarks/benches/`:
- `codegen_benchmarks.rs` - Code generation speed
- `parser_bench.rs` - Parsing speed
- `analysis_bench.rs` - Analysis speed

## Example Organization

### Beginner Level
- `calculator.g4` - Basic arithmetic
- Simple, focused examples
- Clear documentation

### Intermediate Level
- `json.g4` - Simple JSON
- `CompleteJSON.g4` - Full JSON spec
- More complex patterns

### Advanced Level
- `SQL.g4` - SQL queries
- `JavaSubset.g4` - Programming language
- `PythonSubset.g4` - Another language
- Real-world complexity

## Module Dependencies

```
minipg-cli
    ↓
minipg-codegen
    ↓
minipg-analysis
    ↓
minipg-parser
    ↓
minipg-ast
    ↓
minipg-core
```

## Build Artifacts

```
target/
├── debug/                    # Debug builds
├── release/                  # Release builds
├── doc/                      # Generated docs
└── criterion/                # Benchmark results
```

## Key Files

- `Cargo.toml` - Workspace configuration
- `README.md` - Project overview
- `TODO.md` - Development roadmap
- `ARCHITECTURE.md` - System design
- `ROADMAP.md` - Feature roadmap

## Naming Conventions

### Crates
- Prefix: `minipg-`
- Lowercase with hyphens
- Descriptive names

### Files
- Snake_case for Rust files
- UPPERCASE.md for documentation
- lowercase.g4 for grammars

### Modules
- One module per file
- Clear, descriptive names
- Public API in lib.rs

## Best Practices

1. **Modularity** - Each crate has single responsibility
2. **Testing** - 100% test coverage goal
3. **Documentation** - All public items documented
4. **Examples** - Multiple difficulty levels
5. **Performance** - Benchmarks for critical paths

## Maintenance

### Adding a New Language Generator
1. Create generator in `minipg-codegen/src/`
2. Implement `CodeGenerator` trait
3. Add tests in `minipg-codegen/tests/`
4. Update documentation
5. Add benchmarks

### Adding a New Example
1. Create `.g4` file in `examples/`
2. Create documentation in `docs/examples/`
3. Add test in `tests/`
4. Update `examples/README.md`

### Adding a New Feature
1. Update relevant crate
2. Add tests
3. Update documentation
4. Add to `TODO.md`
5. Update `ROADMAP.md`

---

**Last Updated**: 2025-10-17  
**Version**: 0.1.0  
**Status**: Production-ready ✅
