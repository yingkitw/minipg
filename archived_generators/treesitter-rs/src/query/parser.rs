//! Parser for query language (S-expressions).

use super::pattern::{Pattern, PatternNode};
use minipg_core::Result;

/// Parser for query language.
pub struct QueryParser {
    source: String,
    pos: usize,
}

impl QueryParser {
    pub fn new(source: String) -> Self {
        Self { source, pos: 0 }
    }

    /// Parse a query string into patterns.
    pub fn parse(&mut self) -> Result<Vec<Pattern>> {
        let mut patterns = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            // Skip comments
            if self.peek() == ';' {
                self.skip_comment();
                continue;
            }

            // Parse pattern
            let pattern = self.parse_pattern()?;
            patterns.push(pattern);
        }

        Ok(patterns)
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        self.skip_whitespace();
        let mut root = self.parse_node()?;

        // Check for capture after the closing paren
        self.skip_whitespace();
        if self.peek() == '@' {
            self.advance();
            let capture_name = self.parse_identifier()?;
            root = root.with_capture(capture_name);
        }

        Ok(Pattern::new(root))
    }

    fn parse_node(&mut self) -> Result<PatternNode> {
        self.skip_whitespace();

        // Expect '('
        if !self.consume('(') {
            return Err(minipg_core::Error::parse(
                format!("position {}", self.pos),
                "Expected '('",
            ));
        }

        self.skip_whitespace();

        // Parse node type
        let node_type = self.parse_identifier()?;
        let mut node = if node_type == "_" {
            PatternNode::wildcard()
        } else {
            PatternNode::new(node_type)
        };

        // Parse children
        loop {
            self.skip_whitespace();

            if self.peek() == ')' {
                self.advance();
                break;
            }

            // Check for field (name:)
            if self.peek_ahead_for_colon() {
                let field_name = self.parse_identifier()?;
                self.skip_whitespace();
                self.consume(':');
                self.skip_whitespace();

                // Parse the child node for this field
                let mut child = self.parse_node()?;
                child = child.with_field(field_name);
                node.children.push(child);
                continue;
            }

            // Parse child node
            let child = self.parse_node()?;
            node.children.push(child);
        }

        Ok(node)
    }

    fn parse_identifier(&mut self) -> Result<String> {
        let start = self.pos;
        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
                self.advance();
            } else {
                break;
            }
        }

        if start == self.pos {
            return Err(minipg_core::Error::parse(
                format!("position {}", self.pos),
                "Expected identifier",
            ));
        }

        Ok(self.source[start..self.pos].to_string())
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        if !self.is_at_end() {
            self.advance(); // Skip newline
        }
    }

    fn peek(&self) -> char {
        self.source[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_ahead_for_colon(&self) -> bool {
        let mut temp_pos = self.pos;
        while temp_pos < self.source.len() {
            let ch = self.source[temp_pos..].chars().next().unwrap_or('\0');
            if ch == ':' {
                return true;
            }
            if ch.is_whitespace() {
                temp_pos += ch.len_utf8();
            } else if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
                temp_pos += ch.len_utf8();
            } else {
                return false;
            }
        }
        false
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            let ch = self.peek();
            self.pos += ch.len_utf8();
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pattern() {
        let query = "(identifier)";
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].root.node_type, "identifier");
    }

    #[test]
    fn test_parse_pattern_with_capture() {
        let query = "(identifier) @variable";
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].root.capture, Some("variable".to_string()));
    }

    #[test]
    fn test_parse_pattern_with_field() {
        let query = "(function name: (identifier))";
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].root.node_type, "function");
        assert_eq!(patterns[0].root.children.len(), 1);
        assert_eq!(patterns[0].root.children[0].field, Some("name".to_string()));
    }

    #[test]
    fn test_parse_pattern_with_field_and_capture() {
        // Test pattern with field and capture on the whole pattern
        let query = "(function name: (identifier)) @function.decl";
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].root.capture, Some("function.decl".to_string()));
        let child = &patterns[0].root.children[0];
        assert_eq!(child.field, Some("name".to_string()));
        assert_eq!(child.node_type, "identifier");
    }

    #[test]
    fn test_parse_multiple_patterns() {
        let query = r#"
            (identifier) @variable
            (number) @number
            (string) @string
        "#;
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 3);
    }

    #[test]
    fn test_parse_with_comments() {
        let query = r#"
            ; This is a comment
            (identifier) @variable
            ; Another comment
            (number) @number
        "#;
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_parse_wildcard() {
        let query = "(_)";
        let mut parser = QueryParser::new(query.to_string());
        let patterns = parser.parse().unwrap();

        assert_eq!(patterns.len(), 1);
        assert!(patterns[0].root.is_wildcard);
    }
}
