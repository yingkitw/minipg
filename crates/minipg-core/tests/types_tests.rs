//! Types tests.

use minipg_core::types::{CodeGenConfig, GrammarType, SymbolTable};

#[test]
fn test_grammar_type() {
    let lexer = GrammarType::Lexer;
    let parser = GrammarType::Parser;
    let combined = GrammarType::Combined;
    
    assert_ne!(lexer, parser);
    assert_ne!(parser, combined);
}

#[test]
fn test_symbol_table() {
    let mut table = SymbolTable::new();
    
    table.add_token("ID".to_string(), 1);
    table.add_token("NUMBER".to_string(), 2);
    table.add_rule("expr".to_string(), 10);
    table.add_rule("term".to_string(), 11);
    
    assert_eq!(table.get_token("ID"), Some(1));
    assert_eq!(table.get_token("NUMBER"), Some(2));
    assert_eq!(table.get_token("UNKNOWN"), None);
    
    assert_eq!(table.get_rule("expr"), Some(10));
    assert_eq!(table.get_rule("term"), Some(11));
    assert_eq!(table.get_rule("unknown"), None);
}

#[test]
fn test_codegen_config_default() {
    let config = CodeGenConfig::default();
    assert_eq!(config.target_language, "rust");
    assert_eq!(config.output_directory, ".");
    assert!(config.generate_listener);
    assert!(!config.generate_visitor);
}

#[test]
fn test_codegen_config_custom() {
    let config = CodeGenConfig {
        target_language: "python".to_string(),
        output_directory: "./output".to_string(),
        package_name: Some("my_parser".to_string()),
        generate_listener: false,
        generate_visitor: true,
    };
    
    assert_eq!(config.target_language, "python");
    assert_eq!(config.package_name, Some("my_parser".to_string()));
    assert!(!config.generate_listener);
    assert!(config.generate_visitor);
}
