//! Tests for ANTLR4 test suite compatibility.
//!
//! This test suite verifies that minipg passes common ANTLR4 test patterns
//! from the official ANTLR4 test suite.

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

/// Test basic rule parsing (ANTLR4 test suite: basic rules)
#[test]
fn test_antlr4_suite_basic_rules() {
    let grammar_text = r#"
        grammar BasicRules;
        
        rule1: 'a';
        rule2: 'b' 'c';
        rule3: rule1 rule2;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 3);
    assert!(grammar.get_rule("rule1").is_some());
    assert!(grammar.get_rule("rule2").is_some());
    assert!(grammar.get_rule("rule3").is_some());
}

/// Test alternatives (ANTLR4 test suite: alternatives)
#[test]
#[ignore]
fn test_antlr4_suite_alternatives() {
    let grammar_text = r#"
        grammar Alternatives;
        
        rule: 'a' | 'b' | 'c';
        rule2: rule1 | rule2 | rule3;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let rule = grammar.get_rule("rule").expect("rule not found");
    assert_eq!(rule.alternatives.len(), 1);
    assert_eq!(rule.alternatives[0].elements.len(), 3); // 'a' | 'b' | 'c'
}

/// Test repetition operators (ANTLR4 test suite: quantifiers)
#[test]
#[ignore]
fn test_antlr4_suite_quantifiers() {
    let grammar_text = r#"
        grammar Quantifiers;
        
        rule1: 'a'*;
        rule2: 'b'+;
        rule3: 'c'?;
        rule4: 'd'{1,3};
        rule5: 'e'{2};
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 5);
    assert!(grammar.get_rule("rule1").is_some());
    assert!(grammar.get_rule("rule2").is_some());
    assert!(grammar.get_rule("rule3").is_some());
    assert!(grammar.get_rule("rule4").is_some());
    assert!(grammar.get_rule("rule5").is_some());
}

/// Test character classes (ANTLR4 test suite: character classes)
#[test]
fn test_antlr4_suite_character_classes() {
    let grammar_text = r#"
        grammar CharClasses;
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        HEX: [0-9a-fA-F]+;
        WHITESPACE: [ \t\r\n]+;
        NOT_QUOTE: ~["];
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.lexer_rules().count(), 5);
}

/// Test string literals (ANTLR4 test suite: string literals)
#[test]
fn test_antlr4_suite_string_literals() {
    let grammar_text = r#"
        grammar StringLiterals;
        
        rule1: 'hello' 'world';
        rule2: 'a' | 'b' | 'c';
        rule3: 'if' | 'else' | 'while';
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 3);
}

/// Test rule references (ANTLR4 test suite: rule references)
#[test]
#[ignore]
fn test_antlr4_suite_rule_references() {
    let grammar_text = r#"
        grammar RuleReferences;
        
        expr: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | '(' expr ')';
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 3);
    assert_eq!(grammar.lexer_rules().count(), 1);
}

/// Test lexer tokens (ANTLR4 test suite: lexer tokens)
#[test]
fn test_antlr4_suite_lexer_tokens() {
    let grammar_text = r#"
        grammar LexerTokens;
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        PLUS: '+';
        MINUS: '-';
        STAR: '*';
        SLASH: '/';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.lexer_rules().count(), 8);
}

/// Test fragment tokens (ANTLR4 test suite: fragments)
#[test]
fn test_antlr4_suite_fragments() {
    let grammar_text = r#"
        grammar Fragments;
        
        fragment DIGIT: [0-9];
        fragment LETTER: [a-zA-Z];
        fragment HEX_DIGIT: [0-9a-fA-F];
        
        ID: LETTER (LETTER | DIGIT)*;
        NUMBER: DIGIT+;
        HEX: '0x' HEX_DIGIT+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Fragments are lexer rules but marked as fragments
    assert!(grammar.lexer_rules().count() > 0);
}

/// Test lexer commands (ANTLR4 test suite: lexer commands)
#[test]
#[ignore]
fn test_antlr4_suite_lexer_commands() {
    let grammar_text = r#"
        grammar LexerCommands;
        
        ID: [a-zA-Z_]+;
        WS: [ \t\r\n]+ -> skip;
        COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);
        QUOTE: '"' -> mode(STRING_MODE);
        
        mode STRING_MODE;
        STRING_CHAR: ~["];
        END_QUOTE: '"' -> popMode;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert!(grammar.has_modes());
    assert!(grammar.has_channels());
}

/// Test parser rules with arguments (ANTLR4 test suite: rule arguments)
#[test]
fn test_antlr4_suite_rule_arguments() {
    let grammar_text = r#"
        grammar RuleArguments;
        
        expr[int prec]: term;
        term[String type]: factor;
        factor[int x, String y]: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr = grammar.get_rule("expr").expect("expr not found");
    assert_eq!(expr.arguments.len(), 1);
    
    let factor = grammar.get_rule("factor").expect("factor not found");
    assert_eq!(factor.arguments.len(), 2);
}

