//! Simple pattern matching helpers for code generators.
//!
//! This module provides utilities to generate basic pattern matching code
//! for lexer tokenization in various target languages. It's a simplified
//! version compared to the full DFA implementation but provides functional
//! pattern matching for common cases.

use crate::ast::{Element, Rule};

/// Generate simple pattern matching code for a lexer rule
pub fn generate_simple_pattern_match(rule: &Rule, language: &str) -> String {
    let mut code = String::new();
    
    match language {
        "go" => {
            code.push_str(&format!("func (l *{}Lexer) match_{}() bool {{\n", 
                "Grammar", // Will be replaced by caller
                rule.name.to_lowercase()));
            code.push_str("\tstartPos := l.position\n");
            
            // Generate pattern matching logic
            if let Some(alt) = rule.alternatives.first() {
                for element in &alt.elements {
                    code.push_str(&generate_element_match_go(element, "\t"));
                }
            }
            
            code.push_str("\t// Pattern matched if we advanced position\n");
            code.push_str("\treturn l.position > startPos\n");
            code.push_str("}\n");
        }
        "c" => {
            code.push_str(&format!("static int lexer_match_{}(Lexer *lexer) {{\n", rule.name.to_lowercase()));
            code.push_str("    size_t start_pos = lexer->position;\n");
            
            if let Some(alt) = rule.alternatives.first() {
                for element in &alt.elements {
                    code.push_str(&generate_element_match_c(element, "    "));
                }
            }
            
            code.push_str("    return lexer->position > start_pos ? 1 : 0;\n");
            code.push_str("}\n");
        }
        "cpp" | "c++" => {
            code.push_str(&format!("bool Lexer::match_{}() {{\n", rule.name.to_lowercase()));
            code.push_str("    size_t start_pos = position;\n");
            
            if let Some(alt) = rule.alternatives.first() {
                for element in &alt.elements {
                    code.push_str(&generate_element_match_cpp(element, "    "));
                }
            }
            
            code.push_str("    return position > start_pos;\n");
            code.push_str("}\n");
        }
        "java" => {
            code.push_str(&format!("    private boolean match_{}() {{\n", rule.name.to_lowercase()));
            code.push_str("        int startPos = position;\n");
            
            if let Some(alt) = rule.alternatives.first() {
                for element in &alt.elements {
                    code.push_str(&generate_element_match_java(element, "        "));
                }
            }
            
            code.push_str("        return position > startPos;\n");
            code.push_str("    }\n");
        }
        _ => {
            code.push_str("    // Pattern matching not implemented for this language\n");
        }
    }
    
    code
}

