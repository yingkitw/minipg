# Architecture

## Overview

minipg is a parser generator inspired by ANTLR4, designed with modularity and testability as core principles. The architecture follows a pipeline model where grammar files are processed through multiple stages.

## Design Principles

1. **Separation of Concerns**: Each module has a single, well-defined responsibility
2. **Trait-Based Abstraction**: Core capabilities are defined as traits for flexibility
3. **Test-Friendly Design**: All components can be tested in isolation
4. **Type Safety**: Leverage Rust's type system for correctness
5. **Error Handling**: Comprehensive error types with diagnostic information
6. **Performance**: Sub-millisecond code generation with optimized output
7. **Multi-Language**: Consistent API across all target languages

## Module Structure

**Note**: minipg is now a single consolidated crate with modular organization for easier publishing and installation.

### core

The foundation module providing:
- **Error Types**: Unified error handling with `Error` and `Result`
- **Diagnostics**: Rich diagnostic messages with location information
- **Traits**: Core capability traits (`GrammarParser`, `SemanticAnalyzer`, `CodeGenerator`, etc.)
- **Types**: Common types like `GrammarType`, `SymbolTable`, `CodeGenConfig`

Key traits:
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

### ast

Abstract Syntax Tree module:
- **Grammar**: Root node containing rules, options, imports, and named actions
- **Rule**: Individual grammar rules (parser or lexer) with arguments, returns, locals
- **Element**: Grammar elements (terminals, non-terminals, operators) with labels
- **Alternative**: Rule alternatives (sequences of elements) with lexer commands
- **Visitor**: Visitor pattern for AST traversal
- **LexerCommand**: Enum for lexer commands (Skip, Channel, Mode, etc.)

The AST is designed to be:
- Serializable (via serde)
- Immutable by default
- Easy to traverse and transform
- Feature-complete for ANTLR4 compatibility

### parser

Grammar file parsing module:
- **Lexer**: Tokenizes grammar files with context-aware modes (CharClass mode)
- **Parser**: Builds AST from tokens with full ANTLR4 feature support
- **Token**: Token definitions with location info (40+ token types)

Features supported:
- Grammar imports and options
- Named actions (@header, @members, etc.)
- Rule arguments, returns, and locals
- Element labels (id=ID) and list labels (ids+=ID)
- Lexer commands (-> skip, -> channel, etc.)
- Non-greedy quantifiers (.*?, .+?, .??)
- Character classes with Unicode escapes

The parser implements the `GrammarParser` trait from minipg-core.

### analysis

Semantic analysis and validation module:
- **SemanticAnalyzer**: Performs semantic checks
  - Undefined rule detection
  - Duplicate rule detection
  - Left recursion detection
  - Empty alternative warnings
- **GrammarValidator**: Basic grammar validation
- **AnalysisResult**: Contains validated grammar and diagnostics

### codegen

Code generation module for 8 target languages:
- **CodeGenerator**: Main dispatcher for code generation
- **RustCodeGenerator**: Rust-specific code generation with inline DFA
- **PythonCodeGenerator**: Python code with type hints (3.10+)
- **JavaScriptCodeGenerator**: Modern ES6+ JavaScript
- **TypeScriptCodeGenerator**: TypeScript with full type safety
- **GoCodeGenerator**: Idiomatic Go with interfaces
- **JavaCodeGenerator**: Java with proper package structure
- **CCodeGenerator**: C with manual memory management
- **CppCodeGenerator**: Modern C++17+ with RAII and smart pointers
- **Template**: Simple template engine for code generation
- **DfaBuilder**: Generates optimized DFA for tokenization
- **LookupTableBuilder**: Creates const lookup tables for character classes
- **modes**: Lexer mode stack management and channel routing for all languages
- **actions**: Action code generation and language-specific translation
- **rule_body**: Rule body generation for parser implementation

The code generator produces:
- Lexer implementation with optimized tokenization
- Parser implementation with error recovery
- Token type definitions
- Error types (ParseError)
- Visitor/listener patterns (optional)
- Documentation comments
- **Named action insertion** - Custom code from `@header` and `@members`
- **Lexer modes & channels** - Mode stack management and channel routing
- **Action code generation** - Embedded actions and semantic predicates

All 8 generators support:
- Parameterized rules (arguments, returns, locals)
- Named actions (`@header` for imports, `@members` for fields)
- List labels (`ids+=ID`)
- Non-greedy quantifiers
- Character classes with Unicode
- **Lexer modes** - Mode switching, push/pop operations
- **Channels** - Token channel routing
- **Actions** - Embedded action code and semantic predicates
- **Action translation** - Language-specific action conversion

### cli

Command-line interface module:
- **CLI**: Argument parsing with clap
- **Commands**: Command implementations
  - `generate`: Generate parser from grammar
  - `validate`: Validate grammar file
  - `info`: Show grammar information

### mcp

Model Context Protocol (MCP) server module:
- **MinipgServer**: MCP server implementation using rmcp
- **Tool Router**: Routes MCP tool calls to minipg operations
- **Tools**: Exposes minipg functionality via MCP protocol
  - `parse_grammar`: Parse grammar text into AST
  - `generate_parser`: Generate parser code for target language
  - `validate_grammar`: Validate grammar and return diagnostics
- Enables AI assistants and tools to interact with minipg programmatically

## Processing Pipeline

```
Grammar File
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST
    ↓
[Semantic Analysis] → Validated AST + Diagnostics
    ↓
[Code Generator] → Generated Code
    ↓
Output Files
```

## Error Handling Strategy

1. **Parse Errors**: Reported with line/column information
2. **Semantic Errors**: Collected during analysis with diagnostic codes
3. **Code Generation Errors**: Reported with context about what failed
4. **CLI Errors**: User-friendly messages with suggestions

## Testing Strategy

1. **Unit Tests**: Test individual components in isolation (102+ tests)
2. **Snapshot Tests**: Use insta for regression testing
3. **Integration Tests**: Test full pipeline end-to-end
4. **Property Tests**: Property-based testing with proptest for grammar validation
5. **Fuzzing Tests**: Coverage-guided fuzzing for parser robustness
6. **Large File Tests**: Memory profiling for GB+ grammars
7. **Performance Benchmarks**: Comprehensive performance testing

## Extension Points

The architecture supports extension through:
1. **New Target Languages**: Implement `CodeGenerator` trait
2. **Custom Analysis**: Implement `SemanticAnalyzer` trait
3. **AST Transformations**: Use `AstVisitor` or `AstVisitorMut`
4. **Custom Diagnostics**: Extend `Diagnostic` type

## Future Enhancements

1. **Incremental Parsing**: Cache parse results for faster iteration
2. **Parallel Analysis**: Analyze multiple grammars concurrently
3. **Plugin System**: Load custom code generators dynamically
4. **LSP Support**: Language server for IDE integration
