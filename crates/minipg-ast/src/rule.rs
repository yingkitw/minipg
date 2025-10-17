//! Rule AST node definitions.

use crate::element::Alternative;
use serde::{Deserialize, Serialize};

/// Type of rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    Parser,
    Lexer,
}

/// A grammar rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub rule_type: RuleType,
    pub alternatives: Vec<Alternative>,
    pub is_fragment: bool,
}

impl Rule {
    pub fn new(name: String, rule_type: RuleType) -> Self {
        Self {
            name,
            rule_type,
            alternatives: Vec::new(),
            is_fragment: false,
        }
    }

    pub fn parser_rule(name: String) -> Self {
        Self::new(name, RuleType::Parser)
    }

    pub fn lexer_rule(name: String) -> Self {
        Self::new(name, RuleType::Lexer)
    }

    pub fn add_alternative(&mut self, alternative: Alternative) {
        self.alternatives.push(alternative);
    }

    pub fn is_lexer_rule(&self) -> bool {
        self.rule_type == RuleType::Lexer
    }

    pub fn is_parser_rule(&self) -> bool {
        self.rule_type == RuleType::Parser
    }

    pub fn set_fragment(&mut self, is_fragment: bool) {
        self.is_fragment = is_fragment;
    }
}
