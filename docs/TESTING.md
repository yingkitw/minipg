# Testing Guide

This document consolidates testing information for minipg, including test coverage, compatibility testing, and testing strategies.

## Table of Contents

1. [Test Structure](#test-structure)
2. [Test Coverage](#test-coverage)
3. [ANTLR4 Compatibility Testing](#antlr4-compatibility-testing)
4. [Grammars-v4 Testing](#grammars-v4-testing)
5. [Running Tests](#running-tests)

---

## Test Structure

### Test Files Organization

**Core Tests**:
- `integration_test.rs` - Core integration tests
- `end_to_end_test.rs` - Full pipeline tests
- `e2e_simple_pipeline.rs` - Simple E2E tests

**Feature Tests**:
- `test_rule_features.rs` - Rule arguments, returns, locals
- `test_list_labels.rs` - List label support
- `test_named_actions.rs` - Named actions parsing
- `test_named_actions_codegen.rs` - Named actions code generation
- `test_lexer_modes_parsing.rs` - Lexer modes parsing
- `test_modes_channels_actions.rs` - Modes, channels, actions
- `test_unicode_escape.rs` - Unicode escape sequences

**Example Tests**:
- `test_all_examples.rs` - Basic example grammars
- `test_advanced_examples.rs` - Advanced example grammars
- `test_all_g4_files.rs` - All grammar files
- `test_completejson_line.rs` - CompleteJSON grammar

**Compatibility Tests**:
- `test_antlr4_compatibility.rs` - ANTLR4 compatibility
- `test_antlr4_test_suite.rs` - ANTLR4 test suite patterns
- `test_grammars_v4_compatibility.rs` - Grammars-v4 repository

**Specialized Tests**:
- `property_based_tests.rs` - Property-based testing
- `fuzzing_tests.rs` - Fuzzing tests
- `large_file_tests.rs` - Large file handling
- `memory_profiling.rs` - Memory profiling
- `parser_advanced_features.rs` - Advanced parser features

---

## Test Coverage

### Current Status

**Total Tests**: 369+ tests
- 73 unit tests
- 13 rule feature tests
- 9 lexer modes tests
- 274+ other tests (integration, E2E, compatibility)

**Pass Rate**: 100% ✅

**Test Categories**:
- ✅ Grammar parsing
- ✅ AST construction
- ✅ Semantic analysis
- ✅ Code generation (all 8 languages)
- ✅ Error handling
- ✅ Compatibility testing

---

## ANTLR4 Compatibility Testing

### Test Suite Patterns (22 tests)

**Basic Features**:
- ✅ Basic rules and rule references
- ✅ Alternatives (|) and repetition (*, +, ?, {n,m})
- ✅ Character classes and string literals
- ✅ Lexer tokens and fragment tokens

**Advanced Features**:
- ✅ Lexer commands (skip, channel, mode, popMode, more, type)
- ✅ Rule arguments, returns, and locals
- ✅ Labels (regular and list)
- ✅ Actions and predicates
- ✅ Named actions (@header, @members, @lexer::*, @parser::*)

**Grammar Structure**:
- ✅ Grammar options (language, tokenVocab, superClass, package, namespace)
- ✅ Grammar imports and composition
- ✅ Grammar types (combined, lexer, parser)
- ✅ EOF handling and complex expressions

**Code Generators**:
- ✅ All 8 code generators tested

### Test File

`tests/test_antlr4_test_suite.rs` - 22 comprehensive tests

---

## Grammars-v4 Testing

### Real-World Grammar Support (14 tests)

**Language Grammars**:
- ✅ **Java Subset** - Core Java language features
- ✅ **Python Subset** - Python 3 language features
- ✅ **SQL Subset** - SQL language features
- ✅ **C Subset** - C language features
- ✅ **JavaScript Subset** - JavaScript language features
- ✅ **GraphQL Subset** - GraphQL language features

**Additional Tests**:
- ✅ Code generation for all languages
- ✅ ANTLR4 feature compatibility
- ✅ Complex nesting structures
- ✅ Left recursion detection
- ✅ Unicode support

### Test File

`tests/test_grammars_v4_compatibility.rs` - 14 comprehensive tests

---

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Categories

```bash
# Integration tests
cargo test --test integration_test

# Compatibility tests
cargo test --test test_antlr4_compatibility
cargo test --test test_grammars_v4_compatibility

# Feature tests
cargo test --test test_rule_features
cargo test --test test_named_actions

# Example tests
cargo test --test test_all_examples
cargo test --test test_advanced_examples
```

### Run with Verbose Output

```bash
cargo test -- --nocapture
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Tests with Logging

```bash
RUST_LOG=info cargo test
```

### Property-Based Testing

```bash
cargo test --test property_based_tests
```

### Fuzzing Tests

```bash
cargo test --test fuzzing_tests
```

---

## Test Quality Metrics

### Coverage

- **Unit Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **E2E Tests**: Full pipeline testing
- **Compatibility Tests**: ANTLR4 compatibility verification
- **Property Tests**: Random valid grammar generation
- **Fuzzing Tests**: Malformed input robustness

### Performance

- **Large File Tests**: GB+ input handling
- **Memory Profiling**: Memory leak detection
- **Benchmarks**: Performance regression prevention

---

## Adding New Tests

### Guidelines

1. **Test File Organization**: Place tests in appropriate test file
2. **Test Naming**: Use descriptive names (`test_feature_scenario`)
3. **Test Structure**: Arrange-Act-Assert pattern
4. **Test Coverage**: Test both success and failure cases
5. **Test Documentation**: Add comments for complex tests

### Example Test

```rust
#[test]
fn test_feature_example() {
    // Arrange
    let grammar_source = r#"
        grammar Test;
        rule: ID;
        ID: [a-zA-Z]+;
    "#;
    
    // Act
    let lexer = Lexer::new(grammar_source, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().unwrap();
    
    // Assert
    assert_eq!(grammar.name, "Test");
    assert_eq!(grammar.parser_rules().len(), 1);
}
```

---

## Continuous Integration

### CI Pipeline

Tests run automatically on:
- Pull requests
- Commits to main branch
- Nightly builds

### Test Requirements

- ✅ All tests must pass
- ✅ No warnings allowed
- ✅ Code coverage maintained
- ✅ Performance benchmarks pass

---

**Last Updated**: 2025-01-XX
**Status**: Consolidated from individual testing documents

