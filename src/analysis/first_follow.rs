//! First and Follow set computation for grammar analysis.

use crate::ast::{Element, Grammar};
use std::collections::{HashMap, HashSet};

const EPSILON: &str = "ε";
const EOF: &str = "$";

/// Computes First and Follow sets for grammar rules.
pub struct FirstFollowComputer {
    first_sets: HashMap<String, HashSet<String>>,
    follow_sets: HashMap<String, HashSet<String>>,
    nullable: HashSet<String>,
}

impl FirstFollowComputer {
    pub fn new() -> Self {
        Self {
            first_sets: HashMap::new(),
            follow_sets: HashMap::new(),
            nullable: HashSet::new(),
        }
    }
    
    /// Compute First and Follow sets for all rules.
    pub fn compute(&mut self, grammar: &Grammar) {
        self.initialize(grammar);
        self.compute_first_sets(grammar);
        self.compute_follow_sets(grammar);
    }
    
    fn initialize(&mut self, grammar: &Grammar) {
        for rule in &grammar.rules {
            self.first_sets.insert(rule.name.clone(), HashSet::new());
            self.follow_sets.insert(rule.name.clone(), HashSet::new());
        }
        
        // Add EOF to follow set of start rule
        if let Some(start_rule) = grammar.rules.first() {
            self.follow_sets
                .get_mut(&start_rule.name)
                .unwrap()
                .insert(EOF.to_string());
        }
    }
    
    fn compute_first_sets(&mut self, grammar: &Grammar) {
        let mut changed = true;
        
        while changed {
            changed = false;
            
            for rule in &grammar.rules {
                let old_size = self.first_sets.get(&rule.name).unwrap().len();
                
                for alt in &rule.alternatives {
                    self.compute_first_of_sequence(&alt.elements, grammar);
                    
                    // Add first of alternative to first of rule
                    let first_of_alt = self.first_of_sequence(&alt.elements, grammar);
                    if let Some(first_set) = self.first_sets.get_mut(&rule.name) {
                        first_set.extend(first_of_alt);
                    }
                    
                    // Check if alternative is nullable
                    if self.is_sequence_nullable(&alt.elements) {
                        self.nullable.insert(rule.name.clone());
                    }
                }
                
                let new_size = self.first_sets.get(&rule.name).unwrap().len();
                if new_size > old_size {
                    changed = true;
                }
            }
        }
    }
    
    fn compute_first_of_sequence(&mut self, elements: &[Element], grammar: &Grammar) {
        for element in elements {
            match element {
                Element::RuleRef { name, .. } => {
                    // First set already computed in previous iteration
                    if !self.is_nullable(name) {
                        break;
                    }
                }
                Element::Terminal { .. } | Element::StringLiteral { .. } => {
                    break;
                }
                Element::Optional { .. } | Element::ZeroOrMore { .. } => {
                    // These are nullable, continue
                }
                Element::OneOrMore { element, .. } => {
                    // Not nullable, process inner element
                    self.compute_first_of_sequence(&[*element.clone()], grammar);
                    break;
                }
                _ => break,
            }
        }
    }
    
    fn first_of_sequence(&self, elements: &[Element], grammar: &Grammar) -> HashSet<String> {
        let mut result = HashSet::new();
        
        for element in elements {
            match element {
                Element::RuleRef { name, .. } => {
                    if let Some(first) = self.first_sets.get(name) {
                        result.extend(first.iter().filter(|s| *s != EPSILON).cloned());
                    }
                    if !self.is_nullable(name) {
                        break;
                    }
                }
                Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
                    result.insert(value.clone());
                    break;
                }
                Element::Optional { .. } | Element::ZeroOrMore { .. } => {
                    // Nullable, continue
                }
                Element::OneOrMore { element, .. } => {
                    result.extend(self.first_of_sequence(&[*element.clone()], grammar));
                    break;
                }
                Element::Wildcard => {
                    result.insert("ANY".to_string());
                    break;
                }
                _ => break,
            }
        }
        
        if result.is_empty() {
            result.insert(EPSILON.to_string());
        }
        
