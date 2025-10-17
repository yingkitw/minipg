//! Test Unicode escape sequences in character classes

use minipg::parser::{Lexer, Parser};

#[test]
fn test_simple_quote_in_charclass() {
    let grammar = r#"
grammar Test;
SAFE: ["];
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    println!("\nTokens for [\"]:");
    loop {
        let token = lexer.next_token();
        println!("  {:?}: '{}'", token.kind, token.text);
        if token.kind == minipg::parser::token::TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn test_negated_charclass() {
    let grammar = r#"
grammar Test;
SAFE: ~["];
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    println!("\nTokens for ~[\"]:");
    loop {
        let token = lexer.next_token();
        println!("  {:?}: '{}'", token.kind, token.text);
        if token.kind == minipg::parser::token::TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn test_unicode_escape_tokens() {
    let grammar = r#"
grammar Test;
SAFE: ~["\\\u0000-\u001F];
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    println!("\nTokens:");
    loop {
        let token = lexer.next_token();
        println!("  {:?}: '{}'", token.kind, token.text);
        if token.kind == minipg::parser::token::TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn test_double_backslash() {
    let grammar = r#"
grammar Test;
SAFE: [\\];
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    println!("\nTokens for [\\\\]:");
    loop {
        let token = lexer.next_token();
        println!("  {:?}: '{}'", token.kind, token.text);
        if token.kind == minipg::parser::token::TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn test_unicode_escape_parse() {
    let grammar = r#"
grammar Test;
SAFE: ~["\\\u0000-\u001F];
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    match &result {
        Ok(g) => {
            println!("\n✅ Parse succeeded!");
            for rule in g.lexer_rules() {
                println!("  Rule: {}", rule.name);
            }
        }
        Err(e) => {
            println!("\n❌ Parse failed: {}", e);
        }
    }
    
    assert!(result.is_ok(), "Should parse Unicode escapes");
}
