//! Reachability analysis for grammar rules.

use crate::ast::{AstVisitor, Element, Grammar};
use std::collections::{HashMap, HashSet};

/// Analyzes which rules are reachable from the start rule.
pub struct ReachabilityAnalyzer {
    reachable: HashSet<String>,
    dependencies: HashMap<String, HashSet<String>>,
}

impl ReachabilityAnalyzer {
    pub fn new() -> Self {
        Self {
            reachable: HashSet::new(),
            dependencies: HashMap::new(),
        }
    }
    
    /// Analyze reachability starting from the first rule.
    pub fn analyze(&mut self, grammar: &Grammar) -> HashSet<String> {
        if grammar.rules.is_empty() {
            return HashSet::new();
        }
        
        // Build dependency graph
        for rule in &grammar.rules {
            let mut deps = HashSet::new();
            let mut visitor = DependencyCollector::new();
            visitor.visit_rule(rule);
            deps.extend(visitor.dependencies);
            self.dependencies.insert(rule.name.clone(), deps);
        }
        
        // Start from first rule
        let start_rule = &grammar.rules[0].name;
        self.mark_reachable(start_rule);
        
        self.reachable.clone()
    }
    
    fn mark_reachable(&mut self, rule_name: &str) {
        if self.reachable.contains(rule_name) {
            return;
        }
        
        self.reachable.insert(rule_name.to_string());
        
        // Clone dependencies to avoid borrow checker issues
        let deps: Vec<String> = self.dependencies
            .get(rule_name)
            .map(|d| d.iter().cloned().collect())
            .unwrap_or_default();
        
        for dep in deps {
            self.mark_reachable(&dep);
        }
    }
    
    /// Get unreachable rules.
    pub fn unreachable_rules(&self, grammar: &Grammar) -> Vec<String> {
        grammar
            .rules
            .iter()
            .filter(|r| !self.reachable.contains(&r.name))
            .map(|r| r.name.clone())
            .collect()
    }
}

impl Default for ReachabilityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Visitor to collect rule dependencies.
struct DependencyCollector {
    dependencies: HashSet<String>,
}

impl DependencyCollector {
    fn new() -> Self {
        Self {
            dependencies: HashSet::new(),
        }
    }
}

impl AstVisitor for DependencyCollector {
    fn visit_element(&mut self, element: &Element) {
        if let Element::RuleRef { name, .. } = element {
            self.dependencies.insert(name.clone());
        }
        self.walk_element(element);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Alternative, Element, Grammar, Rule};
    use crate::core::types::GrammarType;

    #[test]
    fn test_all_reachable() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule1 = Rule::parser_rule("start".to_string());
        let mut alt = Alternative::new();
        alt.add_element(Element::rule_ref("middle".to_string()));
        rule1.add_alternative(alt);
        grammar.add_rule(rule1);
        
        let mut rule2 = Rule::parser_rule("middle".to_string());
        let mut alt = Alternative::new();
        alt.add_element(Element::rule_ref("end".to_string()));
        rule2.add_alternative(alt);
        grammar.add_rule(rule2);
        
        let rule3 = Rule::parser_rule("end".to_string());
        grammar.add_rule(rule3);
        
        let mut analyzer = ReachabilityAnalyzer::new();
        let reachable = analyzer.analyze(&grammar);
        
        assert_eq!(reachable.len(), 3);
        assert!(reachable.contains("start"));
        assert!(reachable.contains("middle"));
        assert!(reachable.contains("end"));
    }

    #[test]
    fn test_unreachable_rule() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let rule1 = Rule::parser_rule("start".to_string());
        grammar.add_rule(rule1);
        
        let rule2 = Rule::parser_rule("unreachable".to_string());
        grammar.add_rule(rule2);
        
        let mut analyzer = ReachabilityAnalyzer::new();
        analyzer.analyze(&grammar);
        let unreachable = analyzer.unreachable_rules(&grammar);
        
        assert_eq!(unreachable.len(), 1);
        assert_eq!(unreachable[0], "unreachable");
    }
}
