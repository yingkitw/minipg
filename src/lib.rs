//! # minipg - A blazingly fast parser generator with incremental parsing
//!
//! minipg is a modern parser generator with ANTLR4 compatibility and **incremental parsing** capabilities.
//! It generates standalone parsers for 9 target languages without runtime dependencies, and provides
//! complete infrastructure for replacing Tree-sitter in editor integration.
//!
//! ## Features
//!
//! ### ⚡ Incremental Parsing (v0.1.5)
//! - **Position Tracking**: Byte offsets and line/column for every AST node
//! - **Edit Tracking**: Insert, delete, replace operations with automatic point calculation
//! - **Fast Re-parsing**: Foundation for <10ms incremental edits
//! - **Query Language**: Tree-sitter-compatible S-expression queries for pattern matching
//!
//! ### 🌍 Multi-Language Support (9 Languages)
//! - **Rust**, **Python**, **JavaScript**, **TypeScript**, **Go**, **Java**, **C**, **C++**
//! - **Tree-sitter**: Grammar.js for editor syntax highlighting
//!
//! ### 🎯 ANTLR4 Compatible
//! - Advanced character classes with Unicode escapes
//! - Non-greedy quantifiers, lexer modes & channels
//! - Rule arguments, returns, and local variables
//! - Named actions, grammar imports, and more
//!
//! ### 🚀 Performance
//! - Sub-millisecond code generation for typical grammars
//! - <100 KB memory usage
//! - Target: <10ms for incremental edits
//!
//! ## Example
//!
//! ```rust,no_run
//! use minipg::parser::{Lexer, Parser};
//! use minipg::codegen::RustCodeGenerator;
//! use minipg::core::{types::CodeGenConfig, CodeGenerator};
//!
//! let grammar_source = r#"
//!     grammar Calculator;
//!     expr: term (('+' | '-') term)*;
//!     term: factor (('*' | '/') factor)*;
//!     factor: NUMBER | '(' expr ')';
//!     NUMBER: [0-9]+;
//! "#;
//!
//! // Parse the grammar
//! let lexer = Lexer::new(grammar_source, "calculator.g4");
//! let mut parser = Parser::new(lexer);
//! let grammar = parser.parse().expect("Failed to parse grammar");
//!
//! // Generate Rust code
//! let generator = RustCodeGenerator::new();
//! let config = CodeGenConfig::default();
//! let code = generator.generate(&grammar, &config).expect("Failed to generate code");
//! ```

// Re-export core types
pub mod core;
pub use core::*;

// Re-export AST types
pub mod ast;
pub use ast::{Grammar, Rule, Alternative, Element};

// Re-export parser
pub mod parser;
pub use parser::{Lexer, Parser, GrammarParser};

// Re-export incremental parsing
pub mod incremental;
pub use incremental::{Point, Position, Range, Edit, IncrementalParser, SyntaxTree, DefaultIncrementalParser};

// Re-export query language
pub mod query;
pub use query::{QueryParser, Pattern, PatternNode, QueryMatcher, CaptureGroup};

// Re-export analysis
pub mod analysis;
pub use analysis::{SemanticAnalyzer, GrammarValidator, AnalysisResult, GrammarComposer};

// Re-export codegen
pub mod codegen;
pub use codegen::{CodeGenerator, RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator, GoCodeGenerator, CCodeGenerator, CppCodeGenerator, JavaCodeGenerator};

// Re-export MCP server
pub mod mcp;
pub use mcp::{create_server, MinipgServer};
