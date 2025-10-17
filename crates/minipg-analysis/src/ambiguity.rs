//! Ambiguous alternative detection.

use crate::first_follow::FirstFollowComputer;
use minipg_ast::{Grammar, Rule};
use std::collections::HashSet;

/// Detects ambiguous alternatives in grammar rules.
pub struct AmbiguityDetector {
    first_follow: FirstFollowComputer,
}

impl AmbiguityDetector {
    pub fn new() -> Self {
        Self {
            first_follow: FirstFollowComputer::new(),
        }
    }
    
    /// Detect ambiguous alternatives in the grammar.
    pub fn detect(&mut self, grammar: &Grammar) -> Vec<Ambiguity> {
        let mut results = Vec::new();
        
        // Compute first/follow sets
        self.first_follow.compute(grammar);
        
        // Check each rule for ambiguous alternatives
        for rule in grammar.parser_rules() {
            if let Some(ambiguities) = self.check_rule(rule) {
                results.extend(ambiguities);
            }
        }
        
        results
    }
    
    fn check_rule(&self, rule: &Rule) -> Option<Vec<Ambiguity>> {
        if rule.alternatives.len() < 2 {
            return None;
        }
        
        let mut ambiguities = Vec::new();
        
        // Check each pair of alternatives
        for i in 0..rule.alternatives.len() {
            for j in (i + 1)..rule.alternatives.len() {
                let alt1 = &rule.alternatives[i];
                let alt2 = &rule.alternatives[j];
                
                // Get first sets of both alternatives
                let first1 = self.first_of_alternative(alt1);
                let first2 = self.first_of_alternative(alt2);
                
                // Check for overlap
                let overlap: HashSet<_> = first1.intersection(&first2).cloned().collect();
                
                if !overlap.is_empty() {
                    ambiguities.push(Ambiguity {
                        rule_name: rule.name.clone(),
                        alternative1: i,
                        alternative2: j,
                        conflicting_tokens: overlap.into_iter().collect(),
                    });
                }
            }
        }
        
        if ambiguities.is_empty() {
            None
        } else {
            Some(ambiguities)
        }
    }
    
    fn first_of_alternative(&self, alt: &minipg_ast::Alternative) -> HashSet<String> {
        let mut result = HashSet::new();
        
        for element in &alt.elements {
            match element {
                minipg_ast::Element::RuleRef { name, .. } => {
                    if let Some(first) = self.first_follow.first(name) {
                        result.extend(first.iter().cloned());
                    }
                    // If not nullable, stop here
                    if !self.first_follow.is_rule_nullable(name) {
                        break;
                    }
                }
                minipg_ast::Element::Terminal { value, .. }
                | minipg_ast::Element::StringLiteral { value, .. } => {
                    result.insert(value.clone());
                    break;
                }
                minipg_ast::Element::Optional { .. } | minipg_ast::Element::ZeroOrMore { .. } => {
                    // Nullable, continue
                }
                minipg_ast::Element::OneOrMore { element } => {
                    // Get first of inner element
                    result.extend(self.first_of_alternative(&minipg_ast::Alternative {
                        elements: vec![*element.clone()],
                        label: None,
                    }));
                    break;
                }
                _ => break,
            }
        }
        
        result
    }
}

impl Default for AmbiguityDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about detected ambiguity.
#[derive(Debug, Clone)]
pub struct Ambiguity {
    pub rule_name: String,
    pub alternative1: usize,
    pub alternative2: usize,
    pub conflicting_tokens: Vec<String>,
}

impl Ambiguity {
    pub fn description(&self) -> String {
        format!(
            "alternatives {} and {} conflict on: {}",
            self.alternative1 + 1,
            self.alternative2 + 1,
            self.conflicting_tokens.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minipg_ast::{Alternative, Element, Grammar, Rule};
    use minipg_core::types::GrammarType;

    #[test]
    fn test_no_ambiguity() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule = Rule::parser_rule("expr".to_string());
        
        let mut alt1 = Alternative::new();
        alt1.add_element(Element::string_literal("x".to_string()));
        rule.add_alternative(alt1);
        
        let mut alt2 = Alternative::new();
        alt2.add_element(Element::string_literal("y".to_string()));
        rule.add_alternative(alt2);
        
        grammar.add_rule(rule);
        
        let mut detector = AmbiguityDetector::new();
        let results = detector.detect(&grammar);
        
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_ambiguous_alternatives() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule = Rule::parser_rule("expr".to_string());
        
        let mut alt1 = Alternative::new();
        alt1.add_element(Element::string_literal("x".to_string()));
        rule.add_alternative(alt1);
        
        let mut alt2 = Alternative::new();
        alt2.add_element(Element::string_literal("x".to_string()));
        rule.add_alternative(alt2);
        
        grammar.add_rule(rule);
        
        let mut detector = AmbiguityDetector::new();
        let results = detector.detect(&grammar);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].rule_name, "expr");
        assert!(results[0].conflicting_tokens.contains(&"x".to_string()));
    }
}
