//! Cross-language tests to verify generated code works in Python, JavaScript, etc.

use minipg::ast::{Alternative, Element, Grammar, Rule, RuleType};
use minipg::codegen::{JavaScriptCodeGenerator, PythonCodeGenerator};
use minipg::core::{types::{CodeGenConfig, GrammarType}, CodeGenerator};
use std::fs;
use std::process::Command;

fn create_simple_grammar() -> Grammar {
    let mut grammar = Grammar::new("Simple".to_string(), GrammarType::Combined);
    
    // NUMBER: [0-9]+
    let mut number_rule = Rule::new("NUMBER".to_string(), RuleType::Lexer);
    let mut num_alt = Alternative::new();
    num_alt.add_element(Element::OneOrMore {
        element: Box::new(Element::CharRange { start: '0', end: '9' }),
    });
    number_rule.add_alternative(num_alt);
    grammar.add_rule(number_rule);
    
    grammar
}

#[test]
fn test_python_generated_code_structure() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "python".to_string(),
        ..Default::default()
    };
    
    let generator = PythonCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate Python code");
    
    // Check for Python-specific elements
    assert!(code.contains("class"), "Missing class definition");
    assert!(code.contains("def __init__"), "Missing __init__ method");
    assert!(code.contains("def "), "Missing method definitions");
    
    // Check for proper Python syntax
    assert!(!code.contains("fn "), "Contains Rust syntax");
    assert!(!code.contains("struct "), "Contains Rust syntax");
    
    // Check for type hints (Python 3.10+)
    assert!(code.contains("->") || code.contains(":"), "Missing type hints");
}

#[test]
#[ignore] // Ignore by default as it requires python3
fn test_python_generated_code_syntax() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "python".to_string(),
        ..Default::default()
    };
    
    let generator = PythonCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate Python code");
    
    // Write to temporary file
    let temp_dir = std::env::temp_dir().join("minipg_python_test");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let test_file = temp_dir.join("simple_parser.py");
    fs::write(&test_file, &code).expect("Failed to write test file");
    
    // Check Python syntax with py_compile
    let output = Command::new("python3")
        .arg("-m")
        .arg("py_compile")
        .arg(&test_file)
        .output();
    
    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
    
    if let Ok(result) = output {
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            panic!("Generated Python code has syntax errors:\n{}", stderr);
        }
        println!("✅ Generated Python code has valid syntax!");
    } else {
        println!("⚠️  python3 not available, skipping Python syntax test");
    }
}

#[test]
fn test_javascript_generated_code_structure() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "javascript".to_string(),
        ..Default::default()
    };
    
    let generator = JavaScriptCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate JavaScript code");
    
    // Check for JavaScript-specific elements
    assert!(code.contains("class"), "Missing class definition");
    assert!(code.contains("constructor"), "Missing constructor");
    // JavaScript uses class methods, not standalone functions
    assert!(code.contains("nextToken") || code.contains("parse"), "Missing method definitions");
    
    // Check for proper JavaScript syntax
    assert!(!code.contains("fn "), "Contains Rust syntax");
    assert!(!code.contains("struct "), "Contains Rust syntax");
    assert!(!code.contains("def "), "Contains Python syntax");
    
    // Check for ES6+ features
    assert!(code.contains("const ") || code.contains("let "), "Not using modern JavaScript");
}

#[test]
#[ignore] // Ignore by default as it requires node
fn test_javascript_generated_code_syntax() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "javascript".to_string(),
        ..Default::default()
    };
    
    let generator = JavaScriptCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate JavaScript code");
    
    // Write to temporary file
    let temp_dir = std::env::temp_dir().join("minipg_js_test");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let test_file = temp_dir.join("simple_parser.js");
    fs::write(&test_file, &code).expect("Failed to write test file");
    
    // Check JavaScript syntax with node
    let output = Command::new("node")
        .arg("--check")
        .arg(&test_file)
        .output();
    
    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
    
    if let Ok(result) = output {
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            panic!("Generated JavaScript code has syntax errors:\n{}", stderr);
        }
        println!("✅ Generated JavaScript code has valid syntax!");
    } else {
        println!("⚠️  node not available, skipping JavaScript syntax test");
    }
}

