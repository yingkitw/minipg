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
        // Support both patterns:
        // 1. grammar [lexer|parser] <name> ;
        // 2. parser grammar <name> ;
        // 3. lexer grammar <name> ;
        
        let grammar_type = if self.current_token.kind == TokenKind::Parser {
            // Pattern: parser grammar Name;
            self.advance();
            self.expect(TokenKind::Grammar)?;
            GrammarType::Parser
        } else if self.current_token.kind == TokenKind::Lexer {
            // Pattern: lexer grammar Name;
            self.advance();
            self.expect(TokenKind::Grammar)?;
            GrammarType::Lexer
        } else {
            // Pattern: grammar [lexer|parser] Name;
        self.expect(TokenKind::Grammar)?;
        
            // Check for optional type modifier after grammar keyword
            if self.current_token.kind == TokenKind::Lexer {
            self.advance();
            GrammarType::Lexer
        } else if self.current_token.kind == TokenKind::Parser {
            self.advance();
            GrammarType::Parser
        } else {
            GrammarType::Combined
            }
        };

        let name = self.expect_identifier()?;
        self.expect(TokenKind::Semicolon)?;

        let mut grammar = Grammar::new(name, grammar_type);

        // Parse options, imports, channels, named actions, modes, and rules
        while self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Options {
                // Check if this is "options" as a rule name or as a declaration
                // If followed by {, it's a declaration; if followed by :, it's a rule name
                if self.peek_token.kind == TokenKind::LeftBrace {
                self.parse_options(&mut grammar)?;
                } else {
                    let rule = self.parse_rule(&mut grammar)?;
                    grammar.add_rule(rule);
                }
            } else if self.current_token.kind == TokenKind::Import {
                self.parse_import(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "channels" {
                self.parse_channels(&mut grammar)?;
            } else if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "tokens" {
                self.parse_tokens(&mut grammar)?;
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
            } else if self.current_token.kind == TokenKind::Parser || self.current_token.kind == TokenKind::Lexer {
                // Keywords can be rule names
                let rule = self.parse_rule(&mut grammar)?;
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
        
        // Parse first import name
        let import_name = self.expect_identifier()?;
        grammar.add_import(import_name);
        
        // Parse additional comma-separated imports
        while self.current_token.kind == TokenKind::Comma {
            self.advance(); // consume comma
            let import_name = self.expect_identifier()?;
            grammar.add_import(import_name);
        }
        
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    fn parse_channels(&mut self, grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::Identifier)?; // consume "channels"
        self.expect(TokenKind::LeftBrace)?;
        
        while self.current_token.kind != TokenKind::RightBrace && self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Identifier {
                let channel_name = self.expect_identifier()?;
                grammar.add_channel(channel_name);
            }
            
            // Skip comma or semicolon
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind == TokenKind::Semicolon {
                self.advance();
            } else if self.current_token.kind != TokenKind::RightBrace {
                // Skip whitespace/newlines
                break;
            }
        }
        
        self.expect(TokenKind::RightBrace)?;
        Ok(())
    }

    fn parse_tokens(&mut self, _grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::Identifier)?; // consume "tokens"
        self.expect(TokenKind::LeftBrace)?;
        
        while self.current_token.kind != TokenKind::RightBrace && self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Identifier {
                let token_name = self.expect_identifier()?;
                // Tokens are just identifiers, we can store them if needed
                // For now, just skip them
            }
            
            // Skip comma or semicolon
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind == TokenKind::Semicolon {
                self.advance();
            } else if self.current_token.kind != TokenKind::RightBrace {
                break;
            }
        }
        
        self.expect(TokenKind::RightBrace)?;
        Ok(())
    }

    fn parse_named_action(&mut self, grammar: &mut Grammar) -> Result<()> {
        self.expect(TokenKind::At)?;
        
        // Parse action name, which may include namespace prefix (e.g., parser::members, lexer::members)
        let mut action_name = String::new();
        
        // Handle namespace prefixes: parser::, lexer::, etc.
        // Note: "parser" and "lexer" might be tokenized as keywords, so check for both Identifier and keyword tokens
        if self.current_token.kind == TokenKind::Identifier || 
           self.current_token.kind == TokenKind::Parser || 
           self.current_token.kind == TokenKind::Lexer {
            action_name.push_str(&self.current_token.text);
            self.advance();
            
            // Check for namespace separator ::
            if self.current_token.kind == TokenKind::Colon && self.peek_token.kind == TokenKind::Colon {
                self.advance(); // consume first :
                self.advance(); // consume second :
                action_name.push_str("::");
                // After ::, there should be an identifier (like "members")
                // But if there's a { immediately, that's also valid (e.g., @parser:: {)
                if self.current_token.kind == TokenKind::Identifier {
                    action_name.push_str(&self.current_token.text);
                    self.advance();
                }
                // If no identifier after ::, that's okay - use what we have (e.g., @parser::members)
            }
        } else if self.current_token.kind == TokenKind::Colon && self.peek_token.kind == TokenKind::Colon {
            // Handle case like @::members (unlikely but possible)
            self.advance(); // consume first :
            self.advance(); // consume second :
            action_name.push_str("::");
            if self.current_token.kind == TokenKind::Identifier {
                action_name.push_str(&self.current_token.text);
                self.advance();
            }
        } else {
            // If no identifier, try to skip to brace (might be malformed but try to recover)
            if self.current_token.kind == TokenKind::LeftBrace {
                action_name = "unknown".to_string();
            } else {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    "Expected identifier after @ in named action".to_string(),
                ));
            }
        }
        
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

    fn parse_rule(&mut self, grammar: &mut Grammar) -> Result<Rule> {
        // Rule names can be identifiers OR keywords (like "options", "parser", "lexer")
        let name = if self.current_token.kind == TokenKind::Identifier {
            let name = self.current_token.text.clone();
            self.advance();
            name
        } else if self.current_token.kind == TokenKind::Options 
            || self.current_token.kind == TokenKind::Parser 
            || self.current_token.kind == TokenKind::Lexer {
            let name = self.current_token.text.clone();
            self.advance();
            name
        } else {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "Expected rule name (identifier or keyword)".to_string(),
            ));
        };
        
        // Determine if it's a lexer or parser rule based on first character
        let rule_type = if name.chars().next().unwrap().is_uppercase() {
            crate::ast::RuleType::Lexer
        } else {
            crate::ast::RuleType::Parser
        };

        let mut rule = Rule::new(name, rule_type);

        // Parse arguments: rule[int x, String name]
        // Disable CharClass mode for rule arguments
        let old_disable = self.lexer.disable_char_class_mode;
        self.lexer.disable_char_class_mode = true;
        if self.current_token.kind == TokenKind::LeftBracket {
            self.advance();
            self.parse_rule_arguments(&mut rule)?;
            self.expect(TokenKind::RightBracket)?;
        }
        self.lexer.disable_char_class_mode = old_disable;
        
        // Parse returns: returns [Type value] or returns[Type value] (space optional)
        // Only parse as returns clause if followed by [
        if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "returns" {
            // Peek ahead to see if next token is [
            if self.peek_token.kind == TokenKind::LeftBracket {
                self.advance(); // consume "returns"
                // Disable CharClass mode for returns clause
                self.lexer.disable_char_class_mode = true;
                self.advance(); // consume [
            self.parse_rule_returns(&mut rule)?;
                // parse_rule_returns should leave us at ], so consume it
            self.expect(TokenKind::RightBracket)?;
                self.lexer.disable_char_class_mode = old_disable;
            }
            // If not followed by [, it's a rule name, so continue parsing as rule
        }

        // Parse locals: locals [Type var] or locals[Type var] (space optional)
        // Only parse as locals clause if followed by [
        if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "locals" {
            // Peek ahead to see if next token is [
            if self.peek_token.kind == TokenKind::LeftBracket {
                self.advance(); // consume "locals"
                // Disable CharClass mode for locals clause
                self.lexer.disable_char_class_mode = true;
                self.advance(); // consume [
            self.parse_rule_locals(&mut rule)?;
            self.expect(TokenKind::RightBracket)?;
                self.lexer.disable_char_class_mode = old_disable;
            }
            // If not followed by [, it's a rule name, so continue parsing as rule
        }
        
        // Check for options after rule name (before colon) - like "CURRENT options { ... }:"
        if self.current_token.kind == TokenKind::Options {
            self.parse_options(grammar)?;
        }

        self.expect(TokenKind::Colon)?;
        
        // Parse alternatives
        // Empty alternatives are not allowed in these test cases
        let alt = self.parse_alternative()?;
        if alt.elements.is_empty() {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "Empty alternative not allowed".to_string(),
            ));
        }
        rule.add_alternative(alt);

        while self.current_token.kind == TokenKind::Pipe {
            self.advance();
            let alt = self.parse_alternative()?;
            if alt.elements.is_empty() {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    "Empty alternative not allowed".to_string(),
                ));
            }
            rule.add_alternative(alt);
        }
        
        self.expect(TokenKind::Semicolon)?;
        Ok(rule)
    }

    fn parse_fragment_rule(&mut self, grammar: &mut Grammar) -> Result<Rule> {
        self.expect(TokenKind::Fragment)?;
        
        // Parse rule name
        let name = if self.current_token.kind == TokenKind::Identifier {
            let name = self.current_token.text.clone();
            self.advance();
            name
        } else if self.current_token.kind == TokenKind::Options 
            || self.current_token.kind == TokenKind::Parser 
            || self.current_token.kind == TokenKind::Lexer {
            let name = self.current_token.text.clone();
            self.advance();
            name
        } else {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "Expected rule name after fragment".to_string(),
            ));
        };
        
        // Determine rule type
        let rule_type = if name.chars().next().unwrap().is_uppercase() {
            crate::ast::RuleType::Lexer
        } else {
            crate::ast::RuleType::Parser
        };
        
        let mut rule = Rule::new(name, rule_type);
        rule.set_fragment(true);
        
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
        
        // Check for options after fragment rule name (before colon)
        if self.current_token.kind == TokenKind::Options {
            self.parse_options(grammar)?;
        }
        
        self.expect(TokenKind::Colon)?;
        
        // Parse alternatives
        let alt = self.parse_alternative()?;
        rule.add_alternative(alt);
        
        while self.current_token.kind == TokenKind::Pipe {
            self.advance();
            let alt = self.parse_alternative()?;
            rule.add_alternative(alt);
        }
            
        if rule.alternatives.is_empty() {
                return Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                "Rule must have at least one alternative".to_string(),
                ));
        }
        
        self.expect(TokenKind::Semicolon)?;
        Ok(rule)
    }
    
    fn parse_rule_arguments(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        loop {
            // Check if we're at the closing bracket before entering the loop body
            if self.current_token.kind == TokenKind::RightBracket || self.current_token.kind == TokenKind::Eof {
                break;
            }
            
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
            } else {
                // Not an identifier - might be end bracket
                break;
            }
            
            rule.add_argument(arg_name, arg_type);
            
            // Check for comma or closing bracket
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
                // Continue loop to parse next argument
            } else if self.current_token.kind == TokenKind::RightBracket {
                // Found closing bracket - break and let caller consume it
                break;
            } else {
                // Unexpected token - break
                break;
            }
        }
        Ok(())
    }
    
    fn parse_rule_returns(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        // Examples: "bool value", "Type1 name1, Type2 name2", "bool"
        // We're already inside the brackets, so parse until we find ]
        loop {
            // Check if we're at the closing bracket before entering the loop body
            if self.current_token.kind == TokenKind::RightBracket || self.current_token.kind == TokenKind::Eof {
                break;
            }
            
            let mut return_type = None;
            let mut return_name = String::new();
            
            if self.current_token.kind == TokenKind::Identifier {
                let first = self.current_token.text.clone();
                self.advance();
                
                // Check if next token is also an identifier (Type name pattern)
                // Make sure we're not at the end bracket
                if self.current_token.kind == TokenKind::Identifier {
                    return_type = Some(first);
                    return_name = self.current_token.text.clone();
                    self.advance();
                } else {
                    // Single identifier - treat as name only
                    return_name = first;
                }
            } else {
                // Not an identifier - might be end bracket or malformed
                // Break and let caller handle
                break;
            }
            
            rule.add_return(return_name, return_type);
            
            // Check for comma (multiple returns) or closing bracket
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind == TokenKind::RightBracket {
                // Found closing bracket - break and let caller consume it
                break;
            } else {
                // If we're not at comma or bracket, we should break
                // This handles cases where there's no comma between returns
                break;
            }
        }
        Ok(())
    }
    
    fn parse_rule_locals(&mut self, rule: &mut Rule) -> Result<()> {
        // Parse: Type name, Type name, ...
        loop {
            // Check if we're at the closing bracket before entering the loop body
            if self.current_token.kind == TokenKind::RightBracket || self.current_token.kind == TokenKind::Eof {
                break;
            }
            
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
            } else {
                // Not an identifier - might be end bracket
                break;
            }
            
            rule.add_local(local_name, local_type);
            
            // Check for comma or closing bracket
            if self.current_token.kind == TokenKind::Comma {
                self.advance();
            } else if self.current_token.kind == TokenKind::RightBracket {
                // Found closing bracket - break and let caller consume it
                break;
            } else {
                // Unexpected token - break
                break;
            }
        }
        Ok(())
    }

    fn parse_alternative(&mut self) -> Result<Alternative> {
        let mut alt = Alternative::new();
        
        while !self.is_alternative_end() {
            // Skip action blocks { ... } or < ... >
            if self.current_token.kind == TokenKind::LeftBrace {
                self.skip_action_block()?;
                // After an action block, there might be a quantifier (? * +)
                // Skip it for now as it applies to the action, not an element
                if matches!(self.current_token.kind, TokenKind::Question | TokenKind::Star | TokenKind::Plus) {
                    self.advance();
                    // Check for non-greedy modifier
                    if self.current_token.kind == TokenKind::Question {
                        self.advance();
                    }
                }
                continue;
            }
            
            // Skip angle bracket actions: <assoc = right>
            if self.current_token.kind == TokenKind::Identifier && self.current_token.text == "<" {
                self.skip_angle_bracket_action()?;
                continue;
            }
            
            let element = self.parse_element()?;
            alt.add_element(element);
        }
        
        // Handle lexer commands: -> skip, -> channel(HIDDEN), etc.
        // Support multiple comma-separated commands: -> skip, pushMode(StringMode)
        if self.current_token.kind == TokenKind::Arrow {
            self.advance();
            
            // Parse first command
            let mut first_command = None;
            
            while self.current_token.kind == TokenKind::Identifier || self.current_token.kind == TokenKind::Comma {
                if self.current_token.kind == TokenKind::Comma {
                    self.advance();
                    continue;
                }
                
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
                
                // Store the first command (AST only supports one for now)
                if first_command.is_none() {
                    first_command = Some(command);
                }
                
                // Stop if no comma follows
                if self.current_token.kind != TokenKind::Comma {
                    break;
                }
            }
            
            if let Some(cmd) = first_command {
                alt.set_lexer_command(cmd);
            }
        }
        
        Ok(alt)
    }

    fn skip_angle_bracket_action(&mut self) -> Result<()> {
        // Skip angle bracket actions: < ... >
        // We've already consumed the < token
        let mut angle_count = 1;
        
        while angle_count > 0 && self.current_token.kind != TokenKind::Eof {
            // Check for angle brackets - they're tokenized as Identifier tokens with text "<" or ">"
            if self.current_token.kind == TokenKind::Identifier {
                if self.current_token.text == "<" {
                    angle_count += 1;
                } else if self.current_token.text == ">" {
                    angle_count -= 1;
                    if angle_count == 0 {
                        self.advance();
                        break;
                    }
                }
            }
            
            // If we hit a semicolon or other statement-ending token, the angle bracket action might be malformed
            // but we should still try to recover
            if self.current_token.kind == TokenKind::Semicolon && angle_count > 0 {
                // Malformed angle bracket action - try to recover by breaking
                break;
            }
            
            self.advance();
        }
        
        // Don't error if angle_count > 0 - might be malformed but we tried to recover
        Ok(())
    }

    fn skip_action_block(&mut self) -> Result<()> {
        // Skip action blocks: { ... }
        self.expect(TokenKind::LeftBrace)?;
        let mut brace_count = 1;
        
        while brace_count > 0 && self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::LeftBrace {
                brace_count += 1;
            } else if self.current_token.kind == TokenKind::RightBrace {
                brace_count -= 1;
            }
            self.advance();
        }
        
        if brace_count > 0 {
            return Err(Error::parse(
                format!("{}:{}", self.current_token.line, self.current_token.column),
                "Unclosed action block: expected '}' before end of file".to_string(),
            ));
        }
        
        Ok(())
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
                let start_value = self.current_token.text.clone();
                self.advance();
                
                // Check for character range shorthand: 'a'..'z' (equivalent to ['a'..'z'])
                if self.current_token.kind == TokenKind::Range {
                    self.advance(); // consume ..
                    
                    if self.current_token.kind == TokenKind::StringLiteral {
                        let end_value = self.current_token.text.clone();
                        self.advance();
                        
                        // Parse the character literals
                        let start_char = self.parse_char_from_literal(&start_value)?;
                        let end_char = self.parse_char_from_literal(&end_value)?;
                        
                        // Validate range
                        use super::enhanced_errors::validate_char_class_range;
                        validate_char_class_range(start_char, end_char)?;
                        
                        // Create character class element
                        let mut elem = Element::CharClass {
                            negated: false,
                            ranges: vec![(start_char, end_char)],
                        };
                        if let Some(lbl) = label {
                            elem = if is_list {
                                elem.with_list_label(lbl)
                            } else {
                                elem.with_label(lbl)
                            };
                        }
                        return Ok(elem);
                    } else {
                        return Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "Expected character literal after '..' in range".to_string(),
                        ));
                    }
                }
                
                // Regular string literal
                let mut elem = Element::string_literal(start_value);
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
                let first_alt = self.parse_alternative()?;
                if first_alt.elements.is_empty() {
                    return Err(Error::parse(
                        format!("{}:{}", self.current_token.line, self.current_token.column),
                        "Empty alternative not allowed in group".to_string(),
                    ));
                }
                let mut alternatives = vec![first_alt];

                while self.current_token.kind == TokenKind::Pipe {
                    self.advance();
                    let alt = self.parse_alternative()?;
                    if alt.elements.is_empty() {
                        return Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "Empty alternative not allowed in group".to_string(),
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
                // Lexer auto-enters CharClass mode on [
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
                // Lexer auto-exits CharClass mode on ]
                element
            }
            TokenKind::Dot => {
                self.advance();
                Element::Wildcard
            }
            TokenKind::Colon => {
                // Colon can appear as a literal character in patterns like COLON (: .)*?
                // Treat it as a character literal ':'
                self.advance();
                Element::CharClass {
                    negated: false,
                    ranges: vec![(':', ':')],
                }
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
        let mut last_char: Option<char> = None;

        // Parse character class contents
        while self.current_token.kind != TokenKind::RightBracket
            && self.current_token.kind != TokenKind::Eof {

            if self.current_token.kind == TokenKind::StringLiteral
                || self.current_token.kind == TokenKind::CharLiteral
                || self.current_token.kind == TokenKind::Identifier {
                let start_char = self.parse_char_literal()?;
                
                // Check for range (either .. or -)
                // Only treat as range if followed by another character literal
                if (self.current_token.kind == TokenKind::Range || self.current_token.kind == TokenKind::Minus)
                    && (self.peek_token.kind == TokenKind::StringLiteral 
                        || self.peek_token.kind == TokenKind::CharLiteral
                        || self.peek_token.kind == TokenKind::Identifier) {
                    self.advance(); // consume .. or -
                    if self.current_token.kind == TokenKind::StringLiteral 
                        || self.current_token.kind == TokenKind::CharLiteral
                        || self.current_token.kind == TokenKind::Identifier {
                        let end_char = self.parse_char_literal()?;
                        
                        // Validate character range
                        use super::enhanced_errors::validate_char_class_range;
                        validate_char_class_range(start_char, end_char)?;
                        
                        ranges.push((start_char, end_char));
                        last_char = None; // Range consumed both chars
                    } else {
                        // Treat as literal character
                        ranges.push((start_char, start_char));
                        last_char = Some(start_char);
                    }
                } else {
                    // Single character
                    ranges.push((start_char, start_char));
                    last_char = Some(start_char);
                }
            } else if self.current_token.kind == TokenKind::Minus {
                // Minus at beginning/end or after another char is literal
                // If it's followed by a char, it might be a range (handled above)
                // Otherwise, treat as literal
                if last_char.is_some() || ranges.is_empty() {
                    // Treat as literal minus character
                    self.advance();
                    ranges.push(('-', '-'));
                    last_char = Some('-');
                } else {
                    break; // End of character class
                }
            } else {
                // Handle other characters that might appear in character classes
                // In CharClass mode, many characters are treated as identifiers
                // Also handle special tokens that can appear as literals in character classes
                let ch = match self.current_token.kind {
                    TokenKind::Identifier => {
                        let ch_str = &self.current_token.text;
                        if ch_str.len() == 1 {
                            Some(ch_str.chars().next().unwrap())
                        } else {
                            None
                        }
                    }
                    TokenKind::Plus => Some('+'),
                    TokenKind::Star => Some('*'),
                    TokenKind::Question => Some('?'),
                    TokenKind::Not => Some('~'),
                    TokenKind::Semicolon => Some(';'),
                    TokenKind::Colon => Some(':'),
                    TokenKind::Comma => Some(','),
                    TokenKind::Dot => Some('.'),
                    TokenKind::Equals => Some('='),
                    TokenKind::LeftParen => Some('('),
                    TokenKind::RightParen => Some(')'),
                    TokenKind::LeftBrace => Some('{'),
                    TokenKind::RightBrace => Some('}'),
                    TokenKind::Pipe => Some('|'),
                    TokenKind::At => Some('@'),
                    TokenKind::LeftBracket => Some('['), // Nested character classes: [\[
                    TokenKind::RightBracket => Some(']'), // Escaped ]: [\]]
                    _ => None,
                };
                
                if let Some(ch) = ch {
                    ranges.push((ch, ch));
                    last_char = Some(ch);
                    self.advance();
            } else {
                break;
                }
            }
        }
        
        // Empty character classes are valid in ANTLR4 (matches nothing)
        if ranges.is_empty() {
            return Ok(Element::CharClass { negated, ranges: Vec::new() });
        }
        
        Ok(Element::CharClass { negated, ranges })
    }
    
    fn parse_char_from_literal(&self, literal: &str) -> Result<char> {
        // Parse a character from a string literal (which might be a character literal like '0' or 'a')
        // This handles escape sequences and unicode escapes
        use super::enhanced_errors::parse_unicode_escape;
        
        if literal.starts_with("\\u{") || literal.starts_with("\\u") {
            // Unicode escape: \uXXXX or \u{XXXXXX}
            parse_unicode_escape(literal).map_err(|e| {
                Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    format!("{}", e),
                )
            })
        } else if literal.starts_with('\\') && literal.len() >= 2 {
            // Simple escape sequences
            match literal.chars().nth(1).unwrap() {
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                '\\' => Ok('\\'),
                '\'' => Ok('\''),
                '"' => Ok('"'),
                '0' => Ok('\0'),
                'x' => {
                    // Hex escape: \xXX
                    if literal.len() >= 4 {
                        let hex = &literal[2..4];
                        u8::from_str_radix(hex, 16)
                            .ok()
                            .and_then(|b| char::from_u32(b as u32))
                            .ok_or_else(|| Error::parse(
                                format!("{}:{}", self.current_token.line, self.current_token.column),
                                "incomplete hex escape sequence".to_string(),
                            ))
                    } else {
                        Err(Error::parse(
                            format!("{}:{}", self.current_token.line, self.current_token.column),
                            "incomplete hex escape sequence".to_string(),
                        ))
                    }
                }
                c => Ok(c),
            }
        } else {
            // Regular character
            let chars: Vec<char> = literal.chars().collect();
            if chars.is_empty() {
                Err(Error::parse(
                    format!("{}:{}", self.current_token.line, self.current_token.column),
                    "empty character literal".to_string(),
                ))
            } else {
                Ok(chars[0])
            }
        }
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
