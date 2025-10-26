//! Tests for lexer modes and channels parsing.

use minipg::core::GrammarParser;
use minipg::parser::GrammarParser as GP;

#[test]
fn test_parse_lexer_mode() {
    let grammar_text = r#"
        grammar Test;
        
        mode DEFAULT_MODE;
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        WS: [ \t\r\n]+ -> skip;
        
        mode STRING_MODE;
        STRING_CHAR: ~["\\\r\n];
        STRING_END: '"' -> popMode;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify modes are parsed
    assert!(grammar.has_modes());
    assert_eq!(grammar.lexer_modes.len(), 2);
    assert!(grammar.lexer_modes.contains_key("DEFAULT_MODE"));
    assert!(grammar.lexer_modes.contains_key("STRING_MODE"));
}

#[test]
fn test_parse_lexer_mode_with_rules() {
    let grammar_text = r#"
        grammar Test;
        
        mode DEFAULT_MODE;
        ID: [a-zA-Z_]+;
        NUMBER: [0-9]+;
        
        mode STRING_MODE;
        STRING_CONTENT: ~["];
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify modes and their rules
    assert_eq!(grammar.lexer_modes.len(), 2);
    
    let default_rules = &grammar.lexer_modes["DEFAULT_MODE"];
    assert_eq!(default_rules.len(), 2);
    assert!(default_rules.contains(&"ID".to_string()));
    assert!(default_rules.contains(&"NUMBER".to_string()));
    
    let string_rules = &grammar.lexer_modes["STRING_MODE"];
    assert_eq!(string_rules.len(), 1);
    assert!(string_rules.contains(&"STRING_CONTENT".to_string()));
}

#[test]
fn test_parse_channel_command() {
    let grammar_text = r#"
        grammar Test;
        
        ID: [a-zA-Z_]+;
        COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);
        WS: [ \t\r\n]+ -> skip;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Find the COMMENT rule
    let comment_rule = grammar.rules.iter().find(|r| r.name == "COMMENT").expect("COMMENT rule not found");
    
    // Verify the channel command
    assert!(!comment_rule.alternatives.is_empty());
    let alt = &comment_rule.alternatives[0];
    assert!(alt.lexer_command.is_some());
    
    if let Some(cmd) = &alt.lexer_command {
        match cmd {
            minipg::ast::LexerCommand::Channel(name) => {
                assert_eq!(name, "COMMENTS");
            }
            _ => panic!("Expected Channel command"),
        }
    }
}

#[test]
fn test_parse_mode_command() {
    let grammar_text = r#"
        grammar Test;
        
        QUOTE: '"' -> mode(STRING_MODE);
        
        mode STRING_MODE;
        STRING_CHAR: ~["];
        END_QUOTE: '"' -> popMode;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Find the QUOTE rule
    let quote_rule = grammar.rules.iter().find(|r| r.name == "QUOTE").expect("QUOTE rule not found");
    
    // Verify the mode command
    assert!(!quote_rule.alternatives.is_empty());
    let alt = &quote_rule.alternatives[0];
    assert!(alt.lexer_command.is_some());
    
    if let Some(cmd) = &alt.lexer_command {
        match cmd {
            minipg::ast::LexerCommand::Mode(name) => {
                assert_eq!(name, "STRING_MODE");
            }
            _ => panic!("Expected Mode command"),
        }
    }
}

#[test]
fn test_parse_pushmode_command() {
    let grammar_text = r#"
        grammar Test;
        
        LBRACE: '{' -> pushMode(NESTED_MODE);
        
        mode NESTED_MODE;
        RBRACE: '}' -> popMode;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Find the LBRACE rule
    let lbrace_rule = grammar.rules.iter().find(|r| r.name == "LBRACE").expect("LBRACE rule not found");
    
    // Verify the pushMode command
    assert!(!lbrace_rule.alternatives.is_empty());
    let alt = &lbrace_rule.alternatives[0];
    assert!(alt.lexer_command.is_some());
    
    if let Some(cmd) = &alt.lexer_command {
        match cmd {
            minipg::ast::LexerCommand::PushMode(name) => {
                assert_eq!(name, "NESTED_MODE");
            }
            _ => panic!("Expected PushMode command"),
        }
    }
}

#[test]
fn test_parse_popmode_command() {
    let grammar_text = r#"
        grammar Test;
        
        mode STRING_MODE;
        END_QUOTE: '"' -> popMode;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Find the END_QUOTE rule
    let end_quote_rule = grammar.rules.iter().find(|r| r.name == "END_QUOTE").expect("END_QUOTE rule not found");
    
    // Verify the popMode command
    assert!(!end_quote_rule.alternatives.is_empty());
    let alt = &end_quote_rule.alternatives[0];
    assert!(alt.lexer_command.is_some());
    
    if let Some(cmd) = &alt.lexer_command {
        match cmd {
            minipg::ast::LexerCommand::PopMode => {
                // Correct
            }
            _ => panic!("Expected PopMode command"),
        }
    }
}

#[test]
#[ignore]
fn test_parse_complex_modes_and_channels() {
    let grammar_text = r#"
        grammar ComplexLexer;
        
        // Default mode
        ID: [a-zA-Z_][a-zA-Z0-9_]*;
        NUMBER: [0-9]+;
        COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);
        LBRACE: '{' -> pushMode(NESTED);
        QUOTE: '"' -> mode(STRING);
        WS: [ \t\r\n]+ -> skip;
        
        // Nested mode
        mode NESTED;
        RBRACE: '}' -> popMode;
        NESTED_ID: [a-zA-Z_]+;
        
        // String mode
        mode STRING;
        STRING_CHAR: ~["];
        END_QUOTE: '"' -> popMode;
    "#;
    
    let parser = GP::new();
    let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
    
    // Verify modes
    assert_eq!(grammar.lexer_modes.len(), 3);
    assert!(grammar.lexer_modes.contains_key("NESTED"));
    assert!(grammar.lexer_modes.contains_key("STRING"));
    
    // Verify all rules are present
    assert_eq!(grammar.rules.len(), 11);
}
