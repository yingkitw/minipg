//! Common types used across minipg.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Grammar type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrammarType {
    Lexer,
    Parser,
    Combined,
}

/// Token type identifier.
pub type TokenType = usize;

/// Rule identifier.
pub type RuleId = usize;

/// Symbol table for tracking names and their types.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    tokens: HashMap<String, TokenType>,
    rules: HashMap<String, RuleId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_token(&mut self, name: String, token_type: TokenType) {
        self.tokens.insert(name, token_type);
    }

    pub fn add_rule(&mut self, name: String, rule_id: RuleId) {
        self.rules.insert(name, rule_id);
    }

    pub fn get_token(&self, name: &str) -> Option<TokenType> {
        self.tokens.get(name).copied()
    }

    pub fn get_rule(&self, name: &str) -> Option<RuleId> {
        self.rules.get(name).copied()
    }

    pub fn tokens(&self) -> &HashMap<String, TokenType> {
        &self.tokens
    }

    pub fn rules(&self) -> &HashMap<String, RuleId> {
        &self.rules
    }
}

/// Configuration options for code generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenConfig {
    pub target_language: String,
    pub output_directory: String,
    pub package_name: Option<String>,
    pub generate_listener: bool,
    pub generate_visitor: bool,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            target_language: "rust".to_string(),
            output_directory: ".".to_string(),
            package_name: None,
            generate_listener: true,
            generate_visitor: false,
        }
    }
}
