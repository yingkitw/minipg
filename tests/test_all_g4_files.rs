//! Comprehensive tests for all g4 grammar files.
//!
//! This test module validates all 17 grammar files in the examples directory:
//! - Basic grammars (2): calculator, json
//! - Fixed grammars (2): Config, CompleteJSON
//! - Query grammars (1): Query
//! - Advanced grammars (8): Expression, GraphQL, CSS, Markdown, Protocol, YAML, JavaSubset, PythonSubset
//! - SQL grammar (1): SQL
//! - Test grammars (2): test_charclass, test_simple_charclass
//!
//! # Test Coverage
//!
//! Each grammar is tested for:
//! - File existence and readability
//! - Grammar declaration presence
//! - Token definitions
//! - Parser rules
//! - Proper structure

use std::fs;

fn load_grammar(filename: &str) -> String {
    let path = format!("examples/{}", filename);
    fs::read_to_string(&path).expect(&format!("Failed to read {}", path))
}

// ============================================================================
// BASIC GRAMMARS - Simple, foundational examples
// ============================================================================

#[test]
fn test_calculator_g4_exists() {
    let grammar = load_grammar("calculator.g4");
    assert!(!grammar.is_empty(), "calculator.g4 should not be empty");
}

#[test]
fn test_calculator_g4_structure() {
    let grammar = load_grammar("calculator.g4");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("expr"), "Missing expr rule");
}

#[test]
fn test_json_g4_exists() {
    let grammar = load_grammar("json.g4");
    assert!(!grammar.is_empty(), "json.g4 should not be empty");
}

#[test]
fn test_json_g4_structure() {
    let grammar = load_grammar("json.g4");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("value") || grammar.contains("json"), "Missing value or json rule");
}

// ============================================================================
// FIXED GRAMMARS - Previously fixed issues
// ============================================================================

#[test]
fn test_config_g4_exists() {
    let grammar = load_grammar("Config.g4");
    assert!(!grammar.is_empty(), "Config.g4 should not be empty");
}

#[test]
fn test_config_g4_no_eof() {
    let grammar = load_grammar("Config.g4");
    // Verify EOF issue is fixed
    assert!(!grammar.contains("EOF"), "Config.g4 should not reference EOF");
}

#[test]
fn test_config_g4_has_skip() {
    let grammar = load_grammar("Config.g4");
    // Verify token skipping is implemented
    assert!(grammar.contains("-> skip"), "Config.g4 should have token skipping");
}

#[test]
fn test_completejson_g4_exists() {
    let grammar = load_grammar("CompleteJSON.g4");
    assert!(!grammar.is_empty(), "CompleteJSON.g4 should not be empty");
}

#[test]
fn test_completejson_g4_no_eof() {
    let grammar = load_grammar("CompleteJSON.g4");
    // Verify EOF issue is fixed
    assert!(!grammar.contains("EOF"), "CompleteJSON.g4 should not reference EOF");
}

// ============================================================================
// QUERY GRAMMAR - SQL-like query language
// ============================================================================

#[test]
fn test_query_g4_exists() {
    let grammar = load_grammar("Query.g4");
    assert!(!grammar.is_empty(), "Query.g4 should not be empty");
}

#[test]
fn test_query_g4_structure() {
    let grammar = load_grammar("Query.g4");
    assert!(grammar.contains("grammar"), "Missing grammar declaration");
    assert!(grammar.contains("query") || grammar.contains("select"), "Missing query or select rule");
}

#[test]
fn test_query_g4_has_sql_features() {
    let grammar = load_grammar("Query.g4");
    assert!(grammar.contains("SELECT") || grammar.contains("select"), "Missing SELECT");
    assert!(grammar.contains("FROM") || grammar.contains("from"), "Missing FROM");
}

// ============================================================================
// ADVANCED GRAMMARS - Complex language definitions
// ============================================================================

#[test]
fn test_expression_g4_exists() {
    let grammar = load_grammar("Expression.g4");
    assert!(!grammar.is_empty(), "Expression.g4 should not be empty");
}

#[test]
fn test_expression_g4_has_operators() {
    let grammar = load_grammar("Expression.g4");
    assert!(grammar.contains("expr"), "Missing expr rule");
    assert!(grammar.contains("+") || grammar.contains("PLUS"), "Missing addition operator");
}

#[test]
fn test_graphql_g4_exists() {
    let grammar = load_grammar("GraphQL.g4");
    assert!(!grammar.is_empty(), "GraphQL.g4 should not be empty");
}

#[test]
fn test_graphql_g4_has_types() {
    let grammar = load_grammar("GraphQL.g4");
    assert!(grammar.contains("type") || grammar.contains("Type"), "Missing type");
}