fn generate_element_match_go(element: &Element, indent: &str) -> String {
    let mut code = String::new();
    
    match element {
        Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
            code.push_str(&format!("{}// Match string: {}\n", indent, value));
            code.push_str(&format!("{}if l.position + {} <= len(l.input) {{\n", indent, value.len()));
            code.push_str(&format!("{}    if string(l.input[l.position:l.position+{}]) == \"{}\" {{\n", indent, value.len(), value));
            code.push_str(&format!("{}        l.position += {}\n", indent, value.len()));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::CharRange { start, end } => {
            code.push_str(&format!("{}// Match char range: {} to {}\n", indent, start, end));
            code.push_str(&format!("{}if l.position < len(l.input) {{\n", indent));
            code.push_str(&format!("{}    ch := l.input[l.position]\n", indent));
            code.push_str(&format!("{}    if ch >= '{}' && ch <= '{}' {{\n", indent, start, end));
            code.push_str(&format!("{}        l.position++\n", indent));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::CharClass { negated, ranges } => {
            code.push_str(&format!("{}// Match char class{}\n", indent, if *negated { " (negated)" } else { "" }));
            code.push_str(&format!("{}if l.position < len(l.input) {{\n", indent));
            code.push_str(&format!("{}    ch := l.input[l.position]\n", indent));
            code.push_str(&format!("{}    matched := false\n", indent));
            
            for (start, end) in ranges {
                if start == end {
                    code.push_str(&format!("{}    if ch == '{}' {{\n", indent, start));
                } else {
                    code.push_str(&format!("{}    if ch >= '{}' && ch <= '{}' {{\n", indent, start, end));
                }
                code.push_str(&format!("{}        matched = true\n", indent));
                code.push_str(&format!("{}    }}\n", indent));
            }
            
            if *negated {
                code.push_str(&format!("{}    if !matched {{\n", indent));
            } else {
                code.push_str(&format!("{}    if matched {{\n", indent));
            }
            code.push_str(&format!("{}        l.position++\n", indent));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::OneOrMore { element, .. } => {
            code.push_str(&format!("{}// Match one or more\n", indent));
            code.push_str(&format!("{}if !(\n", indent));
            let inner = generate_element_match_go(element, &format!("{}    ", indent));
            code.push_str(&inner);
            code.push_str(&format!("{}) {{\n", indent));
            code.push_str(&format!("{}    return false\n", indent));
            code.push_str(&format!("{}}}\n", indent));
            code.push_str(&format!("{}for {{\n", indent));
            code.push_str(&format!("{}    savedPos := l.position\n", indent));
            let inner = generate_element_match_go(element, &format!("{}    ", indent));
            code.push_str(&inner);
            code.push_str(&format!("{}    if l.position == savedPos {{\n", indent));
            code.push_str(&format!("{}        break\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::ZeroOrMore { element, .. } => {
            code.push_str(&format!("{}// Match zero or more\n", indent));
            code.push_str(&format!("{}for {{\n", indent));
            code.push_str(&format!("{}    savedPos := l.position\n", indent));
            let inner = generate_element_match_go(element, &format!("{}    ", indent));
            code.push_str(&inner);
            code.push_str(&format!("{}    if l.position == savedPos {{\n", indent));
            code.push_str(&format!("{}        break\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::Optional { element, .. } => {
            code.push_str(&format!("{}// Match optional\n", indent));
            code.push_str(&format!("{}savedPos := l.position\n", indent));
            let inner = generate_element_match_go(element, indent);
            code.push_str(&inner);
            code.push_str(&format!("{}if l.position == savedPos {{\n", indent));
            code.push_str(&format!("{}    l.position = savedPos\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        _ => {
            code.push_str(&format!("{}// TODO: Handle element type\n", indent));
        }
    }
    
    code
}

fn generate_element_match_c(element: &Element, indent: &str) -> String {
    let mut code = String::new();
    
    match element {
        Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
            code.push_str(&format!("{}// Match string: {}\n", indent, value));
            code.push_str(&format!("{}if (lexer->position + {} <= lexer->length) {{\n", indent, value.len()));
            code.push_str(&format!("{}    if (strncmp(lexer->input + lexer->position, \"{}\", {}) == 0) {{\n", indent, value, value.len()));
            code.push_str(&format!("{}        lexer->position += {};\n", indent, value.len()));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return 0;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return 0;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::CharRange { start, end } => {
            code.push_str(&format!("{}// Match char range: {} to {}\n", indent, start, end));
            code.push_str(&format!("{}if (lexer->position < lexer->length) {{\n", indent));
            code.push_str(&format!("{}    char ch = lexer->input[lexer->position];\n", indent));
            code.push_str(&format!("{}    if (ch >= '{}' && ch <= '{}') {{\n", indent, start, end));
            code.push_str(&format!("{}        lexer->position++;\n", indent));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return 0;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return 0;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        _ => {
            code.push_str(&format!("{}// TODO: Handle element type\n", indent));
        }
    }
    
    code
}

fn generate_element_match_java(element: &Element, indent: &str) -> String {
    let mut code = String::new();
    
    match element {
        Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
            code.push_str(&format!("{}// Match string: {}\n", indent, value));
            code.push_str(&format!("{}if (position + {} <= input.length()) {{\n", indent, value.len()));
            code.push_str(&format!("{}    if (input.substring(position, position + {}).equals(\"{}\")) {{\n", indent, value.len(), value));
            code.push_str(&format!("{}        position += {};\n", indent, value.len()));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::CharRange { start, end } => {
            code.push_str(&format!("{}// Match char range: {} to {}\n", indent, start, end));
            code.push_str(&format!("{}if (position < input.length()) {{\n", indent));
            code.push_str(&format!("{}    char ch = input.charAt(position);\n", indent));
            code.push_str(&format!("{}    if (ch >= '{}' && ch <= '{}') {{\n", indent, start, end));
            code.push_str(&format!("{}        position++;\n", indent));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        _ => {
            code.push_str(&format!("{}// TODO: Handle element type\n", indent));
        }
    }
    
    code
}

/// Generate C++ code for matching an element
pub fn generate_element_match_cpp(element: &Element, indent: &str) -> String {
    let mut code = String::new();
    
    match element {
        Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
            code.push_str(&format!("{}// Match string: {}\n", indent, value));
            code.push_str(&format!("{}if (position + {} <= input.length()) {{\n", indent, value.len()));
            code.push_str(&format!("{}    if (input.substr(position, {}) == \"{}\") {{\n", indent, value.len(), value));
            code.push_str(&format!("{}        position += {};\n", indent, value.len()));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        Element::CharRange { start, end } => {
            code.push_str(&format!("{}// Match char range: {} to {}\n", indent, start, end));
            code.push_str(&format!("{}if (position < input.length()) {{\n", indent));
            code.push_str(&format!("{}    char ch = input[position];\n", indent));
            code.push_str(&format!("{}    if (ch >= '{}' && ch <= '{}') {{\n", indent, start, end));
            code.push_str(&format!("{}        position++;\n", indent));
            code.push_str(&format!("{}    }} else {{\n", indent));
            code.push_str(&format!("{}        return false;\n", indent));
            code.push_str(&format!("{}    }}\n", indent));
            code.push_str(&format!("{}}} else {{\n", indent));
            code.push_str(&format!("{}    return false;\n", indent));
            code.push_str(&format!("{}}}\n", indent));
        }
        _ => {
            code.push_str(&format!("{}// TODO: Handle element type\n", indent));
        }
    }
    
    code
}
