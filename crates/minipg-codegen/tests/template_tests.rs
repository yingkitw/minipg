//! Template engine tests.

use minipg::codegen::template::Template;
use std::collections::HashMap;

#[test]
fn test_template_single_var() {
    let template = Template::new("Hello {{name}}!");
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "World".to_string());
    
    assert_eq!(template.render(&vars), "Hello World!");
}

#[test]
fn test_template_multiple_vars() {
    let template = Template::new("{{greeting}} {{name}}!");
    let mut vars = HashMap::new();
    vars.insert("greeting".to_string(), "Hello".to_string());
    vars.insert("name".to_string(), "World".to_string());
    
    assert_eq!(template.render(&vars), "Hello World!");
}

#[test]
fn test_template_no_vars() {
    let template = Template::new("Hello World!");
    let vars = HashMap::new();
    
    assert_eq!(template.render(&vars), "Hello World!");
}

#[test]
fn test_template_missing_var() {
    let template = Template::new("Hello {{name}}!");
    let vars = HashMap::new();
    
    // Missing variables are left as-is
    assert_eq!(template.render(&vars), "Hello {{name}}!");
}

#[test]
fn test_template_repeated_var() {
    let template = Template::new("{{name}} says hello to {{name}}");
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "Alice".to_string());
    
    assert_eq!(template.render(&vars), "Alice says hello to Alice");
}
