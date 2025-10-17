//! Tests for named action code generation

use minipg::parser::{Lexer, Parser};
use minipg::codegen::RustCodeGenerator;
use minipg::core::{types::CodeGenConfig, CodeGenerator};

#[test]
fn test_header_action_in_generated_code() {
    let grammar = r#"
grammar Test;

@header {
    use std::collections::HashMap;
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Check that header action is included
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
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Check that members action is included in struct
    assert!(code.contains("Custom members from @members action"));
    assert!(code.contains("count"));
    assert!(code.contains("usize"));
}

#[test]
fn test_multiple_named_actions_in_generated_code() {
    let grammar = r#"
grammar Test;

@header {
    use std::vec::Vec;
}

@members {
    items: Vec<String>,
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Check that both actions are included
    assert!(code.contains("Custom header from @header action"));
    assert!(code.contains("Vec"));
    assert!(code.contains("Custom members from @members action"));
    assert!(code.contains("items"));
}

#[test]
fn test_no_named_actions_still_works() {
    let grammar = r#"
grammar Test;

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let code = generator.generate(&grammar, &config).expect("Failed to generate");
    
    // Should still generate valid code without named actions
    assert!(code.contains("TestParser"));
    assert!(!code.contains("Custom header"));
    assert!(!code.contains("Custom members"));
}
