//! Grammar element AST nodes.

use serde::{Deserialize, Serialize};

/// An alternative in a rule (sequence of elements).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub elements: Vec<Element>,
    pub label: Option<String>,
}

impl Alternative {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            label: None,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
}

impl Default for Alternative {
    fn default() -> Self {
        Self::new()
    }
}

/// Grammar element (terminal, non-terminal, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    /// Reference to another rule
    RuleRef {
        name: String,
        label: Option<String>,
    },
    /// Terminal token
    Terminal {
        value: String,
        label: Option<String>,
    },
    /// String literal
    StringLiteral {
        value: String,
        label: Option<String>,
    },
    /// Character range (for lexer rules)
    CharRange {
        start: char,
        end: char,
    },
    /// Optional element (?)
    Optional {
        element: Box<Element>,
    },
    /// Zero or more (*)
    ZeroOrMore {
        element: Box<Element>,
    },
    /// One or more (+)
    OneOrMore {
        element: Box<Element>,
    },
    /// Grouped elements
    Group {
        alternatives: Vec<Alternative>,
    },
    /// Negation (~)
    Not {
        element: Box<Element>,
    },
    /// Any character (.)
    Wildcard,
    /// End of file
    Eof,
    /// Embedded action code
    Action {
        code: String,
        language: Option<String>,
    },
    /// Semantic predicate
    Predicate {
        code: String,
        language: Option<String>,
    },
}

impl Element {
    pub fn rule_ref(name: String) -> Self {
        Element::RuleRef { name, label: None }
    }

    pub fn terminal(value: String) -> Self {
        Element::Terminal { value, label: None }
    }

    pub fn string_literal(value: String) -> Self {
        Element::StringLiteral { value, label: None }
    }

    pub fn optional(element: Element) -> Self {
        Element::Optional {
            element: Box::new(element),
        }
    }

    pub fn zero_or_more(element: Element) -> Self {
        Element::ZeroOrMore {
            element: Box::new(element),
        }
    }

    pub fn one_or_more(element: Element) -> Self {
        Element::OneOrMore {
            element: Box::new(element),
        }
    }

    pub fn with_label(self, label: String) -> Self {
        match self {
            Element::RuleRef { name, .. } => Element::RuleRef {
                name,
                label: Some(label),
            },
            Element::Terminal { value, .. } => Element::Terminal {
                value,
                label: Some(label),
            },
            Element::StringLiteral { value, .. } => Element::StringLiteral {
                value,
                label: Some(label),
            },
            other => other,
        }
    }

    pub fn action(code: String) -> Self {
        Element::Action {
            code,
            language: None,
        }
    }

    pub fn action_with_language(code: String, language: String) -> Self {
        Element::Action {
            code,
            language: Some(language),
        }
    }

    pub fn predicate(code: String) -> Self {
        Element::Predicate {
            code,
            language: None,
        }
    }

    pub fn predicate_with_language(code: String, language: String) -> Self {
        Element::Predicate {
            code,
            language: Some(language),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternative_new() {
        let alt = Alternative::new();
        assert_eq!(alt.elements.len(), 0);
        assert_eq!(alt.label, None);
    }

    #[test]
    fn test_alternative_with_label() {
        let alt = Alternative::new().with_label("myLabel".to_string());
        assert_eq!(alt.label, Some("myLabel".to_string()));
    }

    #[test]
    fn test_alternative_add_element() {
        let mut alt = Alternative::new();
        alt.add_element(Element::Terminal {
            value: "ID".to_string(),
            label: None,
        });
        assert_eq!(alt.elements.len(), 1);
    }

    #[test]
    fn test_element_rule_ref() {
        let elem = Element::RuleRef {
            name: "expr".to_string(),
            label: Some("e".to_string()),
        };
        match elem {
            Element::RuleRef { name, label } => {
                assert_eq!(name, "expr");
                assert_eq!(label, Some("e".to_string()));
            }
            _ => panic!("Expected RuleRef"),
        }
    }

    #[test]
    fn test_element_terminal() {
        let elem = Element::Terminal {
            value: "ID".to_string(),
            label: None,
        };
        match elem {
            Element::Terminal { value, label } => {
                assert_eq!(value, "ID");
                assert_eq!(label, None);
            }
            _ => panic!("Expected Terminal"),
        }
    }

    #[test]
    fn test_element_action() {
        let elem = Element::action("System.out.println();".to_string());
        match elem {
            Element::Action { code, language } => {
                assert_eq!(code, "System.out.println();");
                assert_eq!(language, None);
            }
            _ => panic!("Expected Action"),
        }
    }

    #[test]
    fn test_element_predicate() {
        let elem = Element::predicate("x > 0".to_string());
        match elem {
            Element::Predicate { code, language } => {
                assert_eq!(code, "x > 0");
                assert_eq!(language, None);
            }
            _ => panic!("Expected Predicate"),
        }
    }
}
