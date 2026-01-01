//! Edit tracking for incremental parsing.

use super::position::Point;
use serde::{Deserialize, Serialize};

/// An edit to a text document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edit {
    /// Start byte offset of the edit
    pub start_byte: usize,
    /// Old end byte offset (before edit)
    pub old_end_byte: usize,
    /// New end byte offset (after edit)
    pub new_end_byte: usize,
    /// Start point of the edit
    pub start_point: Point,
    /// Old end point (before edit)
    pub old_end_point: Point,
    /// New end point (after edit)
    pub new_end_point: Point,
}

impl Edit {
    /// Create a new edit.
    pub fn new(
        start_byte: usize,
        old_end_byte: usize,
        new_end_byte: usize,
        start_point: Point,
        old_end_point: Point,
        new_end_point: Point,
    ) -> Self {
        Self {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_point,
            old_end_point,
            new_end_point,
        }
    }

    /// Create an edit for inserting text.
    pub fn insert(byte: usize, point: Point, text: &str) -> Self {
        let new_end_byte = byte + text.len();
        let new_end_point = Self::advance_point(point, text);

        Self {
            start_byte: byte,
            old_end_byte: byte,
            new_end_byte,
            start_point: point,
            old_end_point: point,
            new_end_point,
        }
    }

    /// Create an edit for deleting text.
    pub fn delete(start_byte: usize, end_byte: usize, start_point: Point, end_point: Point) -> Self {
        Self {
            start_byte,
            old_end_byte: end_byte,
            new_end_byte: start_byte,
            start_point,
            old_end_point: end_point,
            new_end_point: start_point,
        }
    }

    /// Create an edit for replacing text.
    pub fn replace(
        start_byte: usize,
        old_end_byte: usize,
        start_point: Point,
        old_end_point: Point,
        new_text: &str,
    ) -> Self {
        let new_end_byte = start_byte + new_text.len();
        let new_end_point = Self::advance_point(start_point, new_text);

        Self {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_point,
            old_end_point,
            new_end_point,
        }
    }

    /// Get the old byte length (deleted text).
    pub fn old_len(&self) -> usize {
        self.old_end_byte.saturating_sub(self.start_byte)
    }

    /// Get the new byte length (inserted text).
    pub fn new_len(&self) -> usize {
        self.new_end_byte.saturating_sub(self.start_byte)
    }

    /// Get the byte delta (new_len - old_len).
    pub fn byte_delta(&self) -> isize {
        self.new_len() as isize - self.old_len() as isize
    }

    /// Advance a point by the given text.
    fn advance_point(mut point: Point, text: &str) -> Point {
        for ch in text.chars() {
            if ch == '\n' {
                point.row += 1;
                point.column = 0;
            } else {
                point.column += ch.len_utf8();
            }
        }
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_insert() {
        let edit = Edit::insert(10, Point::new(0, 10), "hello");
        assert_eq!(edit.start_byte, 10);
        assert_eq!(edit.old_end_byte, 10);
        assert_eq!(edit.new_end_byte, 15);
        assert_eq!(edit.old_len(), 0);
        assert_eq!(edit.new_len(), 5);
        assert_eq!(edit.byte_delta(), 5);
    }

    #[test]
    fn test_edit_delete() {
        let edit = Edit::delete(10, 15, Point::new(0, 10), Point::new(0, 15));
        assert_eq!(edit.start_byte, 10);
        assert_eq!(edit.old_end_byte, 15);
        assert_eq!(edit.new_end_byte, 10);
        assert_eq!(edit.old_len(), 5);
        assert_eq!(edit.new_len(), 0);
        assert_eq!(edit.byte_delta(), -5);
    }

    #[test]
    fn test_edit_replace() {
        let edit = Edit::replace(
            10,
            15,
            Point::new(0, 10),
            Point::new(0, 15),
            "hello world",
        );
        assert_eq!(edit.start_byte, 10);
        assert_eq!(edit.old_end_byte, 15);
        assert_eq!(edit.new_end_byte, 21);
        assert_eq!(edit.old_len(), 5);
        assert_eq!(edit.new_len(), 11);
        assert_eq!(edit.byte_delta(), 6);
    }

    #[test]
    fn test_advance_point_single_line() {
        let point = Point::new(0, 10);
        let new_point = Edit::advance_point(point, "hello");
        assert_eq!(new_point.row, 0);
        assert_eq!(new_point.column, 15);
    }

    #[test]
    fn test_advance_point_multiline() {
        let point = Point::new(0, 10);
        let new_point = Edit::advance_point(point, "hello\nworld");
        assert_eq!(new_point.row, 1);
        assert_eq!(new_point.column, 5);
    }

    #[test]
    fn test_advance_point_multiple_newlines() {
        let point = Point::new(0, 0);
        let new_point = Edit::advance_point(point, "line1\nline2\nline3");
        assert_eq!(new_point.row, 2);
        assert_eq!(new_point.column, 5);
    }
}
