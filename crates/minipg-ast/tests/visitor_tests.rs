//! Visitor pattern tests.

use minipg::ast::{Alternative, AstVisitor, Element, Grammar, Rule};
use minipg::core::types::GrammarType;

struct RuleCounter {
    count: usize,
}

impl AstVisitor for RuleCounter {
    fn visit_rule(&mut self, rule: &Rule) {
        self.count += 1;
        self.walk_rule(rule);
    }
}

struct ElementCounter {
    count: usize,
}

impl AstVisitor for ElementCounter {
    fn visit_element(&mut self, element: &Element) {
        self.count += 1;
        self.walk_element(element);
    }
}

#[test]
fn test_visitor_counts_rules() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    grammar.add_rule(Rule::parser_rule("expr".to_string()));
    grammar.add_rule(Rule::parser_rule("term".to_string()));
    grammar.add_rule(Rule::parser_rule("factor".to_string()));
    
    let mut visitor = RuleCounter { count: 0 };
    visitor.visit_grammar(&grammar);
    
    assert_eq!(visitor.count, 3);
}

#[test]
fn test_visitor_counts_elements() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    let mut rule = Rule::parser_rule("expr".to_string());
    let mut alt = Alternative::new();
    alt.add_element(Element::rule_ref("term".to_string()));
    alt.add_element(Element::string_literal("+".to_string()));
    alt.add_element(Element::rule_ref("term".to_string()));
    rule.add_alternative(alt);
    grammar.add_rule(rule);
    
    let mut visitor = ElementCounter { count: 0 };
    visitor.visit_grammar(&grammar);
    
    assert_eq!(visitor.count, 3);
}

#[test]
fn test_visitor_walks_nested_elements() {
    let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
    let mut rule = Rule::parser_rule("expr".to_string());
    let mut alt = Alternative::new();
    
    // Create nested element: (term)+
    let inner = Element::rule_ref("term".to_string());
    let one_or_more = Element::one_or_more(inner);
    alt.add_element(one_or_more);
    
    rule.add_alternative(alt);
    grammar.add_rule(rule);
    
    let mut visitor = ElementCounter { count: 0 };
    visitor.visit_grammar(&grammar);
    
    // Should count both the OneOrMore and the inner RuleRef
    assert_eq!(visitor.count, 2);
}