#[test]
fn test_css_g4_exists() {
    let grammar = load_grammar("CSS.g4");
    assert!(!grammar.is_empty(), "CSS.g4 should not be empty");
}

#[test]
fn test_css_g4_has_selectors() {
    let grammar = load_grammar("CSS.g4");
    assert!(grammar.contains("selector") || grammar.contains("Selector"), "Missing selector");
}

#[test]
fn test_markdown_g4_exists() {
    let grammar = load_grammar("Markdown.g4");
    assert!(!grammar.is_empty(), "Markdown.g4 should not be empty");
}

#[test]
fn test_markdown_g4_has_blocks() {
    let grammar = load_grammar("Markdown.g4");
    assert!(grammar.contains("heading") || grammar.contains("Heading"), "Missing heading");
}

#[test]
fn test_protocol_g4_exists() {
    let grammar = load_grammar("Protocol.g4");
    assert!(!grammar.is_empty(), "Protocol.g4 should not be empty");
}

#[test]
fn test_protocol_g4_has_messages() {
    let grammar = load_grammar("Protocol.g4");
    assert!(grammar.contains("message") || grammar.contains("Message"), "Missing message");
}

#[test]
fn test_yaml_g4_exists() {
    let grammar = load_grammar("YAML.g4");
    assert!(!grammar.is_empty(), "YAML.g4 should not be empty");
}

#[test]
fn test_yaml_g4_has_structures() {
    let grammar = load_grammar("YAML.g4");
    assert!(grammar.contains("mapping") || grammar.contains("Mapping"), "Missing mapping");
}

#[test]
fn test_javasubset_g4_exists() {
    let grammar = load_grammar("JavaSubset.g4");
    assert!(!grammar.is_empty(), "JavaSubset.g4 should not be empty");
}

#[test]
fn test_javasubset_g4_has_classes() {
    let grammar = load_grammar("JavaSubset.g4");
    assert!(grammar.contains("class") || grammar.contains("Class"), "Missing class");
}

#[test]
fn test_pythonsubset_g4_exists() {
    let grammar = load_grammar("PythonSubset.g4");
    assert!(!grammar.is_empty(), "PythonSubset.g4 should not be empty");
}

#[test]
fn test_pythonsubset_g4_has_definitions() {
    let grammar = load_grammar("PythonSubset.g4");
    assert!(grammar.contains("def") || grammar.contains("class"), "Missing def or class");
}

// ============================================================================
// SQL GRAMMAR - Structured Query Language
// ============================================================================

#[test]
fn test_sql_g4_exists() {
    let grammar = load_grammar("SQL.g4");
    assert!(!grammar.is_empty(), "SQL.g4 should not be empty");
}

#[test]
fn test_sql_g4_has_sql_features() {
    let grammar = load_grammar("SQL.g4");
    assert!(grammar.contains("SELECT") || grammar.contains("select"), "Missing SELECT");
    assert!(grammar.contains("FROM") || grammar.contains("from"), "Missing FROM");
}

// ============================================================================
// TEST GRAMMARS - Utility grammars for testing
// ============================================================================

#[test]
fn test_charclass_g4_exists() {
    let grammar = load_grammar("test_charclass.g4");
    assert!(!grammar.is_empty(), "test_charclass.g4 should not be empty");
}

#[test]
fn test_charclass_g4_has_charclass() {
    let grammar = load_grammar("test_charclass.g4");
    assert!(grammar.contains("[") && grammar.contains("]"), "Missing character class");
}

#[test]
fn test_simple_charclass_g4_exists() {
    let grammar = load_grammar("test_simple_charclass.g4");
    assert!(!grammar.is_empty(), "test_simple_charclass.g4 should not be empty");
}

// ============================================================================
// BATCH TESTS - All grammars together
// ============================================================================

#[test]
fn test_all_grammars_exist() {
    let grammars = vec![
        "calculator.g4",
        "json.g4",
        "Config.g4",
        "CompleteJSON.g4",
        "Query.g4",
        "Expression.g4",
        "GraphQL.g4",
        "CSS.g4",
        "Markdown.g4",
        "Protocol.g4",
        "YAML.g4",
        "JavaSubset.g4",
        "PythonSubset.g4",
        "SQL.g4",
        "test_charclass.g4",
        "test_simple_charclass.g4",
    ];

    for grammar_file in grammars {
        let grammar = load_grammar(grammar_file);
        assert!(!grammar.is_empty(), "{} should not be empty", grammar_file);
    }
}

