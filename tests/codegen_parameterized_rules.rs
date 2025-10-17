//! Tests for code generation of parameterized rules

use minipg::parser::{Lexer, Parser};
use minipg::codegen::RustCodeGenerator;
use minipg::core::{types::CodeGenConfig, CodeGenerator};

#[test]
fn test_rust_codegen_with_arguments() {
    let grammar = r#"
grammar Test;

rule[int x, String name]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify function signature includes arguments
    assert!(code.contains("pub fn parse_rule(&mut self, x: int, name: String)"), 
            "Should generate method with arguments");
    
    // Verify documentation
    assert!(code.contains("# Arguments"), "Should document arguments");
    assert!(code.contains("* `x: int`"), "Should document x argument");
    assert!(code.contains("* `name: String`"), "Should document name argument");
}

#[test]
fn test_rust_codegen_with_returns() {
    let grammar = r#"
grammar Test;

rule returns [Value result]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify return type
    assert!(code.contains("-> Result<Value>"), "Should have Value return type");
    
    // Verify documentation
    assert!(code.contains("# Returns"), "Should document returns");
    assert!(code.contains("* `result` - Value"), "Should document result");
}

#[test]
fn test_rust_codegen_with_locals() {
    let grammar = r#"
grammar Test;

rule locals [int temp, String buffer]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify local variables are declared
    assert!(code.contains("let mut temp: int;"), "Should declare temp local");
    assert!(code.contains("let mut buffer: String;"), "Should declare buffer local");
}

#[test]
fn test_rust_codegen_with_all_features() {
    let grammar = r#"
grammar Test;

rule[int x] returns [Value result] locals [int temp]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify all features present
    assert!(code.contains("x: int"), "Should have argument");
    assert!(code.contains("-> Result<Value>"), "Should have return type");
    assert!(code.contains("let mut temp: int;"), "Should have local");
}

#[test]
fn test_rust_codegen_multiple_returns() {
    let grammar = r#"
grammar Test;

rule returns [Value result, int code]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify tuple return type
    assert!(code.contains("-> Result<(Value, int)>"), "Should have tuple return type");
}

#[test]
fn test_rust_codegen_no_types() {
    let grammar = r#"
grammar Test;

rule[x] returns [result] locals [temp]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Verify defaults to String for args and AstNode for returns
    assert!(code.contains("x: String"), "Should default to String for untyped args");
    assert!(code.contains("-> Result<AstNode>"), "Should default to AstNode for untyped returns");
    assert!(code.contains("let mut temp: String;"), "Should default to String for untyped locals");
}
