//! Tests for lexer modes, channels, and action code generation.
//!
//! This test module provides comprehensive coverage for:
//! - Lexer mode management (push, pop, switch)
//! - Token channel routing
//! - Action code generation and translation
//! - Semantic predicate generation
//!
//! # Test Organization
//!
//! Tests are organized into logical groups:
//! - **Lexer Modes**: Mode stack operations and initialization
//! - **Channels**: Token channel routing for all languages
//! - **Action Translation**: Converting actions between languages
//! - **Semantic Predicates**: Predicate code generation
//! - **Action Code Generation**: Wrapping action code for each language
//! - **Integration Tests**: End-to-end scenarios
//!
//! # Running Tests
//!
//! Run all tests in this module:
//! ```bash
//! cargo test --test test_modes_channels_actions
//! ```
//!
//! Run specific test:
//! ```bash
//! cargo test --test test_modes_channels_actions test_rust_mode_methods_generation
//! ```
//!
//! Run with output:
//! ```bash
//! cargo test --test test_modes_channels_actions -- --nocapture
//! ```

use minipg::ast::Grammar;
use minipg::core::types::GrammarType;
use minipg::codegen::modes::*;
use minipg::codegen::actions::*;

// ============================================================================
// LEXER MODES TESTS - Mode stack management and initialization
// ============================================================================

#[test]
fn test_grammar_with_modes() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Combined);
    
    // Add modes
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec!["ID".to_string()]);
    grammar.add_lexer_mode("STRING_MODE".to_string(), vec!["STRING_CONTENT".to_string()]);
    
    assert!(grammar.has_modes());
    assert_eq!(grammar.lexer_modes.len(), 2);
}

#[test]
fn test_grammar_with_channels() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Combined);
    
    // Add channels
    grammar.add_channel("COMMENTS".to_string());
    grammar.add_channel("WHITESPACE".to_string());
    
    assert!(grammar.has_channels());
    assert_eq!(grammar.channels.len(), 2);
}

#[test]
fn test_rust_mode_stack_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_rust_mode_stack(&grammar);
    assert!(code.contains("mode_stack: Vec<String>"));
}

#[test]
fn test_rust_mode_methods_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_rust_mode_methods(&grammar);
    assert!(code.contains("current_mode"));
    assert!(code.contains("switch_mode"));
    assert!(code.contains("push_mode"));
    assert!(code.contains("pop_mode"));
}

#[test]
fn test_rust_mode_init_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_rust_mode_init(&grammar);
    assert!(code.contains("mode_stack"));
    assert!(code.contains("DEFAULT_MODE"));
}

#[test]
fn test_python_mode_stack_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_python_mode_stack(&grammar);
    assert!(code.contains("mode_stack"));
    assert!(code.contains("DEFAULT_MODE"));
}

#[test]
fn test_python_mode_methods_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_python_mode_methods(&grammar);
    assert!(code.contains("current_mode"));
    assert!(code.contains("switch_mode"));
    assert!(code.contains("push_mode"));
    assert!(code.contains("pop_mode"));
}

#[test]
fn test_javascript_mode_methods_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_javascript_mode_methods(&grammar);
    assert!(code.contains("currentMode"));
    assert!(code.contains("switchMode"));
    assert!(code.contains("pushMode"));
    assert!(code.contains("popMode"));
}

#[test]
fn test_typescript_mode_methods_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_typescript_mode_methods(&grammar);
    assert!(code.contains("currentMode"));
    assert!(code.contains("switchMode"));
}

#[test]
fn test_go_mode_stack_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_go_mode_stack(&grammar);
    assert!(code.contains("ModeStack"));
    assert!(code.contains("[]string"));
}

#[test]
fn test_go_mode_methods_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    let code = generate_go_mode_methods(&grammar);
    assert!(code.contains("CurrentMode"));
    assert!(code.contains("SwitchMode"));
    assert!(code.contains("PushMode"));
    assert!(code.contains("PopMode"));
}

// ============================================================================
// CHANNELS TESTS
// ============================================================================

#[test]
fn test_rust_channels_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    let code = generate_rust_mode_methods(&grammar);
    assert!(code.contains("send_to_channel"));
    assert!(code.contains("get_channel"));
}

#[test]
fn test_python_channels_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    let code = generate_python_mode_methods(&grammar);
    assert!(code.contains("send_to_channel"));
    assert!(code.contains("get_channel"));
}

#[test]
fn test_javascript_channels_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    let code = generate_javascript_mode_methods(&grammar);
    assert!(code.contains("sendToChannel"));
    assert!(code.contains("getChannel"));
}

