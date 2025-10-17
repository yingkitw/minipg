//! Integration tests for code generation.

use minipg::ast::{Alternative, Element, Grammar, Rule};
use minipg::codegen::{JavaScriptCodeGenerator, PythonCodeGenerator, RustCodeGenerator};
use minipg::core::{
    types::{CodeGenConfig, GrammarType},
    CodeGenerator,
};

fn create_test_grammar() -> Grammar {
    let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Parser);
    
    // Parser rules
    let mut expr = Rule::parser_rule("expr".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("term".to_string()));
    expr.add_alternative(alt);
    grammar.add_rule(expr);
    
    let mut term = Rule::parser_rule("term".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("NUMBER".to_string()));
    term.add_alternative(alt);
    grammar.add_rule(term);
    
    // Lexer rules
    grammar.add_rule(Rule::lexer_rule("NUMBER".to_string()));
    grammar.add_rule(Rule::lexer_rule("PLUS".to_string()));
    grammar.add_rule(Rule::lexer_rule("MINUS".to_string()));
    
    grammar
}

#[test]
fn test_rust_codegen_complete() {
    let grammar = create_test_grammar();
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        generate_visitor: true,
        generate_listener: true,
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    // Verify generated code contains expected elements
    assert!(code.contains("CalculatorLexer"));
    assert!(code.contains("CalculatorParser"));
    assert!(code.contains("TokenKind"));
    assert!(code.contains("NUMBER"));
    assert!(code.contains("trait Visitor"));
    assert!(code.contains("trait Listener"));
}

#[test]
fn test_python_codegen_complete() {
    let grammar = create_test_grammar();
    let generator = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    // Verify Python-specific elements
    assert!(code.contains("class CalculatorLexer"));
    assert!(code.contains("class CalculatorParser"));
    assert!(code.contains("def parse_expr"));
    assert!(code.contains("def parse_term"));
    assert!(code.contains("from dataclasses import dataclass"));
}

#[test]
fn test_javascript_codegen_complete() {
    let grammar = create_test_grammar();
    let generator = JavaScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    // Verify JavaScript-specific elements
    assert!(code.contains("class CalculatorLexer"));
    assert!(code.contains("class CalculatorParser"));
    assert!(code.contains("parseExpr"));
    assert!(code.contains("parseTerm"));
    assert!(code.contains("module.exports"));
}

#[test]
fn test_all_generators_produce_valid_output() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig::default();
    
    let rust_gen = RustCodeGenerator::new();
    let python_gen = PythonCodeGenerator::new();
    let js_gen = JavaScriptCodeGenerator::new();
    
    let rust_code = rust_gen.generate(&grammar, &config).unwrap();
    let python_code = python_gen.generate(&grammar, &config).unwrap();
    let js_code = js_gen.generate(&grammar, &config).unwrap();
    
    // All should produce non-empty output
    assert!(!rust_code.is_empty());
    assert!(!python_code.is_empty());
    assert!(!js_code.is_empty());
    
    // All should contain grammar name
    assert!(rust_code.contains("Calculator"));
    assert!(python_code.contains("Calculator"));
    assert!(js_code.contains("Calculator"));
}

#[test]
fn test_codegen_with_visitor_only() {
    let grammar = create_test_grammar();
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        generate_visitor: true,
        generate_listener: false,
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    assert!(code.contains("trait Visitor"));
    assert!(!code.contains("trait Listener"));
}

#[test]
fn test_codegen_with_listener_only() {
    let grammar = create_test_grammar();
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        generate_visitor: false,
        generate_listener: true,
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    assert!(!code.contains("trait Visitor"));
    assert!(code.contains("trait Listener"));
}

#[test]
fn test_codegen_with_no_patterns() {
    let grammar = create_test_grammar();
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        generate_visitor: false,
        generate_listener: false,
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).unwrap();
    
    assert!(!code.contains("trait Visitor"));
    assert!(!code.contains("trait Listener"));
    assert!(code.contains("CalculatorParser")); // Still generates parser
}
