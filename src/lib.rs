//! # minipg - A blazingly fast parser generator
//!
//! minipg is a modern parser generator with ANTLR4 compatibility, written in Rust.
//! It generates standalone parsers for multiple target languages without runtime dependencies.
//!
//! ## Features
//!
//! - **Multi-Language Support**: Generate parsers for Rust, Python, JavaScript, and TypeScript
//! - **ANTLR4 Compatible**: 100% compatible with ANTLR4 grammar syntax
//! - **Fast Generation**: Sub-millisecond code generation for typical grammars
//! - **Standalone Code**: No runtime dependencies - generates self-contained parsers
//! - **Modern Architecture**: Built with Rust 2024, modular design
//!
//! ## Example
//!
//! ```rust,no_run
//! use minipg::{parse_grammar, generate_code, TargetLanguage, CodegenConfig};
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
//! let grammar = parse_grammar(grammar_source).expect("Failed to parse grammar");
//!
//! // Generate Rust code
//! let config = CodegenConfig {
//!     target_language: TargetLanguage::Rust,
//!     output_dir: "output".into(),
//!     generate_visitor: true,
//!     generate_listener: true,
//! };
//!
//! let code = generate_code(&grammar, &config).expect("Failed to generate code");
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

// Re-export analysis
pub mod analysis;
pub use analysis::{SemanticAnalyzer, GrammarValidator, AnalysisResult};

// Re-export codegen
pub mod codegen;
pub use codegen::{CodeGenerator, RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator};
