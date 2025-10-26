//! Semantic analysis for grammars.

pub mod semantic;
pub mod validator;
pub mod reachability;
pub mod left_recursion;
pub mod first_follow;
pub mod ambiguity;
pub mod composition;

pub use semantic::SemanticAnalyzer;
pub use validator::GrammarValidator;
pub use composition::GrammarComposer;

use crate::ast::Grammar;
use crate::core::Diagnostic;

/// Analysis result containing validated grammar and diagnostics.
#[derive(Debug)]
pub struct AnalysisResult {
    pub grammar: Grammar,
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisResult {
    pub fn new(grammar: Grammar) -> Self {
        Self {
            grammar,
            diagnostics: Vec::new(),
        }
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, crate::core::DiagnosticSeverity::Error))
    }
}
