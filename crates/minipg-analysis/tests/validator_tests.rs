//! Grammar validator tests.

use minipg::analysis::GrammarValidator;
use minipg::ast::{Grammar, Rule};
use minipg::core::{types::GrammarType, GrammarValidator as GrammarValidatorTrait};

#[test]
fn test_valid_grammar() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    grammar.add_rule(Rule::parser_rule("expr".to_string()));
    
    let validator = GrammarValidator::new();
    assert!(validator.validate(&grammar).is_ok());
}

#[test]
fn test_empty_name() {
    let grammar = Grammar::new("".to_string(), GrammarType::Parser);
    
    let validator = GrammarValidator::new();
    let result = validator.validate(&grammar);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("name cannot be empty"));
}

#[test]
fn test_no_rules() {
    let grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    
    let validator = GrammarValidator::new();
    let result = validator.validate(&grammar);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("at least one rule"));
}
