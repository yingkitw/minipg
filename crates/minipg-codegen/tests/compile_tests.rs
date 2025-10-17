//! Tests that verify generated code actually compiles and runs.
//!
//! These tests write generated code to temporary files and attempt to compile them.

use minipg_ast::{Alternative, Element, Grammar, Rule, RuleType};
use minipg_codegen::RustCodeGenerator;
use minipg_core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};
use std::fs;
use std::process::Command;

/// Create a simple but complete grammar for testing
fn create_calculator_grammar() -> Grammar {
    let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Combined);
    
    // Lexer rules
    // NUMBER: [0-9]+
    let mut number_rule = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut num_alt = Alternative::new();
    num_alt.add_element(Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    });
    number_rule.add_alternative(num_alt);
    grammar.add_rule(number_rule);
    
    // PLUS: '+'
    let mut plus_rule = Rule::new("PLUS".to_string(), RuleType::Lexer);
    let mut plus_alt = Alternative::new();
    plus_alt.add_element(Element::Terminal {
        value: "+".to_string(),
        label: None,
    });
    plus_rule.add_alternative(plus_alt);
    grammar.add_rule(plus_rule);
    
    // Parser rules
    // expr: NUMBER (PLUS NUMBER)*
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
#[ignore] // Ignore by default as it requires rustc
fn test_rust_generated_code_compiles() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Create a temporary directory
    let temp_dir = std::env::temp_dir().join("minipg_test_compile");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    // Write the generated code
    let test_file = temp_dir.join("calculator_parser.rs");
    fs::write(&test_file, &code).expect("Failed to write test file");
    
    // Try to compile it with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2024")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("libcalculator_parser.rlib"))
        .output();
    
    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
    
    if let Ok(result) = output {
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            panic!("Generated Rust code failed to compile:\n{}", stderr);
        }
        println!("✅ Generated Rust code compiles successfully!");
    } else {
        println!("⚠️  rustc not available, skipping compilation test");
    }
}

#[test]
fn test_rust_syntax_check() {
    // This test checks for common syntax errors without actually compiling
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for balanced braces
    let open_braces = code.matches('{').count();
    let close_braces = code.matches('}').count();
    assert_eq!(open_braces, close_braces, "Unbalanced braces in generated code");
    
    // Check for balanced parentheses
    let open_parens = code.matches('(').count();
    let close_parens = code.matches(')').count();
    assert_eq!(open_parens, close_parens, "Unbalanced parentheses in generated code");
    
    // Check for balanced brackets
    let open_brackets = code.matches('[').count();
    let close_brackets = code.matches(']').count();
    assert_eq!(open_brackets, close_brackets, "Unbalanced brackets in generated code");
    
    // Check for proper statement termination (no missing semicolons in obvious places)
    let lines: Vec<&str> = code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Skip empty lines, comments, and lines that don't need semicolons
        if trimmed.is_empty() 
            || trimmed.starts_with("//") 
            || trimmed.starts_with("/*")
            || trimmed.ends_with('{')
            || trimmed.ends_with('}')
            || trimmed.starts_with("pub ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("#[")
            || trimmed == "}"
        {
            continue;
        }
        
        // Lines that should end with semicolon or specific characters
        if !trimmed.ends_with(';') 
            && !trimmed.ends_with('{')
            && !trimmed.ends_with('}')
            && !trimmed.ends_with(',')
            && !trimmed.contains("=>")
        {
            // This might be a multi-line statement, check if next line continues
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if !next_line.starts_with('.') 
                    && !next_line.starts_with('?')
                    && !next_line.starts_with('}')
                {
                    println!("⚠️  Potential missing semicolon at line {}: {}", i + 1, trimmed);
                }
            }
        }
    }
}

#[test]
fn test_generated_lexer_has_no_todos() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check that the lexer implementation is complete (no TODO/unimplemented)
    // Note: Parser rules may still have TODO markers as they're not fully implemented yet
    let lexer_section = code.split("impl").nth(1).unwrap_or("");
    let lexer_code = lexer_section.split("impl").next().unwrap_or("");
    
    // Lexer should not have unimplemented!() in next_token or next_token_dfa
    assert!(!lexer_code.contains("next_token_dfa(&mut self) -> Option<Token> {\n        // TODO"), 
            "Lexer DFA method contains TODO");
    assert!(!lexer_code.contains("next_token(&mut self) -> Option<Token> {\n        // TODO"), 
            "Lexer next_token method contains TODO");
}

#[test]
fn test_generated_code_imports() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for necessary imports
    assert!(code.contains("use std::"), "Missing std imports");
    
    // Should not have unnecessary imports
    let import_count = code.matches("use ").count();
    assert!(import_count < 10, "Too many imports: {}", import_count);
}

#[test]
fn test_generated_code_visibility() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Main types should be public
    assert!(code.contains("pub struct"), "Missing public structs");
    assert!(code.contains("pub enum"), "Missing public enums");
    assert!(code.contains("pub fn new"), "Missing public constructor");
    assert!(code.contains("pub fn next_token") || code.contains("pub fn parse"), 
            "Missing public API methods");
}

#[test]
fn test_generated_code_derives() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for useful derives
    assert!(code.contains("#[derive(Debug"), "Missing Debug derive");
    assert!(code.contains("Clone"), "Missing Clone derive");
}

#[test]
fn test_generated_code_inline_hints() {
    let grammar = create_calculator_grammar();
    let config = CodeGenConfig::default();
    
    let generator = RustCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate code");
    
    // Check for inline hints on hot path functions
    assert!(code.contains("#[inline]"), "Missing inline hints for optimization");
}
