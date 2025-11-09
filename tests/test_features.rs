//! Comprehensive tests for ANTLR4 features.
//!
//! This test module consolidates tests for:
//! - Rule features (arguments, returns, locals)
//! - Labels (element and list labels)
//! - Named actions (@header, @members)
//! - Lexer modes and channels
//! - Action code generation
//! - Unicode escape sequences
//!
//! # Test Organization
//!
//! Tests are organized by feature category for easy navigation.

use minipg::core::{CodeGenerator, SemanticAnalyzer, GrammarParser};
use minipg::codegen::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator, GoCodeGenerator};
use minipg::core::types::CodeGenConfig;
use minipg::parser::{GrammarParser as GP, Lexer, Parser};
use minipg::ast::{Element, Grammar, Rule};
use minipg::core::types::GrammarType;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn parse_grammar(grammar_text: &str) -> Grammar {
    let parser = GP::new();
    parser.parse_string(grammar_text, "test.g4").expect("Failed to parse grammar")
}

// ============================================================================
// RULE FEATURES - Arguments, Returns, Locals
// ============================================================================

#[test]
fn test_parse_rule_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[int x, String name]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.arguments.len(), 2);
    assert_eq!(expr_rule.arguments[0].name, "x");
    assert_eq!(expr_rule.arguments[0].arg_type, Some("int".to_string()));
    assert_eq!(expr_rule.arguments[1].name, "name");
    assert_eq!(expr_rule.arguments[1].arg_type, Some("String".to_string()));
}

#[test]
fn test_parse_rule_with_returns() {
    let grammar_text = r#"
        grammar Test;
        
        expr returns [int value, String text]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.returns.len(), 2);
    assert_eq!(expr_rule.returns[0].name, "value");
    assert_eq!(expr_rule.returns[0].return_type, Some("int".to_string()));
    assert_eq!(expr_rule.returns[1].name, "text");
    assert_eq!(expr_rule.returns[1].return_type, Some("String".to_string()));
}

#[test]
fn test_parse_rule_with_locals() {
    let grammar_text = r#"
        grammar Test;
        
        expr locals [int count, String buffer]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.locals.len(), 2);
    assert_eq!(expr_rule.locals[0].name, "count");
    assert_eq!(expr_rule.locals[0].local_type, Some("int".to_string()));
    assert_eq!(expr_rule.locals[1].name, "buffer");
    assert_eq!(expr_rule.locals[1].local_type, Some("String".to_string()));
}

#[test]
fn test_parse_rule_with_all_features() {
    let grammar_text = r#"
        grammar Test;
        
        expr[int x] returns [int result] locals [String temp]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.arguments.len(), 1);
    assert_eq!(expr_rule.returns.len(), 1);
    assert_eq!(expr_rule.locals.len(), 1);
}

// ============================================================================
// LABELS - Element and List Labels
// ============================================================================

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
ID: [a-zA-Z]+;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let rule = &grammar.rules[0];
    let alt = &rule.alternatives[0];
    
    match &alt.elements[0] {
        Element::RuleRef { name, label, is_list } => {
            assert_eq!(name, "ID");
            assert_eq!(label, &Some("id".to_string()));
            assert_eq!(*is_list, false, "Should not be a list label");
        }
        _ => panic!("Expected RuleRef with regular label"),
    }
}

// ============================================================================
// NAMED ACTIONS - @header, @members
// ============================================================================

#[test]
fn test_parse_header_action() {
    let grammar = r#"
grammar Test;

@header {
    use std::collections::HashMap;
}

rule: ID;
ID: [a-zA-Z]+;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.name, "Test");
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("header"));
    
    let header_code = grammar.named_actions.get("header").unwrap();
    assert!(header_code.contains("use"));
    assert!(header_code.contains("HashMap"));
}

#[test]
fn test_parse_members_action() {
    let grammar = r#"
grammar Test;

@members {
    private int count = 0;
    public void increment() { count++; }
}

rule: ID;
ID: [a-zA-Z]+;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("members"));
}

#[test]
fn test_header_action_in_generated_code() {
    let grammar = r#"
grammar Test;

@header {
    use std::collections::HashMap;
}

rule: ID;
ID: [a-zA-Z]+;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    assert!(code.contains("Custom header from @header action"));
    assert!(code.contains("HashMap"));
}

#[test]
fn test_members_action_in_generated_code() {
    let grammar = r#"
grammar Test;

@members {
    count: usize,
}

rule: ID;
ID: [a-zA-Z]+;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    assert!(code.contains("Custom members from @members action"));
    assert!(code.contains("count"));
}

// ============================================================================
// LEXER MODES
// ============================================================================

#[test]
fn test_parse_lexer_mode() {
    let grammar_text = r#"
        grammar Test;
        
        mode DEFAULT_MODE;
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        WS: [ \t\r\n]+ -> skip;
        
        mode STRING_MODE;
        STRING_CHAR: ~["\\\r\n];
        STRING_END: '"' -> popMode;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert!(grammar.has_modes());
    assert_eq!(grammar.lexer_modes.len(), 2);
    assert!(grammar.lexer_modes.contains_key("DEFAULT_MODE"));
    assert!(grammar.lexer_modes.contains_key("STRING_MODE"));
}

#[test]
fn test_parse_lexer_mode_with_rules() {
    let grammar_text = r#"
        grammar Test;
        
        mode DEFAULT_MODE;
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        
        mode STRING_MODE;
        STRING_CONTENT: ~["];
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    assert_eq!(grammar.lexer_modes.len(), 2);
    
    let default_rules = &grammar.lexer_modes["DEFAULT_MODE"];
    assert_eq!(default_rules.len(), 2);
}

// ============================================================================
// UNICODE ESCAPES
// ============================================================================

#[test]
fn test_unicode_escape_in_charclass() {
    let grammar = r#"
grammar Test;
SAFE: ~["\\\u0000-\u001F];
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse Unicode escapes in character class");
}

#[test]
fn test_simple_quote_in_charclass() {
    let grammar = r#"
grammar Test;
SAFE: ["];
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse quote in character class");
}

#[test]
fn test_negated_charclass() {
    let grammar = r#"
grammar Test;
SAFE: ~["];
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse negated character class");
}

// ============================================================================
// CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_rust_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    assert!(code.contains("parse_expr"));
    assert!(code.contains("x:"));
}

#[test]
fn test_python_codegen_with_returns() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr returns [int value]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    assert!(code.contains("def parse_expr"));
}

#[test]
fn test_javascript_codegen_with_locals() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr locals [int temp]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = JavaScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    assert!(code.contains("parseExpr"));
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_complex_grammar_with_all_features() {
    let grammar_text = r#"
        grammar Complex;
        
        @header {
            package com.example;
        }
        
        @members {
            private int count = 0;
        }
        
        expr[int x] returns [int result] locals [String temp]
            : id=ID
            | ids+=ID (',' ids+=ID)*
            ;
        
        mode DEFAULT_MODE;
        ID: [a-zA-Z]+;
        WS: [ \t]+ -> skip;
        
        mode STRING_MODE;
        STRING_CHAR: ~["\\\u0000-\u001F];
        STRING_END: '"' -> popMode;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    // Verify all features
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("members"));
    assert!(grammar.has_modes());
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").unwrap();
    assert_eq!(expr_rule.arguments.len(), 1);
    assert_eq!(expr_rule.returns.len(), 1);
    assert_eq!(expr_rule.locals.len(), 1);
}

