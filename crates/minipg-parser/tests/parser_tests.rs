//! Parser tests with snapshot testing.

use minipg::core::GrammarParser;
use minipg::parser::GrammarParser as Parser;

#[test]
fn test_parse_simple_grammar() {
    let source = r#"
grammar parser Calculator;

expr: term;
term: NUMBER;

NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
"#;

    let parser = Parser::new();
    let result = parser.parse_string(source, "test.g4");
    
    if let Err(ref e) = result {
        eprintln!("Parse error: {}", e);
    }
    assert!(result.is_ok());
    let grammar = result.unwrap();
    
    insta::assert_yaml_snapshot!(grammar);
}

#[test]
fn test_parse_lexer_grammar() {
    let source = r#"
grammar lexer SimpleLexer;

WS: SPACE+;
ID: LETTER;
NUMBER: DIGIT+;

SPACE: ' ' | '\t' | '\r' | '\n';
LETTER: 'a' | 'b' | 'c';
DIGIT: '0' | '1' | '2';
"#;

    let parser = Parser::new();
    let result = parser.parse_string(source, "test.g4");
    
    if let Err(ref e) = result {
        eprintln!("Parse error: {}", e);
    }
    assert!(result.is_ok());
    let grammar = result.unwrap();
    assert_eq!(grammar.name, "SimpleLexer");
}

#[test]
fn test_parse_error() {
    let source = r#"
grammar parser Invalid
"#;

    let parser = Parser::new();
    let result = parser.parse_string(source, "test.g4");
    
    assert!(result.is_err());
}