/// Test parser rules with returns (ANTLR4 test suite: rule returns)
#[test]
fn test_antlr4_suite_rule_returns() {
    let grammar_text = r#"
        grammar RuleReturns;
        
        expr returns [int value]: term;
        term returns [String text, int code]: factor;
        factor returns [double result]: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr = grammar.get_rule("expr").expect("expr not found");
    assert_eq!(expr.returns.len(), 1);
    
    let term = grammar.get_rule("term").expect("term not found");
    assert_eq!(term.returns.len(), 2);
}

/// Test parser rules with locals (ANTLR4 test suite: rule locals)
#[test]
fn test_antlr4_suite_rule_locals() {
    let grammar_text = r#"
        grammar RuleLocals;
        
        expr locals [int count]: term;
        term locals [String buffer, int pos]: factor;
        factor locals [double temp]: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr = grammar.get_rule("expr").expect("expr not found");
    assert_eq!(expr.locals.len(), 1);
}

/// Test labels (ANTLR4 test suite: labels)
#[test]
fn test_antlr4_suite_labels() {
    let grammar_text = r#"
        grammar Labels;
        
        expr: left=term '+' right=term;
        list: items+=item (',' items+=item)*;
        alt: a=rule1 | b=rule2 | c=rule3;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 3);
}

/// Test actions and predicates (ANTLR4 test suite: actions)
#[test]
#[ignore]
fn test_antlr4_suite_actions() {
    let grammar_text = r#"
        grammar Actions;
        
        expr: term { System.out.println("term"); };
        term: factor { count++; };
        factor: { depth > 0 }? NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 3);
}

/// Test named actions (ANTLR4 test suite: named actions)
#[test]
fn test_antlr4_suite_named_actions() {
    let grammar_text = r#"
        grammar NamedActions;
        
        @header {
            package com.example;
            import java.util.*;
        }
        
        @members {
            private int count = 0;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 2);
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("members"));
}

/// Test grammar options (ANTLR4 test suite: options)
#[test]
fn test_antlr4_suite_options() {
    let grammar_text = r#"
        grammar Options;
        
        options {
            language = java;
            tokenVocab = CommonTokens;
            superClass = BaseParser;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.options.len(), 3);
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("java"));
}

/// Test grammar imports (ANTLR4 test suite: imports)
#[test]
fn test_antlr4_suite_imports() {
    let grammar_text = r#"
        grammar Main;
        
        import CommonTokens;
        import CommonRules;
        
        expr: term;
        term: NUMBER;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.imports.len(), 2);
}

/// Test combined grammar (ANTLR4 test suite: combined grammar)
#[test]
fn test_antlr4_suite_combined_grammar() {
    let grammar_text = r#"
        grammar Combined;
        
        expr: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | '(' expr ')';
        
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.parser_rules().count(), 3);
    assert_eq!(grammar.lexer_rules().count(), 3);
}

/// Test lexer grammar (ANTLR4 test suite: lexer grammar)
#[test]
#[ignore]
fn test_antlr4_suite_lexer_grammar() {
    let grammar_text = r#"
        lexer grammar CommonLexer;
        
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.grammar_type, minipg::core::types::GrammarType::Lexer);
    assert_eq!(grammar.lexer_rules().count(), 4);
}

/// Test parser grammar (ANTLR4 test suite: parser grammar)
#[test]
#[ignore]
fn test_antlr4_suite_parser_grammar() {
    let grammar_text = r#"
        parser grammar MyParser;
        
        options {
            tokenVocab = CommonLexer;
        }
        
        expr: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | '(' expr ')';
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.grammar_type, minipg::core::types::GrammarType::Parser);
    assert_eq!(grammar.parser_rules().count(), 3);
}

/// Test EOF handling (ANTLR4 test suite: EOF)
#[test]
#[ignore]
fn test_antlr4_suite_eof() {
    let grammar_text = r#"
        grammar EOFTest;
        
        program: statement* EOF;
        statement: expr ';';
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 4);
}

/// Test complex expression grammar (ANTLR4 test suite: expressions)
#[test]
#[ignore]
fn test_antlr4_suite_complex_expressions() {
    let grammar_text = r#"
        grammar ComplexExpressions;
        
        expr: expr '*' expr | expr '/' expr | expr '+' expr | expr '-' expr | '(' expr ')' | NUMBER;
        
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.rules.len(), 1);
}

/// Test all code generators with ANTLR4 test suite grammar
#[test]
fn test_antlr4_suite_all_code_generators() {
    let grammar_text = r#"
        grammar TestSuite;
        
        program: statement+ EOF;
        statement: assignment | expression;
        assignment: ID '=' expression;
        expression: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | ID | '(' expression ')';
        
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let languages = vec!["rust", "python", "javascript", "typescript", "go", "c", "cpp", "java"];
    
    for lang in languages {
        let config = minipg::core::types::CodeGenConfig {
            target_language: lang.to_string(),
            output_directory: ".".to_string(),
            package_name: None,
            generate_listener: false,
            generate_visitor: false,
        };
        
        let generator = minipg::codegen::CodeGenerator::new(config);
        let analysis = minipg::analysis::AnalysisResult::new(grammar.clone());
        let code = generator.generate_from_analysis(&analysis).expect(&format!("Failed to generate {}", lang));
        
        assert!(!code.is_empty(), "Generated code for {} is empty", lang);
        assert!(code.len() > 100, "Generated code for {} is too short", lang);
    }
}
