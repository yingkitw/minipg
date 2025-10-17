//! Rule AST node definitions.

use super::element::Alternative;
use serde::{Deserialize, Serialize};

/// Type of rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    Parser,
    Lexer,
}

/// Rule argument definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleArg {
    pub name: String,
    pub arg_type: Option<String>,
}

/// Rule return value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleReturn {
    pub name: String,
    pub return_type: Option<String>,
}

/// Rule local variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleLocal {
    pub name: String,
    pub local_type: Option<String>,
}

/// A grammar rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub rule_type: RuleType,
    pub alternatives: Vec<Alternative>,
    pub is_fragment: bool,
    /// Rule arguments: rule[int x, String name]
    pub arguments: Vec<RuleArg>,
    /// Return values: returns [Type value]
    pub returns: Vec<RuleReturn>,
    /// Local variables: locals [Type var]
    pub locals: Vec<RuleLocal>,
}

impl Rule {
    pub fn new(name: String, rule_type: RuleType) -> Self {
        Self {
            name,
            rule_type,
            alternatives: Vec::new(),
            is_fragment: false,
            arguments: Vec::new(),
            returns: Vec::new(),
            locals: Vec::new(),
        }
    }
    
    pub fn add_argument(&mut self, name: String, arg_type: Option<String>) {
        self.arguments.push(RuleArg { name, arg_type });
    }
    
    pub fn add_return(&mut self, name: String, return_type: Option<String>) {
        self.returns.push(RuleReturn { name, return_type });
    }
    
    pub fn add_local(&mut self, name: String, local_type: Option<String>) {
        self.locals.push(RuleLocal { name, local_type });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_with_arguments() {
        let mut rule = Rule::parser_rule("test".to_string());
        rule.add_argument("x".to_string(), Some("int".to_string()));
        rule.add_argument("name".to_string(), Some("String".to_string()));
        
        assert_eq!(rule.arguments.len(), 2);
        assert_eq!(rule.arguments[0].name, "x");
        assert_eq!(rule.arguments[0].arg_type, Some("int".to_string()));
        assert_eq!(rule.arguments[1].name, "name");
        assert_eq!(rule.arguments[1].arg_type, Some("String".to_string()));
    }

    #[test]
    fn test_rule_with_returns() {
        let mut rule = Rule::parser_rule("test".to_string());
        rule.add_return("result".to_string(), Some("Value".to_string()));
        
        assert_eq!(rule.returns.len(), 1);
        assert_eq!(rule.returns[0].name, "result");
        assert_eq!(rule.returns[0].return_type, Some("Value".to_string()));
    }

    #[test]
    fn test_rule_with_locals() {
        let mut rule = Rule::parser_rule("test".to_string());
        rule.add_local("temp".to_string(), Some("int".to_string()));
        rule.add_local("buffer".to_string(), Some("String".to_string()));
        
        assert_eq!(rule.locals.len(), 2);
        assert_eq!(rule.locals[0].name, "temp");
        assert_eq!(rule.locals[1].name, "buffer");
    }

    #[test]
    fn test_rule_with_all_features() {
        let mut rule = Rule::parser_rule("complex".to_string());
        rule.add_argument("input".to_string(), Some("String".to_string()));
        rule.add_return("output".to_string(), Some("Result".to_string()));
        rule.add_local("state".to_string(), Some("int".to_string()));
        
        assert_eq!(rule.arguments.len(), 1);
        assert_eq!(rule.returns.len(), 1);
        assert_eq!(rule.locals.len(), 1);
        assert!(rule.is_parser_rule());
    }

    #[test]
    fn test_rule_arg_without_type() {
        let mut rule = Rule::parser_rule("test".to_string());
        rule.add_argument("x".to_string(), None);
        
        assert_eq!(rule.arguments.len(), 1);
        assert_eq!(rule.arguments[0].name, "x");
        assert_eq!(rule.arguments[0].arg_type, None);
    }
}
