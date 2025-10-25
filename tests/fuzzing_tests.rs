/// Fuzzing tests for parser robustness against malformed and random input
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use proptest::prelude::*;

/// Strategy for generating arbitrary strings (potential malformed grammars)
fn arb_arbitrary_string() -> impl Strategy<Value = String> {
    ".*"
        .prop_map(|s| s.to_string())
}

/// Strategy for generating random bytes
fn arb_random_bytes() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 0..1000)
}

/// Test: Parser doesn't panic on arbitrary strings
proptest! {
    #[test]
    fn fuzz_arbitrary_strings_no_panic(input in arb_arbitrary_string()) {
        let parser = GrammarParser::new();
        let result = parser.parse_string(&input, "fuzz.g4");
        // Should not panic, may return error
        let _ = result;
    }
}

/// Test: Parser doesn't panic on random bytes (as UTF-8)
proptest! {
    #[test]
    fn fuzz_random_bytes_no_panic(bytes in arb_random_bytes()) {
        if let Ok(input) = String::from_utf8(bytes) {
            let parser = GrammarParser::new();
            let result = parser.parse_string(&input, "fuzz.g4");
            let _ = result;
        }
    }
}

/// Test: Parser handles deeply nested structures
proptest! {
    #[test]
    fn fuzz_deeply_nested_structures(depth in 1..100usize) {
        let mut input = "TOKEN".to_string();
        for _ in 0..depth {
            input = format!("({})", input);
        }
        let grammar_text = format!("grammar Test;\nrule: {} ;", input);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles very long identifiers
proptest! {
    #[test]
    fn fuzz_long_identifiers(length in 1..10000usize) {
        let ident = "a".repeat(length);
        let grammar_text = format!("grammar Test;\n{}: TOKEN ;", ident);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles many alternatives
proptest! {
    #[test]
    fn fuzz_many_alternatives(count in 1..500usize) {
        let alts = (0..count)
            .map(|i| format!("TOKEN{}", i))
            .collect::<Vec<_>>()
            .join(" | ");
        let grammar_text = format!("grammar Test;\nrule: {} ;", alts);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles many rules
proptest! {
    #[test]
    fn fuzz_many_rules(count in 1..200usize) {
        let rules = (0..count)
            .map(|i| format!("rule{}: TOKEN ;", i))
            .collect::<Vec<_>>()
            .join("\n");
        let grammar_text = format!("grammar Test;\n{}", rules);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles unmatched delimiters
proptest! {
    #[test]
    fn fuzz_unmatched_delimiters(
        open_count in 0..50usize,
        close_count in 0..50usize
    ) {
        let opens = "(".repeat(open_count);
        let closes = ")".repeat(close_count);
        let grammar_text = format!(
            "grammar Test;\nrule: {} TOKEN {} ;",
            opens, closes
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles mixed valid and invalid syntax
proptest! {
    #[test]
    fn fuzz_mixed_valid_invalid(
        valid_rules in prop::collection::vec("[a-z]+", 0..5),
        invalid_chars in "[\\[\\]{}()@#$%^&*]+"
    ) {
        let rules = valid_rules
            .iter()
            .map(|r| format!("{}: TOKEN ;", r))
            .collect::<Vec<_>>()
            .join("\n");
        let grammar_text = format!(
            "grammar Test;\n{}\n{}\nrule: TOKEN ;",
            rules, invalid_chars
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles escaped characters in strings
proptest! {
    #[test]
    fn fuzz_escaped_characters(
        content in r#"[a-zA-Z0-9\\/'"]+"#
    ) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: '{}' ;",
            content
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles incomplete rules
proptest! {
    #[test]
    fn fuzz_incomplete_rules(
        rule_name in "[a-z]+"
    ) {
        let incomplete_rules = vec![
            format!("{}", rule_name),
            format!("{}:", rule_name),
            format!("{}: ", rule_name),
            format!("{}: TOKEN", rule_name),
            format!("{}: TOKEN |", rule_name),
        ];
        
        for incomplete in incomplete_rules {
            let grammar_text = format!("grammar Test;\n{}", incomplete);
            let parser = GrammarParser::new();
            let result = parser.parse_string(&grammar_text, "fuzz.g4");
            let _ = result;
        }
    }
}

/// Test: Parser handles whitespace variations
proptest! {
    #[test]
    fn fuzz_whitespace_variations(
        spaces_before in 0..20usize,
        spaces_after in 0..20usize,
        tabs in 0..10usize
    ) {
        let before = " ".repeat(spaces_before);
        let after = " ".repeat(spaces_after);
        let tab_str = "\t".repeat(tabs);
        let grammar_text = format!(
            "{}grammar{}Test{};\n{}rule{}:{}TOKEN{};\n",
            before, after, tab_str, before, after, tab_str, after
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles comment variations
proptest! {
    #[test]
    fn fuzz_comment_variations(
        content in r#"[a-zA-Z0-9 ]{0,100}"#
    ) {
        let comment_styles = vec![
            format!("// {}", content),
            format!("/* {} */", content.replace("*/", "* /")),
            format!("/* {} */ rule: TOKEN ;", content.replace("*/", "* /")),
            format!("// {}\nrule: TOKEN ;", content),
        ];
        
        for comment in comment_styles {
            let grammar_text = format!("grammar Test;\n{}", comment);
            let parser = GrammarParser::new();
            let result = parser.parse_string(&grammar_text, "fuzz.g4");
            let _ = result;
        }
    }
}

/// Test: Parser handles character class edge cases
proptest! {
    #[test]
    fn fuzz_char_class_edge_cases(
        content in r#"[a-zA-Z0-9\-\\]+"#
    ) {
        let grammar_text = format!(
            "grammar Test;\nTOKEN: [{}] ;",
            content
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles quantifier edge cases
proptest! {
    #[test]
    fn fuzz_quantifier_edge_cases(
        _count in 0..100usize
    ) {
        let quantifiers = vec![
            "*", "+", "?", "*?", "+?", "??",
        ];
        
        for q in quantifiers {
            let grammar_text = format!(
                "grammar Test;\nTOKEN: 'x'{} ;",
                q
            );
            let parser = GrammarParser::new();
            let result = parser.parse_string(&grammar_text, "fuzz.g4");
            let _ = result;
        }
    }
}

/// Test: Parser handles mixed grammar names
proptest! {
    #[test]
    fn fuzz_grammar_names(
        name in "[a-zA-Z_][a-zA-Z0-9_]{0,50}"
    ) {
        let grammar_text = format!("grammar {} ;\nrule: TOKEN ;", name);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles options blocks
proptest! {
    #[test]
    fn fuzz_options_blocks(
        key in "[a-zA-Z_][a-zA-Z0-9_]{0,20}",
        value in "[a-zA-Z0-9_]{0,20}"
    ) {
        let grammar_text = format!(
            "grammar Test;\noptions {{ {} = {} ; }}\nrule: TOKEN ;",
            key, value
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles action blocks
proptest! {
    #[test]
    fn fuzz_action_blocks(
        content in r#"[a-zA-Z0-9 ]{0,100}"#
    ) {
        let grammar_text = format!(
            "grammar Test;\nrule: TOKEN {{ {} }} ;",
            content
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}

/// Test: Parser handles semantic predicates
proptest! {
    #[test]
    fn fuzz_semantic_predicates(
        content in r#"[a-zA-Z0-9 ]{0,100}"#
    ) {
        let grammar_text = format!(
            "grammar Test;\nrule: TOKEN {{ {} }}? ;",
            content
        );
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar_text, "fuzz.g4");
        let _ = result;
    }
}
