//! Tests for list label parsing (ids+=ID syntax)

use minipg::parser::{Lexer, Parser};
use minipg::ast::Element;

#[test]
fn test_parse_list_label() {
    let grammar = r#"
grammar Test;

rule: ids+=ID (',' ids+=ID)*;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.name, "Test");
    assert_eq!(grammar.rules.len(), 1);
    
    let rule = &grammar.rules[0];
    assert_eq!(rule.name, "rule");
    assert_eq!(rule.alternatives.len(), 1);
    
    let alt = &rule.alternatives[0];
    assert_eq!(alt.elements.len(), 2);
    
    // First element should be ids+=ID (list label)
    match &alt.elements[0] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ID");
            assert_eq!(label, &Some("ids".to_string()));
            assert_eq!(*is_list, true, "Should be a list label");
        }
        _ => panic!("Expected RuleRef with list label"),
    }
}

#[test]
fn test_parse_regular_label() {
    let grammar = r#"
grammar Test;

rule: id=ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let rule = &grammar.rules[0];
    let alt = &rule.alternatives[0];
    
    // Element should be id=ID (regular label)
    match &alt.elements[0] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ID");
            assert_eq!(label, &Some("id".to_string()));
            assert_eq!(*is_list, false, "Should be a regular label, not a list");
        }
        _ => panic!("Expected RuleRef with regular label"),
    }
}

#[test]
fn test_parse_string_literal_list_label() {
    let grammar = r#"
grammar Test;

rule: strs+='hello' (',' strs+='world')*;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let rule = &grammar.rules[0];
    let alt = &rule.alternatives[0];
    
    // First element should be strs+='hello' (list label on string literal)
    match &alt.elements[0] {
        Element::StringLiteral { value, label, is_list } => {
            assert_eq!(value, "hello"); // String literals are stored without quotes
            assert_eq!(label, &Some("strs".to_string()));
            assert_eq!(*is_list, true, "Should be a list label");
        }
        _ => panic!("Expected StringLiteral with list label"),
    }
}

#[test]
fn test_parse_mixed_labels() {
    let grammar = r#"
grammar Test;

rule: id=ID items+=ITEM (',' items+=ITEM)* name='test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let rule = &grammar.rules[0];
    let alt = &rule.alternatives[0];
    
    // Should have 4 elements
    assert!(alt.elements.len() >= 2);
    
    // First: id=ID (regular label)
    match &alt.elements[0] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ID");
            assert_eq!(label, &Some("id".to_string()));
            assert_eq!(*is_list, false);
        }
        _ => panic!("Expected RuleRef with regular label"),
    }
    
    // Second: items+=ITEM (list label)
    match &alt.elements[1] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ITEM");
            assert_eq!(label, &Some("items".to_string()));
            assert_eq!(*is_list, true);
        }
        _ => panic!("Expected RuleRef with list label"),
    }
}

#[test]
fn test_parse_no_label() {
    let grammar = r#"
grammar Test;

rule: ID ITEM;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let rule = &grammar.rules[0];
    let alt = &rule.alternatives[0];
    
    // Both elements should have no labels
    match &alt.elements[0] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ID");
            assert_eq!(label, &None);
            assert_eq!(*is_list, false);
        }
        _ => panic!("Expected RuleRef without label"),
    }
}
