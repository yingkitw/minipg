/// Property-based tests using proptest for grammar validation and parser robustness
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use proptest::prelude::*;

/// Strategy for generating valid rule names
fn arb_rule_name() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,20}"
        .prop_map(|s| s.to_string())
}

/// Strategy for generating valid token names
fn arb_token_name() -> impl Strategy<Value = String> {
    "[A-Z][A-Z0-9_]{0,20}"
        .prop_map(|s| s.to_string())
}

/// Strategy for generating simple grammar rules
fn arb_simple_rule() -> impl Strategy<Value = String> {
    (arb_rule_name(), arb_token_name())
        .prop_map(|(rule, token)| {
            format!("{}: {} ;", rule, token)
        })
}

/// Strategy for generating character classes
fn arb_char_class() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("[a-z]".to_string()),
        Just("[A-Z]".to_string()),
        Just("[0-9]".to_string()),
        Just("[a-zA-Z0-9]".to_string()),
        Just("[^a-z]".to_string()),
        Just("[\\\\]".to_string()),
    ]
}

/// Strategy for generating quantifiers
fn arb_quantifier() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("*".to_string()),
        Just("+".to_string()),
        Just("?".to_string()),
        Just("*?".to_string()),
        Just("+?".to_string()),
        Just("??".to_string()),
    ]
}

/// Test: Grammar with simple rules parses without panic
proptest! {
    #[test]
    fn prop_simple_grammar_parses(rule in arb_simple_rule()) {
        let grammar_text = format!("grammar Test;\n{}", rule);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        // Should either parse successfully or return an error, never panic
        let _ = result;
    }
}

/// Test: Multiple rules in sequence parse without panic
proptest! {
    #[test]
    fn prop_multiple_rules_parse(rules in prop::collection::vec(arb_simple_rule(), 1..10)) {
        let rule_text = rules.join("\n");
        let grammar_text = format!("grammar Test;\n{}", rule_text);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Character classes with various patterns parse without panic
proptest! {
    #[test]
    fn prop_char_classes_parse(char_class in arb_char_class()) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: {} ;",
            char_class
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Quantifiers with character classes parse without panic
proptest! {
    #[test]
    fn prop_quantifiers_parse(
        char_class in arb_char_class(),
        quantifier in arb_quantifier()
    ) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: {}{} ;",
            char_class, quantifier
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Strings with various escape sequences parse without panic
proptest! {
    #[test]
    fn prop_string_escapes_parse(
        content in r#"[a-zA-Z0-9 ]{0,20}"#
    ) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: '{}' ;",
            content.replace("'", "\\'")
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Nested parentheses parse without panic
proptest! {
    #[test]
    fn prop_nested_parens_parse(depth in 1..10usize) {
        let mut expr = "TOKEN".to_string();
        for _ in 0..depth {
            expr = format!("({})", expr);
        }
        let grammar_text = format!(
            "grammar Test;\nrule: {} ;",
            expr
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Alternatives with various patterns parse without panic
proptest! {
    #[test]
    fn prop_alternatives_parse(
        alternatives in prop::collection::vec(arb_token_name(), 1..5)
    ) {
        let alt_text = alternatives.join(" | ");
        let grammar_text = format!(
            "grammar Test;\nrule: {} ;",
            alt_text
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Grammar with comments parses without panic
proptest! {
    #[test]
    fn prop_comments_parse(comment in r#"[a-zA-Z0-9 ]{0,50}"#) {
        let grammar_text = format!(
            "grammar Test;\n// {}\nrule: TOKEN ;",
            comment
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Grammar with block comments parses without panic
proptest! {
    #[test]
    fn prop_block_comments_parse(comment in r#"[a-zA-Z0-9 ]{0,50}"#) {
        let grammar_text = format!(
            "grammar Test;\n/* {} */\nrule: TOKEN ;",
            comment.replace("*/", "* /")
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Unicode escapes parse without panic
proptest! {
    #[test]
    fn prop_unicode_escapes_parse(code_point in 0x0000u32..0xFFFFu32) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: '\\u{:04X}' ;",
            code_point
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Lexer commands parse without panic
proptest! {
    #[test]
    fn prop_lexer_commands_parse(
        rule_name in arb_token_name()
    ) {
        let commands = vec![
            format!("{}: 'x' -> skip ;", rule_name),
            format!("{}: 'y' -> channel(HIDDEN) ;", rule_name),
            format!("{}: 'z' -> mode(STRING) ;", rule_name),
        ];
        
        for cmd in commands {
            let grammar_text = format!("grammar Test;\n{}", cmd);
            let parser = GrammarParser::new();
            let result = parser.parse_string(&grammar_text, "test.g4");
            let _ = result;
        }
    }
}

/// Test: Rule arguments and returns parse without panic
proptest! {
    #[test]
    fn prop_rule_args_returns_parse(
        rule_name in arb_rule_name()
    ) {
        let grammar_text = format!(
            "grammar Test;\n{}[int x, String y] returns [int result] : TOKEN ;",
            rule_name
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Named actions parse without panic
proptest! {
    #[test]
    fn prop_named_actions_parse(
        content in r#"[a-zA-Z0-9 ]{0,50}"#
    ) {
        let grammar_text = format!(
            "grammar Test;\n@header {{ {} }}\nrule: TOKEN ;",
            content
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: List labels parse without panic
proptest! {
    #[test]
    fn prop_list_labels_parse(
        count in 1..5usize
    ) {
        let labels = (0..count)
            .map(|i| format!("ids+=ID{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        let grammar_text = format!(
            "grammar Test;\nrule: {} ;",
            labels
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}

/// Test: Element labels parse without panic
proptest! {
    #[test]
    fn prop_element_labels_parse(
        count in 1..5usize
    ) {
        let labels = (0..count)
            .map(|i| format!("x{}=TOKEN{}", i, i))
            .collect::<Vec<_>>()
            .join(" ");
        let grammar_text = format!(
            "grammar Test;\nrule: {} ;",
            labels
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "test.g4");
        let _ = result;
    }
}
