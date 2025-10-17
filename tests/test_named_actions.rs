//! Tests for named action parsing (@header, @members, etc.)

use minipg::parser::{Lexer, Parser};

#[test]
fn test_parse_header_action() {
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
    
    assert_eq!(grammar.name, "Test");
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("header"));
    
    let header_code = grammar.named_actions.get("header").unwrap();
    // Code is tokenized, so check for key parts
    assert!(header_code.contains("use"));
    assert!(header_code.contains("HashMap"));
}

#[test]
fn test_parse_members_action() {
    let grammar = r#"
grammar Test;

@members {
    private int count = 0;
    public void increment() { count++; }
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    assert!(grammar.named_actions.contains_key("members"));
    
    let members_code = grammar.named_actions.get("members").unwrap();
    assert!(members_code.contains("private int count"));
    assert!(members_code.contains("public void increment"));
}

#[test]
fn test_parse_multiple_named_actions() {
    let grammar = r#"
grammar Test;

@header {
    import java.util.*;
}

@members {
    private List<String> items = new ArrayList<>();
}

@init {
    System.out.println("Initializing");
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 3);
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("members"));
    assert!(grammar.named_actions.contains_key("init"));
}

#[test]
fn test_named_action_with_nested_braces() {
    let grammar = r#"
grammar Test;

@members {
    public void process() {
        if (true) {
            System.out.println("nested");
        }
    }
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 1);
    let members_code = grammar.named_actions.get("members").unwrap();
    // Check for key parts (code is tokenized with spaces)
    assert!(members_code.contains("if"));
    assert!(members_code.contains("true"));
    assert!(members_code.contains("println"));
}

#[test]
fn test_grammar_with_no_named_actions() {
    let grammar = r#"
grammar Test;

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.named_actions.len(), 0);
}

#[test]
fn test_named_action_before_rules() {
    let grammar = r#"
grammar Test;

@header {
    // Header code
}

@footer {
    // Footer code
}

rule: ID;
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    // Should parse both named actions
    assert_eq!(grammar.named_actions.len(), 2);
    assert!(grammar.named_actions.contains_key("header"));
    assert!(grammar.named_actions.contains_key("footer"));
}
