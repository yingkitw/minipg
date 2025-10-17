//! Grammar AST node definitions.

use crate::rule::Rule;
use minipg_core::types::GrammarType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root node of a grammar AST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    pub name: String,
    pub grammar_type: GrammarType,
    pub options: HashMap<String, String>,
    pub rules: Vec<Rule>,
    pub imports: Vec<String>,
}

impl Grammar {
    pub fn new(name: String, grammar_type: GrammarType) -> Self {
        Self {
            name,
            grammar_type,
            options: HashMap::new(),
            rules: Vec::new(),
            imports: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_option(&mut self, key: String, value: String) {
        self.options.insert(key, value);
    }

    pub fn add_import(&mut self, import: String) {
        self.imports.push(import);
    }

    pub fn get_rule(&self, name: &str) -> Option<&Rule> {
        self.rules.iter().find(|r| r.name == name)
    }

    pub fn lexer_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter().filter(|r| r.is_lexer_rule())
    }

    pub fn parser_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter().filter(|r| r.is_parser_rule())
    }
}

/// Generic grammar node for AST traversal.
#[derive(Debug, Clone)]
pub enum GrammarNode {
    Grammar(Grammar),
    Rule(Rule),
}
