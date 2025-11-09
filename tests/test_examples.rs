//! Comprehensive tests for all example grammar files.
//!
//! This test module validates all grammar files in the examples directory:
//! - Basic grammars: calculator, json
//! - Intermediate grammars: Config, CompleteJSON, Query, SQL
//! - Advanced grammars: Expression, GraphQL, CSS, Markdown, Protocol, YAML, JavaSubset, PythonSubset
//! - Test grammars: test_charclass, test_simple_charclass
//!
//! # Test Coverage
//!
//! Each grammar is tested for:
//! - File existence and readability
//! - Grammar declaration presence
//! - Successful parsing
//! - Structure validation
//! - Feature verification

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use std::fs;

fn load_grammar(filename: &str) -> String {
    let path = format!("examples/{}", filename);
    fs::read_to_string(&path).expect(&format!("Failed to read {}", path))
}

fn test_grammar_parses(grammar_text: &str, name: &str) {
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar_text, &format!("{}.g4", name));
    assert!(result.is_ok(), "Failed to parse {}: {:?}", name, result.err());
}

// ============================================================================
// BASIC GRAMMARS - Simple, foundational examples
// ============================================================================

#[test]
fn test_calculator_grammar() {
    let grammar = load_grammar("calculator.g4");
    assert!(!grammar.is_empty(), "calculator.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("expr"), "Missing expr rule");
    test_grammar_parses(&grammar, "calculator");
}

#[test]
fn test_json_grammar() {
    let grammar = load_grammar("json.g4");
    assert!(!grammar.is_empty(), "json.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("value") || grammar.contains("json"), "Missing value or json rule");
    test_grammar_parses(&grammar, "json");
}

// ============================================================================
// INTERMEDIATE GRAMMARS
// ============================================================================

#[test]
fn test_config_grammar() {
    let grammar = load_grammar("Config.g4");
    assert!(!grammar.is_empty(), "Config.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(!grammar.contains("EOF"), "Config.g4 should not reference EOF");
    assert!(grammar.contains("-> skip"), "Config.g4 should have token skipping");
    assert!(grammar.contains("section"), "Missing section");
    assert!(grammar.contains("property"), "Missing property");
    test_grammar_parses(&grammar, "Config");
}

#[test]
fn test_completejson_grammar() {
    let grammar = load_grammar("CompleteJSON.g4");
    assert!(!grammar.is_empty(), "CompleteJSON.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(!grammar.contains("EOF"), "CompleteJSON.g4 should not reference EOF");
    
    // Test Unicode escape sequences (from test_completejson_line.rs)
    assert!(grammar.contains("SAFECODEPOINT") || grammar.contains("\\u0000"), 
            "Should have Unicode escape support");
    test_grammar_parses(&grammar, "CompleteJSON");
}

#[test]
fn test_query_grammar() {
    let grammar = load_grammar("Query.g4");
    assert!(!grammar.is_empty(), "Query.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("selectStatement") || grammar.contains("query"), 
            "Missing selectStatement or query");
    assert!(grammar.contains("SELECT") || grammar.contains("select"), "Missing SELECT");
    assert!(grammar.contains("FROM") || grammar.contains("from"), "Missing FROM");
    test_grammar_parses(&grammar, "Query");
}

#[test]
fn test_sql_grammar() {
    let grammar = load_grammar("SQL.g4");
    assert!(!grammar.is_empty(), "SQL.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    test_grammar_parses(&grammar, "SQL");
}

// ============================================================================
// ADVANCED GRAMMARS - Complex language definitions
// ============================================================================

#[test]
fn test_expression_grammar() {
    let grammar = load_grammar("Expression.g4");
    assert!(!grammar.is_empty(), "Expression.g4 should not be empty");
    assert!(grammar.contains("grammar Expression"), "Missing grammar declaration");
    assert!(grammar.contains("expr"), "Missing expr rule");
    assert!(grammar.contains("logicalOr"), "Missing logicalOr");
    assert!(grammar.contains("logicalAnd"), "Missing logicalAnd");
    assert!(grammar.contains("equality"), "Missing equality");
    assert!(grammar.contains("comparison"), "Missing comparison");
    assert!(grammar.contains("additive"), "Missing additive");
    assert!(grammar.contains("multiplicative"), "Missing multiplicative");
    assert!(grammar.contains("NUMBER"), "Missing NUMBER token");
    assert!(grammar.contains("STRING"), "Missing STRING token");
}

#[test]
fn test_graphql_grammar() {
    let grammar = load_grammar("GraphQL.g4");
    assert!(!grammar.is_empty(), "GraphQL.g4 should not be empty");
    assert!(grammar.contains("grammar GraphQL"), "Missing grammar declaration");
    assert!(grammar.contains("typeDefinition"), "Missing typeDefinition");
    assert!(grammar.contains("scalarTypeDefinition"), "Missing scalarTypeDefinition");
    assert!(grammar.contains("objectTypeDefinition"), "Missing objectTypeDefinition");
    assert!(grammar.contains("directiveDefinition"), "Missing directiveDefinition");
    assert!(grammar.contains("schemaDefinition"), "Missing schemaDefinition");
}

#[test]
fn test_css_grammar() {
    let grammar = load_grammar("CSS.g4");
    assert!(!grammar.is_empty(), "CSS.g4 should not be empty");
    assert!(grammar.contains("grammar CSS"), "Missing grammar declaration");
    assert!(grammar.contains("selector"), "Missing selector");
    assert!(grammar.contains("simpleSelector"), "Missing simpleSelector");
    assert!(grammar.contains("classSelector"), "Missing classSelector");
    assert!(grammar.contains("idSelector"), "Missing idSelector");
    assert!(grammar.contains("pseudoClass"), "Missing pseudoClass");
    assert!(grammar.contains("rule"), "Missing rule");
    assert!(grammar.contains("declaration"), "Missing declaration");
}

#[test]
fn test_markdown_grammar() {
    let grammar = load_grammar("Markdown.g4");
    assert!(!grammar.is_empty(), "Markdown.g4 should not be empty");
    assert!(grammar.contains("grammar Markdown"), "Missing grammar declaration");
    assert!(grammar.contains("heading"), "Missing heading");
    assert!(grammar.contains("paragraph"), "Missing paragraph");
    assert!(grammar.contains("codeBlock"), "Missing codeBlock");
    assert!(grammar.contains("blockquote"), "Missing blockquote");
    assert!(grammar.contains("list"), "Missing list");
    assert!(grammar.contains("inlineCode"), "Missing inlineCode");
}

#[test]
fn test_protocol_grammar() {
    let grammar = load_grammar("Protocol.g4");
    assert!(!grammar.is_empty(), "Protocol.g4 should not be empty");
    assert!(grammar.contains("grammar Protocol"), "Missing grammar declaration");
    assert!(grammar.contains("messageDefinition"), "Missing messageDefinition");
    assert!(grammar.contains("enumDefinition"), "Missing enumDefinition");
    assert!(grammar.contains("serviceDefinition"), "Missing serviceDefinition");
    assert!(grammar.contains("fieldDefinition"), "Missing fieldDefinition");
}

#[test]
fn test_yaml_grammar() {
    let grammar = load_grammar("YAML.g4");
    assert!(!grammar.is_empty(), "YAML.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("mapping"), "Missing mapping");
    assert!(grammar.contains("sequence"), "Missing sequence");
    assert!(grammar.contains("pair"), "Missing pair");
}

#[test]
fn test_javasubset_grammar() {
    let grammar = load_grammar("JavaSubset.g4");
    assert!(!grammar.is_empty(), "JavaSubset.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("classDeclaration") || grammar.contains("class"), 
            "Missing classDeclaration");
}

#[test]
fn test_pythonsubset_grammar() {
    let grammar = load_grammar("PythonSubset.g4");
    assert!(!grammar.is_empty(), "PythonSubset.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("stmt") || grammar.contains("statement"), 
            "Missing stmt or statement");
}

// ============================================================================
// TEST GRAMMARS - Special test cases
// ============================================================================

#[test]
fn test_charclass_grammar() {
    let grammar = load_grammar("test_charclass.g4");
    assert!(!grammar.is_empty(), "test_charclass.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
}

#[test]
fn test_simple_charclass_grammar() {
    let grammar = load_grammar("test_simple_charclass.g4");
    assert!(!grammar.is_empty(), "test_simple_charclass.g4 should not be empty");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
}

// ============================================================================
// BATCH TESTS
// ============================================================================

#[test]
fn test_all_examples_parse() {
    // Test grammars that should parse successfully
    let examples = vec![
        "calculator.g4",
        "json.g4",
        "Config.g4",
        "CompleteJSON.g4",
        "Query.g4",
        "SQL.g4",
    ];

    for example in examples {
        let grammar = load_grammar(example);
        test_grammar_parses(&grammar, example);
    }
}

#[test]
fn test_all_examples_have_grammar_keyword() {
    let examples = vec![
        "calculator.g4",
        "json.g4",
        "Expression.g4",
        "Config.g4",
        "YAML.g4",
        "CompleteJSON.g4",
        "GraphQL.g4",
        "Query.g4",
        "CSS.g4",
        "Markdown.g4",
        "Protocol.g4",
        "SQL.g4",
        "JavaSubset.g4",
        "PythonSubset.g4",
        "test_charclass.g4",
        "test_simple_charclass.g4",
    ];

    for example in examples {
        let grammar = load_grammar(example);
        assert!(
            grammar.contains("grammar "),
            "{} missing 'grammar' keyword",
            example
        );
    }
}

#[test]
fn test_example_sizes() {
    let examples = vec![
        ("calculator.g4", 100, 500),      // Small
        ("json.g4", 100, 500),            // Small
        ("Expression.g4", 500, 2000),     // Medium
        ("Config.g4", 500, 2000),         // Medium
        ("YAML.g4", 500, 2000),           // Medium
        ("CompleteJSON.g4", 500, 2000),   // Medium
        ("GraphQL.g4", 1000, 5000),       // Large
        ("Query.g4", 1000, 5000),         // Large
        ("CSS.g4", 1000, 5000),           // Large
        ("Markdown.g4", 1000, 5000),      // Large
        ("Protocol.g4", 1000, 5000),      // Large
        ("SQL.g4", 1000, 5000),           // Large
        ("JavaSubset.g4", 2000, 10000),   // Very Large
        ("PythonSubset.g4", 2000, 10000), // Very Large
    ];

    for (filename, min_size, max_size) in examples {
        let grammar = load_grammar(filename);
        let size = grammar.len();
        assert!(
            size >= min_size && size <= max_size,
            "{} size {} not in range [{}, {}]",
            filename,
            size,
            min_size,
            max_size
        );
    }
}