#[test]
fn test_typescript_channels_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    let code = generate_typescript_mode_methods(&grammar);
    assert!(code.contains("sendToChannel"));
    assert!(code.contains("getChannel"));
}

#[test]
fn test_go_channels_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    let code = generate_go_mode_methods(&grammar);
    assert!(code.contains("SendToChannel"));
    assert!(code.contains("GetChannel"));
}

// ============================================================================
// ACTION TRANSLATION TESTS
// ============================================================================

#[test]
fn test_rust_to_python_translation() {
    let rust_code = "self.validate()";
    let python_code = translate_action(rust_code, "rust", "python");
    assert!(python_code.contains("validate"));
}

#[test]
fn test_rust_to_javascript_translation() {
    let rust_code = "self.validate()";
    let js_code = translate_action(rust_code, "rust", "javascript");
    assert!(js_code.contains("this.validate"));
}

#[test]
fn test_rust_to_typescript_translation() {
    let rust_code = "self.validate()";
    let ts_code = translate_action(rust_code, "rust", "typescript");
    assert!(ts_code.contains("this.validate"));
}

#[test]
fn test_rust_to_go_translation() {
    let rust_code = "self.validate()";
    let go_code = translate_action(rust_code, "rust", "go");
    assert!(go_code.contains("l.validate"));
}

#[test]
fn test_same_language_translation() {
    let rust_code = "self.validate()";
    let result = translate_action(rust_code, "rust", "rust");
    assert_eq!(result, rust_code);
}

#[test]
fn test_vec_translation() {
    let rust_code = "Vec::new()";
    let python_code = translate_action(rust_code, "rust", "python");
    assert!(python_code.contains("[]"));
}

#[test]
fn test_hashmap_translation() {
    let rust_code = "HashMap::new()";
    let python_code = translate_action(rust_code, "rust", "python");
    assert!(python_code.contains("{}"));
}

// ============================================================================
// SEMANTIC PREDICATE TESTS
// ============================================================================

#[test]
fn test_rust_predicate_generation() {
    let code = "x > 0";
    let predicate = generate_rust_predicate(code);
    assert!(predicate.contains("x > 0"));
    assert!(predicate.contains("Predicate failed"));
}

#[test]
fn test_python_predicate_generation() {
    let code = "x > 0";
    let predicate = generate_python_predicate(code);
    assert!(predicate.contains("x > 0"));
    assert!(predicate.contains("ParseError"));
}

#[test]
fn test_javascript_predicate_generation() {
    let code = "x > 0";
    let predicate = generate_javascript_predicate(code);
    assert!(predicate.contains("x > 0"));
    assert!(predicate.contains("ParseError"));
}

#[test]
fn test_typescript_predicate_generation() {
    let code = "x > 0";
    let predicate = generate_typescript_predicate(code);
    assert!(predicate.contains("x > 0"));
    assert!(predicate.contains("ParseError"));
}

#[test]
fn test_go_predicate_generation() {
    let code = "x > 0";
    let predicate = generate_go_predicate(code);
    assert!(predicate.contains("x > 0"));
    assert!(predicate.contains("ParseError"));
}

// ============================================================================
// ACTION CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_rust_action_generation() {
    let code = "x = 1";
    let action = generate_rust_action(code);
    assert_eq!(action, "{ x = 1 }");
}

#[test]
fn test_python_action_generation() {
    let code = "x = 1";
    let action = generate_python_action(code);
    assert!(action.contains("x = 1"));
}

#[test]
fn test_javascript_action_generation() {
    let code = "x = 1";
    let action = generate_javascript_action(code);
    assert_eq!(action, "{ x = 1 }");
}

#[test]
fn test_typescript_action_generation() {
    let code = "x = 1";
    let action = generate_typescript_action(code);
    assert_eq!(action, "{ x = 1 }");
}

#[test]
fn test_go_action_generation() {
    let code = "x = 1";
    let action = generate_go_action(code);
    assert_eq!(action, "{ x = 1 }");
}

// ============================================================================
// HELPER FUNCTIONS - Utility functions for test setup
// ============================================================================

/// Create a test grammar with modes and channels
fn create_test_grammar_with_features() -> Grammar {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec!["ID".to_string()]);
    grammar.add_lexer_mode("STRING_MODE".to_string(), vec!["STRING_CONTENT".to_string()]);
    grammar.add_channel("COMMENTS".to_string());
    grammar.add_channel("WHITESPACE".to_string());
    
    grammar
}

/// Create a test grammar with only modes
fn create_test_grammar_with_modes() -> Grammar {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec!["ID".to_string()]);
    grammar
}

/// Create a test grammar with only channels
fn create_test_grammar_with_channels() -> Grammar {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    grammar
}

