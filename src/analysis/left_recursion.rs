//! Left recursion detection (direct and indirect).

use crate::ast::{Element, Grammar, Rule};
use std::collections::{HashMap, HashSet};

/// Detects both direct and indirect left recursion in grammar rules.
pub struct LeftRecursionDetector {
    /// First set cache for each rule
    first_sets: HashMap<String, HashSet<String>>,
}

impl LeftRecursionDetector {
    pub fn new() -> Self {
        Self {
            first_sets: HashMap::new(),
        }
    }
    
    /// Detect all left recursive rules (direct and indirect).
    pub fn detect(&mut self, grammar: &Grammar) -> Vec<LeftRecursion> {
        let mut results = Vec::new();
        
        // Build first sets for all rules
        self.build_first_sets(grammar);
        
        // Check each parser rule
        for rule in grammar.parser_rules() {
            if let Some(recursion) = self.check_rule(rule, grammar) {
                results.push(recursion);
            }
        }
        
        results
    }
    
    fn build_first_sets(&mut self, grammar: &Grammar) {
        // Initialize with empty sets
        for rule in &grammar.rules {
            self.first_sets.insert(rule.name.clone(), HashSet::new());
        }
        
        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;
            
            for rule in &grammar.rules {
                let old_size = self.first_sets.get(&rule.name).unwrap().len();
                self.compute_first_set(rule, grammar);
                let new_size = self.first_sets.get(&rule.name).unwrap().len();
                
                if new_size > old_size {
                    changed = true;
                }
            }
        }
    }
    
    fn compute_first_set(&mut self, rule: &Rule, _grammar: &Grammar) {
        let mut first = HashSet::new();
        
        for alt in &rule.alternatives {
            if let Some(first_elem) = alt.elements.first() {
                match first_elem {
                    Element::RuleRef { name, .. } => {
                        // Add first set of referenced rule
                        if let Some(ref_first) = self.first_sets.get(name) {
                            first.extend(ref_first.clone());
                        }
                    }
                    Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
                        first.insert(value.clone());
                    }
                    _ => {}
                }
            }
        }
        
        if let Some(set) = self.first_sets.get_mut(&rule.name) {
            set.extend(first);
        }
    }
    
    fn check_rule(&mut self, rule: &Rule, grammar: &Grammar) -> Option<LeftRecursion> {
        // Check for direct left recursion
        for alt in &rule.alternatives {
            if let Some(first_elem) = alt.elements.first() {
                if let Element::RuleRef { name, .. } = first_elem {
                    if name == &rule.name {
                        return Some(LeftRecursion {
                            rule_name: rule.name.clone(),
                            kind: LeftRecursionKind::Direct,
                            cycle: vec![rule.name.clone()],
                        });
                    }
                }
            }
        }
        
        // Check for indirect left recursion using cycle detection
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if let Some(cycle) = self.find_cycle(&rule.name, &rule.name, grammar, &mut visited, &mut path) {
            if cycle.len() > 1 {
                return Some(LeftRecursion {
                    rule_name: rule.name.clone(),
                    kind: LeftRecursionKind::Indirect,
                    cycle,
                });
            }
        }
        
        None
    }
    
    fn find_cycle(
        &self,
        start: &str,
        current: &str,
        grammar: &Grammar,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if path.contains(&current.to_string()) && current == start && !path.is_empty() {
            // Found a cycle back to start
            let mut cycle = path.clone();
            cycle.push(current.to_string());
            return Some(cycle);
        }
        
        if visited.contains(current) {
            return None;
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        // Get the rule
        if let Some(rule) = grammar.get_rule(current) {
            // Check first element of each alternative
            for alt in &rule.alternatives {
                if let Some(first_elem) = alt.elements.first() {
                    if let Element::RuleRef { name, .. } = first_elem {
                        if let Some(cycle) = self.find_cycle(start, name, grammar, visited, path) {
                            return Some(cycle);
                        }
                    }
                }
            }
        }
        
        path.pop();
        None
    }
}

impl Default for LeftRecursionDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of left recursion detected.
#[derive(Debug, Clone, PartialEq)]
pub enum LeftRecursionKind {
    Direct,
    Indirect,
}

/// Information about detected left recursion.
#[derive(Debug, Clone)]
pub struct LeftRecursion {
    pub rule_name: String,
    pub kind: LeftRecursionKind,
    pub cycle: Vec<String>,
}

impl LeftRecursion {
    pub fn cycle_description(&self) -> String {
        self.cycle.join(" -> ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Alternative, Element, Grammar, Rule};
    use crate::core::types::GrammarType;

    #[test]
    fn test_direct_left_recursion() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule = Rule::parser_rule("expr".to_string());
        let mut alt = Alternative::new();
        alt.add_element(Element::rule_ref("expr".to_string()));
        alt.add_element(Element::string_literal("+".to_string()));
        rule.add_alternative(alt);
        grammar.add_rule(rule);
        
        let mut detector = LeftRecursionDetector::new();
        let results = detector.detect(&grammar);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].kind, LeftRecursionKind::Direct);
        assert_eq!(results[0].rule_name, "expr");
    }

    #[test]
    fn test_indirect_left_recursion() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        // expr -> term
        let mut rule1 = Rule::parser_rule("expr".to_string());
        let mut alt1 = Alternative::new();
        alt1.add_element(Element::rule_ref("term".to_string()));
        rule1.add_alternative(alt1);
        grammar.add_rule(rule1);
        
        // term -> expr
        let mut rule2 = Rule::parser_rule("term".to_string());
        let mut alt2 = Alternative::new();
        alt2.add_element(Element::rule_ref("expr".to_string()));
        rule2.add_alternative(alt2);
        grammar.add_rule(rule2);
        
        let mut detector = LeftRecursionDetector::new();
        let results = detector.detect(&grammar);
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.kind == LeftRecursionKind::Indirect));
    }

    #[test]
    fn test_no_left_recursion() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule = Rule::parser_rule("expr".to_string());
        let mut alt = Alternative::new();
        alt.add_element(Element::string_literal("x".to_string()));
        rule.add_alternative(alt);
        grammar.add_rule(rule);
        
        let mut detector = LeftRecursionDetector::new();
        let results = detector.detect(&grammar);
        
        assert_eq!(results.len(), 0);
    }
}
