//! Lexer for grammar files.

use super::token::{Token, TokenKind};

/// Lexer mode for context-aware tokenization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerMode {
    /// Normal mode - default tokenization
    Normal,
    /// Character class mode - inside [...]
    CharClass,
    /// String literal mode - inside "..."
    String,
}

/// Lexer for tokenizing grammar files.
pub struct Lexer {
    source: Vec<char>,
    #[allow(dead_code)]
    filename: String,
    position: usize,
    line: usize,
    column: usize,
    mode: LexerMode,
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    pub fn new(source: &str, filename: &str) -> Self {
        Self {
            source: source.chars().collect(),
            filename: filename.to_string(),
            position: 0,
            line: 1,
            column: 1,
            mode: LexerMode::Normal,
            mode_stack: Vec::new(),
        }
    }
    
    fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(self.mode);
        self.mode = mode;
    }
    
    fn pop_mode(&mut self) {
        if let Some(prev_mode) = self.mode_stack.pop() {
            self.mode = prev_mode;
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Token::eof(self.line, self.column);
        }

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current_char();

        match ch {
            ':' => {
                self.advance();
                Token::new(TokenKind::Colon, ":".to_string(), start_line, start_column)
            }
            ';' => {
                self.advance();
                Token::new(TokenKind::Semicolon, ";".to_string(), start_line, start_column)
            }
            '|' => {
                self.advance();
                Token::new(TokenKind::Pipe, "|".to_string(), start_line, start_column)
            }
            '?' => {
                self.advance();
                Token::new(TokenKind::Question, "?".to_string(), start_line, start_column)
            }
            '*' => {
                self.advance();
                Token::new(TokenKind::Star, "*".to_string(), start_line, start_column)
            }
            '+' => {
                self.advance();
                Token::new(TokenKind::Plus, "+".to_string(), start_line, start_column)
            }
            '~' => {
                self.advance();
                Token::new(TokenKind::Not, "~".to_string(), start_line, start_column)
            }
            '(' => {
                self.advance();
                Token::new(TokenKind::LeftParen, "(".to_string(), start_line, start_column)
            }
            ')' => {
                self.advance();
                Token::new(TokenKind::RightParen, ")".to_string(), start_line, start_column)
            }
            '{' => {
                self.advance();
                Token::new(TokenKind::LeftBrace, "{".to_string(), start_line, start_column)
            }
            '}' => {
                self.advance();
                Token::new(TokenKind::RightBrace, "}".to_string(), start_line, start_column)
            }
            '[' => {
                self.advance();
                // Enter character class mode
                self.push_mode(LexerMode::CharClass);
                Token::new(TokenKind::LeftBracket, "[".to_string(), start_line, start_column)
            }
            ']' => {
                self.advance();
                // Exit character class mode if we're in one
                if self.mode == LexerMode::CharClass {
                    self.pop_mode();
                }
                Token::new(TokenKind::RightBracket, "]".to_string(), start_line, start_column)
            }
            '=' => {
                self.advance();
                Token::new(TokenKind::Equals, "=".to_string(), start_line, start_column)
            }
            '.' => {
                self.advance();
                if self.current_char() == '.' {
                    self.advance();
                    Token::new(TokenKind::Range, "..".to_string(), start_line, start_column)
                } else {
                    Token::new(TokenKind::Dot, ".".to_string(), start_line, start_column)
                }
            }
            '-' => {
                self.advance();
                if self.current_char() == '>' {
                    self.advance();
                    Token::new(TokenKind::Arrow, "->".to_string(), start_line, start_column)
                } else {
                    Token::error(format!("unexpected character: {}", ch), start_line, start_column)
                }
            }
            '\'' => self.lex_char_or_string_literal(),
            '"' => self.lex_string_literal(),
            _ if ch.is_alphabetic() || ch == '_' => self.lex_identifier_or_keyword(),
            _ => {
                self.advance();
                Token::error(format!("unexpected character: {}", ch), start_line, start_column)
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut text = String::new();

        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            text.push(self.current_char());
            self.advance();
        }

        let kind = match text.as_str() {
            "grammar" => TokenKind::Grammar,
            "lexer" => TokenKind::Lexer,
            "parser" => TokenKind::Parser,
            "fragment" => TokenKind::Fragment,
            "import" => TokenKind::Import,
            "options" => TokenKind::Options,
            _ => TokenKind::Identifier,
        };

        Token::new(kind, text, start_line, start_column)
    }

    fn lex_string_literal(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut text = String::new();
        
        self.advance(); // skip opening quote
        
        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    text.push(self.current_char());
                    self.advance();
                }
            } else {
                text.push(self.current_char());
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Token::error("unterminated string literal".to_string(), start_line, start_column);
        }
        
        self.advance(); // skip closing quote
        Token::new(TokenKind::StringLiteral, text, start_line, start_column)
    }

    fn lex_char_or_string_literal(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut text = String::new();
        
        self.advance(); // skip opening quote
        
        while !self.is_at_end() && self.current_char() != '\'' {
            if self.current_char() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    text.push(self.current_char());
                    self.advance();
                }
            } else {
                text.push(self.current_char());
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Token::error("unterminated character literal".to_string(), start_line, start_column);
        }
        
        self.advance(); // skip closing quote
        
        // Treat as string literal for grammar purposes
        Token::new(TokenKind::StringLiteral, text, start_line, start_column)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.current_char() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_char() == '/' {
                        // Line comment
                        while !self.is_at_end() && self.current_char() != '\n' {
                            self.advance();
                        }
                    } else if self.peek_char() == '*' {
                        // Block comment
                        self.advance(); // skip /
                        self.advance(); // skip *
                        while !self.is_at_end() {
                            if self.current_char() == '*' && self.peek_char() == '/' {
                                self.advance(); // skip *
                                self.advance(); // skip /
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn peek_char(&self) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + 1]
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            if self.source[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}
