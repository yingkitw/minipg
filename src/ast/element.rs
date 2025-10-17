//! Grammar element AST nodes.

use serde::{Deserialize, Serialize};

/// Lexer command (e.g., skip, channel, mode, type)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LexerCommand {
    /// Skip this token
    Skip,
    /// Send to a channel
    Channel(String),
    /// Switch to a mode
    Mode(String),
    /// Change token type
    Type(String),
    /// Push mode
    PushMode(String),
    /// Pop mode
    PopMode,
    /// More (continue current token)
    More,
}

/// An alternative in a rule (sequence of elements).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub elements: Vec<Element>,
    pub label: Option<String>,
    pub lexer_command: Option<LexerCommand>,
}

impl Alternative {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            label: None,
            lexer_command: None,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_lexer_command(mut self, command: LexerCommand) -> Self {
        self.lexer_command = Some(command);
        self
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn set_lexer_command(&mut self, command: LexerCommand) {
        self.lexer_command = Some(command);
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
    /// Character class (e.g., [a-z0-9], ~["\r\n])
    CharClass {
        negated: bool,
        ranges: Vec<(char, char)>, // Vec of (start, end) - single char is (c, c)
    },
    /// Optional element (?)
    Optional {
        element: Box<Element>,
        greedy: bool, // true for ?, false for ??
    },
    /// Zero or more (*)
    ZeroOrMore {
        element: Box<Element>,
        greedy: bool, // true for *, false for *?
    },
    /// One or more (+)
    OneOrMore {
        element: Box<Element>,
        greedy: bool, // true for +, false for +?
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
            greedy: true,
        }
    }

    pub fn optional_non_greedy(element: Element) -> Self {
        Element::Optional {
            element: Box::new(element),
            greedy: false,
        }
    }

    pub fn zero_or_more(element: Element) -> Self {
        Element::ZeroOrMore {
            element: Box::new(element),
            greedy: true,
        }
    }

    pub fn zero_or_more_non_greedy(element: Element) -> Self {
        Element::ZeroOrMore {
            element: Box::new(element),
            greedy: false,
        }
    }

    pub fn one_or_more(element: Element) -> Self {
        Element::OneOrMore {
            element: Box::new(element),
            greedy: true,
        }
    }

    pub fn one_or_more_non_greedy(element: Element) -> Self {
        Element::OneOrMore {
            element: Box::new(element),
            greedy: false,
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

    #[test]
    fn test_char_class_simple() {
        let elem = Element::CharClass {
            negated: false,
            ranges: vec![('a', 'z')],
        };
        match elem {
            Element::CharClass { negated, ranges } => {
                assert!(!negated);
                assert_eq!(ranges.len(), 1);
                assert_eq!(ranges[0], ('a', 'z'));
            }
            _ => panic!("Expected CharClass"),
        }
    }

    #[test]
    fn test_char_class_negated() {
        let elem = Element::CharClass {
            negated: true,
            ranges: vec![('"', '"'), ('\\', '\\'), ('\u{0000}', '\u{001F}')],
        };
        match elem {
            Element::CharClass { negated, ranges } => {
                assert!(negated);
                assert_eq!(ranges.len(), 3);
            }
            _ => panic!("Expected CharClass"),
        }
    }

    #[test]
    fn test_char_class_multiple_ranges() {
        let elem = Element::CharClass {
            negated: false,
            ranges: vec![('a', 'z'), ('A', 'Z'), ('0', '9')],
        };
        match elem {
            Element::CharClass { negated, ranges } => {
                assert!(!negated);
                assert_eq!(ranges.len(), 3);
            }
            _ => panic!("Expected CharClass"),
        }
    }
}
