//! Integration tests for complex real-world grammars.

use minipg_parser::{Lexer, Parser};
use minipg_analysis::SemanticAnalyzer;
use minipg_codegen::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator};
use minipg_core::{types::CodeGenConfig, CodeGenerator, SemanticAnalyzer as SemanticAnalyzerTrait};
use std::fs;

#[test]
fn test_complete_json_grammar_parsing() {
    let grammar_content = fs::read_to_string("examples/CompleteJSON.g4")
        .expect("Failed to read CompleteJSON.g4");
    
    // Parse the grammar
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    // Verify grammar structure
    assert_eq!(grammar.name, "CompleteJSON");
    
    // Count rules
    let parser_rules: Vec<_> = grammar.parser_rules().collect();
    let lexer_rules: Vec<_> = grammar.lexer_rules().collect();
    
    println!("CompleteJSON.g4:");
    println!("  Parser rules: {}", parser_rules.len());
    println!("  Lexer rules: {}", lexer_rules.len());
    
    assert!(parser_rules.len() > 0, "Should have parser rules");
    assert!(lexer_rules.len() > 0, "Should have lexer rules");
}

#[test]
fn test_complete_json_semantic_analysis() {
    let grammar_content = fs::read_to_string("examples/CompleteJSON.g4")
        .expect("Failed to read CompleteJSON.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    // Analyze the grammar
    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&grammar);
    
    assert!(result.is_ok(), "Semantic analysis should succeed");
    
    // Check for errors
    let diagnostics = analyzer.diagnostics();
    let errors: Vec<_> = diagnostics.iter()
        .filter(|d| d.severity == minipg_core::DiagnosticSeverity::Error)
        .collect();
    
    if !errors.is_empty() {
        println!("Errors in CompleteJSON.g4:");
        for error in &errors {
            println!("  - {}", error.message);
        }
    }
    
    println!("✅ CompleteJSON.g4 semantic analysis passed");
}

#[test]
fn test_complete_json_rust_codegen() {
    let grammar_content = fs::read_to_string("examples/CompleteJSON.g4")
        .expect("Failed to read CompleteJSON.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    // Generate Rust code
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config)
        .expect("Failed to generate Rust code");
    
    // Verify generated code
    assert!(code.contains("pub struct CompleteJSONLexer"));
    assert!(code.contains("pub struct CompleteJSONParser"));
    assert!(code.contains("pub struct ParseError"));
    assert!(code.contains("pub fn next_token"));
    assert!(code.contains("pub fn tokenize_all"));
    assert!(code.contains("Result<Token, ParseError>"));
    
    // Check for expected tokens
    assert!(code.contains("STRING"));
    assert!(code.contains("NUMBER"));
    
    println!("✅ CompleteJSON.g4 Rust code generation: {} lines", code.lines().count());
}

#[test]
fn test_complete_json_python_codegen() {
    let grammar_content = fs::read_to_string("examples/CompleteJSON.g4")
        .expect("Failed to read CompleteJSON.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    // Generate Python code
    let generator = PythonCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config)
        .expect("Failed to generate Python code");
    
    // Verify generated code
    assert!(code.contains("class CompleteJSONLexer"));
    assert!(code.contains("class CompleteJSONParser"));
    assert!(code.contains("class ParseError"));
    assert!(code.contains("def next_token"));
    assert!(code.contains("def tokenize_all"));
    
    println!("✅ CompleteJSON.g4 Python code generation: {} lines", code.lines().count());
}

#[test]
fn test_sql_grammar_parsing() {
    let grammar_content = fs::read_to_string("examples/SQL.g4")
        .expect("Failed to read SQL.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse SQL.g4");
    
    assert_eq!(grammar.name, "SQL");
    
    let parser_rules: Vec<_> = grammar.parser_rules().collect();
    let lexer_rules: Vec<_> = grammar.lexer_rules().collect();
    
    println!("SQL.g4:");
    println!("  Parser rules: {}", parser_rules.len());
    println!("  Lexer rules: {}", lexer_rules.len());
    
    assert!(parser_rules.len() > 0);
    assert!(lexer_rules.len() > 0);
}

#[test]
fn test_sql_rust_codegen() {
    let grammar_content = fs::read_to_string("examples/SQL.g4")
        .expect("Failed to read SQL.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse SQL.g4");
    
    // Generate Rust code
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config)
        .expect("Failed to generate Rust code");
    
    // Verify generated code
    assert!(code.contains("pub struct SQLLexer"));
    assert!(code.contains("pub struct SQLParser"));
    assert!(code.contains("pub struct ParseError"));
    
    // Check for SQL keywords
    assert!(code.contains("SELECT"));
    assert!(code.contains("FROM"));
    assert!(code.contains("WHERE"));
    
    println!("✅ SQL.g4 Rust code generation: {} lines", code.lines().count());
}

#[test]
fn test_sql_javascript_codegen() {
    let grammar_content = fs::read_to_string("examples/SQL.g4")
        .expect("Failed to read SQL.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse SQL.g4");
    
    // Generate JavaScript code
    let generator = JavaScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config)
        .expect("Failed to generate JavaScript code");
    
    // Verify generated code
    assert!(code.contains("class SQLLexer"));
    assert!(code.contains("class SQLParser"));
    assert!(code.contains("class ParseError"));
    assert!(code.contains("nextToken()"));
    assert!(code.contains("tokenizeAll()"));
    
    println!("✅ SQL.g4 JavaScript code generation: {} lines", code.lines().count());
}

#[test]
fn test_sql_typescript_codegen() {
    let grammar_content = fs::read_to_string("examples/SQL.g4")
        .expect("Failed to read SQL.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse SQL.g4");
    
    // Generate TypeScript code
    let generator = TypeScriptCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config)
        .expect("Failed to generate TypeScript code");
    
    // Verify generated code
    assert!(code.contains("export class SQLLexer"));
    assert!(code.contains("export class SQLParser"));
    assert!(code.contains("export class ParseError"));
    assert!(code.contains("nextToken(): Token"));
    assert!(code.contains("tokenizeAll()"));
    
    println!("✅ SQL.g4 TypeScript code generation: {} lines", code.lines().count());
}

#[test]
fn test_all_languages_for_json() {
    let grammar_content = fs::read_to_string("examples/CompleteJSON.g4")
        .expect("Failed to read CompleteJSON.g4");
    
    let lexer = Lexer::new(&grammar_content);
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse CompleteJSON.g4");
    
    let config = CodeGenConfig::default();
    
    // Test all generators
    let rust_gen = RustCodeGenerator::new();
    let python_gen = PythonCodeGenerator::new();
    let js_gen = JavaScriptCodeGenerator::new();
    let ts_gen = TypeScriptCodeGenerator::new();
    
    let rust_code = rust_gen.generate(&grammar, &config).expect("Rust generation failed");
    let python_code = python_gen.generate(&grammar, &config).expect("Python generation failed");
    let js_code = js_gen.generate(&grammar, &config).expect("JavaScript generation failed");
    let ts_code = ts_gen.generate(&grammar, &config).expect("TypeScript generation failed");
    
    // All should contain error recovery
    assert!(rust_code.contains("ParseError"));
    assert!(python_code.contains("ParseError"));
    assert!(js_code.contains("ParseError"));
    assert!(ts_code.contains("ParseError"));
    
    println!("✅ All 4 languages successfully generated code for CompleteJSON.g4");
    println!("   Rust: {} lines", rust_code.lines().count());
    println!("   Python: {} lines", python_code.lines().count());
    println!("   JavaScript: {} lines", js_code.lines().count());
    println!("   TypeScript: {} lines", ts_code.lines().count());
}
