//! Incremental parsing support for editor integration.
//!
//! This module provides position tracking and incremental parsing capabilities
//! to enable fast re-parsing of edited documents.

pub mod position;
pub mod edit;
pub mod parser;
pub mod lazy;
pub mod parallel;

pub use position::{Point, Position, Range};
pub use edit::Edit;
pub use parser::{IncrementalParser, SyntaxTree, DefaultIncrementalParser, ParseMetrics, IncrementalConfig};
pub use lazy::{LazyParser, LazyConfig, ParsedRegion};
pub use parallel::{ParallelParser, ParseJob, ParseResult};
