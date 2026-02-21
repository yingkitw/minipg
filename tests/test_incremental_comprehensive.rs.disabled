/// Comprehensive tests for incremental parsing capabilities
/// Tests position tracking, edit operations, and the foundation for incremental re-parsing

use minipg::{Point, Position, Range, Edit};

// ============================================================================
// Position Tracking Tests
// ============================================================================

#[test]
fn test_position_creation_and_access() {
    let pos = Position::new(100, Point::new(5, 10));
    assert_eq!(pos.byte, 100);
    assert_eq!(pos.point.row, 5);
    assert_eq!(pos.point.column, 10);
}

#[test]
fn test_range_creation_and_operations() {
    let range = Range::new(
        Position::new(10, Point::new(0, 10)),
        Position::new(20, Point::new(0, 20)),
    );
    
    // Test contains_byte
    assert!(range.contains_byte(10));
    assert!(range.contains_byte(15));
    assert!(range.contains_byte(19));
    assert!(!range.contains_byte(20));
    assert!(!range.contains_byte(5));
    
    // Test byte_len
    assert_eq!(range.byte_len(), 10);
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
    
    assert!(range1.overlaps(&range2));
    assert!(range2.overlaps(&range1));
    
    let range3 = Range::new(
        Position::new(25, Point::new(0, 25)),
        Position::new(30, Point::new(0, 30)),
    );
    
    assert!(!range1.overlaps(&range3));
    assert!(!range3.overlaps(&range1));
}

// ============================================================================
// Edit Operation Tests
// ============================================================================

#[test]
fn test_edit_insert_operation() {
    let edit = Edit::insert(10, Point::new(0, 10), "hello");
    
    assert_eq!(edit.start_byte, 10);
    assert_eq!(edit.old_end_byte, 10);
    assert_eq!(edit.new_end_byte, 15);
    assert_eq!(edit.old_len(), 0);
    assert_eq!(edit.new_len(), 5);
    assert_eq!(edit.byte_delta(), 5);
}

#[test]
fn test_edit_delete_operation() {
    let edit = Edit::delete(10, 15, Point::new(0, 10), Point::new(0, 15));
    
    assert_eq!(edit.start_byte, 10);
    assert_eq!(edit.old_end_byte, 15);
    assert_eq!(edit.new_end_byte, 10);
    assert_eq!(edit.old_len(), 5);
    assert_eq!(edit.new_len(), 0);
    assert_eq!(edit.byte_delta(), -5);
}

#[test]
fn test_edit_replace_operation() {
    let edit = Edit::replace(10, 15, Point::new(0, 10), Point::new(0, 15), "world");
    
    assert_eq!(edit.start_byte, 10);
    assert_eq!(edit.old_end_byte, 15);
    assert_eq!(edit.new_end_byte, 15);
    assert_eq!(edit.old_len(), 5);
    assert_eq!(edit.new_len(), 5);
    assert_eq!(edit.byte_delta(), 0);
}

#[test]
fn test_edit_multiline_insert() {
    let text = "line1\nline2\nline3";
    let edit = Edit::insert(0, Point::zero(), text);
    
    assert_eq!(edit.new_len(), text.len());
    assert_eq!(edit.new_end_point.row, 2); // 3 lines = row 2
}

#[test]
fn test_edit_with_unicode() {
    let text = "你好世界 🚀";
    let edit = Edit::insert(0, Point::zero(), text);
    
    assert_eq!(edit.new_len(), text.len());
    assert!(edit.new_len() > 10); // Unicode takes more bytes
}

// ============================================================================
// Point Advancement Tests
// ============================================================================

#[test]
fn test_point_zero() {
    let point = Point::zero();
    assert_eq!(point.row, 0);
    assert_eq!(point.column, 0);
}

#[test]
fn test_point_creation() {
    let point = Point::new(5, 10);
    assert_eq!(point.row, 5);
    assert_eq!(point.column, 10);
}

// ============================================================================
// Complex Edit Scenarios
// ============================================================================

#[test]
fn test_edit_sequence_tracking() {
    // Simulate a sequence of edits
    let edits = vec![
        Edit::insert(0, Point::zero(), "grammar Test;\n"),
        Edit::insert(14, Point::new(1, 0), "rule: 'a';\n"),
        Edit::insert(25, Point::new(2, 0), "rule2: 'b';"),
    ];
    
    assert_eq!(edits.len(), 3);
    assert!(edits.iter().all(|e| e.byte_delta() > 0));
}

#[test]
fn test_edit_delete_and_insert() {
    // Delete then insert (like a replace operation)
    let delete = Edit::delete(10, 15, Point::new(0, 10), Point::new(0, 15));
    let insert = Edit::insert(10, Point::new(0, 10), "new");
    
    assert_eq!(delete.byte_delta(), -5);
    assert_eq!(insert.byte_delta(), 3);
    
    // Net change
    let net_delta = delete.byte_delta() + insert.byte_delta();
    assert_eq!(net_delta, -2);
}

#[test]
fn test_large_edit_operations() {
    let large_text = "a".repeat(10000);
    let edit = Edit::insert(0, Point::zero(), &large_text);
    
    assert_eq!(edit.new_len(), 10000);
    assert_eq!(edit.byte_delta(), 10000);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_edit_at_document_start() {
    let edit = Edit::insert(0, Point::zero(), "// Comment\n");
    assert_eq!(edit.start_byte, 0);
    assert_eq!(edit.start_point.row, 0);
    assert_eq!(edit.start_point.column, 0);
}

#[test]
fn test_edit_empty_text() {
    let edit = Edit::insert(10, Point::new(0, 10), "");
    assert_eq!(edit.new_len(), 0);
    assert_eq!(edit.byte_delta(), 0);
}

#[test]
fn test_delete_zero_length() {
    let edit = Edit::delete(10, 10, Point::new(0, 10), Point::new(0, 10));
    assert_eq!(edit.old_len(), 0);
    assert_eq!(edit.byte_delta(), 0);
}

#[test]
fn test_range_zero_length() {
    let range = Range::new(
        Position::new(10, Point::new(0, 10)),
        Position::new(10, Point::new(0, 10)),
    );
    
    assert_eq!(range.byte_len(), 0);
    assert!(!range.contains_byte(10)); // End is exclusive
}

// ============================================================================
// Position Tracking with Real Grammar
// ============================================================================

#[test]
fn test_position_tracking_simple_grammar() {
    // Verify we can create positions for a simple grammar
    let source = "grammar Test;\nrule: 'a';";
    
    // Position at start
    let start = Position::new(0, Point::zero());
    assert_eq!(start.byte, 0);
    
    // Position at "rule"
    let rule_pos = Position::new(14, Point::new(1, 0));
    assert_eq!(rule_pos.byte, 14);
    assert_eq!(rule_pos.point.row, 1);
    
    // Position at end
    let end = Position::new(source.len(), Point::new(1, 11));
    assert_eq!(end.byte, source.len());
}

#[test]
fn test_range_covering_grammar_element() {
    // Create a range covering "rule: 'a';"
    let range = Range::new(
        Position::new(14, Point::new(1, 0)),
        Position::new(24, Point::new(1, 10)),
    );
    
    assert_eq!(range.byte_len(), 10);
    assert!(range.contains_byte(14));
    assert!(range.contains_byte(20));
    assert!(!range.contains_byte(24));
}
