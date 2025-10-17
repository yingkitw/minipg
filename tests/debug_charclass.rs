//! Debug test for character class parsing

use minipg::parser::{Lexer, Parser};

#[test]
fn debug_tokens_for_alphanum() {
    let grammar = r#"
grammar Test;
ALPHANUM: [a-zA-Z0-9];
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    // Print all tokens
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
fn debug_parse_charclass() {
    let grammar = r#"
grammar Test;
ALPHANUM: [a-zA-Z0-9];
"#;

    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    
    match &result {
        Ok(g) => {
            println!("\n✅ Parse succeeded!");
            for rule in g.lexer_rules() {
                println!("  Rule: {}", rule.name);
                println!("  Alternatives: {}", rule.alternatives.len());
            }
        }
        Err(e) => {
            println!("\n❌ Parse failed: {}", e);
        }
    }
    
    assert!(result.is_ok(), "Should parse successfully");
}
