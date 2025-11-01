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

        // Parse options, imports, named actions, modes, and rules
        while self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Options {
                self.parse_options(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Import {
                self.parse_import(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::At {
                self.parse_named_action(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "mode" {
                self.parse_mode(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Identifier {
                let rule = self.parse_rule(&mut grammar)?;
                grammar.add_rule(rule);
            } else if self.current_token.kind == TokenKind::Fragment {
                let rule = self.parse_fragment_rule(&mut grammar)?;
                grammar.add_rule(rule);
            } else {
                use super::enhanced_errors::create_enhanced_error;
                let expected = vec![
                    TokenKind::Options,
                    TokenKind::Import,
                    TokenKind::At,
                    TokenKind::Identifier,
                    TokenKind::Fragment,
                ];
                return Err(create_enhanced_error(
                    &self.current_token,
                    expected,
                    Some("grammar"),
                    None,
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

    fn parse_named_action(&mut self, grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::At)?;
        let action_name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        
        // Read the action code until we find the closing brace
        let mut code = String::new();
        let mut brace_count = 1;
        
        while brace_count > 0 && self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::LeftBrace {
                brace_count += 1;
                code.push('{');
            } else if self.current_token.kind == TokenKind::RightBrace {
                brace_count -= 1;
                if brace_count > 0 {
                    code.push('}');
                }
            } else {
                code.push_str(&self.current_token.text);
                code.push(' ');
            }
            self.advance();
        }
        
        if brace_count > 0 {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                format!("Unclosed named action '{}': expected '}}' before end of file", action_name),
            ));
        }
        
        grammar.add_named_action(action_name, code.trim().to_string());
        Ok(())
    }

    fn parse_mode(&mut self, grammar: &mut Grammar) -> Result<()> {
        // Parse: mode NAME;
        self.expect(TokenKind::Identifier)?; // consume "mode"
        let mode_name = self.expect_identifier()?;
        self.expect(TokenKind::Semicolon)?;
        
        // Collect rules in this mode until we hit another mode or EOF
        let mut mode_rules = Vec::new();
        
        while self.current_token.kind != TokenKind::Eof {
            // Check if we've hit another mode declaration
            if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "mode" {
                break;
            }
            
            // Parse rules in this mode
            if self.current_token.kind == TokenKind::Identifier {
                let rule = self.parse_rule(grammar)?;
                mode_rules.push(rule.name.clone());
                grammar.add_rule(rule);
            } else if self.current_token.kind == TokenKind::Fragment {
                let rule = self.parse_fragment_rule(grammar)?;
                mode_rules.push(rule.name.clone());
                grammar.add_rule(rule);
            } else {
                break;
            }
        }
        
        grammar.add_lexer_mode(mode_name, mode_rules);
        Ok(())
    }

    fn parse_rule(&mut self, _grammar: &mut Grammar) -> Result<Rule> {
        let name = self.expect_identifier()?;
        
        // Determine if it's a lexer or parser rule based on first character
        let rule_type = if name.chars().next().unwrap().is_uppercase() {
            crate::ast::RuleType::Lexer
        } else {
            crate::ast::RuleType::Parser
        };

        let mut rule = Rule::new(name, rule_type);
        
        // Parse arguments: rule[int x, String name]
        if self.current_token.kind == TokenKind::LeftBracket {
            self.advance();
            self.parse_rule_arguments(&mut rule)?;
            self.expect(TokenKind::RightBracket)?;
        }
        
        // Parse returns: returns [Type value]
        if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "returns" {
            self.advance();
            self.expect(TokenKind::LeftBracket)?;
            self.parse_rule_returns(&mut rule)?;
            self.expect(TokenKind::RightBracket)?;
        }
        
        // Parse locals: locals [Type var]
        if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "locals" {
            self.advance();
            self.expect(TokenKind::LeftBracket)?;
            self.parse_rule_locals(&mut rule)?;
            self.expect(TokenKind::RightBracket)?;
        }
        
        self.expect(TokenKind::Colon)?;
        
        // Parse alternatives
        let alt = self.parse_alternative()?;
        
        // Check for empty alternative
        if alt.elements.is_empty() && rule.alternatives.is_empty() {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "Empty rule alternative: rule must have at least one element".to_string(),
            ));
        }
        
        rule.add_alternative(alt);
        
        while self.current_token.kind == TokenKind::Pipe {
            self.advance();
            let alt = self.parse_alternative()?;
            
            // Check for empty alternative
            if alt.elements.is_empty() {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    "Empty alternative detected. Did you mean to add content between '|'?".to_string(),
                ));
            }
            
            rule.add_alternative(alt);
        }
        
        self.expect(TokenKind::Semicolon)?;
        Ok(rule)
    }

    fn parse_fragment_rule(&mut self, grammar: &mut Grammar) -> Result<Rule> {
        self.expect(TokenKind::Fragment)?;
        let mut rule = self.parse_rule(grammar)?;
        rule.set_fragment(true);
        Ok(rule)
    }
    
    fn parse_rule_arguments(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        while self.current_token.kind != TokenKind::RightBracket && self.current_token.kind != TokenKind::Eof {
            // Try to parse type (optional)
            let mut arg_type = None;
            let mut arg_name = String::new();
            
            if self.current_token.kind == TokenKind::Identifier {
                let first = self.current_token.text.clone();
                self.advance();
                
                // Check if there's another identifier (name after type)
                if self.current_token.kind == TokenKind::Identifier {
                    arg_type = Some(first);
                    arg_name = self.current_token.text.clone();
                    self.advance();
                } else {
                    // Just a name without type
                    arg_name = first;
                }
            }
            
            rule.add_argument(arg_name, arg_type);
            
            // Check for comma
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind != TokenKind::RightBracket {
                break;
            }
        }
        Ok(())
    }
    
    fn parse_rule_returns(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        while self.current_token.kind != TokenKind::RightBracket && self.current_token.kind != TokenKind::Eof {
            let mut return_type = None;
            let mut return_name = String::new();
            
            if self.current_token.kind == TokenKind::Identifier {
                let first = self.current_token.text.clone();
                self.advance();
                
                if self.current_token.kind == TokenKind::Identifier {
                    return_type = Some(first);
                    return_name = self.current_token.text.clone();
                    self.advance();
                } else {
                    return_name = first;
                }
            }
            
            rule.add_return(return_name, return_type);
            
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind != TokenKind::RightBracket {
                break;
            }
        }
        Ok(())
    }
    
    fn parse_rule_locals(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        while self.current_token.kind != TokenKind::RightBracket && self.current_token.kind != TokenKind::Eof {
            let mut local_type = None;
            let mut local_name = String::new();
            
            if self.current_token.kind == TokenKind::Identifier {
                let first = self.current_token.text.clone();
                self.advance();
                
                if self.current_token.kind == TokenKind::Identifier {
                    local_type = Some(first);
                    local_name = self.current_token.text.clone();
                    self.advance();
                } else {
                    local_name = first;
                }
            }
            
            rule.add_local(local_name, local_type);
            
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind != TokenKind::RightBracket {
                break;
            }
        }
        Ok(())
    }

    fn parse_alternative(&mut self) -> Result<Alternative> {
        let mut alt = Alternative::new();
        
        while !self.is_alternative_end() {
            let element = self.parse_element()?;
            alt.add_element(element);
        }
        
        // Handle lexer commands: -> skip, -> channel(HIDDEN), etc.
        if self.current_token.kind == TokenKind::Arrow {
            self.advance();
            if self.current_token.kind == TokenKind::Identifier {
                let command_name = self.current_token.text.clone();
                self.advance();
                
                let command = match command_name.as_str() {
                    "skip" => crate::ast::LexerCommand::Skip,
                    "more" => crate::ast::LexerCommand::More,
                    "popMode" => crate::ast::LexerCommand::PopMode,
                    "channel" | "mode" | "type" | "pushMode" => {
                        // These commands require a parameter in parentheses
                        if self.current_token.kind == TokenKind::LeftParen {
                            self.advance();
                            let param = if self.current_token.kind == TokenKind::Identifier {
                                let p = self.current_token.text.clone();
                                self.advance();
                                p
                            } else {
                                String::new()
                            };
                            if self.current_token.kind == TokenKind::RightParen {
                                self.advance();
                            }
                            match command_name.as_str() {
                                "channel" => crate::ast::LexerCommand::Channel(param),
                                "mode" => crate::ast::LexerCommand::Mode(param),
                                "type" => crate::ast::LexerCommand::Type(param),
                                "pushMode" => crate::ast::LexerCommand::PushMode(param),
                                _ => crate::ast::LexerCommand::Skip, // fallback
                            }
                        } else {
                            crate::ast::LexerCommand::Skip // fallback if no params
                        }
                    }
                    _ => crate::ast::LexerCommand::Skip, // unknown commands default to skip
                };
                
                alt.set_lexer_command(command);
            }
        }
        
        Ok(alt)
    }

    fn parse_element(&mut self) -> Result<Element> {
        // Check for label (id=element or ids+=element)
        let (label, is_list) = if self.current_token.kind == TokenKind::Identifier {
            // Look ahead to see if next token is = or +=
            let next_kind = self.peek_token.kind;
            if next_kind == TokenKind::Equals || next_kind == TokenKind::PlusEquals {
                // This is a label
                let label_name = self.expect_identifier()?;
                let is_list = self.current_token.kind == TokenKind::PlusEquals;
                self.advance(); // consume = or +=
                (Some(label_name), is_list)
            } else {
                (None, false)
            }
        } else {
            (None, false)
        };
        
        let element = match self.current_token.kind {
            TokenKind::Identifier => {
                let name = self.expect_identifier()?;
                let mut elem = Element::rule_ref(name);
                if let Some(lbl) = label {
                    elem = if is_list {
                        elem.with_list_label(lbl)
                    } else {
                        elem.with_label(lbl)
                    };
                }
                elem
            }
            TokenKind::StringLiteral => {
                let value = self.current_token.text.clone();
                self.advance();
                let mut elem = Element::string_literal(value);
                if let Some(lbl) = label {
                    elem = if is_list {
                        elem.with_list_label(lbl)
                    } else {
                        elem.with_label(lbl)
                    };
                }
                elem
            }
            TokenKind::LeftParen => {
                self.advance();
                let mut alternatives = vec![self.parse_alternative()?];
                
                while self.current_token.kind == TokenKind::Pipe {
                    self.advance();
                    let alt = self.parse_alternative()?;
                    
                    // Check for empty alternative in group
                    if alt.elements.is_empty() {
                        return Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "Empty alternative in group. Did you mean to add content between '|'?".to_string(),
                        ));
                    }
                    
                    alternatives.push(alt);
                }
                
                if self.current_token.kind == TokenKind::Eof {
                    return Err(Error::parse(
                        format!("{}:{}", self.current_token.line, self.current_token.column),
                        "Unclosed group: expected ')' before end of file".to_string(),
                    ));
                }
                
                self.expect(TokenKind::RightParen)?;
                Element::Group { alternatives }
            }
            TokenKind::LeftBracket => {
                self.advance();
                // Parse character class
                let element = self.parse_char_class()?;
                
                if self.current_token.kind == TokenKind::Eof {
                    return Err(Error::parse(
                        format!("{}:{}", self.current_token.line, self.current_token.column),
                        "Unclosed character class: expected ']' before end of file".to_string(),
                    ));
                }
                
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
                use super::enhanced_errors::create_enhanced_error;
                let expected = vec![
                    TokenKind::Identifier,
                    TokenKind::StringLiteral,
                    TokenKind::CharLiteral,
                    TokenKind::LeftParen,
                    TokenKind::LeftBracket,
                    TokenKind::Dot,
                    TokenKind::Not,
                ];
                return Err(create_enhanced_error(
                    &self.current_token,
                    expected,
                    None,
                    None,
                ));
            }
        };

        // Handle suffixes (?, *, +) with optional non-greedy modifier (??, *?, +?)
        let element = match self.current_token.kind {
            TokenKind::Question => {
                self.advance();
                // Check for non-greedy modifier ??
                if self.current_token.kind == TokenKind::Question {
                    self.advance();
                    Element::optional_non_greedy(element)
                } else {
                    Element::optional(element)
                }
            }
            TokenKind::Star => {
                self.advance();
                // Check for non-greedy modifier *?
                if self.current_token.kind == TokenKind::Question {
                    self.advance();
                    Element::zero_or_more_non_greedy(element)
                } else {
                    Element::zero_or_more(element)
                }
            }
            TokenKind::Plus => {
                self.advance();
                // Check for non-greedy modifier +?
                if self.current_token.kind == TokenKind::Question {
                    self.advance();
                    Element::one_or_more_non_greedy(element)
                } else {
                    Element::one_or_more(element)
                }
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
                || self.current_token.kind == TokenKind::CharLiteral
                || self.current_token.kind == TokenKind::Identifier {
                let start_char = self.parse_char_literal()?;
                
                // Check for range (either .. or -)
                if self.current_token.kind == TokenKind::Range || self.current_token.kind == TokenKind::Minus {
                    self.advance();
                    if self.current_token.kind == TokenKind::StringLiteral 
                        || self.current_token.kind == TokenKind::CharLiteral
                        || self.current_token.kind == TokenKind::Identifier {
                        let end_char = self.parse_char_literal()?;
                        
                        // Validate character range
                        use super::enhanced_errors::validate_char_class_range;
                        validate_char_class_range(start_char, end_char)?;
                        
                        ranges.push((start_char, end_char));
                    } else {
                        use super::enhanced_errors::create_enhanced_error;
                        return Err(create_enhanced_error(
                            &self.current_token,
                            vec![TokenKind::StringLiteral, TokenKind::CharLiteral, TokenKind::Identifier],
                            Some("character class range"),
                            None,
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
        use super::enhanced_errors::parse_unicode_escape;
        
        let ch = if text.starts_with("\\u{") || text.starts_with("\\u") {
            // Unicode escape: \uXXXX or \u{XXXXXX}
            parse_unicode_escape(text).map_err(|e| {
                // Preserve line/column information
                Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    format!("{}", e),
                )
            })?
        } else if text.starts_with('\\') && text.len() >= 2 {
            // Simple escape sequences
            match text.chars().nth(1).unwrap() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                '0' => '\0',
                'x' => {
                    // Hex escape: \xXX
                    if text.len() >= 4 {
                        let hex = &text[2..4];
                        u8::from_str_radix(hex, 16)
                            .ok()
                            .and_then(|b| char::from_u32(b as u32))
                            .unwrap_or('\0')
                    } else {
                        return Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "incomplete hex escape sequence".to_string(),
                        ));
                    }
                }
                c => c,
            }
        } else {
            let chars: Vec<char> = text.chars().collect();
            if chars.is_empty() {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    "empty character literal".to_string(),
                ));
            }
            chars[0]
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
                | TokenKind::Arrow
                | TokenKind::Eof
        )
    }

    fn expect(&mut self, kind: TokenKind) -> Result<()> {
        if self.current_token.kind == kind {
            self.advance();
            Ok(())
        } else {
            use super::enhanced_errors::create_enhanced_error;
            Err(create_enhanced_error(
                &self.current_token,
                vec![kind],
                None,
                None,
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if self.current_token.kind == TokenKind::Identifier {
            let text = self.current_token.text.clone();
            self.advance();
            Ok(text)
        } else {
            use super::enhanced_errors::create_enhanced_error;
            Err(create_enhanced_error(
                &self.current_token,
                vec![TokenKind::Identifier],
                None,
                None,
            ))
        }
    }

    fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }
}
