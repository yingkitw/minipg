//! Semantic analysis implementation.

use crate::{
    ambiguity::AmbiguityDetector,
    left_recursion::LeftRecursionDetector, reachability::ReachabilityAnalyzer, AnalysisResult,
};
use minipg_ast::{AstVisitor, Grammar};
use minipg_core::{Diagnostic, Result, SemanticAnalyzer as SemanticAnalyzerTrait};
use std::collections::{HashMap, HashSet};

/// Semantic analyzer for grammars.
pub struct SemanticAnalyzer {
    diagnostics: Vec<Diagnostic>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    fn check_undefined_rules(&mut self, grammar: &Grammar) {
        let defined_rules: HashSet<String> = grammar.rules.iter().map(|r| r.name.clone()).collect();
        let mut referenced_rules: HashSet<String> = HashSet::new();

        // Collect all rule references
        let mut visitor = RuleRefCollector::new();
        visitor.visit_grammar(grammar);
        referenced_rules.extend(visitor.references);

        // Check for undefined rules
        for rule_ref in &referenced_rules {
            if !defined_rules.contains(rule_ref) {
                self.diagnostics.push(
                    Diagnostic::error(format!("undefined rule: {}", rule_ref))
                        .with_code("E001"),
                );
            }
        }
    }

    fn check_duplicate_rules(&mut self, grammar: &Grammar) {
        let mut seen: HashMap<String, usize> = HashMap::new();

        for rule in &grammar.rules {
            if let Some(count) = seen.get_mut(&rule.name) {
                *count += 1;
                if *count == 2 {
                    self.diagnostics.push(
                        Diagnostic::error(format!("duplicate rule definition: {}", rule.name))
                            .with_code("E002"),
                    );
                }
            } else {
                seen.insert(rule.name.clone(), 1);
            }
        }
    }

    fn check_empty_alternatives(&mut self, grammar: &Grammar) {
        for rule in &grammar.rules {
            for (i, alt) in rule.alternatives.iter().enumerate() {
                if alt.elements.is_empty() {
                    self.diagnostics.push(
                        Diagnostic::warning(format!(
                            "empty alternative in rule '{}' (alternative {})",
                            rule.name,
                            i + 1
                        ))
                        .with_code("W001"),
                    );
                }
            }
        }
    }

    fn check_left_recursion(&mut self, grammar: &Grammar) {
        // Use advanced left recursion detector
        let mut detector = LeftRecursionDetector::new();
        let recursions = detector.detect(grammar);
        
        for recursion in recursions {
            let message = match recursion.kind {
                crate::left_recursion::LeftRecursionKind::Direct => {
                    format!("direct left recursion in rule '{}'", recursion.rule_name)
                }
                crate::left_recursion::LeftRecursionKind::Indirect => {
                    format!(
                        "indirect left recursion in rule '{}': {}",
                        recursion.rule_name,
                        recursion.cycle_description()
                    )
                }
            };
            
            self.diagnostics.push(Diagnostic::warning(message).with_code("W002"));
        }
    }
    
    fn check_unreachable_rules(&mut self, grammar: &Grammar) {
        let mut analyzer = ReachabilityAnalyzer::new();
        analyzer.analyze(grammar);
        let unreachable = analyzer.unreachable_rules(grammar);
        
        for rule_name in unreachable {
            self.diagnostics.push(
                Diagnostic::warning(format!("unreachable rule: {}", rule_name))
                    .with_code("W003"),
            );
        }
    }
    
    fn check_ambiguous_alternatives(&mut self, grammar: &Grammar) {
        let mut detector = AmbiguityDetector::new();
        let ambiguities = detector.detect(grammar);
        
        for ambiguity in ambiguities {
            self.diagnostics.push(
                Diagnostic::warning(format!(
                    "ambiguous alternatives in rule '{}': {}",
                    ambiguity.rule_name,
                    ambiguity.description()
                ))
                .with_code("W004"),
            );
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzerTrait for SemanticAnalyzer {
    type Input = Grammar;
    type Output = AnalysisResult;

    fn analyze(&self, input: &Self::Input) -> Result<Self::Output> {
        let mut analyzer = Self::new();
        
        analyzer.check_undefined_rules(input);
        analyzer.check_duplicate_rules(input);
        analyzer.check_empty_alternatives(input);
        analyzer.check_left_recursion(input);
        analyzer.check_unreachable_rules(input);
        analyzer.check_ambiguous_alternatives(input);

        let mut result = AnalysisResult::new(input.clone());
        result.diagnostics = analyzer.diagnostics;

        Ok(result)
    }

    fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// Visitor to collect rule references.
struct RuleRefCollector {
    references: HashSet<String>,
}

impl RuleRefCollector {
    fn new() -> Self {
        Self {
            references: HashSet::new(),
        }
    }
}

impl AstVisitor for RuleRefCollector {
    fn visit_element(&mut self, element: &minipg_ast::Element) {
        if let minipg_ast::Element::RuleRef { name, .. } = element {
            self.references.insert(name.clone());
        }
        self.walk_element(element);
    }
}
