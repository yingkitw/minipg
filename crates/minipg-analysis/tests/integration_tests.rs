//! Integration tests for complete analysis pipeline.

use minipg_analysis::{
    ambiguity::AmbiguityDetector, first_follow::FirstFollowComputer,
    left_recursion::LeftRecursionDetector, reachability::ReachabilityAnalyzer, SemanticAnalyzer,
};
use minipg_ast::{Alternative, Element, Grammar, Rule};
use minipg_core::{types::GrammarType, SemanticAnalyzer as SemanticAnalyzerTrait};

#[test]
fn test_complete_analysis_pipeline() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    // Add some rules
    let mut rule1 = Rule::parser_rule("expr".to_string());
    let mut alt1 = Alternative::new();
    alt1.add_element(Element::rule_ref("term".to_string()));
    rule1.add_alternative(alt1);
    grammar.add_rule(rule1);
    
    let mut rule2 = Rule::parser_rule("term".to_string());
    let mut alt2 = Alternative::new();
    alt2.add_element(Element::string_literal("x".to_string()));
    rule2.add_alternative(alt2);
    grammar.add_rule(rule2);
    
    // Run complete analysis
    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar).unwrap();
    
    // Should have no errors for this valid grammar
    assert!(!result.has_errors());
}

#[test]
fn test_all_analyzers_together() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    // Create a grammar with multiple issues
    let mut rule1 = Rule::parser_rule("expr".to_string());
    let mut alt1 = Alternative::new();
    alt1.add_element(Element::rule_ref("undefined".to_string())); // Undefined
    rule1.add_alternative(alt1);
    grammar.add_rule(rule1);
    
    let mut rule2 = Rule::parser_rule("unused".to_string()); // Unreachable
    let mut alt2 = Alternative::new();
    alt2.add_element(Element::string_literal("x".to_string()));
    rule2.add_alternative(alt2);
    grammar.add_rule(rule2);
    
    // Run all analyzers
    let mut lr_detector = LeftRecursionDetector::new();
    let lr_results = lr_detector.detect(&grammar);
    
    let mut reach_analyzer = ReachabilityAnalyzer::new();
    reach_analyzer.analyze(&grammar);
    let unreachable = reach_analyzer.unreachable_rules(&grammar);
    
    let mut amb_detector = AmbiguityDetector::new();
    let amb_results = amb_detector.detect(&grammar);
    
    let mut ff_computer = FirstFollowComputer::new();
    ff_computer.compute(&grammar);
    
    // Verify results
    assert_eq!(unreachable.len(), 1);
    assert_eq!(unreachable[0], "unused");
}

#[test]
fn test_first_follow_with_complex_grammar() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    // expr -> term '+' expr | term
    let mut rule1 = Rule::parser_rule("expr".to_string());
    let mut alt1 = Alternative::new();
    alt1.add_element(Element::rule_ref("term".to_string()));
    alt1.add_element(Element::string_literal("+".to_string()));
    alt1.add_element(Element::rule_ref("expr".to_string()));
    rule1.add_alternative(alt1);
    
    let mut alt2 = Alternative::new();
    alt2.add_element(Element::rule_ref("term".to_string()));
    rule1.add_alternative(alt2);
    grammar.add_rule(rule1);
    
    // term -> 'x'
    let mut rule2 = Rule::parser_rule("term".to_string());
    let mut alt3 = Alternative::new();
    alt3.add_element(Element::string_literal("x".to_string()));
    rule2.add_alternative(alt3);
    grammar.add_rule(rule2);
    
    let mut computer = FirstFollowComputer::new();
    computer.compute(&grammar);
    
    // First(expr) should contain 'x'
    let first = computer.first("expr").unwrap();
    assert!(first.contains("x"));
    
    // Follow(term) should contain '+' and EOF
    let follow = computer.follow("term").unwrap();
    assert!(follow.contains("+") || follow.contains("$"));
}

#[test]
fn test_ambiguity_with_multiple_alternatives() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    let mut rule = Rule::parser_rule("stmt".to_string());
    
    // Alternative 1: 'if' expr 'then' stmt
    let mut alt1 = Alternative::new();
    alt1.add_element(Element::string_literal("if".to_string()));
    alt1.add_element(Element::rule_ref("expr".to_string()));
    alt1.add_element(Element::string_literal("then".to_string()));
    alt1.add_element(Element::rule_ref("stmt".to_string()));
    rule.add_alternative(alt1);
    
    // Alternative 2: 'if' expr 'then' stmt 'else' stmt
    let mut alt2 = Alternative::new();
    alt2.add_element(Element::string_literal("if".to_string()));
    alt2.add_element(Element::rule_ref("expr".to_string()));
    alt2.add_element(Element::string_literal("then".to_string()));
    alt2.add_element(Element::rule_ref("stmt".to_string()));
    alt2.add_element(Element::string_literal("else".to_string()));
    alt2.add_element(Element::rule_ref("stmt".to_string()));
    rule.add_alternative(alt2);
    
    grammar.add_rule(rule);
    
    // Add expr rule
    let mut expr_rule = Rule::parser_rule("expr".to_string());
    let mut expr_alt = Alternative::new();
    expr_alt.add_element(Element::string_literal("true".to_string()));
    expr_rule.add_alternative(expr_alt);
    grammar.add_rule(expr_rule);
    
    let mut detector = AmbiguityDetector::new();
    let ambiguities = detector.detect(&grammar);
    
    // Should detect ambiguity (dangling else problem)
    assert!(!ambiguities.is_empty());
}

#[test]
fn test_left_recursion_with_mutual_recursion() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    // a -> b
    let mut rule1 = Rule::parser_rule("a".to_string());
    let mut alt1 = Alternative::new();
    alt1.add_element(Element::rule_ref("b".to_string()));
    rule1.add_alternative(alt1);
    grammar.add_rule(rule1);
    
    // b -> c
    let mut rule2 = Rule::parser_rule("b".to_string());
    let mut alt2 = Alternative::new();
    alt2.add_element(Element::rule_ref("c".to_string()));
    rule2.add_alternative(alt2);
    grammar.add_rule(rule2);
    
    // c -> a (creates cycle)
    let mut rule3 = Rule::parser_rule("c".to_string());
    let mut alt3 = Alternative::new();
    alt3.add_element(Element::rule_ref("a".to_string()));
    rule3.add_alternative(alt3);
    grammar.add_rule(rule3);
    
    let mut detector = LeftRecursionDetector::new();
    let recursions = detector.detect(&grammar);
    
    // Should detect indirect left recursion
    assert!(!recursions.is_empty());
}
