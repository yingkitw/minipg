//! Error handling tests.

use minipg_core::Error;

#[test]
fn test_parse_error() {
    let err = Error::parse("file.g4:10:5", "unexpected token");
    assert!(matches!(err, Error::Parse { .. }));
    assert!(err.to_string().contains("Parse error"));
    assert!(err.to_string().contains("file.g4:10:5"));
}

#[test]
fn test_semantic_error() {
    let err = Error::semantic("undefined rule");
    assert!(matches!(err, Error::Semantic(_)));
    assert!(err.to_string().contains("Semantic error"));
}

#[test]
fn test_codegen_error() {
    let err = Error::codegen("failed to generate code");
    assert!(matches!(err, Error::CodeGen(_)));
    assert!(err.to_string().contains("Code generation error"));
}

#[test]
fn test_invalid_grammar_error() {
    let err = Error::invalid_grammar("empty grammar");
    assert!(matches!(err, Error::InvalidGrammar(_)));
    assert!(err.to_string().contains("Invalid grammar"));
}

#[test]
fn test_internal_error() {
    let err = Error::internal("unexpected state");
    assert!(matches!(err, Error::Internal(_)));
    assert!(err.to_string().contains("Internal error"));
}

#[test]
fn test_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = Error::from(io_err);
    assert!(matches!(err, Error::Io(_)));
}
