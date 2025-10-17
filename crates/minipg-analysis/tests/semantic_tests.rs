//! Semantic analysis tests.

use minipg_ast::{Alternative, Element, Grammar, Rule};
use minipg_analysis::SemanticAnalyzer;
use minipg_core::{types::GrammarType, SemanticAnalyzer as SemanticAnalyzerTrait};

#[test]
fn test_undefined_rule_detection() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    let mut rule = Rule::parser_rule("start".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("undefined_rule".to_string()));
    rule.add_alternative(alt);
    grammar.add_rule(rule);

    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar).unwrap();
    
    assert!(result.has_errors());
    insta::assert_yaml_snapshot!(result.diagnostics);
}

#[test]
fn test_duplicate_rule_detection() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    grammar.add_rule(Rule::parser_rule("rule1".to_string()));
    grammar.add_rule(Rule::parser_rule("rule1".to_string()));

    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar).unwrap();
    
    assert!(result.has_errors());
    insta::assert_yaml_snapshot!(result.diagnostics);
}

#[test]
fn test_left_recursion_detection() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    // Add term rule first to avoid undefined rule error
    let term_rule = Rule::parser_rule("term".to_string());
    grammar.add_rule(term_rule);
    
    let mut rule = Rule::parser_rule("expr".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("expr".to_string()));
    alt.add_element(Element::string_literal("+".to_string()));
    alt.add_element(Element::rule_ref("term".to_string()));
    rule.add_alternative(alt);
    grammar.add_rule(rule);

    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar).unwrap();
    
    assert!(!result.has_errors());
    assert!(!result.diagnostics.is_empty()); // Should have warning
    insta::assert_yaml_snapshot!(result.diagnostics);
}
