//! Parser for grammar files.

use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use crate::ast::{Alternative, Element, Grammar, Rule};
use crate::core::{types::GrammarType, Error, Result};

/// Parser for grammar files.
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse(&mut self) -> Result<Grammar> {
        self.parse_grammar()
    }

    fn parse_grammar(&mut self) -> Result<Grammar> {
        // grammar <type> <name> ;
        self.expect(TokenKind::Grammar)?;
        
        let grammar_type = if self.current_token.kind == TokenKind::Lexer {
            self.advance();
            GrammarType::Lexer
        } else if self.current_token.kind == TokenKind::Parser {
            self.advance();
            GrammarType::Parser
        } else {
            GrammarType::Combined
        };

        let name = self.expect_identifier()?;
        self.expect(TokenKind::Semicolon)?;

        let mut grammar = Grammar::new(name, grammar_type);

        // Parse options, imports, and rules
        while self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Options {
                self.parse_options(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Import {
                self.parse_import(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Identifier {
                let rule = self.parse_rule()?;
                grammar.add_rule(rule);
            } else if self.current_token.kind == TokenKind::Fragment {
                let rule = self.parse_fragment_rule()?;
                grammar.add_rule(rule);
            } else {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    format!("unexpected token: {}", self.current_token.kind),
                ));
            }
        }

        Ok(grammar)
    }

    fn parse_options(&mut self, grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::Options)?;
        self.expect(TokenKind::LeftBrace)?;

        while self.current_token.kind != TokenKind::RightBrace {
            let key = self.expect_identifier()?;
            self.expect(TokenKind::Equals)?;
            let value = self.expect_identifier()?;
            self.expect(TokenKind::Semicolon)?;
            grammar.add_option(key, value);
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(())
    }

    fn parse_import(&mut self, grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::Import)?;
        let import_name = self.expect_identifier()?;
        self.expect(TokenKind::Semicolon)?;
        grammar.add_import(import_name);
        Ok(())
    }

    fn parse_rule(&mut self) -> Result<Rule> {
        let name = self.expect_identifier()?;
        
        // Determine if it's a lexer or parser rule based on first character
        let rule_type = if name.chars().next().unwrap().is_uppercase() {
            crate::ast::RuleType::Lexer
        } else {
            crate::ast::RuleType::Parser
        };

        let mut rule = Rule::new(name, rule_type);
        
        self.expect(TokenKind::Colon)?;
        
        // Parse alternatives
        let alt = self.parse_alternative()?;
        rule.add_alternative(alt);
        
        while self.current_token.kind == TokenKind::Pipe {
            self.advance();
            let alt = self.parse_alternative()?;
            rule.add_alternative(alt);
        }
        
        self.expect(TokenKind::Semicolon)?;
        Ok(rule)
    }

    fn parse_fragment_rule(&mut self) -> Result<Rule> {
        self.expect(TokenKind::Fragment)?;
        let mut rule = self.parse_rule()?;
        rule.set_fragment(true);
        Ok(rule)
    }

    fn parse_alternative(&mut self) -> Result<Alternative> {
        let mut alt = Alternative::new();
        
        while !self.is_alternative_end() {
            let element = self.parse_element()?;
            alt.add_element(element);
        }
        
        Ok(alt)
    }

    fn parse_element(&mut self) -> Result<Element> {
        let element = match self.current_token.kind {
            TokenKind::Identifier => {
                let name = self.expect_identifier()?;
                Element::rule_ref(name)
            }
            TokenKind::StringLiteral => {
                let value = self.current_token.text.clone();
                self.advance();
                Element::string_literal(value)
            }
            TokenKind::LeftParen => {
                self.advance();
                let mut alternatives = vec![self.parse_alternative()?];
                
                while self.current_token.kind == TokenKind::Pipe {
                    self.advance();
                    alternatives.push(self.parse_alternative()?);
                }
                
                self.expect(TokenKind::RightParen)?;
                Element::Group { alternatives }
            }
            TokenKind::LeftBracket => {
                self.advance();
                // Parse character class
                let element = self.parse_char_class()?;
                self.expect(TokenKind::RightBracket)?;
                element
            }
            TokenKind::Dot => {
                self.advance();
                Element::Wildcard
            }
            TokenKind::Not => {
                self.advance();
                let element = self.parse_element()?;
                Element::Not {
                    element: Box::new(element),
                }
            }
            _ => {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    format!("unexpected token in element: {}", self.current_token.kind),
                ));
            }
        };

        // Handle suffixes (?, *, +)
        let element = match self.current_token.kind {
            TokenKind::Question => {
                self.advance();
                Element::optional(element)
            }
            TokenKind::Star => {
                self.advance();
                Element::zero_or_more(element)
            }
            TokenKind::Plus => {
                self.advance();
                Element::one_or_more(element)
            }
            _ => element,
        };

        Ok(element)
    }

    fn parse_char_class(&mut self) -> Result<Element> {
        // Check for negation
        let negated = if self.current_token.kind == TokenKind::Not {
            self.advance();
            true
        } else {
            false
        };
        
        let mut ranges = Vec::new();
        
        // Parse character class contents
        while self.current_token.kind != TokenKind::RightBracket 
            && self.current_token.kind != TokenKind::Eof {
            
            if self.current_token.kind == TokenKind::StringLiteral 
                || self.current_token.kind == TokenKind::CharLiteral {
                let start_char = self.parse_char_literal()?;
                
                // Check for range
                if self.current_token.kind == TokenKind::Range {
                    self.advance();
                    if self.current_token.kind == TokenKind::StringLiteral 
                        || self.current_token.kind == TokenKind::CharLiteral {
                        let end_char = self.parse_char_literal()?;
                        ranges.push((start_char, end_char));
                    } else {
                        return Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "expected character after range operator".to_string(),
                        ));
                    }
                } else {
                    // Single character
                    ranges.push((start_char, start_char));
                }
            } else {
                break;
            }
        }
        
        if ranges.is_empty() {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "empty character class".to_string(),
            ));
        }
        
        Ok(Element::CharClass { negated, ranges })
    }
    
    fn parse_char_literal(&mut self) -> Result<char> {
        let text = &self.current_token.text;
        let ch = if text.starts_with("\\u") {
            // Unicode escape: \uXXXX
            let hex = &text[2..];
            u32::from_str_radix(hex, 16)
                .ok()
                .and_then(|code| char::from_u32(code))
                .ok_or_else(|| Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    format!("invalid unicode escape: {}", text),
                ))?
        } else if text.starts_with('\\') && text.len() == 2 {
            // Simple escape sequences
            match text.chars().nth(1).unwrap() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                c => c,
            }
        } else {
            text.chars().next().unwrap_or('\0')
        };
        
        self.advance();
        Ok(ch)
    }

    fn is_alternative_end(&self) -> bool {
        matches!(
            self.current_token.kind,
            TokenKind::Pipe
                | TokenKind::Semicolon
                | TokenKind::RightParen
                | TokenKind::RightBracket
                | TokenKind::Eof
        )
    }

    fn expect(&mut self, kind: TokenKind) -> Result<()> {
        if self.current_token.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                format!("expected {}, found {}", kind, self.current_token.kind),
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if self.current_token.kind == TokenKind::Identifier {
            let text = self.current_token.text.clone();
            self.advance();
            Ok(text)
        } else {
            Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                format!("expected identifier, found {}", self.current_token.kind),
            ))
        }
    }

    fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }
}
