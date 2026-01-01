//! Pattern representation for queries.

use serde::{Deserialize, Serialize};

/// A pattern in a query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pattern {
    /// Root node of the pattern
    pub root: PatternNode,
    /// Capture groups in this pattern
    pub captures: Vec<String>,
}

impl Pattern {
    pub fn new(root: PatternNode) -> Self {
        let captures = Self::extract_captures(&root);
        Self { root, captures }
    }

    fn extract_captures(node: &PatternNode) -> Vec<String> {
        let mut captures = Vec::new();
        Self::collect_captures(node, &mut captures);
        captures
    }

    fn collect_captures(node: &PatternNode, captures: &mut Vec<String>) {
        if let Some(ref capture) = node.capture {
            if !captures.contains(capture) {
                captures.push(capture.clone());
            }
        }
        for child in &node.children {
            Self::collect_captures(child, captures);
        }
    }
}

/// A node in a pattern tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternNode {
    /// Node type (rule name or token type)
    pub node_type: String,
    /// Field name (e.g., "name:" in "name: (identifier)")
    pub field: Option<String>,
    /// Capture name (e.g., "@function" in "(identifier) @function")
    pub capture: Option<String>,
    /// Child patterns
    pub children: Vec<PatternNode>,
    /// Whether this is a wildcard (_)
    pub is_wildcard: bool,
    /// Whether this is negated (!)
    pub is_negated: bool,
}

impl PatternNode {
    pub fn new(node_type: String) -> Self {
        Self {
            node_type,
            field: None,
            capture: None,
            children: Vec::new(),
            is_wildcard: false,
            is_negated: false,
        }
    }

    pub fn wildcard() -> Self {
        Self {
            node_type: "_".to_string(),
            field: None,
            capture: None,
            children: Vec::new(),
            is_wildcard: true,
            is_negated: false,
        }
    }

    pub fn with_field(mut self, field: String) -> Self {
        self.field = Some(field);
        self
    }

    pub fn with_capture(mut self, capture: String) -> Self {
        self.capture = Some(capture);
        self
    }

    pub fn with_children(mut self, children: Vec<PatternNode>) -> Self {
        self.children = children;
        self
    }

    pub fn negated(mut self) -> Self {
        self.is_negated = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_node_creation() {
        let node = PatternNode::new("identifier".to_string());
        assert_eq!(node.node_type, "identifier");
        assert_eq!(node.field, None);
        assert_eq!(node.capture, None);
        assert!(!node.is_wildcard);
    }

    #[test]
    fn test_pattern_node_with_capture() {
        let node = PatternNode::new("identifier".to_string())
            .with_capture("variable".to_string());
        assert_eq!(node.capture, Some("variable".to_string()));
    }

    #[test]
    fn test_pattern_node_with_field() {
        let node = PatternNode::new("identifier".to_string())
            .with_field("name".to_string());
        assert_eq!(node.field, Some("name".to_string()));
    }

    #[test]
    fn test_wildcard_node() {
        let node = PatternNode::wildcard();
        assert!(node.is_wildcard);
        assert_eq!(node.node_type, "_");
    }

    #[test]
    fn test_pattern_extract_captures() {
        let root = PatternNode::new("function".to_string())
            .with_children(vec![
                PatternNode::new("identifier".to_string())
                    .with_field("name".to_string())
                    .with_capture("function.name".to_string()),
                PatternNode::new("parameters".to_string())
                    .with_capture("function.params".to_string()),
            ]);

        let pattern = Pattern::new(root);
        assert_eq!(pattern.captures.len(), 2);
        assert!(pattern.captures.contains(&"function.name".to_string()));
        assert!(pattern.captures.contains(&"function.params".to_string()));
    }
}
