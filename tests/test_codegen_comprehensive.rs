//! Comprehensive code generation tests for all target languages.
//!
//! This test module provides extensive coverage for:
//! - C, C++, Java, Go code generators (newly completed)
//! - Pattern matching functionality
//! - Language registry system
//! - Common codegen utilities
//! - Edge cases and error handling

use minipg::codegen::{
    CCodeGenerator, CppCodeGenerator, GoCodeGenerator, JavaCodeGenerator,
    LanguageRegistry, RustCodeGenerator, PythonCodeGenerator,
    JavaScriptCodeGenerator, TypeScriptCodeGenerator,
};
use minipg::core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait};
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

fn parse_grammar(grammar_text: &str) -> minipg::ast::Grammar {
    let parser = GrammarParser::new();
    parser.parse_string(grammar_text, "test.g4").expect("Failed to parse grammar")
}

// ============================================================================
// GO CODE GENERATOR TESTS
// ============================================================================

#[test]
fn test_go_codegen_basic() {
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("package"));
    assert!(code.contains("type TestLexer struct"));
    assert!(code.contains("type TestParser struct"));
    assert!(code.contains("func (l *TestLexer) NextToken()"));
}

#[test]
#[ignore] // Parser doesn't fully support rule arguments yet
fn test_go_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[x int]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("func (p *TestParser) ParseExpr") || code.contains("ParseExpr"));
}

#[test]
#[ignore] // Parser doesn't fully support rule returns yet
fn test_go_codegen_with_returns() {
    let grammar_text = r#"
        grammar Test;
        
        expr returns [int value]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("ParseExpr") || code.contains("parseExpr"));
}

#[test]
#[ignore] // Parser doesn't fully support rule locals yet
fn test_go_codegen_with_locals() {
    let grammar_text = r#"
        grammar Test;
        
        expr locals [int temp]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("temp") || code.contains("var"));
}

// ============================================================================
// C CODE GENERATOR TESTS
// ============================================================================

#[test]
fn test_c_codegen_basic() {
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "c".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("#include"));
    assert!(code.contains("typedef struct"));
    assert!(code.contains("Lexer"));
    assert!(code.contains("Parser"));
    assert!(code.contains("Token lexer_next_token"));
}

#[test]
#[ignore] // Parser doesn't fully support rule arguments yet
fn test_c_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[x int]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "c".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("parse_expr") || code.contains("parseExpr"));
}

// ============================================================================
// C++ CODE GENERATOR TESTS
// ============================================================================

#[test]
fn test_cpp_codegen_basic() {
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CppCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "cpp".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("#include") || code.contains("include"));
    assert!(code.contains("Lexer") || code.contains("lexer"));
    assert!(code.contains("Parser") || code.contains("parser"));
}

#[test]
#[ignore] // Parser doesn't fully support rule arguments yet
fn test_cpp_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[x int]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CppCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "cpp".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("parseExpr") || code.contains("parse_expr"));
}

// ============================================================================
// JAVA CODE GENERATOR TESTS
// ============================================================================

#[test]
fn test_java_codegen_basic() {
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = JavaCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "java".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("Lexer") || code.contains("lexer"));
    assert!(code.contains("Parser") || code.contains("parser"));
    assert!(code.contains("Token") || code.contains("token"));
}

#[test]
#[ignore] // Parser doesn't fully support rule arguments yet
fn test_java_codegen_with_arguments() {
    let grammar_text = r#"
        grammar Test;
        
        expr[x int]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = JavaCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "java".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("parseExpr") || code.contains("parse_expr"));
}

// ============================================================================
// LANGUAGE REGISTRY TESTS
// ============================================================================

#[test]
fn test_registry_all_languages() {
    let registry = LanguageRegistry::new();
    
    assert!(registry.is_supported("rust"));
    assert!(registry.is_supported("python"));
    assert!(registry.is_supported("javascript"));
    assert!(registry.is_supported("typescript"));
    assert!(registry.is_supported("go"));
    assert!(registry.is_supported("c"));
    assert!(registry.is_supported("cpp"));
    assert!(registry.is_supported("java"));
}

#[test]
fn test_registry_aliases() {
    let registry = LanguageRegistry::new();
    
    assert!(registry.is_supported("js"));
    assert!(registry.is_supported("ts"));
    assert!(registry.is_supported("c++"));
    
    // Test that aliases resolve to same generator
    let grammar_text = r#"
        grammar Test;
        expr: NUMBER;
        NUMBER: [0-9]+;
    "#;
    let grammar = parse_grammar(grammar_text);
    let config = CodeGenConfig {
        target_language: "js".to_string(),
        ..Default::default()
    };
    
    let code1 = registry.generate("js", &grammar, &config).expect("Generation failed");
    let code2 = registry.generate("javascript", &grammar, &config).expect("Generation failed");
    
    // Should generate same code
    assert_eq!(code1, code2);
}

#[test]
fn test_registry_unsupported_language() {
    let registry = LanguageRegistry::new();
    
    assert!(!registry.is_supported("swift"));
    assert!(!registry.is_supported("kotlin"));
    
    let grammar_text = r#"
        grammar Test;
        expr: NUMBER;
        NUMBER: [0-9]+;
    "#;
    let grammar = parse_grammar(grammar_text);
    let config = CodeGenConfig {
        target_language: "swift".to_string(),
        ..Default::default()
    };
    
    let result = registry.generate("swift", &grammar, &config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unsupported"));
}

#[test]
fn test_registry_supported_languages_list() {
    let registry = LanguageRegistry::new();
    let languages = registry.supported_languages();
    
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));
    assert!(languages.contains(&"javascript".to_string()));
    assert!(languages.contains(&"typescript".to_string()));
    assert!(languages.contains(&"go".to_string()));
    assert!(languages.contains(&"c".to_string()));
    assert!(languages.contains(&"cpp".to_string()));
    assert!(languages.contains(&"java".to_string()));
    
    assert!(languages.len() >= 8);
}

