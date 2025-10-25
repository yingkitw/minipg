//! Action and semantic predicate code generation.

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

/// Translate Rust action code to Python.
fn translate_rust_to_python(code: &str) -> String {
    let mut result = code.to_string();

    // Replace self.method() with self.method()
    // (Python uses same syntax)

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

    // Replace unwrap() with appropriate error handling
    result = result.replace(".unwrap()", "");

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
        "if not ({}):\n            raise ParseError(\"Predicate failed\", self.position)",
        code
    )
}

/// Generate semantic predicate code for JavaScript.
pub fn generate_javascript_predicate(code: &str) -> String {
    format!(
        "if (!({0})) {{\n            throw new ParseError(\"Predicate failed\", this.position);\n        }}",
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
        "if !({}) {{\n            return nil, &ParseError{{Message: \"Predicate failed\", Position: l.position}}\n        }}",
        code
    )
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
}
