//! Semantic analysis for grammars.

pub mod ambiguity;
pub mod composition;
pub mod first_follow;
pub mod hooks;
pub mod left_recursion;
pub mod reachability;
pub mod semantic;
pub mod validator;

pub use composition::GrammarComposer;
pub use hooks::{AnalysisContext, AnalysisHook, HookRegistry, HookResult};
pub use hooks::{ComplexityHook, NamingConventionHook};
pub use semantic::SemanticAnalyzer;
pub use validator::GrammarValidator;

use crate::ast::Grammar;
use crate::Diagnostic;

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
            .any(|d| matches!(d.severity, crate::DiagnosticSeverity::Error))
    }
}
