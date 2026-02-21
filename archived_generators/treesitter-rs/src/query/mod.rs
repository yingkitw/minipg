//! Query language for pattern matching in syntax trees.
//!
//! This module provides a Tree-sitter-compatible query language for
//! extracting patterns from syntax trees, primarily used for syntax highlighting.

pub mod capture;
pub mod matcher;
pub mod parser;
pub mod pattern;

pub use capture::CaptureGroup;
pub use matcher::{Capture, Match, QueryMatcher};
pub use parser::QueryParser;
pub use pattern::{Pattern, PatternNode};
