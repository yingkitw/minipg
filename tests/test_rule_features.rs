//! Tests for rule arguments, returns, and local variables.

use minipg::core::{CodeGenerator, SemanticAnalyzer, GrammarParser};
use minipg::codegen::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator, GoCodeGenerator};
use minipg::core::types::CodeGenConfig;
use minipg::parser::GrammarParser as GP;

#[test]
fn test_parse_rule_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[int x, String name]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
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
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
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
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
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
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.arguments.len(), 1);
    assert_eq!(expr_rule.returns.len(), 1);
    assert_eq!(expr_rule.locals.len(), 1);
}

#[test]
fn test_rust_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify arguments are in the generated code
    assert!(code.contains("fn parse_expr(&mut self, x: int)"));
}

#[test]
fn test_rust_codegen_with_returns() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr returns [int value]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify return type is in the generated code
    assert!(code.contains("-> Result<int>"));
}

#[test]
fn test_rust_codegen_with_locals() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr locals [int count]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify local variables are in the generated code
    assert!(code.contains("let mut count: int;"));
}

#[test]
fn test_python_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify arguments are in the generated code
    assert!(code.contains("def parse_expr(self, x: int)"));
}

#[test]
fn test_python_codegen_with_returns() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr returns [int value]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify return type is in the generated code
    assert!(code.contains("-> int:"));
}

#[test]
fn test_javascript_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = JavaScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify arguments are in the generated code
    assert!(code.contains("parseExpr(x)"));
}

#[test]
fn test_go_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = GoCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify arguments are in the generated code
    assert!(code.contains("x int"));
}

#[test]
fn test_typescript_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Calculator;
        
        expr[int x]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = TypeScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Verify arguments are in the generated code
    assert!(code.contains("parseExpr(x: int)"));
}

#[test]
fn test_complex_rule_with_multiple_arguments_and_returns() {
    let grammar_text = r#"
        grammar Complex;
        
        expr[int x, String name] returns [int result, String output]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    
    // Verify parsing
    assert_eq!(expr_rule.arguments.len(), 2);
    assert_eq!(expr_rule.returns.len(), 2);
    
    // Verify code generation for Rust
    let analyzer = minipg::analysis::SemanticAnalyzer::new();
    let analyzed = <minipg::analysis::SemanticAnalyzer as SemanticAnalyzer>::analyze(&analyzer, &grammar).expect("Failed to analyze");
    
    let codegen = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = codegen.generate(&analyzed.grammar, &config).expect("Failed to generate");
    
    // Should have tuple return type for multiple returns
    assert!(code.contains("-> Result<(int, String)>"));
}
