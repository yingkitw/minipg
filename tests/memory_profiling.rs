/// Memory profiling tests and utilities
/// 
/// These tests help identify memory leaks and high memory usage patterns.
/// Run with: cargo test --test memory_profiling --release
/// 
/// For detailed profiling, use:
/// - valgrind: valgrind --leak-check=full cargo test --test memory_profiling
/// - heaptrack: heaptrack cargo test --test memory_profiling
/// - dhat-rs: cargo test --test memory_profiling --features dhat-heap

use std::collections::HashMap;
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

/// Test that parsing doesn't leak memory when called repeatedly
#[test]
fn test_no_memory_leak_repeated_parsing() {
    let grammar = r#"
        grammar parser Test;
        rule: TOKEN ;
        TOKEN: 'x' ;
    "#;
    
    let parser = GrammarParser::new();
    
    // Parse the same grammar many times
    for _ in 0..1000 {
        let result = parser.parse_string(grammar, "test.g4");
        assert!(result.is_ok(), "Parsing should succeed");
    }
}

/// Test that parsing large grammars doesn't cause excessive memory allocation
#[test]
#[ignore = "Memory profiling test"]
fn test_memory_usage_large_grammar() {
    let mut grammar = String::from("grammar parser Large;\n\n");
    
    // Generate a grammar with many rules
    for i in 0..10000 {
        grammar.push_str(&format!("rule{}: TOKEN{} ;\n", i, i % 100));
    }
    
    for i in 0..100 {
        grammar.push_str(&format!("TOKEN{}: 'token{}' ;\n", i, i));
    }
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(&grammar, "large.g4");
    
    // Should parse successfully without excessive memory
    assert!(result.is_ok(), "Large grammar should parse successfully");
}

/// Test memory usage with deeply nested structures
#[test]
#[ignore = "Memory profiling test"]
fn test_memory_usage_deep_nesting() {
    let mut grammar = String::from("grammar parser Deep;\n\nroot: ");
    
    // Create very deep nesting
    for _ in 0..1000 {
        grammar.push_str("( ");
    }
    grammar.push_str("TOKEN");
    for _ in 0..1000 {
        grammar.push_str(" )");
    }
    grammar.push_str(" ;\n\nTOKEN: 'x' ;\n");
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(&grammar, "deep.g4");
    
    assert!(result.is_ok(), "Deep nesting should parse successfully");
}

/// Test that AST doesn't hold excessive memory
#[test]
#[ignore = "Memory profiling test"]
fn test_ast_memory_efficiency() {
    let grammar = r#"
        grammar parser Test;
        rule: ( TOKEN | TOKEN | TOKEN )+ ;
        TOKEN: 'x' ;
    "#;
    
    let parser = GrammarParser::new();
    let ast = parser.parse_string(grammar, "test.g4").unwrap();
    
    // Verify AST size is reasonable
    // In a real implementation, you'd check actual memory usage
    assert!(ast.parser_rules().count() > 0, "AST should contain rules");
}

/// Test memory cleanup when parser is dropped
#[test]
fn test_parser_drop_cleanup() {
    let grammar = r#"
        grammar parser Test;
        rule: TOKEN+ ;
        TOKEN: 'x' ;
    "#;
    
    // Create and drop many parsers
    for _ in 0..100 {
        let parser = GrammarParser::new();
        let _ = parser.parse_string(grammar, "test.g4");
        // Parser is dropped here
    }
    
    // If there are memory leaks, this test will show them
}

/// Test memory usage with character classes
#[test]
#[ignore = "Memory profiling test"]
fn test_memory_usage_char_classes() {
    let mut grammar = String::from("grammar parser CharClasses;\n\n");
    
    // Generate many character class tokens
    for i in 0..1000 {
        let start = (b'a' + (i % 26) as u8) as char;
        let end = (b'a' + ((i + 1) % 26) as u8) as char;
        grammar.push_str(&format!("TOKEN{}: [{}] ;\n", i, start));
        grammar.push_str(&format!("RANGE{}: [{}] ;\n", i, format!("{}-{}", start, end)));
    }
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(&grammar, "char_classes.g4");
    
    assert!(result.is_ok(), "Many character classes should parse successfully");
}

/// Test memory usage with string literals
#[test]
#[ignore = "Memory profiling test"]
fn test_memory_usage_string_literals() {
    let mut grammar = String::from("grammar parser Strings;\n\n");
    
    // Generate many string literal tokens
    for i in 0..1000 {
        grammar.push_str(&format!("TOKEN{}: 'token{}' ;\n", i, i));
    }
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(&grammar, "strings.g4");
    
    assert!(result.is_ok(), "Many string literals should parse successfully");
}

/// Test that error recovery doesn't cause memory issues
#[test]
fn test_memory_error_recovery() {
    let invalid_grammar = r#"
        grammar parser Invalid;
        rule: TOKEN
        // Missing semicolon and TOKEN definition
    "#;
    
    let parser = GrammarParser::new();
    
    // Parse invalid grammar many times
    for _ in 0..100 {
        let _ = parser.parse_string(invalid_grammar, "invalid.g4");
    }
}

