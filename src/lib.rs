//! # minipg - Minimal Parser Generator
//!
//! A modern parser generator supporting ANTLR4 grammars with code generation
//! for Rust, Python, and JavaScript.

// Core types and traits
pub mod diagnostic;
pub mod error;
pub mod traits;
pub mod types;

// ANTLR4 parser and AST
pub mod ast;
pub mod parser;

// Analysis and code generation
pub mod analysis;
pub mod codegen;

// CLI (only for binary)
#[cfg(feature = "cli")]
pub mod cli;

// Re-exports for convenience
pub use diagnostic::{Diagnostic, DiagnosticSeverity, Location};
pub use error::{Error, Result};
pub use traits::{CodeGenerator, GrammarParser, GrammarValidator, SemanticAnalyzer};
pub use types::{CodeGenConfig, GrammarType, Point, Position, Range, SymbolTable};
pub use ast::Grammar;