#[test]
fn test_all_grammars_have_declaration() {
    let grammars = vec![
        "calculator.g4",
        "json.g4",
        "Config.g4",
        "CompleteJSON.g4",
        "Query.g4",
        "Expression.g4",
        "GraphQL.g4",
        "CSS.g4",
        "Markdown.g4",
        "Protocol.g4",
        "YAML.g4",
        "JavaSubset.g4",
        "PythonSubset.g4",
        "SQL.g4",
        "test_charclass.g4",
        "test_simple_charclass.g4",
    ];

    for grammar_file in grammars {
        let grammar = load_grammar(grammar_file);
        assert!(grammar.contains("grammar "), "{} missing grammar declaration", grammar_file);
    }
}

#[test]
fn test_all_grammars_have_rules() {
    let grammars = vec![
        "calculator.g4",
        "json.g4",
        "Config.g4",
        "CompleteJSON.g4",
        "Query.g4",
        "Expression.g4",
        "GraphQL.g4",
        "CSS.g4",
        "Markdown.g4",
        "Protocol.g4",
        "YAML.g4",
        "JavaSubset.g4",
        "PythonSubset.g4",
        "SQL.g4",
        "test_charclass.g4",
        "test_simple_charclass.g4",
    ];

    for grammar_file in grammars {
        let grammar = load_grammar(grammar_file);
        // Check for lowercase identifiers (parser rules)
        let has_rule = grammar.lines().any(|line| {
            let trimmed = line.trim();
            // Rule can be "name:" or just "name" on its own line
            trimmed.starts_with(|c: char| c.is_lowercase() && c.is_alphabetic())
        });
        assert!(has_rule, "{} missing parser rules", grammar_file);
    }
}

#[test]
fn test_all_grammars_have_tokens() {
    let grammars = vec![
        "calculator.g4",
        "json.g4",
        "Config.g4",
        "CompleteJSON.g4",
        "Query.g4",
        "Expression.g4",
        "GraphQL.g4",
        "CSS.g4",
        "Markdown.g4",
        "Protocol.g4",
        "YAML.g4",
        "JavaSubset.g4",
        "PythonSubset.g4",
        "SQL.g4",
        "test_charclass.g4",
        "test_simple_charclass.g4",
    ];

    for grammar_file in grammars {
        let grammar = load_grammar(grammar_file);
        // Check for uppercase identifiers (tokens)
        let has_token = grammar.lines().any(|line| {
            let trimmed = line.trim();
            // Token can be "NAME:" or just "NAME" on its own line
            trimmed.starts_with(|c: char| c.is_uppercase() && c.is_alphabetic())
        });
        assert!(has_token, "{} missing token definitions", grammar_file);
    }
}

#[test]
fn test_grammar_file_count() {
    // Verify we have 16 grammar files (excluding README.md)
    let entries = fs::read_dir("examples").expect("Failed to read examples directory");
    let g4_files: Vec<_> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().map_or(false, |ext| ext == "g4") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    assert!(g4_files.len() >= 16, "Expected at least 16 g4 files, found {}", g4_files.len());
}

#[test]
fn test_grammar_sizes() {
    let grammars = vec![
        ("calculator.g4", 100, 500),
        ("json.g4", 100, 500),
        ("Config.g4", 200, 1000),
        ("CompleteJSON.g4", 200, 1000),
        ("Query.g4", 500, 5000),
        ("Expression.g4", 500, 5000),
        ("GraphQL.g4", 1000, 10000),
        ("CSS.g4", 500, 5000),
        ("Markdown.g4", 500, 5000),
        ("Protocol.g4", 500, 5000),
        ("YAML.g4", 200, 1000),
        ("JavaSubset.g4", 1000, 10000),
        ("PythonSubset.g4", 1000, 10000),
        ("SQL.g4", 500, 5000),
        ("test_charclass.g4", 50, 500),
        ("test_simple_charclass.g4", 50, 500),
    ];

    for (filename, min_size, max_size) in grammars {
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

// ============================================================================
// DOCUMENTATION TESTS - Verify README examples
// ============================================================================

#[test]
fn test_readme_mentions_examples() {
    let readme = fs::read_to_string("examples/README.md").expect("Failed to read examples/README.md");
    assert!(!readme.is_empty(), "examples/README.md should not be empty");
    assert!(readme.contains("grammar"), "README should mention grammars");
}

#[test]
fn test_all_grammars_documented() {
    let readme = fs::read_to_string("examples/README.md").expect("Failed to read examples/README.md");
    
    let grammars = vec![
        "calculator",
        "json",
        "Config",
        "CompleteJSON",
        "Query",
        "Expression",
        "GraphQL",
        "CSS",
        "Markdown",
        "Protocol",
        "YAML",
        "JavaSubset",
        "PythonSubset",
        "SQL",
    ];

    for grammar in grammars {
        assert!(readme.contains(grammar), "README should mention {}", grammar);
    }
}
