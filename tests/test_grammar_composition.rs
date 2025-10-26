//! Tests for grammar composition and imports.

use minipg::analysis::GrammarComposer;
use minipg::ast::Grammar;
use minipg::core::types::GrammarType;
use minipg::core::GrammarParser;

#[test]
fn test_grammar_composer_new() {
    let composer = GrammarComposer::new();
    assert_eq!(composer.search_paths.len(), 1);
}

#[test]
fn test_grammar_composer_add_search_path() {
    let mut composer = GrammarComposer::new();
    composer.add_search_path("/path/to/grammars");
    assert_eq!(composer.search_paths.len(), 2);
}

#[test]
fn test_grammar_composer_default() {
    let composer = GrammarComposer::default();
    assert_eq!(composer.search_paths.len(), 1);
}

#[test]
fn test_merge_grammar_rules() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    let rule1 = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    let rule2 = minipg::ast::Rule::new("term".to_string(), minipg::ast::RuleType::Parser);
    
    target.add_rule(rule1);
    source.add_rule(rule2);
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.rules.len(), 2);
    assert!(target.rules.iter().any(|r| r.name == "expr"));
    assert!(target.rules.iter().any(|r| r.name == "term"));
}

#[test]
fn test_merge_grammar_options() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    target.add_option("key1".to_string(), "value1".to_string());
    source.add_option("key2".to_string(), "value2".to_string());
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.options.len(), 2);
    assert_eq!(target.options.get("key1").map(|s| s.as_str()), Some("value1"));
    assert_eq!(target.options.get("key2").map(|s| s.as_str()), Some("value2"));
}

#[test]
fn test_merge_grammar_options_override() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    target.add_option("key".to_string(), "target_value".to_string());
    source.add_option("key".to_string(), "source_value".to_string());
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.options.len(), 1);
    assert_eq!(target.options.get("key").map(|s| s.as_str()), Some("source_value"));
}

#[test]
fn test_merge_grammar_named_actions() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    target.add_named_action("header".to_string(), "// target header".to_string());
    source.add_named_action("members".to_string(), "// members".to_string());
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.named_actions.len(), 2);
    assert!(target.named_actions.contains_key("header"));
    assert!(target.named_actions.contains_key("members"));
}

#[test]
fn test_merge_grammar_channels() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    target.add_channel("COMMENTS".to_string());
    source.add_channel("HIDDEN".to_string());
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.channels.len(), 2);
    assert!(target.channels.contains("COMMENTS"));
    assert!(target.channels.contains("HIDDEN"));
}

#[test]
fn test_merge_grammar_lexer_modes() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    target.add_lexer_mode("MODE1".to_string(), vec!["RULE1".to_string()]);
    source.add_lexer_mode("MODE2".to_string(), vec!["RULE2".to_string()]);
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.lexer_modes.len(), 2);
    assert!(target.lexer_modes.contains_key("MODE1"));
    assert!(target.lexer_modes.contains_key("MODE2"));
}

#[test]
fn test_merge_grammar_conflict() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    let rule1 = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    let rule2 = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    
    target.add_rule(rule1);
    source.add_rule(rule2);
    
    let composer = GrammarComposer::new();
    let result = composer.merge_grammar(&mut target, &source);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("conflicting rules"));
}

#[test]
fn test_merge_grammar_multiple_conflicts() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let mut source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    let rule1 = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    let rule2 = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    let rule3 = minipg::ast::Rule::new("term".to_string(), minipg::ast::RuleType::Parser);
    let rule4 = minipg::ast::Rule::new("term".to_string(), minipg::ast::RuleType::Parser);
    
    target.add_rule(rule1);
    target.add_rule(rule3);
    source.add_rule(rule2);
    source.add_rule(rule4);
    
    let composer = GrammarComposer::new();
    let result = composer.merge_grammar(&mut target, &source);
    
    assert!(result.is_err());
}

#[test]
fn test_grammar_composer_validate() {
    let grammar = Grammar::new("Test".to_string(), GrammarType::Combined);
    let composer = GrammarComposer::new();
    
    let result = composer.validate(&grammar);
    assert!(result.is_ok());
}

#[test]
fn test_grammar_composition_with_parser() {
    let grammar_text = r#"
        grammar Test;
        
        import Base;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = minipg::parser::GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify import was parsed
    assert_eq!(grammar.imports.len(), 1);
    assert_eq!(grammar.imports[0], "Base");
}

#[test]
fn test_grammar_composition_multiple_imports() {
    let grammar_text = r#"
        grammar Test;
        
        import Base;
        import Common;
        import Utilities;
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = minipg::parser::GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify all imports were parsed
    assert_eq!(grammar.imports.len(), 3);
    assert!(grammar.imports.contains(&"Base".to_string()));
    assert!(grammar.imports.contains(&"Common".to_string()));
    assert!(grammar.imports.contains(&"Utilities".to_string()));
}

#[test]
fn test_grammar_composition_with_options() {
    let grammar_text = r#"
        grammar Test;
        
        options {
            language = java;
            tokenVocab = CommonTokens;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = minipg::parser::GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify options were parsed
    assert_eq!(grammar.options.len(), 2);
    assert_eq!(grammar.options.get("language").map(|s| s.as_str()), Some("java"));
    assert_eq!(grammar.options.get("tokenVocab").map(|s| s.as_str()), Some("CommonTokens"));
}

#[test]
fn test_grammar_composition_import_and_options() {
    let grammar_text = r#"
        grammar Test;
        
        import Base;
        
        options {
            language = python;
        }
        
        expr: term;
        term: NUMBER;
        
        NUMBER: [0-9]+;
    "#;
    
    let parser = minipg::parser::GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify both imports and options were parsed
    assert_eq!(grammar.imports.len(), 1);
    assert_eq!(grammar.options.len(), 1);
}

#[test]
fn test_merge_preserves_target_rules() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    let rule = minipg::ast::Rule::new("expr".to_string(), minipg::ast::RuleType::Parser);
    target.add_rule(rule);
    
    let initial_count = target.rules.len();
    
    let composer = GrammarComposer::new();
    composer.merge_grammar(&mut target, &source).expect("merge failed");
    
    assert_eq!(target.rules.len(), initial_count);
}

#[test]
fn test_merge_empty_grammars() {
    let mut target = Grammar::new("Target".to_string(), GrammarType::Combined);
    let source = Grammar::new("Source".to_string(), GrammarType::Combined);
    
    let composer = GrammarComposer::new();
    let result = composer.merge_grammar(&mut target, &source);
    
    assert!(result.is_ok());
    assert_eq!(target.rules.len(), 0);
}

#[test]
fn test_grammar_composition_complex_scenario() {
    let grammar_text = r#"
        grammar ComplexComposition;
        
        import Base;
        import Common;
        
        options {
            language = rust;
            tokenVocab = BaseTokens;
        }
        
        @header {
            use std::collections::HashMap;
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
    
    let parser = minipg::parser::GrammarParser::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify all components were parsed
    assert_eq!(grammar.imports.len(), 2);
    assert_eq!(grammar.options.len(), 2);
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.rules.len() > 0);
}
