//! Tests to validate error recovery features in generated code.

use minipg::codegen::RustCodeGenerator;
use minipg::core::{types::CodeGenConfig, CodeGenerator};
use minipg::ast::{Alternative, Element, Grammar, Rule, RuleType};
use minipg::core::types::GrammarType;

#[test]
fn test_error_recovery_in_generated_code() {
    // Create a simple grammar for testing
    let mut grammar = Grammar::new("ErrorTest".to_string(), GrammarType::Combined);
    
    // Add a NUMBER token
    let mut number_rule = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    });
    number_rule.add_alternative(alt);
    grammar.add_rule(number_rule);
    
    // Generate code
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Verify error recovery features
    assert!(code.contains("ParseError"), "Missing ParseError type");
    assert!(code.contains("pub message: String"), "Missing error message field");
    assert!(code.contains("pub position: usize"), "Missing position field");
    assert!(code.contains("pub expected: Vec<String>"), "Missing expected field");
    assert!(code.contains("pub found: Option<String>"), "Missing found field");
    
    // Verify error recovery methods
    assert!(code.contains("with_expected"), "Missing with_expected method");
    assert!(code.contains("with_found"), "Missing with_found method");
    
    // Verify Display implementation
    assert!(code.contains("impl fmt::Display for ParseError"), "Missing Display impl");
    assert!(code.contains("impl std::error::Error for ParseError"), "Missing Error impl");
    
    // Verify tokenize_all for error collection
    assert!(code.contains("pub fn tokenize_all"), "Missing tokenize_all method");
    assert!(code.contains("Vec<ParseError>"), "Missing error collection");
    
    // Verify Result type usage
    assert!(code.contains("Result<Token, ParseError>"), "Missing Result type");
    
    // Verify error recovery logic
    assert!(code.contains("Error recovery"), "Missing error recovery comment");
    assert!(code.contains("Unexpected character"), "Missing error message generation");
    
    println!("✅ Error recovery features validated!");
}

#[test]
fn test_token_position_tracking() {
    let mut grammar = Grammar::new("PositionTest".to_string(), GrammarType::Combined);
    
    // Add a simple token
    let mut id_rule = Rule::new("ID".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::CharRange { start: 'a', end: 'z' });
    id_rule.add_alternative(alt);
    grammar.add_rule(id_rule);
    
    // Generate code
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Verify position tracking in tokens
    assert!(code.contains("pub position: usize"), "Missing position field in Token");
    assert!(code.contains("position: token_start"), "Missing position assignment in DFA");
    assert!(code.contains("position: start_pos"), "Missing position tracking");
    
    println!("✅ Position tracking validated!");
}

#[test]
fn test_error_context_information() {
    let mut grammar = Grammar::new("ContextTest".to_string(), GrammarType::Combined);
    
    // Add tokens
    let mut num_rule = Rule::new("NUM".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::CharRange { start: '0', end: '9' });
    num_rule.add_alternative(alt);
    grammar.add_rule(num_rule);
    
    // Generate code
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Verify error context
    assert!(code.contains("ParseError::new"), "Missing ParseError constructor");
    assert!(code.contains("with_found(invalid_char.to_string())"), "Missing found context");
    
    // Verify error message formatting
    assert!(code.contains("Parse error at position"), "Missing position in error message");
    assert!(code.contains("expected:"), "Missing expected in error message");
    assert!(code.contains("found:"), "Missing found in error message");
    
    println!("✅ Error context information validated!");
}

#[test]
fn test_tokenize_all_error_collection() {
    let mut grammar = Grammar::new("CollectionTest".to_string(), GrammarType::Combined);
    
    // Add a token
    let mut tok_rule = Rule::new("TOK".to_string(), RuleType::Lexer);
    let mut alt = Alternative::new();
    alt.add_element(Element::Terminal {
        value: "tok".to_string(),
        label: None,
    });
    tok_rule.add_alternative(alt);
    grammar.add_rule(tok_rule);
    
    // Generate code
    let config = CodeGenConfig::default();
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Verify tokenize_all implementation
    assert!(code.contains("pub fn tokenize_all(&mut self) -> (Vec<Token>, Vec<ParseError>)"), 
            "Missing tokenize_all signature");
    assert!(code.contains("let mut tokens = Vec::new()"), "Missing tokens collection");
    assert!(code.contains("let mut errors = Vec::new()"), "Missing errors collection");
    assert!(code.contains("Err(err) => {"), "Missing error handling");
    assert!(code.contains("errors.push(err)"), "Missing error collection");
    assert!(code.contains("// Continue tokenizing after error"), "Missing recovery comment");
    
    println!("✅ Error collection validated!");
}
