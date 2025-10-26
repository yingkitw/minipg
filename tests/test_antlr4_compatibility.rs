//! Tests for full ANTLR4 compatibility.

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

#[test]
#[ignore]
fn test_named_action_header() {
    let grammar_text = r#"
        grammar Test;
        
        @header {
            package com.example;
            import java.util.*;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("header"));
    let header = grammar.named_actions.get("header").unwrap();
    assert!(header.contains("package com.example"));
}

#[test]
fn test_named_action_members() {
    let grammar_text = r#"
        grammar Test;
        
        @members {
            private int count = 0;
            private String buffer = "";
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("members"));
    let members = grammar.named_actions.get("members").unwrap();
    assert!(members.contains("count"));
}

#[test]
fn test_multiple_named_actions() {
    let grammar_text = r#"
        grammar Test;
        
        @header {
            package com.example;
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

#[test]
fn test_grammar_options_language() {
    let grammar_text = r#"
        grammar Test;
        
        options {
            language = java;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.options.len(), 1);
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("java"));
}

#[test]
fn test_grammar_options_token_vocab() {
    let grammar_text = r#"
        grammar Test;
        
        options {
            tokenVocab = CommonTokens;
        }
        
        expr: term;
        term: ID;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.options.len(), 1);
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("CommonTokens"));
}

#[test]
fn test_grammar_options_multiple() {
    let grammar_text = r#"
        grammar Test;
        
        options {
            language = python;
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
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("python"));
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("CommonTokens"));
    assert_eq!(grammar.options.get("superClass").map(|s| s.as_str()), Some("BaseParser"));
}

#[test]
#[ignore]
fn test_lexer_and_parser_grammars() {
    let lexer_grammar = r#"
        lexer grammar CommonLexer;
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(lexer_grammar, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.grammar_type, minipg::core::types::GrammarType::Lexer);
}

#[test]
#[ignore]
fn test_parser_grammar_with_token_vocab() {
    let parser_grammar = r#"
        parser grammar MyParser;
        
        options {
            tokenVocab = CommonLexer;
        }
        
        program: statement+;
        statement: assignment | expression;
        assignment: ID '=' expression;
        expression: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | ID | '(' expression ')';
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(parser_grammar, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.grammar_type, minipg::core::types::GrammarType::Parser);
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("CommonLexer"));
}

#[test]
fn test_combined_grammar_with_all_features() {
    let grammar_text = r#"
        grammar CompleteExample;
        
        options {
            language = java;
            tokenVocab = CommonTokens;
        }
        
        @header {
            package com.example.parser;
            import java.util.*;
        }
        
        @members {
            private int depth = 0;
            private Stack<Integer> modeStack = new Stack<>();
        }
        
        program: statement+ EOF;
        statement: assignment | expression;
        assignment: ID '=' expression;
        expression: term (('+' | '-') term)*;
        term: factor (('*' | '/') factor)*;
        factor: NUMBER | ID | '(' expression ')';
        
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify all components
    assert_eq!(grammar.grammar_type, minipg::core::types::GrammarType::Combined);
    assert_eq!(grammar.options.len(), 2);
    assert_eq!(grammar.named_actions.len(), 2);
    assert!(grammar.rules.len() > 0);
}

#[test]
#[ignore]
fn test_antlr4_standard_options() {
    let grammar_text = r#"
        grammar StandardOptions;
        
        options {
            language = python;
            tokenVocab = Tokens;
            superClass = BaseParser;
            package = com.example;
            namespace = Example;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify all standard ANTLR4 options are parsed
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("python"));
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("Tokens"));
    assert_eq!(grammar.options.get("superClass").map(|s| s.as_str()), Some("BaseParser"));
    assert_eq!(grammar.options.get("package").map(|s| s.as_str()), Some("com.example"));
    assert_eq!(grammar.options.get("namespace").map(|s| s.as_str()), Some("Example"));
}

#[test]
#[ignore]
fn test_named_actions_with_nested_braces() {
    let grammar_text = r#"
        grammar NestedBraces;
        
        @header {
            public class MyParser {
                public void init() {
                    System.out.println("Initialized");
                }
            }
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    let header = grammar.named_actions.get("header").unwrap();
    assert!(header.contains("public class MyParser"));
    assert!(header.contains("System.out.println"));
}

#[test]
fn test_rule_arguments_with_types() {
    let grammar_text = r#"
        grammar RuleArgs;
        
        expr[int x, String name]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.arguments.len(), 2);
}

#[test]
fn test_rule_returns() {
    let grammar_text = r#"
        grammar RuleReturns;
        
        expr returns [int value, String text]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.returns.len(), 2);
}

#[test]
#[ignore]
fn test_rule_locals() {
    let grammar_text = r#"
        grammar RuleLocals;
        
        expr locals [int count, String buffer]: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    let expr_rule = grammar.rules.iter().find(|r| r.name == "expr").expect("expr rule not found");
    assert_eq!(expr_rule.locals.len(), 1);
}

#[test]
fn test_lexer_modes() {
    let grammar_text = r#"
        grammar LexerModes;
        
        QUOTE: '"' -> mode(STRING_MODE);
        ID: [a-zA-Z_]+;
        
        mode STRING_MODE;
        STRING_CHAR: ~["];
        END_QUOTE: '"' -> popMode;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert!(grammar.has_modes());
    assert_eq!(grammar.lexer_modes.len(), 1);
}

#[test]
#[ignore]
fn test_lexer_channels() {
    let grammar_text = r#"
        grammar LexerChannels;
        
        ID: [a-zA-Z_]+;
        COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    assert!(grammar.has_channels());
    assert!(grammar.channels.contains("COMMENTS"));
}

#[test]
#[ignore]
fn test_antlr4_complete_json_grammar() {
    let grammar_text = r#"
        grammar JSON;
        
        options {
            language = java;
        }
        
        @header {
            package com.example.json;
        }
        
        json: value EOF;
        value: object | array | STRING | NUMBER | 'true' | 'false' | 'null';
        object: '{' (pair (',' pair)*)? '}';
        pair: STRING ':' value;
        array: '[' (value (',' value)*)? ']';
        
        STRING: '"' (~["\\\r\n] | '\\' .)* '"';
        NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify ANTLR4 compatibility
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("java"));
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.rules.len() > 0);
}
