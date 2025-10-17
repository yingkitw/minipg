//! Integration tests for the full pipeline.

use minipg_analysis::SemanticAnalyzer;
use minipg_codegen::{CodeGenerator, RustCodeGenerator};
use minipg_core::{
    types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, GrammarParser,
    SemanticAnalyzer as SemanticAnalyzerTrait,
};
use minipg_parser::GrammarParser as Parser;

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
fn test_pipeline_with_errors() {
    let source = r#"
grammar parser Invalid;

expr: undefined_rule;
"#;

    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();

    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    
    assert!(analysis.has_errors());
    assert!(!analysis.diagnostics.is_empty());
}

#[test]
fn test_lexer_grammar_pipeline() {
    let source = r#"
grammar lexer SimpleLexer;

ID: LETTER+;
NUMBER: DIGIT+;

LETTER: 'a' | 'b' | 'c';
DIGIT: '0' | '1' | '2';
"#;

    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar).unwrap();
    assert!(!analysis.has_errors());
    
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).unwrap();
    
    assert!(code.contains("SimpleLexerLexer"));
}

#[test]
fn test_grammar_with_options() {
    let source = r#"
grammar parser Test;

options {
    language = rust;
}

expr: term;
term: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1';
"#;

    let parser = Parser::new();
    let grammar = parser.parse_string(source, "test.g4").unwrap();
    
    assert_eq!(grammar.options.get("language"), Some(&"rust".to_string()));
}
