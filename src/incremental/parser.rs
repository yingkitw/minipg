//! Incremental parser implementation.

use super::edit::Edit;
use super::position::{Point, Position, Range};
use crate::ast::Grammar;
use crate::core::{Result, traits::GrammarParser as GrammarParserTrait};

/// A syntax tree with position information for incremental parsing.
#[derive(Debug, Clone)]
pub struct SyntaxTree {
    /// The parsed grammar
    pub grammar: Grammar,
    /// Source text
    pub source: String,
    /// Root range
    pub range: Range,
}

impl SyntaxTree {
    pub fn new(grammar: Grammar, source: String) -> Self {
        let end_byte = source.len();
        let end_point = Self::compute_end_point(&source);
        
        Self {
            grammar,
            source,
            range: Range::new(
                Position::zero(),
                Position::new(end_byte, end_point),
            ),
        }
    }

    fn compute_end_point(text: &str) -> Point {
        let mut row = 0;
        let mut column = 0;
        
        for ch in text.chars() {
            if ch == '\n' {
                row += 1;
                column = 0;
            } else {
                column += ch.len_utf8();
            }
        }
        
        Point::new(row, column)
    }
}

/// Trait for incremental parsing.
pub trait IncrementalParser {
    /// Parse source text into a syntax tree.
    fn parse(&self, source: &str) -> Result<SyntaxTree>;

    /// Parse incrementally, reusing unchanged parts of the old tree.
    fn parse_incremental(&self, old_tree: &SyntaxTree, edit: &Edit) -> Result<SyntaxTree>;

    /// Apply an edit to source text.
    fn apply_edit(source: &str, edit: &Edit, new_text: &str) -> String {
        let mut result = String::with_capacity(source.len() + edit.byte_delta().abs() as usize);
        
        // Add text before edit
        result.push_str(&source[..edit.start_byte]);
        
        // Add new text
        result.push_str(new_text);
        
        // Add text after edit
        if edit.old_end_byte < source.len() {
            result.push_str(&source[edit.old_end_byte..]);
        }
        
        result
    }
}

/// Default incremental parser implementation.
pub struct DefaultIncrementalParser {
    parser: crate::parser::GrammarParser,
}

impl DefaultIncrementalParser {
    pub fn new() -> Self {
        Self {
            parser: crate::parser::GrammarParser::new(),
        }
    }
}

impl Default for DefaultIncrementalParser {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalParser for DefaultIncrementalParser {
    fn parse(&self, source: &str) -> Result<SyntaxTree> {
        let grammar = self.parser.parse_string(source, "input")?;
        Ok(SyntaxTree::new(grammar, source.to_string()))
    }

    fn parse_incremental(&self, old_tree: &SyntaxTree, edit: &Edit) -> Result<SyntaxTree> {
        // For now, we do a full re-parse
        // TODO: Implement true incremental parsing with subtree reuse
        
        // This is a placeholder that demonstrates the API
        // Future implementation will:
        // 1. Identify affected ranges
        // 2. Invalidate affected subtrees
        // 3. Re-parse only invalidated regions
        // 4. Reuse unchanged subtrees
        
        let _ = old_tree; // Will be used in future implementation
        let _ = edit; // Will be used in future implementation
        
        // For now, just re-parse everything
        self.parse(&old_tree.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_edit_insert() {
        let source = "hello world";
        let edit = Edit::insert(6, Point::new(0, 6), "beautiful ");
        let new_text = "beautiful ";
        let result = DefaultIncrementalParser::apply_edit(source, &edit, new_text);
        assert_eq!(result, "hello beautiful world");
    }

    #[test]
    fn test_apply_edit_delete() {
        let source = "hello beautiful world";
        let edit = Edit::delete(6, 16, Point::new(0, 6), Point::new(0, 16));
        let result = DefaultIncrementalParser::apply_edit(source, &edit, "");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_apply_edit_replace() {
        let source = "hello world";
        let edit = Edit::replace(6, 11, Point::new(0, 6), Point::new(0, 11), "Rust");
        let result = DefaultIncrementalParser::apply_edit(source, &edit, "Rust");
        assert_eq!(result, "hello Rust");
    }

    #[test]
    fn test_compute_end_point_single_line() {
        let text = "hello world";
        let point = SyntaxTree::compute_end_point(text);
        assert_eq!(point.row, 0);
        assert_eq!(point.column, 11);
    }

    #[test]
    fn test_compute_end_point_multiline() {
        let text = "line1\nline2\nline3";
        let point = SyntaxTree::compute_end_point(text);
        assert_eq!(point.row, 2);
        assert_eq!(point.column, 5);
    }

    #[test]
    fn test_syntax_tree_creation() {
        let grammar = Grammar::new("Test".to_string(), crate::core::types::GrammarType::Combined);
        let source = "grammar Test;".to_string();
        let tree = SyntaxTree::new(grammar, source.clone());
        
        assert_eq!(tree.source, source);
        assert_eq!(tree.range.start.byte, 0);
        assert_eq!(tree.range.end.byte, source.len());
    }
}