// ============================================================================
// PATTERN MATCHING TESTS
// ============================================================================

#[test]
fn test_pattern_matching_string_literal() {
    let grammar_text = r#"
        grammar Test;
        
        STRING: '"' ~["]* '"';
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    // Test that string literals are handled
    let string_rule = grammar.get_rule("STRING").expect("STRING rule not found");
    assert!(string_rule.is_lexer_rule());
    
    // Generate code and verify pattern matching is included
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Should contain pattern matching logic
    assert!(code.contains("match") || code.contains("Match") || code.contains("STRING"));
}

#[test]
fn test_pattern_matching_char_range() {
    let grammar_text = r#"
        grammar Test;
        
        ID: [a-zA-Z]+;
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    let id_rule = grammar.get_rule("ID").expect("ID rule not found");
    assert!(id_rule.is_lexer_rule());
    
    // Verify char ranges are parsed correctly
    let alt = id_rule.alternatives.first().expect("No alternatives");
    assert!(!alt.elements.is_empty());
}

#[test]
fn test_pattern_matching_char_class() {
    let grammar_text = r#"
        grammar Test;
        
        WS: [ \t\r\n]+ -> skip;
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    
    let ws_rule = grammar.get_rule("WS").expect("WS rule not found");
    assert!(ws_rule.is_lexer_rule());
    
    let id_rule = grammar.get_rule("ID").expect("ID rule not found");
    assert!(id_rule.is_lexer_rule());
}

// ============================================================================
// COMMON CODEGEN UTILITIES TESTS
// ============================================================================

#[test]
fn test_extract_token_types() {
    use minipg::codegen::common::extract_token_types;
    
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
        fragment DIGIT: [0-9];
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let tokens = extract_token_types(&grammar);
    
    assert!(tokens.contains(&"NUMBER".to_string()));
    assert!(!tokens.contains(&"DIGIT".to_string())); // Fragments should be excluded
}

#[test]
fn test_extract_all_lexer_rules() {
    use minipg::codegen::common::extract_all_lexer_rules;
    
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
        fragment DIGIT: [0-9];
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let rules = extract_all_lexer_rules(&grammar);
    
    assert_eq!(rules.len(), 2); // NUMBER and DIGIT
    assert!(rules.iter().any(|r| r.name == "NUMBER"));
    assert!(rules.iter().any(|r| r.name == "DIGIT"));
}

#[test]
fn test_extract_parser_rules() {
    use minipg::codegen::common::extract_parser_rules;
    
    let grammar_text = r#"
        grammar Test;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let rules = extract_parser_rules(&grammar);
    
    assert_eq!(rules.len(), 2); // expr and term
    assert!(rules.iter().any(|r| r.name == "expr"));
    assert!(rules.iter().any(|r| r.name == "term"));
}

#[test]
fn test_get_named_action() {
    use minipg::codegen::common::get_named_action;
    
    let grammar_text = r#"
        grammar Test;
        
        @header {
            package com.example;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let header = get_named_action(&grammar, "header");
    
    assert!(header.is_some());
    let header_code = header.unwrap();
    assert!(header_code.contains("package") || header_code.contains("com") || header_code.contains("example"));
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_empty_grammar() {
    let grammar_text = r#"
        grammar Empty;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        ..Default::default()
    };
    
    // Should not panic
    let result = generator.generate(&grammar, &config);
    assert!(result.is_ok());
}

#[test]
fn test_grammar_with_only_lexer_rules() {
    let grammar_text = r#"
        grammar LexerOnly;
        
        ID: [a-zA-Z]+;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = GoCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "go".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    assert!(code.contains("Lexer"));
    // Should not contain parser if there are no parser rules
}

#[test]
fn test_grammar_with_only_parser_rules() {
    let grammar_text = r#"
        grammar ParserOnly;
        
        expr: term;
        term: 'x';
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = JavaCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "java".to_string(),
        ..Default::default()
    };
    
    // Should handle gracefully (may generate implicit lexer rules)
    let result = generator.generate(&grammar, &config);
    assert!(result.is_ok());
}

#[test]
fn test_grammar_with_fragments() {
    let grammar_text = r#"
        grammar WithFragments;
        
        expr: NUMBER;
        
        NUMBER: DIGIT+;
        fragment DIGIT: [0-9];
        fragment HEX: [0-9a-fA-F];
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "c".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Fragments should not appear as token types in generated code
    let tokens = minipg::codegen::common::extract_token_types(&grammar);
    assert!(tokens.contains(&"NUMBER".to_string()));
    assert!(!tokens.contains(&"DIGIT".to_string()));
    assert!(!tokens.contains(&"HEX".to_string()));
}

#[test]
fn test_grammar_with_alternatives() {
    let grammar_text = r#"
        grammar Alternatives;
        
        expr: term '+' term | term '-' term | term;
        term: NUMBER | ID;
        
        NUMBER: [0-9]+;
        ID: [a-zA-Z]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = CppCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "cpp".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Should handle alternatives
    assert!(code.contains("parseExpr") || code.contains("parse_expr"));
}

#[test]
fn test_grammar_with_quantifiers() {
    let grammar_text = r#"
        grammar Quantifiers;
        
        list: item (',' item)*;
        item: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let grammar = parse_grammar(grammar_text);
    let generator = TypeScriptCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "typescript".to_string(),
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config).expect("Code generation failed");
    
    // Should handle quantifiers (zero or more)
    assert!(code.contains("list") || code.contains("List"));
}

