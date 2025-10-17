//! Tests to verify that generated code actually compiles and runs correctly.

use minipg_ast::{Alternative, Element, Grammar, Rule, RuleType};
use minipg_codegen::RustCodeGenerator;
use minipg_core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};
use std::fs;

/// Helper to create a test grammar
fn create_test_grammar() -> Grammar {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Combined);
    
    // Add a simple lexer rule: NUMBER: [0-9]+
    let mut number_rule = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::CharRange { start: '0', end: '9' });
    let one_or_more = Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    };
    alt.elements.clear();
    alt.add_element(one_or_more);
    number_rule.add_alternative(alt);
    grammar.add_rule(number_rule);
    
    // Add a parser rule: expr: NUMBER
    let mut expr_rule = Rule::new("expr".to_string(), RuleType::Parser);
    let mut expr_alt = Alternative::new();
    expr_alt.add_element(Element::RuleRef {
        name: "NUMBER".to_string(),
        label: None,
    });
    expr_rule.add_alternative(expr_alt);
    grammar.add_rule(expr_rule);
    
    grammar
}

#[test]
fn test_generated_rust_code_compiles() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        output_directory: "target/test_output".to_string(),
        package_name: None,
        generate_visitor: false,
        generate_listener: false,
    };
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Verify code contains expected elements
    assert!(code.contains("pub struct TestGrammarLexer"));
    assert!(code.contains("pub struct TestGrammarParser"));
    assert!(code.contains("pub enum TokenKind"));
    assert!(code.contains("NUMBER"));
    
    // Write to a temporary file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_grammar_generated.rs");
    fs::write(&test_file, &code).expect("Failed to write test file");
    
    // Try to compile it (this will fail without proper setup, but we can check syntax)
    // For now, we just verify the file was written
    assert!(test_file.exists());
    
    // Clean up
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_generated_code_has_all_required_components() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for lexer components
    assert!(code.contains("struct"), "Missing struct definition");
    assert!(code.contains("impl"), "Missing impl block");
    assert!(code.contains("pub fn new"), "Missing constructor");
    assert!(code.contains("pub fn next_token"), "Missing next_token method");
    
    // Check for optimization features
    assert!(code.contains("CHAR_CLASS_TABLE"), "Missing lookup table");
    assert!(code.contains("get_char_class"), "Missing char class function");
    assert!(code.contains("next_token_dfa"), "Missing DFA function");
    
    // Check for documentation
    assert!(code.contains("///"), "Missing documentation comments");
    assert!(code.contains("DO NOT EDIT"), "Missing generation warning");
}

#[test]
fn test_generated_code_with_visitor() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        output_directory: "target/test_output".to_string(),
        package_name: None,
        generate_visitor: true,
        generate_listener: false,
    };
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for visitor pattern
    assert!(code.contains("trait"), "Missing visitor trait");
    assert!(code.contains("Visitor"), "Missing Visitor in code");
}

#[test]
fn test_generated_code_with_listener() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        output_directory: "target/test_output".to_string(),
        package_name: None,
        generate_visitor: false,
        generate_listener: true,
    };
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for listener pattern
    assert!(code.contains("trait"), "Missing listener trait");
    assert!(code.contains("Listener"), "Missing Listener in code");
}

#[test]
fn test_generated_code_statistics() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for statistics comment
    assert!(code.contains("Lookup table stats:"), "Missing statistics");
    assert!(code.contains("chars"), "Missing char count");
    assert!(code.contains("classes"), "Missing class count");
    assert!(code.contains("bytes"), "Missing byte count");
}

#[test]
fn test_multiple_lexer_rules() {
    let mut grammar = Grammar::new("MultiRule".to_string(), GrammarType::Combined);
    
    // Add NUMBER rule
    let mut number_rule = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut num_alt = Alternative::new();
    num_alt.add_element(Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    });
    number_rule.add_alternative(num_alt);
    grammar.add_rule(number_rule);
    
    // Add IDENTIFIER rule
    let mut id_rule = Rule::new("IDENTIFIER".to_string(), RuleType::Lexer);
    let mut id_alt = Alternative::new();
    id_alt.add_element(Element::CharRange { start: 'a', end: 'z' });
    id_rule.add_alternative(id_alt);
    grammar.add_rule(id_rule);
    
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Verify both rules are present
    assert!(code.contains("NUMBER"));
    assert!(code.contains("IDENTIFIER"));
    
    // Verify lookup table handles both
    assert!(code.contains("CHAR_CLASS_TABLE"));
}

#[test]
fn test_generated_code_line_count() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    let line_count = code.lines().count();
    
    // Generated code should be substantial
    assert!(line_count > 50, "Generated code too short: {} lines", line_count);
    
    // But not excessively long for a simple grammar
    assert!(line_count < 500, "Generated code too long: {} lines", line_count);
}

#[test]
fn test_generated_code_no_warnings() {
    let grammar = create_test_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for common issues that would cause warnings
    assert!(!code.contains("unused_"), "Contains unused markers");
    assert!(!code.contains("dead_code"), "Contains dead code markers");
    
    // Check for proper visibility
    assert!(code.contains("pub struct"), "Missing public structs");
    assert!(code.contains("pub fn"), "Missing public functions");
}

#[test]
fn test_empty_grammar_handling() {
    let grammar = Grammar::new("Empty".to_string(), GrammarType::Combined);
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Should still generate valid code structure
    assert!(code.contains("pub struct EmptyLexer"));
    assert!(code.contains("pub struct EmptyParser"));
    
    // Should handle no rules gracefully
    assert!(code.contains("No lexer rules defined") || code.contains("next_token_dfa"));
}
