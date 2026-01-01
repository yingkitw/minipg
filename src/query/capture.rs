//! Capture groups for query results.

use crate::incremental::Range;
use serde::{Deserialize, Serialize};

/// A capture group in a query match.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureGroup {
    /// Name of the capture (e.g., "function.name")
    pub name: String,
    /// Range of the captured node
    pub range: Range,
    /// Text of the captured node
    pub text: String,
}

impl CaptureGroup {
    pub fn new(name: String, range: Range, text: String) -> Self {
        Self { name, range, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incremental::{Point, Position};

    #[test]
    fn test_capture_group_creation() {
        let range = Range::new(
            Position::new(0, Point::new(0, 0)),
            Position::new(5, Point::new(0, 5)),
        );
        let capture = CaptureGroup::new(
            "variable".to_string(),
            range,
            "hello".to_string(),
        );

        assert_eq!(capture.name, "variable");
        assert_eq!(capture.text, "hello");
        assert_eq!(capture.range.byte_len(), 5);
    }
}
