//! Core types and traits for the minipg parser generator.
//!
//! This crate provides the foundational abstractions used across all minipg crates,
//! including error handling, diagnostics, and core traits.

pub mod error;
pub mod diagnostic;
pub mod traits;
pub mod types;

pub use error::{Error, Result};
pub use diagnostic::{Diagnostic, DiagnosticSeverity, Location};
pub use traits::*;
