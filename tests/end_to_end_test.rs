//! End-to-end integration tests for the complete pipeline.

use minipg_analysis::SemanticAnalyzer;
use minipg_ast::Grammar;
use minipg_codegen::RustCodeGenerator;
use minipg_core::{
    types::CodeGenConfig, CodeGenerator, GrammarParser, SemanticAnalyzer as SemanticAnalyzerTrait,
};
use minipg_parser::GrammarParser as Parser;

#[test]
fn test_complete_pipeline_simple_grammar() {
    let source = r#"
grammar parser Calculator;

expr: term;
term: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
"#;
    
    // Parse
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    // Analyze
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should have no errors
    assert!(!analysis.has_errors());
    
    // Generate code
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).unwrap();
    
    // Verify output
    assert!(code.contains("CalculatorParser"));
    assert!(code.contains("NUMBER"));
}

#[test]
fn test_pipeline_with_warnings() {
    let source = r#"
grammar parser Test;

used: 'x';
unused: 'y';
"#;
    
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should have warnings but no errors
    assert!(!analysis.has_errors());
    assert!(!analysis.diagnostics.is_empty());
    
    // Should detect unreachable rule
    let has_unreachable = analysis
        .diagnostics
        .iter()
        .any(|d| d.message.contains("unreachable"));
    assert!(has_unreachable);
}

#[test]
fn test_pipeline_with_errors() {
    let source = r#"
grammar parser Test;

rule1: undefined_rule;
"#;
    
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should have errors
    assert!(analysis.has_errors());
    
    // Should detect undefined rule
    let has_undefined = analysis
        .diagnostics
        .iter()
        .any(|d| d.message.contains("undefined"));
    assert!(has_undefined);
}

#[test]
fn test_pipeline_with_left_recursion() {
    let source = r#"
grammar parser Test;

expr: expr '+' term | term;
term: 'x';
"#;
    
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should detect left recursion
    let has_left_recursion = analysis
        .diagnostics
        .iter()
        .any(|d| d.message.contains("left recursion"));
    assert!(has_left_recursion);
}

#[test]
fn test_pipeline_with_ambiguity() {
    let source = r#"
grammar parser Test;

stmt: 'x' | 'x';
"#;
    
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should detect ambiguous alternatives
    let has_ambiguity = analysis
        .diagnostics
        .iter()
        .any(|d| d.message.contains("ambiguous"));
    assert!(has_ambiguity);
}

#[test]
fn test_multi_language_generation() {
    let source = r#"
grammar parser Simple;

expr: 'x';
"#;
    
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    assert!(!analysis.has_errors());
    
    // Generate Rust
    let rust_gen = minipg_codegen::RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let rust_code = rust_gen.generate(&grammar, &config).unwrap();
    assert!(rust_code.contains("SimpleParser"));
    
    // Generate Python
    let python_gen = minipg_codegen::PythonCodeGenerator::new();
    let python_code = python_gen.generate(&grammar, &config).unwrap();
    assert!(python_code.contains("class SimpleParser"));
    
    // Generate JavaScript
    let js_gen = minipg_codegen::JavaScriptCodeGenerator::new();
    let js_code = js_gen.generate(&grammar, &config).unwrap();
    assert!(js_code.contains("class SimpleParser"));
}
