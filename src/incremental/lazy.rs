//! Lazy parsing support for editor integration.
//!
//! Prioritizes visible regions for parsing and defers off-screen content.

use super::position::{Point, Range, Position};
use std::sync::{Arc, RwLock};

/// Configuration for lazy parsing.
#[derive(Debug, Clone)]
pub struct LazyConfig {
    /// Buffer lines around visible region
    pub buffer_lines: usize,
}

impl Default for LazyConfig {
    fn default() -> Self {
        Self { buffer_lines: 50 }
    }
}

/// A parsed region with priority.
#[derive(Debug, Clone)]
pub struct ParsedRegion {
    pub range: Range,
    pub parsed: bool,
    pub priority: i32,
}

impl ParsedRegion {
    pub fn new(range: Range, priority: i32) -> Self {
        Self {
            range,
            parsed: false,
            priority,
        }
    }
}

/// Lazy parser that prioritizes visible regions.
pub struct LazyParser {
    config: LazyConfig,
    visible_range: Arc<RwLock<Option<Range>>>,
}

impl LazyParser {
    pub fn new(config: LazyConfig) -> Self {
        Self {
            config,
            visible_range: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(LazyConfig::default())
    }

    /// Set the visible range (editor viewport).
    pub fn set_visible_range(&self, range: Range) {
        *self.visible_range.write().unwrap() = Some(range);
    }

    /// Get the current visible range.
    pub fn visible_range(&self) -> Option<Range> {
        *self.visible_range.read().unwrap()
    }

    /// Partition source into regions by priority.
    pub fn partition_source(&self, source: &str) -> Vec<ParsedRegion> {
        let total_lines = source.lines().count();
        let visible = self.visible_range.read().unwrap();

        // If no visible range, treat entire source as single region
        let visible_range = match *visible {
            Some(v) => v,
            None => return vec![ParsedRegion::new(
                Range::new(Position::zero(), self.end_position(source)),
                50
            )],
        };

        // Calculate buffer zone
        let start_line = visible_range.start.point.row.saturating_sub(self.config.buffer_lines);
        let end_line = (visible_range.end.point.row + self.config.buffer_lines).min(total_lines);

        // Create regions with priorities
        vec![
            ParsedRegion::new(
                Range::new(
                    Position::new(self.byte_offset_for_line(source, start_line), Point::new(start_line, 0)),
                    Position::new(self.byte_offset_for_line(source, end_line), Point::new(end_line, 0)),
                ),
                100 // High priority for visible + buffer
            ),
        ]
    }

    /// Calculate end position for source text.
    fn end_position(&self, text: &str) -> Position {
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

        Position::new(text.len(), Point::new(row, column))
    }

    /// Get byte offset for a line number.
    fn byte_offset_for_line(&self, source: &str, line: usize) -> usize {
        let mut current_line = 0;
        let mut byte_offset = 0;

        for ch in source.chars() {
            if current_line == line {
                return byte_offset;
            }
            if ch == '\n' {
                current_line += 1;
            }
            byte_offset += ch.len_utf8();
        }

        source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_config_default() {
        let config = LazyConfig::default();
        assert_eq!(config.buffer_lines, 50);
    }

    #[test]
    fn test_set_visible_range() {
        let parser = LazyParser::with_default_config();
        let range = Range::new(
            Position::new(0, Point::new(0, 0)),
            Position::new(100, Point::new(5, 0)),
        );
        parser.set_visible_range(range);

        let retrieved = parser.visible_range();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().start.byte, 0);
    }

    #[test]
    fn test_partition_source() {
        let parser = LazyParser::with_default_config();
        let source = "line1\nline2\nline3\nline4\nline5";

        let visible = Range::new(
            Position::new(0, Point::new(1, 0)),
            Position::new(20, Point::new(2, 0)),
        );
        parser.set_visible_range(visible);

        let regions = parser.partition_source(source);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].priority, 100);
    }

    #[test]
    fn test_byte_offset_for_line() {
        let parser = LazyParser::with_default_config();
        let source = "line1\nline2\nline3";

        assert_eq!(parser.byte_offset_for_line(source, 0), 0);
        assert_eq!(parser.byte_offset_for_line(source, 1), 6);
        assert_eq!(parser.byte_offset_for_line(source, 2), 12);
    }

    #[test]
    fn test_end_position() {
        let parser = LazyParser::with_default_config();

        // Single line
        let pos = parser.end_position("hello");
        assert_eq!(pos.point.row, 0);
        assert_eq!(pos.point.column, 5);

        // Multi-line ending with newline
        let pos = parser.end_position("line1\nline2\n");
        assert_eq!(pos.point.row, 2);
        assert_eq!(pos.point.column, 0);

        // Multi-line not ending with newline
        let pos = parser.end_position("line1\nline2");
        assert_eq!(pos.point.row, 1);
        assert_eq!(pos.point.column, 5);
    }

    #[test]
    fn test_partition_with_empty_source() {
        let parser = LazyParser::with_default_config();
        let source = "";
        let regions = parser.partition_source(source);

        // Should have at least one region even for empty source
        assert!(!regions.is_empty());
    }

    #[test]
    fn test_partition_with_single_line() {
        let parser = LazyParser::with_default_config();
        let source = "single line";
        let regions = parser.partition_source(source);

        assert!(!regions.is_empty());
    }

    #[test]
    fn test_partition_with_large_buffer() {
        let config = LazyConfig {
            buffer_lines: 100, // Large buffer
        };
        let parser = LazyParser::new(config);
        let source = "line1\nline2\nline3\nline4\nline5";

        // Set visible range to middle
        let visible = Range::new(
            Position::new(6, Point::new(1, 0)),
            Position::new(12, Point::new(2, 5)),
        );
        parser.set_visible_range(visible);

        let regions = parser.partition_source(source);
        // With large buffer, we should get fewer regions
        assert!(!regions.is_empty());
    }

    #[test]
    fn test_partition_with_zero_buffer() {
        let config = LazyConfig {
            buffer_lines: 0,
        };
        let parser = LazyParser::new(config);
        let source = "line1\nline2\nline3\nline4\nline5";

        // Set visible range to middle (lines 1-2)
        let visible = Range::new(
            Position::new(6, Point::new(1, 0)),
            Position::new(12, Point::new(2, 5)),
        );
        parser.set_visible_range(visible);

        let regions = parser.partition_source(source);
        // With zero buffer, should get single high-priority region
        assert!(!regions.is_empty());
        assert_eq!(regions.len(), 1);

        // The region should cover lines 1-2 with high priority
        let region = &regions[0];
        assert_eq!(region.priority, 100);
        // Region should start at line 1
        assert_eq!(region.range.start.point.row, 1);
        // Region should extend to at least line 2
        assert!(region.range.end.point.row >= 2);
    }

    #[test]
    fn test_byte_offset_for_line_out_of_bounds() {
        let parser = LazyParser::with_default_config();
        let source = "line1\nline2";

        // Line beyond source should return source length
        let offset = parser.byte_offset_for_line(source, 10);
        assert_eq!(offset, source.len());
    }

    #[test]
    fn test_config_custom_buffer() {
        let config = LazyConfig {
            buffer_lines: 50,
        };
        let parser = LazyParser::new(config);

        // Config should be stored
        assert_eq!(parser.config.buffer_lines, 50);
    }

    #[test]
    fn test_partition_source_without_visible_range() {
        let parser = LazyParser::with_default_config();
        let source = "line1\nline2\nline3";

        // No visible range set
        let regions = parser.partition_source(source);
        // Should still work, just without prioritization
        assert!(!regions.is_empty());
    }

    #[test]
    fn test_visible_range_full_document() {
        let parser = LazyParser::with_default_config();
        let source = "line1\nline2\nline3";

        // Set visible range to entire document
        let visible = Range::new(
            Position::zero(),
            parser.end_position(source),
        );
        parser.set_visible_range(visible);

        let regions = parser.partition_source(source);
        // Should have single high-priority region
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].priority, 100);
    }

    #[test]
    fn test_end_position_with_unicode() {
        let parser = LazyParser::with_default_config();

        // Unicode characters (multi-byte)
        let pos = parser.end_position("hello 世界");
        assert_eq!(pos.point.row, 0);
        // Column should be byte count, not char count
        assert_eq!(pos.point.column, 12); // 5 + 1 + 6 (3 bytes for each Chinese char)
    }

    #[test]
    fn test_end_position_empty_string() {
        let parser = LazyParser::with_default_config();
        let pos = parser.end_position("");

        assert_eq!(pos.point.row, 0);
        assert_eq!(pos.point.column, 0);
        assert_eq!(pos.byte, 0);
    }

    #[test]
    fn test_end_position_only_newlines() {
        let parser = LazyParser::with_default_config();
        let pos = parser.end_position("\n\n\n");

        assert_eq!(pos.point.row, 3);
        assert_eq!(pos.point.column, 0);
        assert_eq!(pos.byte, 3);
    }
}
