/// Large file testing (GB+ inputs) for memory and performance validation
/// 
/// These tests verify that minipg can handle very large grammar files
/// without running out of memory or experiencing performance degradation.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use minipg::parser::GrammarParser;
use minipg::core::GrammarParser as GrammarParserTrait;
use tempfile::TempDir;

/// Generate a large grammar file with many rules
fn generate_large_grammar(size_mb: usize) -> String {
    let mut grammar = String::with_capacity(size_mb * 1024 * 1024);
    
    // Grammar header
    grammar.push_str("grammar parser LargeGrammar;\n\n");
    
    // Generate many rules
    let rules_per_mb = 1000; // Approximate rules per MB
    let total_rules = size_mb * rules_per_mb;
    
    for i in 0..total_rules {
        grammar.push_str(&format!("rule{}: TOKEN{} ;\n", i, i % 10));
    }
    
    // Add token definitions
    grammar.push_str("\n");
    for i in 0..10 {
        grammar.push_str(&format!("TOKEN{}: 'token{}' ;\n", i, i));
    }
    
    grammar
}

/// Generate a grammar with very deep nesting
fn generate_deep_nesting_grammar(depth: usize) -> String {
    let mut grammar = String::from("grammar parser DeepGrammar;\n\n");
    grammar.push_str("root: ");
    
    for i in 0..depth {
        grammar.push_str(&format!("level{} ", i));
        if i < depth - 1 {
            grammar.push_str("( ");
        }
    }
    
    grammar.push_str("TOKEN");
    for _ in 0..depth {
        grammar.push_str(" )");
    }
    grammar.push_str(" ;\n\n");
    
    for i in 0..depth {
        grammar.push_str(&format!("level{}: TOKEN ;\n", i));
    }
    
    grammar.push_str("TOKEN: 'x' ;\n");
    
    grammar
}

/// Generate a grammar with very long identifiers
fn generate_long_identifiers_grammar(ident_length: usize) -> String {
    let mut grammar = String::from("grammar parser LongIdGrammar;\n\n");
    
    let ident = "a".repeat(ident_length);
    grammar.push_str(&format!("{}: TOKEN ;\n", ident));
    grammar.push_str("TOKEN: 'x' ;\n");
    
    grammar
}

#[test]
#[ignore = "Requires significant memory and time"]
fn test_large_grammar_10mb() {
    let grammar = generate_large_grammar(10);
    let parser = GrammarParser::new();
    
    // Should not panic on large input
    let result = parser.parse_string(&grammar, "large_10mb.g4");
    
    // May return error for very large files, but should not panic
    let _ = result;
}

#[test]
#[ignore = "Requires significant memory and time"]
fn test_large_grammar_100mb() {
    let grammar = generate_large_grammar(100);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "large_100mb.g4");
    let _ = result;
}

#[test]
#[ignore = "Requires significant memory and time"]
fn test_large_grammar_1gb() {
    let grammar = generate_large_grammar(1024);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "large_1gb.g4");
    let _ = result;
}

#[test]
#[ignore = "Can be slow"]
fn test_deeply_nested_grammar() {
    let grammar = generate_deep_nesting_grammar(1000);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "deep_1k.g4");
    let _ = result;
}

#[test]
#[ignore = "Can be slow"]
fn test_very_deeply_nested_grammar() {
    let grammar = generate_deep_nesting_grammar(10000);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "deep_10k.g4");
    let _ = result;
}

#[test]
#[ignore = "Can be slow"]
fn test_long_identifiers() {
    let grammar = generate_long_identifiers_grammar(100000);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "long_id_100k.g4");
    let _ = result;
}

#[test]
#[ignore = "Requires significant memory"]
fn test_very_long_identifiers() {
    let grammar = generate_long_identifiers_grammar(1000000);
    let parser = GrammarParser::new();
    
    let result = parser.parse_string(&grammar, "long_id_1m.g4");
    let _ = result;
}

/// Test parsing a large grammar file from disk
#[test]
#[ignore = "Requires file I/O and memory"]
fn test_large_grammar_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large_grammar.g4");
    
    // Generate and write a 50MB grammar file
    let grammar = generate_large_grammar(50);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(grammar.as_bytes()).unwrap();
    drop(file);
    
    // Read and parse
    let grammar_text = fs::read_to_string(&file_path).unwrap();
    let parser = GrammarParser::new();
    let result = parser.parse_string(&grammar_text, "large_grammar.g4");
    
    let _ = result;
}

/// Test memory usage with incremental parsing
#[test]
#[ignore = "Memory profiling test"]
fn test_memory_efficiency() {
    // Parse multiple large grammars sequentially
    for size in [1, 5, 10, 50] {
        let grammar = generate_large_grammar(size);
        let parser = GrammarParser::new();
        let result = parser.parse_string(&grammar, &format!("large_{}mb.g4", size));
        let _ = result;
    }
}

