//! Custom semantic analysis hooks for extensibility.

use crate::ast::Grammar;
use crate::core::{Diagnostic, DiagnosticSeverity};
use std::sync::Arc;

/// Context for semantic analysis hooks.
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub grammar: Grammar,
    pub source: String,
}

impl AnalysisContext {
    pub fn new(grammar: Grammar, source: impl Into<String>) -> Self {
        Self {
            grammar,
            source: source.into(),
        }
    }
}

/// Result of a semantic analysis hook.
#[derive(Debug, Clone)]
pub struct HookResult {
    pub diagnostics: Vec<Diagnostic>,
    pub grammar: Option<Grammar>,
}

impl HookResult {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            grammar: None,
        }
    }

    pub fn with_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, DiagnosticSeverity::Error))
    }
}

impl Default for HookResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for semantic analysis hooks.
pub trait AnalysisHook: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, context: &AnalysisContext) -> HookResult;
}

/// Boxed hook for storage.
pub type BoxedHook = Arc<dyn AnalysisHook>;

/// Simple registry for analysis hooks.
pub struct HookRegistry {
    hooks: Vec<BoxedHook>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    /// Register a hook.
    pub fn register(&mut self, hook: BoxedHook) {
        self.hooks.push(hook);
    }

    /// Run all hooks on a grammar.
    pub fn run_all(&self, context: &AnalysisContext) -> HookResult {
        let mut result = HookResult::new();
        let mut current_grammar = context.grammar.clone();

        for hook in &self.hooks {
            let hook_context = AnalysisContext {
                grammar: current_grammar.clone(),
                source: context.source.clone(),
            };

            let hook_result = hook.run(&hook_context);

            // Stop on errors
            if hook_result.has_errors() {
                result.diagnostics.extend(hook_result.diagnostics);
                return result;
            }

            result.diagnostics.extend(hook_result.diagnostics);

            // Update grammar if modified
            if let Some(grammar) = hook_result.grammar {
                current_grammar = grammar;
            }
        }

        result.grammar = Some(current_grammar);
        result
    }

    pub fn count(&self) -> usize {
        self.hooks.len()
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Example hook: Naming convention checker.
pub struct NamingConventionHook {
    pub lexer_suffix: Option<String>,
}

impl NamingConventionHook {
    pub fn new() -> Self {
        Self { lexer_suffix: None }
    }

    pub fn with_lexer_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.lexer_suffix = Some(suffix.into());
        self
    }
}

impl Default for NamingConventionHook {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisHook for NamingConventionHook {
    fn name(&self) -> &str {
        "naming-convention"
    }

    fn description(&self) -> &str {
        "Check naming conventions for lexer rules"
    }

    fn run(&self, context: &AnalysisContext) -> HookResult {
        let mut result = HookResult::new();

        if let Some(suffix) = &self.lexer_suffix {
            for rule in &context.grammar.rules {
                // Check if it's a lexer rule (uppercase first letter)
                if rule.name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    if !rule.name.ends_with(suffix) {
                        result.diagnostics.push(
                            Diagnostic::warning(format!(
                                "Lexer rule '{}' should end with '{}'",
                                rule.name, suffix
                            ))
                            .with_code("N001")
                        );
                    }
                }
            }
        }

        result
    }
}

/// Example hook: Rule complexity checker.
pub struct ComplexityHook {
    pub max_alternatives: usize,
}

impl ComplexityHook {
    pub fn new() -> Self {
        Self { max_alternatives: 10 }
    }

    pub fn with_max_alternatives(mut self, max: usize) -> Self {
        self.max_alternatives = max;
        self
    }
}

impl Default for ComplexityHook {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisHook for ComplexityHook {
    fn name(&self) -> &str {
        "complexity"
    }

    fn description(&self) -> &str {
        "Check for overly complex rules"
    }

    fn run(&self, context: &AnalysisContext) -> HookResult {
        let mut result = HookResult::new();

        for rule in &context.grammar.rules {
            if rule.alternatives.len() > self.max_alternatives {
                result.diagnostics.push(
                    Diagnostic::warning(format!(
                        "Rule '{}' has {} alternatives (max: {})",
                        rule.name,
                        rule.alternatives.len(),
                        self.max_alternatives
                    ))
                    .with_code("C001")
                );
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Rule, Alternative};

    #[test]
    fn test_analysis_context() {
        let grammar = Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let context = AnalysisContext::new(grammar.clone(), "test.g4");

        assert_eq!(context.source, "test.g4");
    }

    #[test]
    fn test_hook_result_new() {
        let result = HookResult::new();
        assert!(!result.has_errors());
    }

    #[test]
    fn test_hook_result_with_diagnostic() {
        let result = HookResult::new()
            .with_diagnostic(Diagnostic::error("test error"));

        assert!(result.has_errors());
    }

    #[test]
    fn test_hook_registry() {
        let mut registry = HookRegistry::new();
        assert_eq!(registry.count(), 0);

        let hook = Arc::new(NamingConventionHook::new());
        registry.register(hook);

        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_naming_convention_hook() {
        let hook = NamingConventionHook::new()
            .with_lexer_suffix("_TOKEN");

        let mut grammar = Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let rule = Rule::new("ID".to_string(), crate::ast::RuleType::Lexer);
        grammar.add_rule(rule);

        let context = AnalysisContext::new(grammar, "test.g4");
        let result = hook.run(&context);

        assert!(result.diagnostics.len() > 0);
    }

    #[test]
    fn test_complexity_hook() {
        let hook = ComplexityHook::new()
            .with_max_alternatives(2);

        let mut grammar = Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let mut rule = Rule::new("test".to_string(), crate::ast::RuleType::Parser);

        for _ in 0..5 {
            rule.add_alternative(Alternative::new());
        }

        grammar.add_rule(rule);

        let context = AnalysisContext::new(grammar, "test.g4");
        let result = hook.run(&context);

        assert!(result.diagnostics.len() > 0);
    }

    #[test]
    fn test_hook_registry_run_all() {
        let mut registry = HookRegistry::new();

        let hook1 = Arc::new(NamingConventionHook::new().with_lexer_suffix("_TOKEN"));
        let hook2 = Arc::new(ComplexityHook::new().with_max_alternatives(2));

        registry.register(hook1);
        registry.register(hook2);

        let grammar = Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let context = AnalysisContext::new(grammar, "test.g4");

        let result = registry.run_all(&context);
        // Should have run both hooks
    }
}
