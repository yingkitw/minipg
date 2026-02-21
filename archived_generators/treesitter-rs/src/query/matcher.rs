//! Pattern matching engine for queries.

use super::capture::CaptureGroup;
use super::pattern::{Pattern, PatternNode};
use minipg_antlr::{Element, Grammar, Rule};
use minipg_core::{Point, Position, Range};

/// A match result from a query.
#[derive(Debug, Clone)]
pub struct Match {
    /// Pattern index that matched
    pub pattern_index: usize,
    /// Captured groups
    pub captures: Vec<Capture>,
}

/// A single capture in a match.
#[derive(Debug, Clone)]
pub struct Capture {
    /// Capture group
    pub group: CaptureGroup,
    /// Pattern node that produced this capture
    pub node_index: usize,
}

/// Query matcher for finding patterns in syntax trees.
pub struct QueryMatcher {
    patterns: Vec<Pattern>,
}

impl QueryMatcher {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }

    /// Find all matches in a grammar.
    pub fn find_matches(&self, grammar: &Grammar, source: &str) -> Vec<Match> {
        let mut matches = Vec::new();

        for (pattern_idx, pattern) in self.patterns.iter().enumerate() {
            // Match against rules
            for rule in &grammar.rules {
                if let Some(rule_matches) = self.match_rule(pattern, rule, source, pattern_idx) {
                    matches.extend(rule_matches);
                }
            }
        }

        matches
    }

    fn match_rule(
        &self,
        pattern: &Pattern,
        rule: &Rule,
        source: &str,
        pattern_idx: usize,
    ) -> Option<Vec<Match>> {
        let mut matches = Vec::new();

        // Try to match pattern against rule name
        if self.matches_node_type(&pattern.root, &rule.name) {
            let mut captures = Vec::new();

            if let Some(ref capture_name) = pattern.root.capture {
                // Create a capture for the rule
                let range = Range::new(
                    Position::new(0, Point::new(0, 0)),
                    Position::new(rule.name.len(), Point::new(0, rule.name.len())),
                );

                captures.push(Capture {
                    group: CaptureGroup::new(capture_name.clone(), range, rule.name.clone()),
                    node_index: 0,
                });
            }

            // Match children against rule elements
            for alt in &rule.alternatives {
                for element in &alt.elements {
                    if let Some(element_captures) = self.match_element(pattern, element, source) {
                        captures.extend(element_captures);
                    }
                }
            }

            if !captures.is_empty() || pattern.root.capture.is_some() {
                matches.push(Match {
                    pattern_index: pattern_idx,
                    captures,
                });
            }
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    fn match_element(
        &self,
        pattern: &Pattern,
        element: &Element,
        _source: &str,
    ) -> Option<Vec<Capture>> {
        let mut captures = Vec::new();

        // Match against element type
        match element {
            Element::RuleRef { name, .. } => {
                for child in &pattern.root.children {
                    if self.matches_node_type(child, name) {
                        if let Some(ref capture_name) = child.capture {
                            let range = Range::new(
                                Position::new(0, Point::new(0, 0)),
                                Position::new(name.len(), Point::new(0, name.len())),
                            );

                            captures.push(Capture {
                                group: CaptureGroup::new(capture_name.clone(), range, name.clone()),
                                node_index: 0,
                            });
                        }
                    }
                }
            }
            Element::StringLiteral { value, .. } => {
                for child in &pattern.root.children {
                    if child.node_type == "string" || child.is_wildcard {
                        if let Some(ref capture_name) = child.capture {
                            let range = Range::new(
                                Position::new(0, Point::new(0, 0)),
                                Position::new(value.len(), Point::new(0, value.len())),
                            );

                            captures.push(Capture {
                                group: CaptureGroup::new(
                                    capture_name.clone(),
                                    range,
                                    value.clone(),
                                ),
                                node_index: 0,
                            });
                        }
                    }
                }
            }
            _ => {}
        }

        if captures.is_empty() {
            None
        } else {
            Some(captures)
        }
    }

    fn matches_node_type(&self, node: &PatternNode, type_name: &str) -> bool {
        if node.is_wildcard {
            return true;
        }
        node.node_type == type_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minipg_antlr::{ast::Alternative, ast::RuleType};
    use minipg_core::types::GrammarType;

    #[test]
    fn test_query_matcher_creation() {
        let patterns = vec![Pattern::new(PatternNode::new("identifier".to_string()))];
        let matcher = QueryMatcher::new(patterns);
        assert_eq!(matcher.patterns.len(), 1);
    }

    #[test]
    fn test_match_rule_name() {
        let pattern = Pattern::new(
            PatternNode::new("test_rule".to_string()).with_capture("rule".to_string()),
        );

        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Combined);
        let mut rule = Rule::new("test_rule".to_string(), RuleType::Parser);
        rule.alternatives.push(Alternative::new());
        grammar.add_rule(rule);

        let matcher = QueryMatcher::new(vec![pattern]);
        let matches = matcher.find_matches(&grammar, "");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].captures.len(), 1);
        assert_eq!(matches[0].captures[0].group.name, "rule");
    }

    #[test]
    fn test_wildcard_match() {
        let pattern = Pattern::new(PatternNode::wildcard().with_capture("any".to_string()));

        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Combined);
        let mut rule = Rule::new("any_rule".to_string(), RuleType::Parser);
        rule.alternatives.push(Alternative::new());
        grammar.add_rule(rule);

        let matcher = QueryMatcher::new(vec![pattern]);
        let matches = matcher.find_matches(&grammar, "");

        assert_eq!(matches.len(), 1);
    }
}