// ============================================================================
// INTEGRATION TESTS - End-to-end scenarios
// ============================================================================

#[test]
fn test_grammar_with_modes_and_channels() {
    let grammar = create_test_grammar_with_features();
    
    assert!(grammar.has_modes());
    assert!(grammar.has_channels());
    assert_eq!(grammar.lexer_modes.len(), 2);
    assert_eq!(grammar.channels.len(), 2);
}

#[test]
fn test_all_languages_mode_generation() {
    let mut grammar = Grammar::new("TestGrammar".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    
    // Rust
    let rust_code = generate_rust_mode_methods(&grammar);
    assert!(!rust_code.is_empty());
    assert!(rust_code.contains("current_mode"));
    
    // Python
    let python_code = generate_python_mode_methods(&grammar);
    assert!(!python_code.is_empty());
    assert!(python_code.contains("current_mode"));
    
    // JavaScript
    let js_code = generate_javascript_mode_methods(&grammar);
    assert!(!js_code.is_empty());
    assert!(js_code.contains("currentMode"));
    
    // TypeScript
    let ts_code = generate_typescript_mode_methods(&grammar);
    assert!(!ts_code.is_empty());
    assert!(ts_code.contains("currentMode"));
    
    // Go
    let go_code = generate_go_mode_methods(&grammar);
    assert!(!go_code.is_empty());
    assert!(go_code.contains("CurrentMode"));
}

#[test]
fn test_action_translation_consistency() {
    let actions = vec![
        "self.validate()",
        "self.combine(x, y)",
        "self.process()",
    ];
    
    for action in actions {
        let rust_result = translate_action(action, "rust", "rust");
        assert_eq!(rust_result, action);
        
        let python_result = translate_action(action, "rust", "python");
        assert!(!python_result.is_empty());
        
        let js_result = translate_action(action, "rust", "javascript");
        assert!(!js_result.is_empty());
    }
}

#[test]
fn test_predicate_generation_consistency() {
    let predicates = vec![
        "x > 0",
        "self.is_valid()",
        "token.kind == TokenKind::ID",
    ];
    
    for predicate in predicates {
        let rust_pred = generate_rust_predicate(predicate);
        assert!(rust_pred.contains(predicate));
        
        let python_pred = generate_python_predicate(predicate);
        assert!(python_pred.contains(predicate));
        
        let js_pred = generate_javascript_predicate(predicate);
        assert!(js_pred.contains(predicate));
    }
}

// ============================================================================
// ADVANCED SCENARIOS - Complex real-world use cases
// ============================================================================

#[test]
fn test_complex_mode_switching_scenario() {
    /// Scenario: String literal lexing with mode switching
    /// DEFAULT_MODE -> STRING_MODE (on quote) -> DEFAULT_MODE (on quote)
    
    let mut grammar = create_test_grammar_with_modes();
    
    // Verify we can add multiple modes for complex scenarios
    grammar.add_lexer_mode("STRING_MODE".to_string(), vec!["STRING_CONTENT".to_string()]);
    grammar.add_lexer_mode("COMMENT_MODE".to_string(), vec!["COMMENT_CONTENT".to_string()]);
    
    assert_eq!(grammar.lexer_modes.len(), 3);
    assert!(grammar.has_modes());
}

#[test]
fn test_complex_channel_routing_scenario() {
    /// Scenario: Routing tokens to different channels
    /// - Main channel: tokens
    /// - Comments channel: comment tokens
    /// - Whitespace channel: whitespace tokens
    
    let mut grammar = create_test_grammar_with_channels();
    grammar.add_channel("MAIN".to_string());
    grammar.add_channel("COMMENTS".to_string());
    
    assert_eq!(grammar.channels.len(), 2);
    assert!(grammar.has_channels());
}

#[test]
fn test_action_translation_with_complex_code() {
    /// Scenario: Translating complex action code with multiple statements
    
    let complex_action = r#"
        self.validate();
        self.process();
        let result = self.combine(x, y);
    "#;
    
    let python_result = translate_action(complex_action, "rust", "python");
    assert!(!python_result.is_empty());
    assert!(python_result.contains("validate"));
    
    let js_result = translate_action(complex_action, "rust", "javascript");
    assert!(!js_result.is_empty());
    assert!(js_result.contains("this.validate"));
}

#[test]
fn test_predicate_with_complex_condition() {
    /// Scenario: Complex predicate with multiple conditions
    
    let complex_predicate = "self.is_valid() && token.kind == TokenKind::ID && position > 0";
    
    let rust_pred = generate_rust_predicate(complex_predicate);
    assert!(rust_pred.contains("is_valid"));
    assert!(rust_pred.contains("TokenKind::ID"));
    assert!(rust_pred.contains("position > 0"));
}

#[test]
fn test_all_language_consistency() {
    /// Scenario: Verify all languages generate consistent code structure
    
    let grammar = create_test_grammar_with_features();
    
    // Rust
    let rust_stack = generate_rust_mode_stack(&grammar);
    let rust_methods = generate_rust_mode_methods(&grammar);
    assert!(!rust_stack.is_empty());
    assert!(!rust_methods.is_empty());
    
    // Python
    let python_stack = generate_python_mode_stack(&grammar);
    let python_methods = generate_python_mode_methods(&grammar);
    assert!(!python_stack.is_empty());
    assert!(!python_methods.is_empty());
    
    // JavaScript
    let js_stack = generate_javascript_mode_stack(&grammar);
    let js_methods = generate_javascript_mode_methods(&grammar);
    assert!(!js_stack.is_empty());
    assert!(!js_methods.is_empty());
    
    // TypeScript
    let ts_stack = generate_typescript_mode_stack(&grammar);
    let ts_methods = generate_typescript_mode_methods(&grammar);
    assert!(!ts_stack.is_empty());
    assert!(!ts_methods.is_empty());
    
    // Go
    let go_stack = generate_go_mode_stack(&grammar);
    let go_methods = generate_go_mode_methods(&grammar);
    assert!(!go_stack.is_empty());
    assert!(!go_methods.is_empty());
}

#[test]
fn test_empty_grammar_handling() {
    /// Scenario: Verify graceful handling of grammars without modes/channels
    
    let grammar = Grammar::new("EmptyGrammar".to_string(), GrammarType::Lexer);
    
    assert!(!grammar.has_modes());
    assert!(!grammar.has_channels());
    
    let rust_stack = generate_rust_mode_stack(&grammar);
    assert!(rust_stack.is_empty());
}

#[test]
fn test_action_translation_edge_cases() {
    /// Scenario: Edge cases in action translation
    
    // Empty action
    let empty = translate_action("", "rust", "python");
    assert_eq!(empty, "");
    
    // Action with no self references
    let no_self = "let x = 1;";
    let result = translate_action(no_self, "rust", "python");
    assert!(result.contains("let x = 1"));
    
    // Action with multiple self references
    let multi_self = "self.a(); self.b(); self.c();";
    let result = translate_action(multi_self, "rust", "javascript");
    assert!(result.contains("this.a"));
    assert!(result.contains("this.b"));
    assert!(result.contains("this.c"));
}

#[test]
fn test_predicate_error_handling() {
    /// Scenario: Verify predicates include proper error handling
    
    let predicate = "x > 0";
    
    let rust_pred = generate_rust_predicate(predicate);
    assert!(rust_pred.contains("Err"));
    
    let python_pred = generate_python_predicate(predicate);
    assert!(python_pred.contains("raise"));
    
    let js_pred = generate_javascript_predicate(predicate);
    assert!(js_pred.contains("throw"));
    
    let go_pred = generate_go_predicate(predicate);
    assert!(go_pred.contains("ParseError"));
}

// ============================================================================
// DOCUMENTATION TESTS - Verify examples in documentation work
// ============================================================================

#[test]
fn test_mode_stack_operations_example() {
    /// Example from documentation: Mode stack operations
    
    let mut grammar = Grammar::new("Example".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_lexer_mode("STRING_MODE".to_string(), vec![]);
    
    // Verify mode methods are generated
    let methods = generate_rust_mode_methods(&grammar);
    assert!(methods.contains("push_mode"));
    assert!(methods.contains("pop_mode"));
    assert!(methods.contains("switch_mode"));
    assert!(methods.contains("current_mode"));
}

#[test]
fn test_channel_routing_example() {
    /// Example from documentation: Channel routing
    
    let mut grammar = Grammar::new("Example".to_string(), GrammarType::Lexer);
    grammar.add_lexer_mode("DEFAULT_MODE".to_string(), vec![]);
    grammar.add_channel("COMMENTS".to_string());
    
    // Verify channel methods are generated
    let methods = generate_rust_mode_methods(&grammar);
    assert!(methods.contains("send_to_channel"));
    assert!(methods.contains("get_channel"));
}

#[test]
fn test_action_generation_example() {
    /// Example from documentation: Action code generation
    
    let action = "self.validate()";
    
    // Rust
    let rust_action = generate_rust_action(action);
    assert_eq!(rust_action, "{ self.validate() }");
    
    // Python
    let python_action = generate_python_action(action);
    assert!(python_action.contains("validate"));
    
    // JavaScript
    let js_action = generate_javascript_action(action);
    assert_eq!(js_action, "{ self.validate() }");
}
