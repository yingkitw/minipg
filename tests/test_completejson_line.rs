//! Test the exact line from CompleteJSON.g4

use minipg::parser::{Lexer, Parser};

#[test]
fn test_safecodepoint_tokens() {
    let grammar = r#"
grammar Test;

fragment SAFECODEPOINT
    : ~ ["\\\u0000-\u001F]
    ;
"#;

    let mut lexer = Lexer::new(grammar, "test.g4");
    
    println!("\nTokens:");
    loop {
        let token = lexer.next_token();
        println!("  Line {}, Col {}: {:?} = '{}'", token.line, token.column, token.kind, token.text);
        if token.kind == minipg::parser::token::TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn test_safecodepoint_parse() {
    let grammar = r#"
grammar Test;

fragment SAFECODEPOINT
    : ~ ["\\\u0000-\u001F]
    ;
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
    
    assert!(result.is_ok(), "Should parse SAFECODEPOINT");
}
