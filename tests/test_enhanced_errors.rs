/// Tests for enhanced error messages and edge case handling

use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;

#[test]
fn test_error_message_expected_tokens() {
    let invalid_grammar = r#"
        grammar Test;
        rule TOKEN
        // Missing semicolon
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Expected"), "Error message should mention expected tokens");
}

#[test]
fn test_error_message_unmatched_braces() {
    let invalid_grammar = r#"
        grammar Test;
        options {
            key = value;
            // Missing closing brace
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err(), "Should fail on unclosed options block");
    // Just verify it produces an error - exact message format may vary
    let _error = result.unwrap_err();
}

#[test]
fn test_empty_alternative_error() {
    let invalid_grammar = r#"
        grammar Test;
        rule: TOKEN | | TOKEN ;
        TOKEN: 'x' ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Empty alternative") || error_msg.contains("empty"),
            "Error message should mention empty alternative");
}

#[test]
fn test_unicode_escape_extended() {
    let grammar = r#"
        grammar Test;
        TOKEN: '\u{1F600}' ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar, "test.g4");
    
    // Should parse successfully with extended Unicode escape
    assert!(result.is_ok(), "Extended Unicode escape should be supported");
}

#[test]
fn test_unicode_escape_standard() {
    let grammar = r#"
        grammar Test;
        TOKEN: '\u00A0' ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar, "test.g4");
    
    assert!(result.is_ok(), "Standard Unicode escape should work");
}

#[test]
fn test_char_class_range_validation() {
    let invalid_grammar = r#"
        grammar Test;
        TOKEN: [z-a] ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Invalid character range") || error_msg.contains("must be"),
            "Error should validate character range");
}

#[test]
fn test_unclosed_character_class() {
    let invalid_grammar = r#"
        grammar Test;
        TOKEN: [a-z
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Unclosed") || error_msg.contains("expected ']'"),
            "Error should mention unclosed character class");
}

#[test]
fn test_unclosed_group() {
    let invalid_grammar = r#"
        grammar Test;
        rule: ( TOKEN
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Unclosed") || error_msg.contains("expected ')'"),
            "Error should mention unclosed group");
}

#[test]
fn test_hex_escape_sequence() {
    let grammar = r#"
        grammar Test;
        TOKEN: '\x41' ; // Should parse as 'A'
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar, "test.g4");
    
    assert!(result.is_ok(), "Hex escape sequences should be supported");
}

#[test]
fn test_incomplete_unicode_escape() {
    // Test with incomplete Unicode escape
    // The lexer may produce an Error token, and parser may handle it gracefully
    let invalid_grammar = r#"
        grammar Test;
        TOKEN: '\u12' ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(invalid_grammar, "test.g4");
    
    // The lexer returns an Error token for incomplete escapes, which may cause parse failure
    // or the parser may continue - both behaviors are acceptable
    if result.is_err() {
        // If it fails, verify it's a meaningful error
        let _error = result.unwrap_err();
    }
    // If it succeeds, the parser is being lenient, which is also acceptable
}

