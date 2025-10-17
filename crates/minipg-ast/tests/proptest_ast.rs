//! Property-based tests for AST operations.

use minipg_ast::{Alternative, Element, Grammar, Rule};
use minipg_core::types::GrammarType;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_grammar_add_rule(name in "[A-Z][a-zA-Z0-9]*") {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        let rule = Rule::lexer_rule(name.clone());
        grammar.add_rule(rule);
        
        prop_assert_eq!(grammar.rules.len(), 1);
        prop_assert_eq!(&grammar.rules[0].name, &name);
    }
    
    #[test]
    fn test_rule_add_alternatives(count in 1..10usize) {
        let mut rule = Rule::parser_rule("test".to_string());
        
        for _ in 0..count {
            rule.add_alternative(Alternative::new());
        }
        
        prop_assert_eq!(rule.alternatives.len(), count);
    }
    
    #[test]
    fn test_alternative_add_elements(count in 1..10usize) {
        let mut alt = Alternative::new();
        
        for i in 0..count {
            alt.add_element(Element::string_literal(format!("x{}", i)));
        }
        
        prop_assert_eq!(alt.elements.len(), count);
    }
    
    #[test]
    fn test_grammar_get_rule(name in "[a-z][a-zA-Z0-9]*") {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        let rule = Rule::parser_rule(name.clone());
        grammar.add_rule(rule);
        
        let found = grammar.get_rule(&name);
        prop_assert!(found.is_some());
        prop_assert_eq!(&found.unwrap().name, &name);
    }
    
    #[test]
    fn test_element_with_label(label in "[a-z]+") {
        let elem = Element::rule_ref("test".to_string()).with_label(label.clone());
        
        match elem {
            Element::RuleRef { label: Some(l), .. } => {
                prop_assert_eq!(l, label);
            }
            _ => prop_assert!(false, "Expected RuleRef with label"),
        }
    }
}
