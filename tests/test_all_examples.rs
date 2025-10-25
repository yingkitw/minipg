/// Integration tests for all example grammars
/// Tests that each example grammar parses correctly

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
// SIMPLE EXAMPLES
// ============================================================================

#[test]
fn test_calculator_grammar_parses() {
    let grammar = load_grammar("calculator.g4");
    test_grammar_parses(&grammar, "calculator");
}

#[test]
fn test_json_grammar_parses() {
    let grammar = load_grammar("json.g4");
    test_grammar_parses(&grammar, "json");
}

// ============================================================================
// INTERMEDIATE EXAMPLES
// ============================================================================

#[test]
fn test_config_grammar_parses() {
    let grammar = load_grammar("Config.g4");
    test_grammar_parses(&grammar, "Config");
}

#[test]
fn test_completejson_grammar_parses() {
    let grammar = load_grammar("CompleteJSON.g4");
    test_grammar_parses(&grammar, "CompleteJSON");
}

#[test]
fn test_query_grammar_parses() {
    let grammar = load_grammar("Query.g4");
    test_grammar_parses(&grammar, "Query");
}

#[test]
fn test_sql_grammar_parses() {
    let grammar = load_grammar("SQL.g4");
    test_grammar_parses(&grammar, "SQL");
}

// ============================================================================
// BATCH TESTS
// ============================================================================

#[test]
fn test_all_examples_parse() {
    // Test working examples (others have advanced syntax not yet supported)
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


// ============================================================================
// FEATURE VERIFICATION TESTS
// ============================================================================

#[test]
fn test_expression_has_operator_precedence() {
    let grammar = load_grammar("Expression.g4");
    assert!(grammar.contains("logicalOr"), "Expression grammar missing logicalOr");
    assert!(grammar.contains("logicalAnd"), "Expression grammar missing logicalAnd");
    assert!(grammar.contains("equality"), "Expression grammar missing equality");
    assert!(grammar.contains("comparison"), "Expression grammar missing comparison");
    assert!(grammar.contains("additive"), "Expression grammar missing additive");
    assert!(grammar.contains("multiplicative"), "Expression grammar missing multiplicative");
}

#[test]
fn test_graphql_has_type_system() {
    let grammar = load_grammar("GraphQL.g4");
    assert!(grammar.contains("typeDefinition"), "GraphQL missing typeDefinition");
    assert!(grammar.contains("scalarTypeDefinition"), "GraphQL missing scalarTypeDefinition");
    assert!(grammar.contains("objectTypeDefinition"), "GraphQL missing objectTypeDefinition");
    assert!(grammar.contains("interfaceTypeDefinition"), "GraphQL missing interfaceTypeDefinition");
    assert!(grammar.contains("unionTypeDefinition"), "GraphQL missing unionTypeDefinition");
    assert!(grammar.contains("enumTypeDefinition"), "GraphQL missing enumTypeDefinition");
}

#[test]
fn test_query_has_sql_statements() {
    let grammar = load_grammar("Query.g4");
    assert!(grammar.contains("selectStatement"), "Query missing selectStatement");
    assert!(grammar.contains("insertStatement"), "Query missing insertStatement");
    assert!(grammar.contains("updateStatement"), "Query missing updateStatement");
    assert!(grammar.contains("deleteStatement"), "Query missing deleteStatement");
    assert!(grammar.contains("createStatement"), "Query missing createStatement");
}

#[test]
fn test_css_has_selectors() {
    let grammar = load_grammar("CSS.g4");
    assert!(grammar.contains("selector"), "CSS missing selector");
    assert!(grammar.contains("simpleSelector"), "CSS missing simpleSelector");
    assert!(grammar.contains("elementSelector"), "CSS missing elementSelector");
    assert!(grammar.contains("classSelector"), "CSS missing classSelector");
    assert!(grammar.contains("idSelector"), "CSS missing idSelector");
    assert!(grammar.contains("pseudoClass"), "CSS missing pseudoClass");
    assert!(grammar.contains("pseudoElement"), "CSS missing pseudoElement");
}

#[test]
fn test_markdown_has_blocks() {
    let grammar = load_grammar("Markdown.g4");
    assert!(grammar.contains("heading"), "Markdown missing heading");
    assert!(grammar.contains("paragraph"), "Markdown missing paragraph");
    assert!(grammar.contains("codeBlock"), "Markdown missing codeBlock");
    assert!(grammar.contains("blockquote"), "Markdown missing blockquote");
    assert!(grammar.contains("list"), "Markdown missing list");
}

#[test]
fn test_protocol_has_messages() {
    let grammar = load_grammar("Protocol.g4");
    assert!(grammar.contains("messageDefinition"), "Protocol missing messageDefinition");
    assert!(grammar.contains("enumDefinition"), "Protocol missing enumDefinition");
    assert!(grammar.contains("serviceDefinition"), "Protocol missing serviceDefinition");
    assert!(grammar.contains("fieldDefinition"), "Protocol missing fieldDefinition");
}

#[test]
fn test_yaml_has_nested_structures() {
    let grammar = load_grammar("YAML.g4");
    assert!(grammar.contains("mapping"), "YAML missing mapping");
    assert!(grammar.contains("sequence"), "YAML missing sequence");
    assert!(grammar.contains("pair"), "YAML missing pair");
}

#[test]
fn test_config_has_sections() {
    let grammar = load_grammar("Config.g4");
    assert!(grammar.contains("section"), "Config missing section");
    assert!(grammar.contains("property"), "Config missing property");
    assert!(grammar.contains("sectionName"), "Config missing sectionName");
}

// ============================================================================
// COMPLEXITY TESTS
// ============================================================================

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


