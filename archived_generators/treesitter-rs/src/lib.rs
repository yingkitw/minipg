//! # treesitter-rs
//!
//! Tree-sitter grammar generation and query support for minipg.
//!
//! This crate provides Tree-sitter compatibility features including:
//! - Grammar.js generation from ANTLR grammars
//! - Tree-sitter compatible query language
//! - Pattern matching and capture groups

// Re-export core types
pub use minipg_core::*;

// Re-export ANTLR types (needed for Tree-sitter generation)
pub use minipg_antlr::{Alternative, Element, Grammar, Rule};

pub mod codegen;
pub mod query;

pub use codegen::treesitter::TreeSitterCodeGenerator;
pub use query::{Capture, CaptureGroup, Match, Pattern, PatternNode, QueryMatcher, QueryParser};
