//! AST tests.

use minipg_ast::{Alternative, Element, Grammar, Rule};
use minipg_core::types::GrammarType;

#[test]
fn test_grammar_creation() {
    let grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    assert_eq!(grammar.name, "Test");
    assert_eq!(grammar.grammar_type, GrammarType::Parser);
    assert!(grammar.rules.is_empty());
    assert!(grammar.options.is_empty());
}

#[test]
fn test_grammar_add_rule() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    let rule = Rule::parser_rule("expr".to_string());
    grammar.add_rule(rule);
    
    assert_eq!(grammar.rules.len(), 1);
    assert_eq!(grammar.rules[0].name, "expr");
}

#[test]
fn test_grammar_get_rule() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    grammar.add_rule(Rule::parser_rule("expr".to_string()));
    
    assert!(grammar.get_rule("expr").is_some());
    assert!(grammar.get_rule("unknown").is_none());
}

#[test]
fn test_grammar_options() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    grammar.add_option("language".to_string(), "rust".to_string());
    
    assert_eq!(grammar.options.get("language"), Some(&"rust".to_string()));
}

#[test]
fn test_grammar_imports() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    grammar.add_import("Common".to_string());
    
    assert_eq!(grammar.imports.len(), 1);
    assert_eq!(grammar.imports[0], "Common");
}

#[test]
fn test_rule_creation() {
    let rule = Rule::parser_rule("expr".to_string());
    assert_eq!(rule.name, "expr");
    assert!(rule.is_parser_rule());
    assert!(!rule.is_lexer_rule());
    assert!(!rule.is_fragment);
}

#[test]
fn test_lexer_rule() {
    let rule = Rule::lexer_rule("ID".to_string());
    assert!(rule.is_lexer_rule());
    assert!(!rule.is_parser_rule());
}

#[test]
fn test_fragment_rule() {
    let mut rule = Rule::lexer_rule("DIGIT".to_string());
    rule.set_fragment(true);
    assert!(rule.is_fragment);
}

#[test]
fn test_rule_alternatives() {
    let mut rule = Rule::parser_rule("expr".to_string());
    let alt = Alternative::new();
    rule.add_alternative(alt);
    
    assert_eq!(rule.alternatives.len(), 1);
}

#[test]
fn test_alternative_creation() {
    let mut alt = Alternative::new();
    assert!(alt.elements.is_empty());
    assert!(alt.label.is_none());
    
    alt.add_element(Element::rule_ref("term".to_string()));
    assert_eq!(alt.elements.len(), 1);
}

#[test]
fn test_alternative_with_label() {
    let alt = Alternative::new().with_label("myLabel".to_string());
    assert_eq!(alt.label, Some("myLabel".to_string()));
}

#[test]
fn test_element_rule_ref() {
    let elem = Element::rule_ref("expr".to_string());
    match elem {
        Element::RuleRef { name, label } => {
            assert_eq!(name, "expr");
            assert!(label.is_none());
        }
        _ => panic!("Expected RuleRef"),
    }
}

#[test]
fn test_element_terminal() {
    let elem = Element::terminal("ID".to_string());
    match elem {
        Element::Terminal { value, .. } => {
            assert_eq!(value, "ID");
        }
        _ => panic!("Expected Terminal"),
    }
}

#[test]
fn test_element_string_literal() {
    let elem = Element::string_literal("+".to_string());
    match elem {
        Element::StringLiteral { value, .. } => {
            assert_eq!(value, "+");
        }
        _ => panic!("Expected StringLiteral"),
    }
}

#[test]
fn test_element_optional() {
    let inner = Element::rule_ref("term".to_string());
    let elem = Element::optional(inner);
    match elem {
        Element::Optional { .. } => {}
        _ => panic!("Expected Optional"),
    }
}

#[test]
fn test_element_zero_or_more() {
    let inner = Element::rule_ref("term".to_string());
    let elem = Element::zero_or_more(inner);
    match elem {
        Element::ZeroOrMore { .. } => {}
        _ => panic!("Expected ZeroOrMore"),
    }
}

#[test]
fn test_element_one_or_more() {
    let inner = Element::rule_ref("term".to_string());
    let elem = Element::one_or_more(inner);
    match elem {
        Element::OneOrMore { .. } => {}
        _ => panic!("Expected OneOrMore"),
    }
}

#[test]
fn test_element_with_label() {
    let elem = Element::rule_ref("expr".to_string()).with_label("e".to_string());
    match elem {
        Element::RuleRef { label, .. } => {
            assert_eq!(label, Some("e".to_string()));
        }
        _ => panic!("Expected RuleRef with label"),
    }
}

#[test]
fn test_grammar_lexer_rules() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Combined);
    grammar.add_rule(Rule::parser_rule("expr".to_string()));
    grammar.add_rule(Rule::lexer_rule("ID".to_string()));
    grammar.add_rule(Rule::lexer_rule("NUMBER".to_string()));
    
    let lexer_rules: Vec<_> = grammar.lexer_rules().collect();
    assert_eq!(lexer_rules.len(), 2);
}

#[test]
fn test_grammar_parser_rules() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Combined);
    grammar.add_rule(Rule::parser_rule("expr".to_string()));
    grammar.add_rule(Rule::parser_rule("term".to_string()));
    grammar.add_rule(Rule::lexer_rule("ID".to_string()));
    
    let parser_rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(parser_rules.len(), 2);
}
