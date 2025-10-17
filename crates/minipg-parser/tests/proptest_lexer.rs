//! Property-based tests for the lexer.

use minipg::parser::{Lexer, TokenKind};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_lexer_doesnt_crash(s in "\\PC*") {
        let mut lexer = Lexer::new(&s, "test.g4");
        // Just ensure it doesn't crash
        while lexer.next_token().kind != TokenKind::Eof {
            // Continue
        }
    }
    
    #[test]
    fn test_identifier_roundtrip(name in "[a-z][a-zA-Z0-9_]*") {
        let mut lexer = Lexer::new(&name, "test.g4");
        let token = lexer.next_token();
        
        prop_assert_eq!(token.kind, TokenKind::Identifier);
        prop_assert_eq!(token.text, name);
    }
    
    #[test]
    fn test_string_literal_roundtrip(content in "[a-zA-Z0-9 ]+") {
        let input = format!("\"{}\"", content);
        let mut lexer = Lexer::new(&input, "test.g4");
        let token = lexer.next_token();
        
        prop_assert_eq!(token.kind, TokenKind::StringLiteral);
        prop_assert_eq!(token.text, content);
    }
    
    #[test]
    fn test_whitespace_ignored(ws in "[ \t\n\r]+", name in "[a-z]+") {
        let input = format!("{}{}", ws, name);
        let mut lexer = Lexer::new(&input, "test.g4");
        let token = lexer.next_token();
        
        // Whitespace should be skipped
        prop_assert_eq!(token.kind, TokenKind::Identifier);
        prop_assert_eq!(token.text, name);
    }
    
    #[test]
    fn test_multiple_tokens(count in 1..10usize) {
        let input = "id ".repeat(count);
        let mut lexer = Lexer::new(&input, "test.g4");
        
        let mut token_count = 0;
        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof {
                break;
            }
            token_count += 1;
        }
        
        prop_assert_eq!(token_count, count);
    }
}
