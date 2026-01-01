//! Query language for pattern matching in syntax trees.
//!
//! This module provides a Tree-sitter-compatible query language for
//! extracting patterns from syntax trees, primarily used for syntax highlighting.

pub mod parser;
pub mod pattern;
pub mod matcher;
pub mod capture;

pub use parser::QueryParser;
pub use pattern::{Pattern, PatternNode};
pub use matcher::{QueryMatcher, Match, Capture};
pub use capture::CaptureGroup;
