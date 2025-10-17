//! Code generation tests.

use minipg::ast::{Alternative, Element, Grammar, Rule};
use minipg::codegen::RustCodeGenerator;
use minipg::core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};

#[test]
fn test_rust_codegen_simple() {
    let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Parser);
    
    let mut expr_rule = Rule::parser_rule("expr".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("term".to_string()));
    expr_rule.add_alternative(alt);
    grammar.add_rule(expr_rule);
    
    let mut term_rule = Rule::parser_rule("term".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("NUMBER".to_string()));
    term_rule.add_alternative(alt);
    grammar.add_rule(term_rule);
    
    let mut number_rule = Rule::lexer_rule("NUMBER".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::one_or_more(Element::CharRange {
        start: '0',
        end: '9',
    }));
    number_rule.add_alternative(alt);
    grammar.add_rule(number_rule);

    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig::default();
    let result = generator.generate(&grammar, &config);
    
    assert!(result.is_ok());
    let code = result.unwrap();
    
    insta::assert_snapshot!(code);
}
