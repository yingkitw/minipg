//! Rule body generation for parser code.
//!
//! This module generates the actual parsing logic for rules based on
//! their alternatives and elements, including error recovery.

use crate::ast::{Alternative, Element, Rule};
use std::collections::HashSet;

/// Context for rule body generation
pub struct RuleBodyContext {
    pub indent: usize,
    pub current_token_var: String,
    pub peek_token_var: String,
    pub error_recovery: bool,
    pub visited_rules: HashSet<String>,
}

impl RuleBodyContext {
    pub fn new() -> Self {
        Self {
            indent: 2,
            current_token_var: "current_token".to_string(),
            peek_token_var: "peek_token".to_string(),
            error_recovery: true,
            visited_rules: HashSet::new(),
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    pub fn with_token_vars(mut self, current: String, peek: String) -> Self {
        self.current_token_var = current;
        self.peek_token_var = peek;
        self
    }
}

impl Default for RuleBodyContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate Rust code for a rule body
pub fn generate_rust_rule_body(rule: &Rule, ctx: &mut RuleBodyContext) -> String {
    let mut code = String::new();
    
    if ctx.visited_rules.contains(&rule.name) {
        // Prevent infinite recursion
        code.push_str(&format!("{}// Recursive rule: {}\n", " ".repeat(ctx.indent), rule.name));
        code.push_str(&format!("{}Err(ParseError::new(\"Recursive rule detected\", self.position))\n", " ".repeat(ctx.indent)));
        return code;
    }
    
    ctx.visited_rules.insert(rule.name.clone());
    
    // Handle multiple alternatives
    if rule.alternatives.len() > 1 {
        code.push_str(&generate_alternatives(rule, ctx, "rust"));
    } else if let Some(alt) = rule.alternatives.first() {
        code.push_str(&generate_alternative_body(alt, rule, ctx, "rust"));
    } else {
        code.push_str(&format!("{}// Empty rule\n", " ".repeat(ctx.indent)));
        code.push_str(&format!("{}Ok(AstNode::{}(Box::new(())))\n", 
            " ".repeat(ctx.indent),
            to_pascal_case(&rule.name)));
    }
    
    ctx.visited_rules.remove(&rule.name);
    code
}

/// Generate code for multiple alternatives (if/else or match)
fn generate_alternatives(rule: &Rule, ctx: &mut RuleBodyContext, lang: &str) -> String {
    let mut code = String::new();
    let indent_str = " ".repeat(ctx.indent);
    
    if lang == "rust" {
        // Try each alternative in order (first match wins)
        for (i, alt) in rule.alternatives.iter().enumerate() {
            if i == 0 {
                code.push_str(&format!("{}// Try first alternative\n", indent_str));
                code.push_str(&format!("{}let start_pos = self.position;\n", indent_str));
                code.push_str(&format!("{}match {{\n", indent_str));
                
                // Save position for backtracking
                code.push_str(&format!("{}    let saved_pos = self.position;\n", indent_str));
                ctx.indent = ctx.indent + 4;
                code.push_str(&generate_alternative_body(alt, rule, ctx, lang));
                ctx.indent = ctx.indent - 4;
                
                // If successful, return
                code.push_str(&format!("{}    Ok(result) => return Ok(result),\n", indent_str));
                code.push_str(&format!("{}    Err(_) => {{\n", indent_str));
                code.push_str(&format!("{}        self.position = saved_pos;\n", indent_str));
                code.push_str(&format!("{}        // Try next alternative\n", indent_str));
            } else {
                code.push_str(&format!("{}    match {{\n", indent_str));
                code.push_str(&format!("{}        let saved_pos = self.position;\n", indent_str));
                ctx.indent = ctx.indent + 8;
                code.push_str(&generate_alternative_body(alt, rule, ctx, lang));
                ctx.indent = ctx.indent - 8;
                code.push_str(&format!("{}        Ok(result) => return Ok(result),\n", indent_str));
                code.push_str(&format!("{}        Err(_) => {{\n", indent_str));
                code.push_str(&format!("{}            self.position = saved_pos;\n", indent_str));
            }
        }
        
        // Close all nested blocks
        for _ in 0..rule.alternatives.len() {
            code.push_str(&format!("{}    }}\n", indent_str));
        }
        
        // All alternatives failed
        code.push_str(&format!("{}Err(ParseError::new(\n", indent_str));
        code.push_str(&format!("{}    format!(\"Expected one of alternatives in rule '{}'\"),\n", 
            indent_str, rule.name));
        code.push_str(&format!("{}    start_pos\n", indent_str));
        code.push_str(&format!("{}))\n", indent_str));
    }
    
    code
}

/// Generate code for a single alternative body
fn generate_alternative_body(alt: &Alternative, rule: &Rule, ctx: &mut RuleBodyContext, lang: &str) -> String {
    let mut code = String::new();
    let indent_str = " ".repeat(ctx.indent);
    
    // Generate code for each element in sequence
    let mut collected_vars = Vec::new();
    let mut list_vars = Vec::new();
    
    for (i, element) in alt.elements.iter().enumerate() {
        let elem_code = generate_element_code(element, rule, ctx, lang, i);
        
        // Check if element has a label
        match element {
            Element::RuleRef { label, is_list, .. } => {
                if let Some(lbl) = label {
                    if *is_list {
                        list_vars.push((lbl.clone(), elem_code));
                    } else {
                        collected_vars.push((lbl.clone(), elem_code));
                    }
                } else {
                    code.push_str(&elem_code);
                }
            }
            Element::Terminal { label, is_list, .. } |
            Element::StringLiteral { label, is_list, .. } => {
                if let Some(lbl) = label {
                    if *is_list {
                        list_vars.push((lbl.clone(), elem_code));
                    } else {
                        collected_vars.push((lbl.clone(), elem_code));
                    }
                } else {
                    code.push_str(&elem_code);
                }
            }
            _ => {
                code.push_str(&elem_code);
            }
        }
    }
    
    // Build result node
    if lang == "rust" {
        code.push_str(&format!("{}Ok(AstNode::{}(Box::new(())))\n", 
            indent_str,
            to_pascal_case(&rule.name)));
    }
    
    code
}

/// Generate code for a single element
fn generate_element_code(element: &Element, rule: &Rule, ctx: &mut RuleBodyContext, lang: &str, _index: usize) -> String {
    let mut code = String::new();
    let indent_str = " ".repeat(ctx.indent);
    
    match element {
        Element::RuleRef { name, label, is_list } => {
            if lang == "rust" {
                let method_name = format!("parse_{}", name);
                if *is_list {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}{}.push(self.{}()?);\n", 
                            indent_str, lbl, method_name));
                    } else {
                        code.push_str(&format!("{}let _ = self.{}()?;\n", indent_str, method_name));
                    }
                } else {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}let {} = self.{}()?;\n", 
                            indent_str, lbl, method_name));
                    } else {
                        code.push_str(&format!("{}self.{}()?;\n", indent_str, method_name));
                    }
                }
            }
        }
        Element::Terminal { value, label, is_list } => {
            if lang == "rust" {
                // Check current token at position
                code.push_str(&format!("{}if self.position < self.tokens.len() && self.tokens[self.position].kind == TokenKind::{} {{\n", 
                    indent_str, value));
                
                if *is_list {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}{}.push(self.tokens[self.position].clone());\n", 
                            " ".repeat(ctx.indent + 4), lbl));
                    }
                } else {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}let {} = self.tokens[self.position].clone();\n", 
                            " ".repeat(ctx.indent + 4), lbl));
                    }
                }
                
                code.push_str(&format!("{}self.position += 1;\n", " ".repeat(ctx.indent + 4)));
                code.push_str(&format!("{}}} else {{\n", indent_str));
                code.push_str(&format!("{}    return Err(ParseError::new(\n", indent_str));
                code.push_str(&format!("{}        format!(\"Expected token: {}\"),\n", indent_str, value));
                code.push_str(&format!("{}        self.position\n", indent_str));
                code.push_str(&format!("{}    ));\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        Element::StringLiteral { value, label, is_list } => {
            // Similar to Terminal but check string literal
            if lang == "rust" {
                let escaped_value = escape_string(value);
                code.push_str(&format!("{}if let Some(TokenKind::StringLiteral(s)) = self.{}.kind.as_string_literal() {{\n", 
                    indent_str, ctx.current_token_var));
                code.push_str(&format!("{}    if s == \"{}\" {{\n", 
                    " ".repeat(ctx.indent + 4), escaped_value));
                
                if *is_list {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}{}.push(self.{}.clone());\n", 
                            " ".repeat(ctx.indent + 8), lbl, ctx.current_token_var));
                    }
                } else {
                    if let Some(lbl) = label {
                        code.push_str(&format!("{}let {} = self.{}.clone();\n", 
                            " ".repeat(ctx.indent + 8), lbl, ctx.current_token_var));
                    }
                }
                
                code.push_str(&format!("{}        self.advance();\n", " ".repeat(ctx.indent + 8)));
                code.push_str(&format!("{}    }} else {{\n", " ".repeat(ctx.indent + 4)));
                code.push_str(&format!("{}        return Err(ParseError::new(\n", indent_str));
                code.push_str(&format!("{}            format!(\"Expected string: '{}'\"),\n", 
                    indent_str, escaped_value));
                code.push_str(&format!("{}            self.position\n", indent_str));
                code.push_str(&format!("{}        ));\n", indent_str));
                code.push_str(&format!("{}    }}\n", " ".repeat(ctx.indent + 4)));
                code.push_str(&format!("{}}} else {{\n", indent_str));
                code.push_str(&format!("{}    return Err(ParseError::new(\n", indent_str));
                code.push_str(&format!("{}        format!(\"Expected string literal: '{}'\"),\n", 
                    indent_str, escaped_value));
                code.push_str(&format!("{}        self.position\n", indent_str));
                code.push_str(&format!("{}    ));\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        Element::Optional { element, .. } => {
            if lang == "rust" {
                code.push_str(&format!("{}// Optional element\n", indent_str));
                code.push_str(&format!("{}let saved_pos = self.position;\n", indent_str));
                let inner_code = generate_element_code(element, rule, ctx, lang, 0);
                // Wrap in match to make it optional
                code.push_str(&format!("{}match {{\n", indent_str));
                code.push_str(&inner_code);
                code.push_str(&format!("{}    Ok(_) => {{}},\n", indent_str));
                code.push_str(&format!("{}    Err(_) => {{\n", indent_str));
                code.push_str(&format!("{}        self.position = saved_pos;\n", indent_str));
                code.push_str(&format!("{}    }}\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        Element::ZeroOrMore { element, .. } => {
            if lang == "rust" {
                code.push_str(&format!("{}// Zero or more\n", indent_str));
                code.push_str(&format!("{}loop {{\n", indent_str));
                code.push_str(&format!("{}    let saved_pos = self.position;\n", indent_str));
                code.push_str(&format!("{}    match {{\n", indent_str));
                ctx.indent = ctx.indent + 8;
                let inner_code = generate_element_code(element, rule, ctx, lang, 0);
                ctx.indent = ctx.indent - 8;
                code.push_str(&inner_code);
                code.push_str(&format!("{}        Ok(_) => continue,\n", indent_str));
                code.push_str(&format!("{}        Err(_) => {{\n", indent_str));
                code.push_str(&format!("{}            self.position = saved_pos;\n", indent_str));
                code.push_str(&format!("{}            break;\n", indent_str));
                code.push_str(&format!("{}        }}\n", indent_str));
                code.push_str(&format!("{}    }}\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        Element::OneOrMore { element, .. } => {
            if lang == "rust" {
                code.push_str(&format!("{}// One or more\n", indent_str));
                // First one is required
                code.push_str(&generate_element_code(element, rule, ctx, lang, 0));
                // Then zero or more
                code.push_str(&format!("{}while {{\n", indent_str));
                code.push_str(&format!("{}    let saved_pos = self.position;\n", indent_str));
                code.push_str(&format!("{}    match {{\n", indent_str));
                ctx.indent = ctx.indent + 8;
                let inner_code = generate_element_code(element, rule, ctx, lang, 0);
                ctx.indent = ctx.indent - 8;
                code.push_str(&inner_code);
                code.push_str(&format!("{}        Ok(_) => true,\n", indent_str));
                code.push_str(&format!("{}        Err(_) => {{\n", indent_str));
                code.push_str(&format!("{}            self.position = saved_pos;\n", indent_str));
                code.push_str(&format!("{}            false\n", indent_str));
                code.push_str(&format!("{}        }}\n", indent_str));
                code.push_str(&format!("{}    }}\n", indent_str));
                code.push_str(&format!("{}}} {{}}\n", indent_str));
            }
        }
        Element::Group { alternatives } => {
            if alternatives.len() > 1 {
                // Multiple alternatives in group
                code.push_str(&generate_group_alternatives(alternatives, rule, ctx, lang));
            } else if let Some(alt) = alternatives.first() {
                // Single alternative - just parse elements
                for elem in &alt.elements {
                    code.push_str(&generate_element_code(elem, rule, ctx, lang, 0));
                }
            }
        }
        Element::Eof => {
            if lang == "rust" {
                code.push_str(&format!("{}if self.{}.kind == TokenKind::Eof {{\n", 
                    indent_str, ctx.current_token_var));
                code.push_str(&format!("{}    // EOF reached\n", indent_str));
                code.push_str(&format!("{}}} else {{\n", indent_str));
                code.push_str(&format!("{}    return Err(ParseError::new(\n", indent_str));
                code.push_str(&format!("{}        \"Expected EOF\".to_string(),\n", indent_str));
                code.push_str(&format!("{}        self.position\n", indent_str));
                code.push_str(&format!("{}    ));\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        Element::Action { code: action_code, .. } => {
            code.push_str(&format!("{}{}\n", indent_str, action_code));
        }
        Element::Predicate { code: pred_code, .. } => {
            if lang == "rust" {
                code.push_str(&format!("{}if !({}) {{\n", indent_str, pred_code));
                code.push_str(&format!("{}    return Err(ParseError::new(\n", indent_str));
                code.push_str(&format!("{}        \"Semantic predicate failed\".to_string(),\n", indent_str));
                code.push_str(&format!("{}        self.position\n", indent_str));
                code.push_str(&format!("{}    ));\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        _ => {
            code.push_str(&format!("{}// TODO: Handle element type\n", indent_str));
        }
    }
    
    code
}

fn generate_group_alternatives(alts: &[Alternative], rule: &Rule, ctx: &mut RuleBodyContext, lang: &str) -> String {
    // Similar to generate_alternatives but for nested group
    let mut code = String::new();
    let indent_str = " ".repeat(ctx.indent);
    
            if lang == "rust" && alts.len() > 1 {
                for (i, alt) in alts.iter().enumerate() {
                    if i == 0 {
                        code.push_str(&format!("{}let saved_pos = self.position;\n", indent_str));
                    } else {
                        code.push_str(&format!("{}self.position = saved_pos;\n", indent_str));
                    }
                    code.push_str(&format!("{}match {{\n", indent_str));
                    ctx.indent = ctx.indent + 4;
                    let alt_code = generate_alternative_body(alt, rule, ctx, lang);
                    ctx.indent = ctx.indent - 4;
            code.push_str(&alt_code);
            code.push_str(&format!("{}    Ok(result) => return Ok(result),\n", indent_str));
            code.push_str(&format!("{}    Err(_) => {{\n", indent_str));
        }
        // Close all blocks
        for _ in 0..alts.len() {
            code.push_str(&format!("{}    }}\n", indent_str));
        }
        code.push_str(&format!("{}Err(ParseError::new(\"No alternative matched\".to_string(), saved_pos))\n", indent_str));
    } else if let Some(alt) = alts.first() {
        for elem in &alt.elements {
            code.push_str(&generate_element_code(elem, rule, ctx, lang, 0));
        }
    }
    
    code
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

fn escape_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