        result
    }
    
    fn compute_follow_sets(&mut self, grammar: &Grammar) {
        let mut changed = true;
        
        while changed {
            changed = false;
            
            for rule in &grammar.rules {
                for alt in &rule.alternatives {
                    for (i, element) in alt.elements.iter().enumerate() {
                        if let Element::RuleRef { name, .. } = element {
                            // Skip if rule doesn't exist (undefined rule)
                            if !self.follow_sets.contains_key(name) {
                                continue;
                            }
                            
                            let old_size = self.follow_sets.get(name).unwrap().len();
                            
                            // Get first of rest
                            let rest = &alt.elements[i + 1..];
                            let first_of_rest = self.first_of_sequence(rest, grammar);
                            
                            // Add first of rest (except epsilon) to follow
                            if let Some(follow) = self.follow_sets.get_mut(name) {
                                follow.extend(first_of_rest.iter().filter(|s| *s != EPSILON).cloned());
                            }
                            
                            // If rest is nullable, add follow of rule
                            if self.is_sequence_nullable(rest) {
                                if let Some(rule_follow) = self.follow_sets.get(&rule.name).cloned() {
                                    if let Some(follow) = self.follow_sets.get_mut(name) {
                                        follow.extend(rule_follow);
                                    }
                                }
                            }
                            
                            let new_size = self.follow_sets.get(name).unwrap().len();
                            if new_size > old_size {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn is_nullable(&self, rule_name: &str) -> bool {
        self.nullable.contains(rule_name)
    }
    
    fn is_sequence_nullable(&self, elements: &[Element]) -> bool {
        if elements.is_empty() {
            return true;
        }
        
        for element in elements {
            match element {
                Element::RuleRef { name, .. } => {
                    if !self.is_nullable(name) {
                        return false;
                    }
                }
                Element::Optional { .. } | Element::ZeroOrMore { .. } => {
                    // Nullable, continue
                }
                Element::Terminal { .. } | Element::StringLiteral { .. } | Element::OneOrMore { .. } => {
                    return false;
                }
                _ => return false,
            }
        }
        
        true
    }
    
    /// Get First set for a rule.
    pub fn first(&self, rule_name: &str) -> Option<&HashSet<String>> {
        self.first_sets.get(rule_name)
    }
    
    /// Get Follow set for a rule.
    pub fn follow(&self, rule_name: &str) -> Option<&HashSet<String>> {
        self.follow_sets.get(rule_name)
    }
    
    /// Check if a rule is nullable.
    pub fn is_rule_nullable(&self, rule_name: &str) -> bool {
        self.is_nullable(rule_name)
    }
}

impl Default for FirstFollowComputer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Alternative, Element, Grammar, Rule};
    use crate::core::types::GrammarType;

    #[test]
    fn test_first_set_terminal() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        let mut rule = Rule::parser_rule("expr".to_string());
        let mut alt = Alternative::new();
        alt.add_element(Element::string_literal("x".to_string()));
        rule.add_alternative(alt);
        grammar.add_rule(rule);
        
        let mut computer = FirstFollowComputer::new();
        computer.compute(&grammar);
        
        let first = computer.first("expr").unwrap();
        assert!(first.contains("x"));
    }

    #[test]
    fn test_first_set_rule_ref() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        // expr -> term
        let mut rule1 = Rule::parser_rule("expr".to_string());
        let mut alt1 = Alternative::new();
        alt1.add_element(Element::rule_ref("term".to_string()));
        rule1.add_alternative(alt1);
        grammar.add_rule(rule1);
        
        // term -> 'x'
        let mut rule2 = Rule::parser_rule("term".to_string());
        let mut alt2 = Alternative::new();
        alt2.add_element(Element::string_literal("x".to_string()));
        rule2.add_alternative(alt2);
        grammar.add_rule(rule2);
        
        let mut computer = FirstFollowComputer::new();
        computer.compute(&grammar);
        
        let first = computer.first("expr").unwrap();
        assert!(first.contains("x"));
    }

    #[test]
    fn test_follow_set() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        
        // expr -> term 'y'
        let mut rule1 = Rule::parser_rule("expr".to_string());
        let mut alt1 = Alternative::new();
        alt1.add_element(Element::rule_ref("term".to_string()));
        alt1.add_element(Element::string_literal("y".to_string()));
        rule1.add_alternative(alt1);
        grammar.add_rule(rule1);
        
        // term -> 'x'
        let mut rule2 = Rule::parser_rule("term".to_string());
        let mut alt2 = Alternative::new();
        alt2.add_element(Element::string_literal("x".to_string()));
        rule2.add_alternative(alt2);
        grammar.add_rule(rule2);
        
        let mut computer = FirstFollowComputer::new();
        computer.compute(&grammar);
        
        let follow = computer.follow("term").unwrap();
        assert!(follow.contains("y"));
    }
}
