//! Action and semantic predicate code generation.
//!
//! This module provides functions for:
//! - Translating action code between languages
//! - Generating semantic predicates in target languages
//! - Wrapping action code with proper syntax

use crate::ast::Element;

/// Translate action code from source language to target language.
pub fn translate_action(code: &str, from_lang: &str, to_lang: &str) -> String {
    // If languages are the same, return as-is
    if from_lang == to_lang {
        return code.to_string();
    }

    match (from_lang, to_lang) {
        // Rust to Python
        ("rust", "python") => translate_rust_to_python(code),
        // Rust to JavaScript
        ("rust", "javascript") => translate_rust_to_javascript(code),
        // Rust to TypeScript
        ("rust", "typescript") => translate_rust_to_typescript(code),
        // Rust to Go
        ("rust", "go") => translate_rust_to_go(code),
        // Default: return as-is with comment
        _ => format!("// TODO: Translate action from {} to {}\n{}", from_lang, to_lang, code),
    }
}

/// Translate an action element to target language code.
pub fn translate_action_element(element: &Element, target_lang: &str) -> Option<String> {
    match element {
        Element::Action { code, language } => {
            let source_lang = language.as_deref().unwrap_or("generic");
            let translated = if source_lang == target_lang {
                code.clone()
            } else if source_lang == "generic" {
                code.clone()
            } else {
                translate_action(code, source_lang, target_lang)
            };
            Some(translated)
        }
        Element::Predicate { code, language } => {
            let source_lang = language.as_deref().unwrap_or("generic");
            let translated = if source_lang == target_lang {
                code.clone()
            } else if source_lang == "generic" {
                code.clone()
            } else {
                translate_action(code, source_lang, target_lang)
            };
            Some(translated)
        }
        _ => None,
    }
}

/// Translate Rust action code to Python.
fn translate_rust_to_python(code: &str) -> String {
    let mut result = code.to_string();

    // Replace self. with self. (same in Python)
    // No change needed

    // Replace Vec::new() with []
    result = result.replace("Vec::new()", "[]");

    // Replace HashMap::new() with {}
    result = result.replace("HashMap::new()", "{}");

    // Replace .push() with .append()
    result = result.replace(".push(", ".append(");

    // Replace .clone() with (Python doesn't need it usually)
    result = result.replace(".clone()", "");

    // Replace .to_string() with str()
    result = result.replace(".to_string()", "");

    // Replace unwrap() - remove it, Python will raise naturally
    result = result.replace(".unwrap()", "");

    // Replace .len() with len() for strings
    result = result.replace("self.input.len()", "len(self.input)");

    // Replace .chars() with iteration
    result = result.replace(".chars()", "");

    // Replace true/false with True/False
    result = result.replace(" true", " True");
    result = result.replace(" false", " False");
    result = result.replace("(true", "(True");
    result = result.replace("(false", "(False");

    result
}

/// Translate Rust action code to JavaScript.
fn translate_rust_to_javascript(code: &str) -> String {
    let mut result = code.to_string();

    // Replace self. with this.
    result = result.replace("self.", "this.");

    // Replace Vec::new() with []
    result = result.replace("Vec::new()", "[]");

    // Replace HashMap::new() with {}
    result = result.replace("HashMap::new()", "{}");

    // Replace .push() with .push() (same in JS)
    // (no change needed)

    // Replace .clone() with spread operator
    result = result.replace(".clone()", "");

    // Replace .to_string() with .toString()
    result = result.replace(".to_string()", ".toString()");

    // Replace unwrap() with optional chaining
    result = result.replace(".unwrap()", "");

    // Replace .len() with .length
    result = result.replace(".len()", ".length");

    // Replace true/false (already correct in JS)
    // No change needed

    // Replace .chars() with iteration
    result = result.replace(".chars()", "");

    result
}

/// Translate Rust action code to TypeScript.
fn translate_rust_to_typescript(code: &str) -> String {
    // TypeScript is similar to JavaScript
    translate_rust_to_javascript(code)
}

/// Translate Rust action code to Go.
fn translate_rust_to_go(code: &str) -> String {
    let mut result = code.to_string();

    // Replace self. with l. (lexer) or p. (parser)
    result = result.replace("self.", "l.");

    // Replace Vec::new() with make([]Token, 0)
    result = result.replace("Vec::new()", "make([]Token, 0)");

    // Replace HashMap::new() with make(map[string]Token)
    result = result.replace("HashMap::new()", "make(map[string]Token)");

    // Replace .push() with append()
    result = result.replace(".push(", "append(");

    // Replace .clone() with (Go doesn't need it)
    result = result.replace(".clone()", "");

    // Replace .to_string() with fmt.Sprintf
    result = result.replace(".to_string()", "");

    // Replace unwrap() with error checking
    result = result.replace(".unwrap()", "");

    // Replace .len() with len()
    result = result.replace(".len()", "");

    // Replace true/false (already correct in Go)
    // No change needed

    result
}

/// Generate semantic predicate code for Rust.
pub fn generate_rust_predicate(code: &str) -> String {
    format!(
        "if !({{ {} }}) {{\n            return Err(ParseError::new(\"Predicate failed\".to_string(), self.position));\n        }}",
        code
    )
}

/// Generate semantic predicate code for Python.
pub fn generate_python_predicate(code: &str) -> String {
    format!(
        "if not ({}):\n    raise ParseError(\"Predicate failed\", self.position)",
        code
    )
}

/// Generate semantic predicate code for JavaScript.
pub fn generate_javascript_predicate(code: &str) -> String {
    format!(
        "if (!({0})) {{\n  throw new ParseError(\"Predicate failed\", this.position);\n}}",
        code
    )
}

