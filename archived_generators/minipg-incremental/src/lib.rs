//! Incremental parsing support for editor integration.
//!
//! This crate provides position tracking and incremental parsing capabilities
//! to enable fast re-parsing of edited documents.

pub mod edit;
pub mod lazy;
pub mod parallel;
pub mod parser;
pub mod position;

pub use edit::Edit;
pub use lazy::{LazyConfig, LazyParser, ParsedRegion};
pub use parallel::{ParallelParser, ParseJob, ParseResult};
pub use parser::{
    DefaultIncrementalParser, IncrementalConfig, IncrementalParser, ParseMetrics, SyntaxTree,
};
pub use position::{Point, Position, Range};
