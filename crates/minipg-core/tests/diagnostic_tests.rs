//! Diagnostic tests.

use minipg_core::{Diagnostic, DiagnosticSeverity, Location};

#[test]
fn test_location_display() {
    let loc = Location::new("test.g4", 10, 5);
    assert_eq!(loc.to_string(), "test.g4:10:5");
}

#[test]
fn test_error_diagnostic() {
    let diag = Diagnostic::error("test error");
    assert_eq!(diag.severity, DiagnosticSeverity::Error);
    assert_eq!(diag.message, "test error");
    assert!(diag.location.is_none());
}

#[test]
fn test_warning_diagnostic() {
    let diag = Diagnostic::warning("test warning");
    assert_eq!(diag.severity, DiagnosticSeverity::Warning);
    assert_eq!(diag.message, "test warning");
}

#[test]
fn test_diagnostic_with_location() {
    let loc = Location::new("test.g4", 10, 5);
    let diag = Diagnostic::error("test error").with_location(loc);
    assert!(diag.location.is_some());
    assert!(diag.to_string().contains("test.g4:10:5"));
}

#[test]
fn test_diagnostic_with_code() {
    let diag = Diagnostic::error("test error").with_code("E001");
    assert_eq!(diag.code, Some("E001".to_string()));
    assert!(diag.to_string().contains("[E001]"));
}

#[test]
fn test_diagnostic_display() {
    let diag = Diagnostic::error("test error");
    let display = diag.to_string();
    assert!(display.contains("error"));
    assert!(display.contains("test error"));
}
