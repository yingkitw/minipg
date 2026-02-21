//! Common types used across minipg.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A point in a text document (line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Point {
    /// Zero-indexed line number
    pub row: usize,
    /// Zero-indexed column number (UTF-8 byte offset within line)
    pub column: usize,
}

impl Point {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn zero() -> Self {
        Self { row: 0, column: 0 }
    }
}

/// A position in a text document with both byte offset and point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Byte offset from start of document
    pub byte: usize,
    /// Line and column position
    pub point: Point,
}

impl Position {
    pub fn new(byte: usize, point: Point) -> Self {
        Self { byte, point }
    }

    pub fn zero() -> Self {
        Self {
            byte: 0,
            point: Point::zero(),
        }
    }
}

/// A range in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Check if this range contains a byte offset.
    pub fn contains_byte(&self, byte: usize) -> bool {
        self.start.byte <= byte && byte < self.end.byte
    }

    /// Check if this range overlaps with another range.
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start.byte < other.end.byte && other.start.byte < self.end.byte
    }

    /// Get the byte length of this range.
    pub fn byte_len(&self) -> usize {
        self.end.byte.saturating_sub(self.start.byte)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_type_variants() {
        assert_eq!(GrammarType::Lexer, GrammarType::Lexer);
        assert_eq!(GrammarType::Parser, GrammarType::Parser);
        assert_eq!(GrammarType::Combined, GrammarType::Combined);
        assert_ne!(GrammarType::Lexer, GrammarType::Parser);
    }

    #[test]
    fn test_symbol_table_new() {
        let table = SymbolTable::new();
        assert_eq!(table.tokens().len(), 0);
        assert_eq!(table.rules().len(), 0);
    }

    #[test]
    fn test_symbol_table_add_token() {
        let mut table = SymbolTable::new();
        table.add_token("ID".to_string(), 1);
        assert_eq!(table.get_token("ID"), Some(1));
        assert_eq!(table.get_token("UNKNOWN"), None);
    }

    #[test]
    fn test_symbol_table_add_rule() {
        let mut table = SymbolTable::new();
        table.add_rule("expr".to_string(), 0);
        assert_eq!(table.get_rule("expr"), Some(0));
        assert_eq!(table.get_rule("unknown"), None);
    }

    #[test]
    fn test_symbol_table_multiple_entries() {
        let mut table = SymbolTable::new();
        table.add_token("ID".to_string(), 1);
        table.add_token("NUMBER".to_string(), 2);
        table.add_rule("expr".to_string(), 0);
        table.add_rule("term".to_string(), 1);

        assert_eq!(table.tokens().len(), 2);
        assert_eq!(table.rules().len(), 2);
    }

    #[test]
    fn test_codegen_config_default() {
        let config = CodeGenConfig::default();
        assert_eq!(config.target_language, "rust");
        assert_eq!(config.output_directory, ".");
        assert_eq!(config.package_name, None);
        assert!(config.generate_listener);
        assert!(!config.generate_visitor);
    }

    #[test]
    fn test_codegen_config_custom() {
        let config = CodeGenConfig {
            target_language: "python".to_string(),
            output_directory: "output".to_string(),
            package_name: Some("my_parser".to_string()),
            generate_listener: false,
            generate_visitor: true,
        };

        assert_eq!(config.target_language, "python");
        assert_eq!(config.output_directory, "output");
        assert_eq!(config.package_name, Some("my_parser".to_string()));
        assert!(!config.generate_listener);
        assert!(config.generate_visitor);
    }
}
