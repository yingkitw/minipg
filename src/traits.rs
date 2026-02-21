//! Core traits for capability-facing abstractions.

use super::{Diagnostic, Result};
use std::path::Path;

/// Trait for parsing grammar files.
pub trait GrammarParser {
    type Output;

    /// Parse a grammar from a file.
    fn parse_file(&self, path: &Path) -> Result<Self::Output>;

    /// Parse a grammar from a string.
    fn parse_string(&self, source: &str, filename: &str) -> Result<Self::Output>;
}

/// Trait for semantic analysis of grammars.
pub trait SemanticAnalyzer {
    type Input;
    type Output;

    /// Perform semantic analysis on the input.
    fn analyze(&self, input: &Self::Input) -> Result<Self::Output>;

    /// Get diagnostics from the analysis.
    fn diagnostics(&self) -> &[Diagnostic];
}

/// Trait for code generation.
pub trait CodeGenerator {
    type Input;
    type Config;

    /// Generate code for the given input and configuration.
    fn generate(&self, input: &Self::Input, config: &Self::Config) -> Result<String>;

    /// Get the target language name.
    fn target_language(&self) -> &str;
}

/// Trait for grammar transformation.
pub trait GrammarTransformer {
    type Input;
    type Output;

    /// Transform the grammar.
    fn transform(&self, input: Self::Input) -> Result<Self::Output>;
}

/// Trait for error reporting.
pub trait ErrorReporter {
    /// Report a diagnostic.
    fn report(&mut self, diagnostic: Diagnostic);

    /// Get all reported diagnostics.
    fn diagnostics(&self) -> &[Diagnostic];

    /// Check if any errors have been reported.
    fn has_errors(&self) -> bool;

    /// Get the number of errors.
    fn error_count(&self) -> usize;

    /// Get the number of warnings.
    fn warning_count(&self) -> usize;
}

/// Trait for grammar validation.
pub trait GrammarValidator {
    type Input;

    /// Validate the grammar.
    fn validate(&self, input: &Self::Input) -> Result<()>;
}
