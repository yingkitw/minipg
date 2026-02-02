//! Position tracking for incremental parsing.

use serde::{Deserialize, Serialize};

/// A point in a text document (line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Point {
    /// Zero-indexed line number
    pub row: usize,
    /// Zero-indexed column number (UTF-8 byte offset within line)
    pub column: usize,
}

impl Point {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn zero() -> Self {
        Self { row: 0, column: 0 }
    }
}

/// A position in a text document with both byte offset and point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Byte offset from start of document
    pub byte: usize,
    /// Line and column position
    pub point: Point,
}

impl Position {
    pub fn new(byte: usize, point: Point) -> Self {
        Self { byte, point }
    }

    pub fn zero() -> Self {
        Self {
            byte: 0,
            point: Point::zero(),
        }
    }
}

/// A range in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Check if this range contains a byte offset.
    pub fn contains_byte(&self, byte: usize) -> bool {
        self.start.byte <= byte && byte < self.end.byte
    }

    /// Check if this range overlaps with another range.
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start.byte < other.end.byte && other.start.byte < self.end.byte
    }

    /// Get the byte length of this range.
    pub fn byte_len(&self) -> usize {
        self.end.byte.saturating_sub(self.start.byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point::new(5, 10);
        assert_eq!(point.row, 5);
        assert_eq!(point.column, 10);
    }

    #[test]
    fn test_point_zero() {
        let point = Point::zero();
        assert_eq!(point.row, 0);
        assert_eq!(point.column, 0);
    }

    #[test]
    fn test_position_creation() {
        let pos = Position::new(100, Point::new(5, 10));
        assert_eq!(pos.byte, 100);
        assert_eq!(pos.point.row, 5);
        assert_eq!(pos.point.column, 10);
    }

    #[test]
    fn test_range_contains_byte() {
        let range = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(20, Point::new(0, 20)),
        );
        assert!(range.contains_byte(10));
        assert!(range.contains_byte(15));
        assert!(!range.contains_byte(20));
        assert!(!range.contains_byte(5));
    }

    #[test]
    fn test_range_overlaps() {
        let range1 = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(20, Point::new(0, 20)),
        );
        let range2 = Range::new(
            Position::new(15, Point::new(0, 15)),
            Position::new(25, Point::new(0, 25)),
        );
        let range3 = Range::new(
            Position::new(25, Point::new(0, 25)),
            Position::new(30, Point::new(0, 30)),
        );

        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));
        assert!(!range1.overlaps(&range3));
        assert!(!range3.overlaps(&range1));
    }

    #[test]
    fn test_range_byte_len() {
        let range = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(20, Point::new(0, 20)),
        );
        assert_eq!(range.byte_len(), 10);
    }

    #[test]
    fn test_position_zero() {
        let pos = Position::zero();
        assert_eq!(pos.byte, 0);
        assert_eq!(pos.point.row, 0);
        assert_eq!(pos.point.column, 0);
    }

    #[test]
    fn test_range_empty() {
        let range = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(10, Point::new(0, 10)),
        );
        assert_eq!(range.byte_len(), 0);
        assert!(!range.contains_byte(10)); // End is exclusive
    }

    #[test]
    fn test_range_contains_byte_at_boundaries() {
        let range = Range::new(
            Position::new(5, Point::new(0, 5)),
            Position::new(15, Point::new(0, 15)),
        );

        // Start is inclusive
        assert!(range.contains_byte(5));
        // End is exclusive
        assert!(!range.contains_byte(15));
        // Before start
        assert!(!range.contains_byte(4));
        // After end
        assert!(!range.contains_byte(16));
    }

    #[test]
    fn test_range_overlaps_adjacent() {
        let range1 = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(20, Point::new(0, 20)),
        );
        let range2 = Range::new(
            Position::new(20, Point::new(0, 20)),
            Position::new(30, Point::new(0, 30)),
        );

        // Adjacent ranges should not overlap (end is exclusive)
        assert!(!range1.overlaps(&range2));
        assert!(!range2.overlaps(&range1));
    }

    #[test]
    fn test_range_overlaps_contained() {
        let outer = Range::new(
            Position::new(0, Point::new(0, 0)),
            Position::new(100, Point::new(0, 100)),
        );
        let inner = Range::new(
            Position::new(25, Point::new(0, 25)),
            Position::new(50, Point::new(0, 50)),
        );

        assert!(outer.overlaps(&inner));
        assert!(inner.overlaps(&outer));
    }

    #[test]
    fn test_range_overlaps_identical() {
        let range = Range::new(
            Position::new(10, Point::new(0, 10)),
            Position::new(20, Point::new(0, 20)),
        );

        assert!(range.overlaps(&range));
    }

    #[test]
    fn test_point_ordering() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(0, 10);
        let point3 = Point::new(5, 0);

        assert!(point1 < point2);
        assert!(point2 < point3);
        assert!(point1 < point3);
    }

    #[test]
    fn test_range_byte_len_with_saturating_sub() {
        // Test with end < start (should return 0 due to saturating_sub)
        let range = Range::new(
            Position::new(20, Point::new(0, 20)),
            Position::new(10, Point::new(0, 10)),
        );
        assert_eq!(range.byte_len(), 0);
    }

    #[test]
    fn test_point_equality() {
        let point1 = Point::new(5, 10);
        let point2 = Point::new(5, 10);
        let point3 = Point::new(5, 11);
        let point4 = Point::new(6, 10);

        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
        assert_ne!(point1, point4);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(100, Point::new(5, 10));
        let pos2 = Position::new(100, Point::new(5, 10));
        let pos3 = Position::new(101, Point::new(5, 10));

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }
}
