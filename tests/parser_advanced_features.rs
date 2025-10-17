//! Tests for advanced parser features: arguments, returns, locals

use minipg::parser::{Lexer, Parser};

#[test]
fn test_parse_rule_with_arguments() {
    let grammar = r#"
grammar Test;

rule[int x, String name]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse rule with arguments");
    let grammar = result.unwrap();
    
    let rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.name, "rule");
    assert_eq!(rule.arguments.len(), 2);
    assert_eq!(rule.arguments[0].name, "x");
    assert_eq!(rule.arguments[0].arg_type, Some("int".to_string()));
    assert_eq!(rule.arguments[1].name, "name");
    assert_eq!(rule.arguments[1].arg_type, Some("String".to_string()));
}

#[test]
fn test_parse_rule_with_returns() {
    let grammar = r#"
grammar Test;

rule returns [Value result]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse rule with returns");
    let grammar = result.unwrap();
    
    let rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.returns.len(), 1);
    assert_eq!(rule.returns[0].name, "result");
    assert_eq!(rule.returns[0].return_type, Some("Value".to_string()));
}

#[test]
fn test_parse_rule_with_locals() {
    let grammar = r#"
grammar Test;

rule locals [int temp]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse rule with locals");
    let grammar = result.unwrap();
    
    let rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.locals.len(), 1);
    assert_eq!(rule.locals[0].name, "temp");
    assert_eq!(rule.locals[0].local_type, Some("int".to_string()));
}

#[test]
fn test_parse_rule_with_all_features() {
    let grammar = r#"
grammar Test;

rule[int x] returns [Value result] locals [int temp]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse rule with all features");
    let grammar = result.unwrap();
    
    let rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.arguments.len(), 1);
    assert_eq!(rule.returns.len(), 1);
    assert_eq!(rule.locals.len(), 1);
}

#[test]
fn test_parse_rule_without_types() {
    let grammar = r#"
grammar Test;

rule[x] returns [result] locals [temp]: 'test';
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    assert!(result.is_ok(), "Should parse rule without explicit types");
    let grammar = result.unwrap();
    
    let rules: Vec<_> = grammar.parser_rules().collect();
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.arguments.len(), 1);
    assert_eq!(rule.arguments[0].arg_type, None);
}
