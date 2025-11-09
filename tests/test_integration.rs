//! Comprehensive integration tests for the complete minipg pipeline.
//!
//! This test module consolidates:
//! - Full pipeline tests (parse → analyze → generate)
//! - End-to-end tests with real grammars
//! - Integration tests for complex grammars
//! - Error handling and validation tests
//!
//! # Test Organization
//!
//! Tests are organized by category:
//! 1. Simple Pipeline Tests - Basic parse/analyze/generate flow
//! 2. End-to-End Tests - Complete workflows with real grammars
//! 3. Error Handling Tests - Validation and error recovery
//! 4. Multi-Language Tests - Code generation for all target languages

use minipg::analysis::SemanticAnalyzer;
use minipg::codegen::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator, GoCodeGenerator};
use minipg::core::{
    types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, GrammarParser,
    SemanticAnalyzer as SemanticAnalyzerTrait,
};
use minipg::parser::{GrammarParser as Parser, Lexer};
use std::fs;

// ============================================================================
// SIMPLE PIPELINE TESTS
// ============================================================================

#[test]
fn test_full_pipeline() {
    let source = r#"
grammar parser Simple;

expr: term;
term: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2';
"#;

    // Parse
    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    assert_eq!(grammar.name, "Simple");

    // Analyze
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    assert!(!analysis.has_errors());

    // Generate
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).unwrap();
    
    assert!(code.contains("SimpleParser"));
    assert!(code.contains("SimpleLexer"));
}

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
fn test_e2e_simple_calculator() {
    let grammar = r#"
grammar Calc;

expr: term;
term: NUMBER;

NUMBER: DIGIT+;
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
"#;

    let code = generate_parser_code(grammar);
    
    assert!(code.contains("struct CalcLexer"), "Should generate lexer struct");
    assert!(code.contains("struct CalcParser"), "Should generate parser struct");
    assert!(code.contains("fn next_token"), "Should have tokenization method");
    assert!(code.contains("TokenKind::NUMBER"), "Should define NUMBER token");
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_pipeline_with_errors() {
    let source = r#"
grammar parser Invalid;

expr: undefined_rule;
"#;

    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    // Should detect undefined rule
    assert!(analysis.has_errors());
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
    
    // May have warnings about unused rules
    // This is acceptable - warnings don't prevent code generation
}

// ============================================================================
// REAL-WORLD GRAMMAR TESTS
// ============================================================================

#[test]
fn test_complete_json_grammar_parsing() {
    let grammar_content = match fs::read_to_string("examples/CompleteJSON.g4") {
        Ok(content) => content,
        Err(_) => {
            // Skip if file doesn't exist
            return;
        }
    };
    
    let lexer = Lexer::new(&grammar_content, "test.g4");
    let mut parser = minipg::parser::Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    assert_eq!(grammar.name, "CompleteJSON");
    
    let parser_rules: Vec<_> = grammar.parser_rules().collect();
    let lexer_rules: Vec<_> = grammar.lexer_rules().collect();
    
    assert!(parser_rules.len() > 0, "Should have parser rules");
    assert!(lexer_rules.len() > 0, "Should have lexer rules");
}

#[test]
fn test_complete_json_semantic_analysis() {
    let grammar_content = match fs::read_to_string("examples/CompleteJSON.g4") {
        Ok(content) => content,
        Err(_) => return,
    };
    
    let lexer = Lexer::new(&grammar_content, "test.g4");
    let mut parser = minipg::parser::Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar);
    
    assert!(result.is_ok(), "Semantic analysis should succeed");
}

#[test]
fn test_complete_json_code_generation() {
    let grammar_content = match fs::read_to_string("examples/CompleteJSON.g4") {
        Ok(content) => content,
        Err(_) => return,
    };
    
    let lexer = Lexer::new(&grammar_content, "test.g4");
    let mut parser = minipg::parser::Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    // Test code generation for multiple languages
    let config = CodeGenConfig::default();
    
    let rust_gen = RustCodeGenerator::new();
    let rust_code = rust_gen.generate(&grammar, &config).expect("Failed to generate Rust");
    assert!(!rust_code.is_empty(), "Generated Rust code should not be empty");
    
    let python_gen = PythonCodeGenerator::new();
    let python_code = python_gen.generate(&grammar, &config).expect("Failed to generate Python");
    assert!(!python_code.is_empty(), "Generated Python code should not be empty");
    
    let js_gen = JavaScriptCodeGenerator::new();
    let js_code = js_gen.generate(&grammar, &config).expect("Failed to generate JavaScript");
    assert!(!js_code.is_empty(), "Generated JavaScript code should not be empty");
    
    let ts_gen = TypeScriptCodeGenerator::new();
    let ts_code = ts_gen.generate(&grammar, &config).expect("Failed to generate TypeScript");
    assert!(!ts_code.is_empty(), "Generated TypeScript code should not be empty");
    
    let go_gen = GoCodeGenerator::new();
    let go_code = go_gen.generate(&grammar, &config).expect("Failed to generate Go");
    assert!(!go_code.is_empty(), "Generated Go code should not be empty");
}

// ============================================================================
// MULTI-LANGUAGE TESTS
// ============================================================================

#[test]
fn test_multi_language_code_generation() {
    let source = r#"
grammar parser MultiLang;

expr: term;
term: NUMBER;

NUMBER: [0-9]+;
"#;

    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let config = CodeGenConfig::default();
    
    let rust_gen = RustCodeGenerator::new();
    let rust_code = rust_gen.generate(&grammar, &config).expect("Failed to generate Rust");
    assert!(!rust_code.is_empty(), "Generated Rust code should not be empty");
    
    let python_gen = PythonCodeGenerator::new();
    let python_code = python_gen.generate(&grammar, &config).expect("Failed to generate Python");
    assert!(!python_code.is_empty(), "Generated Python code should not be empty");
    
    let js_gen = JavaScriptCodeGenerator::new();
    let js_code = js_gen.generate(&grammar, &config).expect("Failed to generate JavaScript");
    assert!(!js_code.is_empty(), "Generated JavaScript code should not be empty");
    
    let ts_gen = TypeScriptCodeGenerator::new();
    let ts_code = ts_gen.generate(&grammar, &config).expect("Failed to generate TypeScript");
    assert!(!ts_code.is_empty(), "Generated TypeScript code should not be empty");
    
    let go_gen = GoCodeGenerator::new();
    let go_code = go_gen.generate(&grammar, &config).expect("Failed to generate Go");
    assert!(!go_code.is_empty(), "Generated Go code should not be empty");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn generate_parser_code(grammar_source: &str) -> String {
    let lexer = Lexer::new(grammar_source, "test.g4");
    let mut parser = minipg::parser::Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse grammar");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    generator.generate(&grammar, &config).expect("Failed to generate code")
}