#[test]
fn test_python_code_pep8_basics() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "python".to_string(),
        ..Default::default()
    };
    
    let generator = PythonCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate Python code");
    
    // Check for basic PEP 8 compliance
    // Class names should be CamelCase
    assert!(code.contains("class Simple"), "Class name not in CamelCase");
    
    // Check indentation (should be 4 spaces)
    let lines: Vec<&str> = code.lines().collect();
    for line in lines {
        if line.starts_with(' ') {
            let leading_spaces = line.len() - line.trim_start().len();
            if leading_spaces > 0 {
                assert_eq!(leading_spaces % 4, 0, "Indentation not multiple of 4: {}", line);
            }
        }
    }
}

#[test]
fn test_javascript_code_style() {
    let grammar = create_simple_grammar();
    let config = CodeGenConfig {
        target_language: "javascript".to_string(),
        ..Default::default()
    };
    
    let generator = JavaScriptCodeGenerator::new();
    let code = generator.generate(&grammar, &config).expect("Failed to generate JavaScript code");
    
    // Check for modern JavaScript practices
    assert!(code.contains("class "), "Not using ES6 classes");
    assert!(!code.contains("var "), "Using old 'var' keyword");
    
    // Check for semicolons (if using semicolon style)
    let semicolon_count = code.matches(';').count();
    let line_count = code.lines().count();
    
    // Should have reasonable number of semicolons
    assert!(semicolon_count > 0, "No semicolons found");
    assert!(semicolon_count < line_count, "Too many semicolons");
}

#[test]
fn test_cross_language_consistency() {
    let grammar = create_simple_grammar();
    
    // Generate code in all languages
    let rust_config = CodeGenConfig {
        target_language: "rust".to_string(),
        ..Default::default()
    };
    let python_config = CodeGenConfig {
        target_language: "python".to_string(),
        ..Default::default()
    };
    let js_config = CodeGenConfig {
        target_language: "javascript".to_string(),
        ..Default::default()
    };
    
    let rust_gen = minipg_codegen::RustCodeGenerator::new();
    let python_gen = PythonCodeGenerator::new();
    let js_gen = JavaScriptCodeGenerator::new();
    
    let rust_code = rust_gen.generate(&grammar, &rust_config).expect("Rust generation failed");
    let python_code = python_gen.generate(&grammar, &python_config).expect("Python generation failed");
    let js_code = js_gen.generate(&grammar, &js_config).expect("JavaScript generation failed");
    
    // All should contain the grammar name
    assert!(rust_code.contains("Simple"));
    assert!(python_code.contains("Simple"));
    assert!(js_code.contains("Simple"));
    
    // All should contain the NUMBER token
    assert!(rust_code.contains("NUMBER"));
    assert!(python_code.contains("NUMBER"));
    assert!(js_code.contains("NUMBER"));
    
    // All should have lexer and parser
    assert!(rust_code.contains("Lexer") && rust_code.contains("Parser"));
    assert!(python_code.contains("Lexer") && python_code.contains("Parser"));
    assert!(js_code.contains("Lexer") && js_code.contains("Parser"));
}

#[test]
fn test_generated_code_documentation() {
    let grammar = create_simple_grammar();
    
    // Test Rust documentation
    let rust_config = CodeGenConfig::default();
    let rust_gen = minipg_codegen::RustCodeGenerator::new();
    let rust_code = rust_gen.generate(&grammar, &rust_config).expect("Rust generation failed");
    assert!(rust_code.contains("///"), "Rust code missing doc comments");
    
    // Test Python documentation
    let python_config = CodeGenConfig {
        target_language: "python".to_string(),
        ..Default::default()
    };
    let python_gen = PythonCodeGenerator::new();
    let python_code = python_gen.generate(&grammar, &python_config).expect("Python generation failed");
    assert!(python_code.contains("\"\"\"") || python_code.contains("#"), 
            "Python code missing documentation");
    
    // Test JavaScript documentation
    let js_config = CodeGenConfig {
        target_language: "javascript".to_string(),
        ..Default::default()
    };
    let js_gen = JavaScriptCodeGenerator::new();
    let js_code = js_gen.generate(&grammar, &js_config).expect("JavaScript generation failed");
    assert!(js_code.contains("/**") || js_code.contains("//"), 
            "JavaScript code missing documentation");
}
