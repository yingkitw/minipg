//! Lexer for grammar files.

use super::token::{Token, TokenKind};

/// Lexer mode for context-aware tokenization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerMode {
    /// Normal mode - default tokenization
    Normal,
    /// Character class mode - inside [...]
    CharClass,
    /// String literal mode - inside "..." (reserved for future use)
    #[allow(dead_code)]
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
    last_token_kind: Option<TokenKind>,
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
            last_token_kind: None,
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
        // In CharClass mode, don't skip comments - treat / as a regular character
        if self.mode != LexerMode::CharClass {
            self.skip_whitespace_and_comments();
        } else {
            // In CharClass mode, only skip whitespace, not comments
            self.skip_whitespace();
        }

        if self.is_at_end() {
            return Token::eof(self.line, self.column);
        }

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current_char();

        let token = match ch {
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
            ',' => {
                self.advance();
                Token::new(TokenKind::Comma, ",".to_string(), start_line, start_column)
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
                if self.mode == LexerMode::CharClass {
                    // In CharClass mode, + is just a character
                    let text = self.current_char().to_string();
                    self.advance();
                    Token::new(TokenKind::Identifier, text, start_line, start_column)
                } else {
                    self.advance();
                    // Check for +=
                    if self.current_char() == '=' {
                        self.advance();
                        Token::new(TokenKind::PlusEquals, "+=".to_string(), start_line, start_column)
                    } else {
                        Token::new(TokenKind::Plus, "+".to_string(), start_line, start_column)
                    }
                }
            }
            '~' => {
                self.advance();
                Token::new(TokenKind::Not, "~".to_string(), start_line, start_column)
            }
            '@' => {
                self.advance();
                Token::new(TokenKind::At, "@".to_string(), start_line, start_column)
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
                // Enter character class mode in lexer rule contexts
                // In ANTLR4, [...] is always a character class in lexer rules
                // Common contexts: after : (rule start), | (alternative), ~ (negation), ( (grouping), ] (after another char class), ? * + (after quantifiers)
                if matches!(self.last_token_kind, 
                    Some(TokenKind::Colon) | Some(TokenKind::Pipe) | Some(TokenKind::Not) | 
                    Some(TokenKind::LeftParen) | Some(TokenKind::RightBracket) | 
                    Some(TokenKind::Question) | Some(TokenKind::Star) | Some(TokenKind::Plus)) {
                    self.push_mode(LexerMode::CharClass);
                }
                Token::new(TokenKind::LeftBracket, "[".to_string(), start_line, start_column)
            }
            ']' => {
                self.advance();
                // Exit character class mode
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
                    // Standalone minus (used in character classes for ranges)
                    Token::new(TokenKind::Minus, "-".to_string(), start_line, start_column)
                }
            }
            '\'' => {
                if self.mode == LexerMode::CharClass {
                    // In CharClass mode, single quote is just a character
                    let text = self.current_char().to_string();
                    self.advance();
                    Token::new(TokenKind::Identifier, text, start_line, start_column)
                } else {
                    self.lex_char_or_string_literal()
                }
            }
            '"' => {
                if self.mode == LexerMode::CharClass {
                    // In CharClass mode, double quote is just a character
                    let text = self.current_char().to_string();
                    self.advance();
                    Token::new(TokenKind::Identifier, text, start_line, start_column)
                } else {
                    self.lex_string_literal()
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => self.lex_identifier_or_keyword(),
            _ if ch.is_numeric() => {
                // Standalone digit - treat as identifier for character class ranges
                let text = ch.to_string();
                self.advance();
                Token::new(TokenKind::Identifier, text, start_line, start_column)
            }
            '\\' if self.mode == LexerMode::CharClass => {
                // Handle escape sequences in character classes
                self.lex_escape_sequence(start_line, start_column)
            }
            _ if self.mode == LexerMode::CharClass => {
                // In CharClass mode, treat any other character as a literal character
                let text = ch.to_string();
                self.advance();
                Token::new(TokenKind::Identifier, text, start_line, start_column)
            }
            _ => {
                self.advance();
                Token::error(format!("unexpected character: {}", ch), start_line, start_column)
            }
        };
        
        // Track the token kind for context-aware lexing
        self.last_token_kind = Some(token.kind);
        token
    }

    fn lex_escape_sequence(&mut self, start_line: usize, start_column: usize) -> Token {
        let mut text = String::new();
        text.push(self.current_char()); // backslash
        self.advance();
        
        if self.is_at_end() {
            return Token::error("incomplete escape sequence".to_string(), start_line, start_column);
        }
        
        let escape_char = self.current_char();
        text.push(escape_char);
        self.advance();
        
        // Handle Unicode escapes: \uXXXX or \u{XXXXXX}
        if escape_char == 'u' {
            if self.current_char() == '{' {
                // Extended Unicode escape: \u{XXXXXX}
                text.push('{');
                self.advance();
                let mut digit_count = 0;
                while !self.is_at_end() && self.current_char() != '}' && digit_count < 6 {
                    if !self.current_char().is_ascii_hexdigit() {
                        return Token::error("invalid hex digit in unicode escape".to_string(), start_line, start_column);
                    }
                    text.push(self.current_char());
                    self.advance();
                    digit_count += 1;
                }
                if self.is_at_end() || self.current_char() != '}' {
                    return Token::error("unclosed unicode escape sequence".to_string(), start_line, start_column);
                }
                text.push('}');
                self.advance();
            } else {
                // Standard Unicode escape: \uXXXX (4 hex digits)
                for _ in 0..4 {
                    if self.is_at_end() || !self.current_char().is_ascii_hexdigit() {
                        return Token::error("invalid unicode escape: expected 4 hex digits".to_string(), start_line, start_column);
                    }
                    text.push(self.current_char());
                    self.advance();
                }
            }
        }
        // For other escapes like \n, \r, \t, \\, etc., we already consumed them
        
        // Return as StringLiteral so parser can handle it with parse_char_literal
        Token::new(TokenKind::StringLiteral, text, start_line, start_column)
    }
    
    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut text = String::new();

        // In CharClass mode, only consume a single character
        if self.mode == LexerMode::CharClass {
            text.push(self.current_char());
            self.advance();
        } else {
            // Normal mode: consume full identifier
            while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
                text.push(self.current_char());
                self.advance();
            }
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

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.current_char() {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                _ => break,
            }
        }
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