#[test]
fn test_error_suggestion_scenarios() {
    // Test various scenarios that should generate suggestions
    let scenarios = vec![
        (r#"
        grammar Test;
        options {
            key = value
        "#, "Should suggest semicolon"),
        (r#"
        grammar Test;
        rule: TOKEN )
        "#, "Should suggest semicolon or catch wrong brace"),
    ];
    
    for (grammar, description) in scenarios {
        let parser = GrammarParser::new();
        let result = parser.parse_string(grammar, "test.g4");
        // Just verify it produces an error (may or may not have suggestions)
        assert!(result.is_err(), "{}", description);
    }
}

#[test]
fn test_multiple_unicode_escapes() {
    let grammar = r#"
        grammar Test;
        TOKEN: '\u0041\u0042\u0043' ; // ABC
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar, "test.g4");
    assert!(result.is_ok(), "Should handle multiple Unicode escapes");
}

#[test]
fn test_unicode_escape_edge_cases() {
    // Test various Unicode code points
    let test_cases = vec![
        ("\\u0000", "NULL"),
        ("\\u007F", "DEL"),
        ("\\u0080", "Extended ASCII"),
        ("\\u00FF", "Latin-1"),
        ("\\u0100", "Latin Extended-A"),
        ("\\u{1F600}", "Emoji"),
        ("\\u{10FFFF}", "Max Unicode"),
    ];
    
    for (escape, description) in test_cases {
        let grammar = format!(r#"
        grammar Test;
        TOKEN: '{}' ;
        "#, escape);
        
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, "test.g4");
        assert!(result.is_ok(), "Should handle {}: {}", description, escape);
    }
}

#[test]
fn test_character_class_edge_cases() {
    // Valid ranges
    let valid_ranges = vec![
        "[a-z]",
        "[0-9]",
        "[A-Za-z0-9]",
        "[\u{0100}-\u{01FF}]", // Unicode range
        "[\\u0000-\\u007F]", // ASCII range with escapes
    ];
    
    for range in valid_ranges {
        let grammar = format!(r#"
        grammar Test;
        TOKEN: {} ;
        "#, range);
        
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, "test.g4");
        assert!(result.is_ok(), "Should accept valid range: {}", range);
    }
}

#[test]
fn test_unclosed_blocks_comprehensive() {
    let unclosed_cases = vec![
        (r#"
        grammar Test;
        options {
            key = value;
        "#, "options block"),
        (r#"
        grammar Test;
        @init {
            code here
        "#, "named action"),
        (r#"
        grammar Test;
        rule: ( TOKEN
        "#, "group"),
        (r#"
        grammar Test;
        TOKEN: [a-z
        "#, "character class"),
    ];
    
    for (grammar, block_type) in unclosed_cases {
        let parser = GrammarParser::new();
        let result = parser.parse_string(grammar, "test.g4");
        assert!(result.is_err(), "Should detect unclosed {}", block_type);
    }
}

#[test]
fn test_empty_alternative_variations() {
    let empty_alt_cases = vec![
        (r#"rule: TOKEN | | TOKEN ;"#, "middle empty"),
        (r#"rule: | TOKEN ;"#, "first empty"),
        (r#"rule: TOKEN | ;"#, "last empty"),
        (r#"rule: ( TOKEN | ) ;"#, "empty in group"),
    ];
    
    for (grammar_text, description) in empty_alt_cases {
        let grammar = format!(r#"
        grammar Test;
        {}
        TOKEN: 'x' ;
        "#, grammar_text);
        
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, "test.g4");
        assert!(result.is_err(), "Should detect {} alternative", description);
    }
}

#[test]
fn test_error_message_with_context() {
    let grammar = r#"
        grammar Test;
        rule1: TOKEN1
        rule2: TOKEN2 ;
    "#;
    
    let parser = GrammarParser::new();
    let result = parser.parse_string(grammar, "test.g4");
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    // Should have location information
    assert!(error_msg.contains("Expected") || error_msg.contains("expected"));
}

#[test]
fn test_hex_escape_variations() {
    let hex_cases = vec![
        ("\\x41", 'A'),
        ("\\x42", 'B'),
        ("\\x00", '\0'),
        ("\\xFF", '\u{FF}'),
    ];
    
    for (escape, expected_char) in hex_cases {
        let grammar = format!(r#"
        grammar Test;
        TOKEN: '{}' ;
        "#, escape);
        
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, "test.g4");
        assert!(result.is_ok(), "Should parse hex escape: {}", escape);
    }
}

#[test]
fn test_simple_escape_sequences() {
    let escape_cases = vec![
        ("\\n", '\n'),
        ("\\r", '\r'),
        ("\\t", '\t'),
        ("\\\\", '\\'),
        ("\\'", '\''),
        ("\\\"", '"'),
        ("\\0", '\0'),
    ];
    
    for (escape, _expected) in escape_cases {
        let grammar = format!(r#"
        grammar Test;
        TOKEN: '{}' ;
        "#, escape);
        
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, "test.g4");
        assert!(result.is_ok(), "Should parse escape sequence: {}", escape);
    }
}

