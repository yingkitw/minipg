//! Lexer tests.

use minipg::parser::{Lexer, TokenKind};

#[test]
fn test_lexer_keywords() {
    let mut lexer = Lexer::new("grammar lexer parser fragment import options", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Grammar);
    assert_eq!(lexer.next_token().kind, TokenKind::Lexer);
    assert_eq!(lexer.next_token().kind, TokenKind::Parser);
    assert_eq!(lexer.next_token().kind, TokenKind::Fragment);
    assert_eq!(lexer.next_token().kind, TokenKind::Import);
    assert_eq!(lexer.next_token().kind, TokenKind::Options);
    assert_eq!(lexer.next_token().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_identifiers() {
    let mut lexer = Lexer::new("expr term NUMBER ID", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn test_lexer_operators() {
    let mut lexer = Lexer::new(": ; | ? * + . ~ ( ) { } [ ] = ->", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Colon);
    assert_eq!(lexer.next_token().kind, TokenKind::Semicolon);
    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(lexer.next_token().kind, TokenKind::Question);
    assert_eq!(lexer.next_token().kind, TokenKind::Star);
    assert_eq!(lexer.next_token().kind, TokenKind::Plus);
    assert_eq!(lexer.next_token().kind, TokenKind::Dot);
    assert_eq!(lexer.next_token().kind, TokenKind::Not);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RightParen);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::LeftBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::RightBracket);
    assert_eq!(lexer.next_token().kind, TokenKind::Equals);
    assert_eq!(lexer.next_token().kind, TokenKind::Arrow);
}

#[test]
fn test_lexer_string_literal() {
    let mut lexer = Lexer::new(r#""hello" "world""#, "test.g4");
    
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.text, "hello");
    
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.text, "world");
}

#[test]
fn test_lexer_char_literal() {
    let mut lexer = Lexer::new("'a' 'b' '+'", "test.g4");
    
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.text, "a");
    
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.text, "b");
    
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.text, "+");
}

#[test]
fn test_lexer_line_comment() {
    let mut lexer = Lexer::new("expr // this is a comment\nterm", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn test_lexer_block_comment() {
    let mut lexer = Lexer::new("expr /* block comment */ term", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn test_lexer_whitespace() {
    let mut lexer = Lexer::new("  expr  \t\n  term  ", "test.g4");
    
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
}

#[test]
fn test_lexer_range() {
    let mut lexer = Lexer::new("..", "test.g4");
    assert_eq!(lexer.next_token().kind, TokenKind::Range);
}

#[test]
fn test_lexer_escaped_string() {
    let mut lexer = Lexer::new(r#""hello\nworld""#, "test.g4");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert!(token.text.contains("n")); // escaped \n becomes just n
}

#[test]
fn test_lexer_unterminated_string() {
    let mut lexer = Lexer::new(r#""unterminated"#, "test.g4");
    let token = lexer.next_token();
    assert_eq!(token.kind, TokenKind::Error);
}

#[test]
fn test_lexer_token_text() {
    let mut lexer = Lexer::new("myRule", "test.g4");
    let token = lexer.next_token();
    assert_eq!(token.text, "myRule");
}
