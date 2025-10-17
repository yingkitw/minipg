# minipg - Mini Parser Generator

A modern parser generator written in Rust, inspired by ANTLR4. minipg generates efficient parsers from grammar specifications.

## Features

- **Modular Architecture**: Organized into focused crates
- **Trait-Based Design**: Extensible and testable
- **Rich Diagnostics**: Detailed error messages with location information
- **AST with Visitor Pattern**: Flexible tree traversal
- **Semantic Analysis**: 
  - Undefined rule detection
  - Duplicate rule detection
  - Left recursion detection
  - Reachability analysis
  - Empty alternative warnings
- **Code Generation**: 
  - Generates Rust parsers
  - Visitor pattern generation
  - Listener pattern generation
  - Configurable output
- **CLI Tool**: Easy-to-use command-line interface
- **Error Recovery**: Robust error handling and recovery strategies
- **Comprehensive Documentation**: User guide, API docs, and syntax reference
- **Snapshot Testing**: Comprehensive tests using insta for regression prevention
- **Multiple Targets**: Currently supports Rust code generation (more targets planned)

## Architecture

minipg is organized into the following crates:

- **minipg-core**: Core traits, error types, and diagnostics
- **minipg-ast**: Abstract Syntax Tree definitions and visitor patterns
- **minipg-parser**: Grammar file parser (lexer + parser)
- **minipg-analysis**: Semantic analysis and validation
- **minipg-codegen**: Code generation for target languages
- **minipg-cli**: Command-line interface

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Installation

```bash
cargo install --path crates/minipg-cli
```

## Usage

### Generate a Parser

```bash
minipg generate grammar.g4 -o output/ -l rust
```

### Validate a Grammar

```bash
minipg validate grammar.g4
```

### Show Grammar Information

```bash
minipg info grammar.g4
```

## Grammar Syntax

minipg uses a syntax similar to ANTLR4:

```
grammar parser Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

## Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Complete guide to using minipg
- **[Grammar Syntax Reference](docs/GRAMMAR_SYNTAX.md)** - Detailed syntax specification
- **[API Documentation](docs/API.md)** - API reference for library usage
- **[Architecture](ARCHITECTURE.md)** - Design and architecture overview
- **[Examples](examples/)** - Example grammars

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test --all
```

All 68 tests pass with 100% success rate.

### Running with Logging

```bash
RUST_LOG=info cargo run -- generate grammar.g4
```

## Project Status

See [TODO.md](TODO.md) for current tasks and roadmap.

## License

MIT OR Apache-2.0
