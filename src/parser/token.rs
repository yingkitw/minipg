//! Token definitions for the grammar lexer.

use std::fmt;

/// Token kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Grammar,
    Lexer,
    Parser,
    Fragment,
    Import,
    Options,

    // Identifiers and literals
    Identifier,
    StringLiteral,
    CharLiteral,

    // Operators and punctuation
    Colon,
    Semicolon,
    Pipe,
    Question,
    Star,
    Plus,
    Dot,
    Range,
    Not,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Equals,
    Arrow,

    // Special
    Eof,
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Grammar => write!(f, "grammar"),
            TokenKind::Lexer => write!(f, "lexer"),
            TokenKind::Parser => write!(f, "parser"),
            TokenKind::Fragment => write!(f, "fragment"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Options => write!(f, "options"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::StringLiteral => write!(f, "string literal"),
            TokenKind::CharLiteral => write!(f, "char literal"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Range => write!(f, ".."),
            TokenKind::Not => write!(f, "~"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Eof => write!(f, "end of file"),
            TokenKind::Error => write!(f, "error"),
        }
    }
}

/// A token with location information.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, text: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            text,
            line,
            column,
        }
    }

    pub fn eof(line: usize, column: usize) -> Self {
        Self::new(TokenKind::Eof, String::new(), line, column)
    }

    pub fn error(text: String, line: usize, column: usize) -> Self {
        Self::new(TokenKind::Error, text, line, column)
    }
}
