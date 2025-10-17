//! Visitor pattern for AST traversal.

use super::{Alternative, Element, Grammar, Rule};

/// Immutable visitor for AST traversal.
pub trait AstVisitor {
    fn visit_grammar(&mut self, grammar: &Grammar) {
        self.walk_grammar(grammar);
    }

    fn visit_rule(&mut self, rule: &Rule) {
        self.walk_rule(rule);
    }

    fn visit_alternative(&mut self, alternative: &Alternative) {
        self.walk_alternative(alternative);
    }

    fn visit_element(&mut self, element: &Element) {
        self.walk_element(element);
    }

    fn walk_grammar(&mut self, grammar: &Grammar) {
        for rule in &grammar.rules {
            self.visit_rule(rule);
        }
    }

    fn walk_rule(&mut self, rule: &Rule) {
        for alternative in &rule.alternatives {
            self.visit_alternative(alternative);
        }
    }

    fn walk_alternative(&mut self, alternative: &Alternative) {
        for element in &alternative.elements {
            self.visit_element(element);
        }
    }

    fn walk_element(&mut self, element: &Element) {
        match element {
            Element::Optional { element } => self.visit_element(element),
            Element::ZeroOrMore { element } => self.visit_element(element),
            Element::OneOrMore { element } => self.visit_element(element),
            Element::Not { element } => self.visit_element(element),
            Element::Group { alternatives } => {
                for alt in alternatives {
                    self.visit_alternative(alt);
                }
            }
            _ => {}
        }
    }
}

/// Mutable visitor for AST transformation.
pub trait AstVisitorMut {
    fn visit_grammar_mut(&mut self, grammar: &mut Grammar) {
        self.walk_grammar_mut(grammar);
    }

    fn visit_rule_mut(&mut self, rule: &mut Rule) {
        self.walk_rule_mut(rule);
    }

    fn visit_alternative_mut(&mut self, alternative: &mut Alternative) {
        self.walk_alternative_mut(alternative);
    }

    fn visit_element_mut(&mut self, element: &mut Element) {
        self.walk_element_mut(element);
    }

    fn walk_grammar_mut(&mut self, grammar: &mut Grammar) {
        for rule in &mut grammar.rules {
            self.visit_rule_mut(rule);
        }
    }

    fn walk_rule_mut(&mut self, rule: &mut Rule) {
        for alternative in &mut rule.alternatives {
            self.visit_alternative_mut(alternative);
        }
    }

    fn walk_alternative_mut(&mut self, alternative: &mut Alternative) {
        for element in &mut alternative.elements {
            self.visit_element_mut(element);
        }
    }

    fn walk_element_mut(&mut self, element: &mut Element) {
        match element {
            Element::Optional { element } => self.visit_element_mut(element),
            Element::ZeroOrMore { element } => self.visit_element_mut(element),
            Element::OneOrMore { element } => self.visit_element_mut(element),
            Element::Not { element } => self.visit_element_mut(element),
            Element::Group { alternatives } => {
                for alt in alternatives {
                    self.visit_alternative_mut(alt);
                }
            }
            _ => {}
        }
    }
}