/// Generate semantic predicate code for TypeScript.
pub fn generate_typescript_predicate(code: &str) -> String {
    generate_javascript_predicate(code)
}

/// Generate semantic predicate code for Go.
pub fn generate_go_predicate(code: &str) -> String {
    format!(
        "if !({}) {{\n  return nil, &ParseError{{Message: \"Predicate failed\", Position: l.position}}\n}}",
        code
    )
}

/// Generate semantic predicate code for a target language.
pub fn generate_predicate_for_language(code: &str, language: &str) -> String {
    match language {
        "rust" => generate_rust_predicate(code),
        "python" => generate_python_predicate(code),
        "javascript" => generate_javascript_predicate(code),
        "typescript" => generate_typescript_predicate(code),
        "go" => generate_go_predicate(code),
        _ => format!("// TODO: Generate predicate for {}\n{}", language, code),
    }
}

/// Generate action code wrapper for Rust.
pub fn generate_rust_action(code: &str) -> String {
    format!("{{ {} }}", code)
}

/// Generate action code wrapper for Python.
pub fn generate_python_action(code: &str) -> String {
    // Indent the code for Python
    let indented = code
        .lines()
        .map(|line| format!("    {}", line))
        .collect::<Vec<_>>()
        .join("\n");
    indented
}

/// Generate action code wrapper for JavaScript.
pub fn generate_javascript_action(code: &str) -> String {
    format!("{{ {} }}", code)
}

/// Generate action code wrapper for TypeScript.
pub fn generate_typescript_action(code: &str) -> String {
    generate_javascript_action(code)
}

/// Generate action code wrapper for Go.
pub fn generate_go_action(code: &str) -> String {
    format!("{{ {} }}", code)
}

/// Generate action code wrapper for a target language.
pub fn generate_action_for_language(code: &str, language: &str) -> String {
    match language {
        "rust" => generate_rust_action(code),
        "python" => generate_python_action(code),
        "javascript" => generate_javascript_action(code),
        "typescript" => generate_typescript_action(code),
        "go" => generate_go_action(code),
        _ => format!("// TODO: Generate action for {}\n{}", language, code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_to_python_translation() {
        let rust_code = "self.validate()";
        let python_code = translate_rust_to_python(rust_code);
        assert!(python_code.contains("validate"));
    }

    #[test]
    fn test_rust_to_javascript_translation() {
        let rust_code = "self.validate()";
        let js_code = translate_rust_to_javascript(rust_code);
        assert!(js_code.contains("this.validate"));
    }

    #[test]
    fn test_predicate_generation() {
        let code = "x > 0";
        let rust_pred = generate_rust_predicate(code);
        assert!(rust_pred.contains("x > 0"));
        assert!(rust_pred.contains("Predicate failed"));
    }

    #[test]
    fn test_action_generation() {
        let code = "x = 1";
        let rust_action = generate_rust_action(code);
        assert_eq!(rust_action, "{ x = 1 }");
    }

    #[test]
    fn test_translate_action_element_action() {
        let elem = Element::action_with_language("self.validate()".to_string(), "rust".to_string());
        let result = translate_action_element(&elem, "javascript");
        assert!(result.is_some());
        let translated = result.unwrap();
        assert!(translated.contains("this.validate"));
    }

    #[test]
    fn test_translate_action_element_predicate() {
        let elem = Element::predicate("x > 0".to_string());
        let result = translate_action_element(&elem, "python");
        assert!(result.is_some());
    }

    #[test]
    fn test_generate_predicate_for_language_rust() {
        let code = "x > 0";
        let rust_pred = generate_predicate_for_language(code, "rust");
        assert!(rust_pred.contains("x > 0"));
        assert!(rust_pred.contains("Predicate failed"));
    }

    #[test]
    fn test_generate_predicate_for_language_python() {
        let code = "x > 0";
        let python_pred = generate_predicate_for_language(code, "python");
        assert!(python_pred.contains("x > 0"));
        assert!(python_pred.contains("if not"));
    }

    #[test]
    fn test_generate_predicate_for_language_javascript() {
        let code = "x > 0";
        let js_pred = generate_predicate_for_language(code, "javascript");
        assert!(js_pred.contains("x > 0"));
        assert!(js_pred.contains("if (!"));
    }

    #[test]
    fn test_generate_action_for_language_rust() {
        let code = "x = 1";
        let rust_action = generate_action_for_language(code, "rust");
        assert_eq!(rust_action, "{ x = 1 }");
    }

    #[test]
    fn test_generate_action_for_language_python() {
        let code = "x = 1";
        let python_action = generate_action_for_language(code, "python");
        assert!(python_action.contains("x = 1"));
    }

    #[test]
    fn test_generate_action_for_language_javascript() {
        let code = "x = 1";
        let js_action = generate_action_for_language(code, "javascript");
        assert_eq!(js_action, "{ x = 1 }");
    }

    #[test]
    fn test_rust_to_python_vec_translation() {
        let rust_code = "let v = Vec::new();";
        let python_code = translate_rust_to_python(rust_code);
        assert!(python_code.contains("[]"));
    }

    #[test]
    fn test_rust_to_javascript_self_translation() {
        let rust_code = "self.position += 1;";
        let js_code = translate_rust_to_javascript(rust_code);
        assert!(js_code.contains("this.position"));
    }

    #[test]
    fn test_rust_to_python_bool_translation() {
        let rust_code = "if true { }";
        let python_code = translate_rust_to_python(rust_code);
        assert!(python_code.contains("True"));
    }
}
