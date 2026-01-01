/// Comprehensive tests for query language capabilities
/// Tests S-expression parsing, pattern matching, and capture groups

use minipg::{QueryParser, QueryMatcher, Pattern, PatternNode};

// ============================================================================
// Basic Query Parsing Tests
// ============================================================================

#[test]
fn test_parse_simple_pattern() {
    let query = "(identifier) @variable";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.node_type, "identifier");
    assert_eq!(patterns[0].root.capture, Some("variable".to_string()));
}

#[test]
fn test_parse_pattern_without_capture() {
    let query = "(identifier)";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.node_type, "identifier");
    assert_eq!(patterns[0].root.capture, None);
}

#[test]
fn test_parse_wildcard_pattern() {
    let query = "(_) @any";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert!(patterns[0].root.is_wildcard);
    assert_eq!(patterns[0].root.capture, Some("any".to_string()));
}

// ============================================================================
// Field Matching Tests
// ============================================================================

#[test]
fn test_parse_pattern_with_field() {
    let query = "(function name: (identifier))";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.node_type, "function");
    assert_eq!(patterns[0].root.children.len(), 1);
    assert_eq!(patterns[0].root.children[0].field, Some("name".to_string()));
    assert_eq!(patterns[0].root.children[0].node_type, "identifier");
}

#[test]
fn test_parse_pattern_with_multiple_fields() {
    let query = "(function name: (identifier) body: (block))";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.children.len(), 2);
    assert_eq!(patterns[0].root.children[0].field, Some("name".to_string()));
    assert_eq!(patterns[0].root.children[1].field, Some("body".to_string()));
}

// ============================================================================
// Multiple Pattern Tests
// ============================================================================

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
    assert_eq!(patterns[0].root.capture, Some("variable".to_string()));
    assert_eq!(patterns[1].root.capture, Some("number".to_string()));
    assert_eq!(patterns[2].root.capture, Some("string".to_string()));
}

// ============================================================================
// Comment Handling Tests
// ============================================================================

#[test]
fn test_parse_query_with_comments() {
    let query = r#"
        ; This is a comment
        (identifier) @variable
        ; Another comment
        (number) @number  ; inline comment
    "#;
    
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 2);
}

#[test]
fn test_parse_query_only_comments() {
    let query = r#"
        ; Just comments
        ; No patterns
    "#;
    
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 0);
}

// ============================================================================
// Capture Name Tests
// ============================================================================

#[test]
fn test_parse_dotted_capture_names() {
    let query = "(identifier) @variable.name";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.capture, Some("variable.name".to_string()));
}

#[test]
fn test_parse_hyphenated_capture_names() {
    let query = "(identifier) @variable-name";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.capture, Some("variable-name".to_string()));
}

// ============================================================================
// Pattern Node Builder Tests
// ============================================================================

#[test]
fn test_pattern_node_creation() {
    let node = PatternNode::new("function".to_string());
    assert_eq!(node.node_type, "function");
    assert_eq!(node.field, None);
    assert_eq!(node.capture, None);
    assert!(!node.is_wildcard);
}

#[test]
fn test_pattern_node_with_field() {
    let node = PatternNode::new("function".to_string())
        .with_field("name".to_string());
    
    assert_eq!(node.field, Some("name".to_string()));
}

#[test]
fn test_pattern_node_with_capture() {
    let node = PatternNode::new("function".to_string())
        .with_capture("func".to_string());
    
    assert_eq!(node.capture, Some("func".to_string()));
}

#[test]
fn test_pattern_node_wildcard() {
    let node = PatternNode::wildcard();
    assert!(node.is_wildcard);
}

// ============================================================================
// Pattern Creation Tests
// ============================================================================

#[test]
fn test_pattern_creation_simple() {
    let node = PatternNode::new("identifier".to_string());
    let pattern = Pattern::new(node);
    
    assert_eq!(pattern.root.node_type, "identifier");
}

#[test]
fn test_pattern_with_capture() {
    let node = PatternNode::new("identifier".to_string())
        .with_capture("var".to_string());
    let pattern = Pattern::new(node);
    
    assert_eq!(pattern.root.capture, Some("var".to_string()));
    assert_eq!(pattern.captures.len(), 1);
    assert_eq!(pattern.captures[0], "var");
}

#[test]
fn test_pattern_with_nested_captures() {
    let mut root = PatternNode::new("function".to_string());
    root.children.push(
        PatternNode::new("identifier".to_string())
            .with_capture("name".to_string())
    );
    root.children.push(
        PatternNode::new("block".to_string())
            .with_capture("body".to_string())
    );
    
    let pattern = Pattern::new(root);
    
    assert_eq!(pattern.captures.len(), 2);
    assert!(pattern.captures.contains(&"name".to_string()));
    assert!(pattern.captures.contains(&"body".to_string()));
}

// ============================================================================
// Query Matcher Tests
// ============================================================================

#[test]
fn test_query_matcher_creation() {
    let patterns = vec![
        Pattern::new(PatternNode::new("identifier".to_string())),
        Pattern::new(PatternNode::new("number".to_string())),
    ];
    
    let _matcher = QueryMatcher::new(patterns);
    // Matcher created successfully
}

// ============================================================================
// Real-World Query Examples
// ============================================================================

#[test]
fn test_syntax_highlighting_query() {
    let query = r#"
        ; Keywords
        (grammar) @keyword
        
        ; Comments
        (line_comment) @comment
        
        ; Strings
        (string_literal) @string
    "#;
    
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 3);
}

#[test]
fn test_antlr4_highlight_query() {
    let query = r#"
        ; ANTLR4 specific highlighting
        (parser_rule name: (identifier)) @function.parser
        (lexer_rule name: (identifier)) @function.lexer
        (fragment) @keyword.modifier
    "#;
    
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 3);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_parse_empty_query() {
    let query = "";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 0);
}

#[test]
fn test_parse_whitespace_only() {
    let query = "   \n\n   \t\t   ";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 0);
}

#[test]
fn test_parse_pattern_with_extra_whitespace() {
    let query = "  \n  (identifier)  \n  @variable  \n  ";
    let mut parser = QueryParser::new(query.to_string());
    let patterns = parser.parse().unwrap();
    
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].root.capture, Some("variable".to_string()));
}
