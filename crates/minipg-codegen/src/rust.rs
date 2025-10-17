//! Rust code generator.

use crate::dfa::{DfaBuilder, generate_dfa_match};
use crate::lookup_table::{LookupTableBuilder, generate_optimized_char_match};
use crate::visitor_gen::{generate_listener, generate_visitor};
use minipg_ast::{Grammar, Rule};
use minipg_core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// Rust code generator.
pub struct RustCodeGenerator;

impl RustCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_parser_struct(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        
        code.push_str(&format!("/// Parser for {} grammar.\n", grammar.name));
        code.push_str("#[derive(Debug)]\n");
        code.push_str(&format!("pub struct {}Parser {{\n", grammar.name));
        code.push_str("    tokens: Vec<Token>,\n");
        code.push_str("    position: usize,\n");
        code.push_str("}\n\n");
        
        code
    }

    fn generate_parser_impl(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        
        code.push_str(&format!("impl {}Parser {{\n", grammar.name));
        code.push_str("    #[inline]\n");
        code.push_str("    pub fn new(tokens: Vec<Token>) -> Self {\n");
        code.push_str("        Self { tokens, position: 0 }\n");
        code.push_str("    }\n\n");
        
        // Generate methods for each parser rule
        for rule in grammar.parser_rules() {
            code.push_str(&self.generate_rule_method(rule));
        }
        
        code.push_str("}\n\n");
        code
    }

    fn generate_rule_method(&self, rule: &Rule) -> String {
        let mut code = String::new();
        
        code.push_str(&format!("    /// Parse {} rule.\n", rule.name));
        code.push_str(&format!("    pub fn parse_{}(&mut self) -> Result<AstNode> {{\n", rule.name));
        code.push_str("        // TODO: Implement rule parsing\n");
        code.push_str("        unimplemented!()\n");
        code.push_str("    }\n\n");
        
        code
    }

    fn generate_lexer(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        
        code.push_str(&format!("/// Lexer for {} grammar.\n", grammar.name));
        code.push_str("/// \n");
        code.push_str("/// This lexer uses an optimized DFA (Deterministic Finite Automaton)\n");
        code.push_str("/// generated at compile time for efficient tokenization.\n");
        code.push_str("#[derive(Debug)]\n");
        code.push_str(&format!("pub struct {}Lexer {{\n", grammar.name));
        code.push_str("    input: Vec<char>,\n");
        code.push_str("    position: usize,\n");
        code.push_str("}\n\n");
        
        code.push_str(&format!("impl {}Lexer {{\n", grammar.name));
        code.push_str("    /// Create a new lexer from input string.\n");
        code.push_str("    #[inline]\n");
        code.push_str("    pub fn new(input: &str) -> Self {\n");
        code.push_str("        Self {\n");
        code.push_str("            input: input.chars().collect(),\n");
        code.push_str("            position: 0,\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");
        
        code.push_str("    /// Get the next token from the input.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Returns Ok(Token) on success, or Err(ParseError) if tokenization fails.\n");
        code.push_str("    pub fn next_token(&mut self) -> Result<Token, ParseError> {\n");
        code.push_str("        // Skip whitespace\n");
        code.push_str("        self.skip_whitespace();\n\n");
        code.push_str("        let start_pos = self.position;\n\n");
        code.push_str("        // Check for EOF\n");
        code.push_str("        if self.position >= self.input.len() {\n");
        code.push_str("            return Ok(Token {\n");
        code.push_str("                kind: TokenKind::Eof,\n");
        code.push_str("                text: String::new(),\n");
        code.push_str("                position: start_pos,\n");
        code.push_str("            });\n");
        code.push_str("        }\n\n");
        code.push_str("        // Use DFA for tokenization\n");
        code.push_str("        match self.next_token_dfa() {\n");
        code.push_str("            Some(mut token) => {\n");
        code.push_str("                token.position = start_pos;\n");
        code.push_str("                Ok(token)\n");
        code.push_str("            }\n");
        code.push_str("            None => {\n");
        code.push_str("                // Error recovery: skip invalid character and try again\n");
        code.push_str("                let invalid_char = self.input[self.position];\n");
        code.push_str("                self.position += 1;\n");
        code.push_str("                Err(ParseError::new(\n");
        code.push_str("                    format!(\"Unexpected character: '{}'\", invalid_char),\n");
        code.push_str("                    start_pos,\n");
        code.push_str("                ).with_found(invalid_char.to_string()))\n");
        code.push_str("            }\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");
        
        code.push_str("    /// Tokenize all input and collect errors.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Returns all successfully parsed tokens and a list of errors encountered.\n");
        code.push_str("    pub fn tokenize_all(&mut self) -> (Vec<Token>, Vec<ParseError>) {\n");
        code.push_str("        let mut tokens = Vec::new();\n");
        code.push_str("        let mut errors = Vec::new();\n\n");
        code.push_str("        loop {\n");
        code.push_str("            match self.next_token() {\n");
        code.push_str("                Ok(token) => {\n");
        code.push_str("                    let is_eof = token.kind == TokenKind::Eof;\n");
        code.push_str("                    tokens.push(token);\n");
        code.push_str("                    if is_eof {\n");
        code.push_str("                        break;\n");
        code.push_str("                    }\n");
        code.push_str("                }\n");
        code.push_str("                Err(err) => {\n");
        code.push_str("                    errors.push(err);\n");
        code.push_str("                    // Continue tokenizing after error\n");
        code.push_str("                    if self.position >= self.input.len() {\n");
        code.push_str("                        break;\n");
        code.push_str("                    }\n");
        code.push_str("                }\n");
        code.push_str("            }\n");
        code.push_str("        }\n\n");
        code.push_str("        (tokens, errors)\n");
        code.push_str("    }\n\n");
        
        code.push_str("    #[inline(always)]\n");
        code.push_str("    fn skip_whitespace(&mut self) {\n");
        code.push_str("        while self.position < self.input.len() {\n");
        code.push_str("            match self.input[self.position] {\n");
        code.push_str("                ' ' | '\\t' | '\\r' | '\\n' => self.position += 1,\n");
        code.push_str("                _ => break,\n");
        code.push_str("            }\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");
        
        // Generate lookup table for character classes
        let lexer_rules: Vec<_> = grammar.lexer_rules().collect();
        if !lexer_rules.is_empty() {
            let mut lookup_builder = LookupTableBuilder::new();
            lookup_builder.build_from_rules(&lexer_rules);
            
            // Generate lookup table
            code.push_str(&lookup_builder.generate_lookup_table());
            
            // Generate token type table
            let token_names: Vec<String> = lexer_rules
                .iter()
                .filter(|r| !r.is_fragment)
                .map(|r| r.name.clone())
                .collect();
            code.push_str(&lookup_builder.generate_token_type_table(&token_names));
            
            // Generate optimized character matching functions
            code.push_str(&generate_optimized_char_match(&lookup_builder));
            
            // Generate DFA-based tokenization
            let mut dfa_builder = DfaBuilder::new();
            let states = dfa_builder.build_from_rules(&lexer_rules);
            code.push_str(&generate_dfa_match(&states));
            
            // Add statistics as comment
            let stats = lookup_builder.stats();
            code.push_str(&format!("    // Lookup table stats: {} chars, {} classes, {} bytes\n",
                stats.total_chars, stats.total_classes, stats.memory_bytes()));
        } else {
            code.push_str("    fn next_token_dfa(&mut self) -> Option<Token> {\n");
            code.push_str("        // No lexer rules defined\n");
            code.push_str("        None\n");
            code.push_str("    }\n");
        }
        
        code.push_str("}\n\n");
        
        code
    }

    fn generate_ast_types(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        
        code.push_str("/// AST node types.\n");
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str("pub enum AstNode {\n");
        
        for rule in grammar.parser_rules() {
            let variant_name = self.to_pascal_case(&rule.name);
            code.push_str(&format!("    {}(Box<{}>),\n", variant_name, variant_name));
        }
        
        code.push_str("}\n\n");
        code
    }

    fn to_pascal_case(&self, s: &str) -> String {
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
}

impl Default for RustCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGeneratorTrait for RustCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;

    fn generate(&self, input: &Self::Input, config: &Self::Config) -> Result<String> {
        let mut code = String::new();
        
        // Header
        code.push_str(&format!("// Generated parser for {} grammar\n", input.name));
        code.push_str("// DO NOT EDIT - This file is automatically generated\n\n");
        
        // Imports
        code.push_str("use std::fmt;\n\n");
        
        // Error types for parsing
        code.push_str("/// Parse error with context information.\n");
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub struct ParseError {\n");
        code.push_str("    pub message: String,\n");
        code.push_str("    pub position: usize,\n");
        code.push_str("    pub expected: Vec<String>,\n");
        code.push_str("    pub found: Option<String>,\n");
        code.push_str("}\n\n");
        
        code.push_str("impl ParseError {\n");
        code.push_str("    pub fn new(message: String, position: usize) -> Self {\n");
        code.push_str("        Self {\n");
        code.push_str("            message,\n");
        code.push_str("            position,\n");
        code.push_str("            expected: Vec::new(),\n");
        code.push_str("            found: None,\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");
        code.push_str("    pub fn with_expected(mut self, expected: Vec<String>) -> Self {\n");
        code.push_str("        self.expected = expected;\n");
        code.push_str("        self\n");
        code.push_str("    }\n\n");
        code.push_str("    pub fn with_found(mut self, found: String) -> Self {\n");
        code.push_str("        self.found = Some(found);\n");
        code.push_str("        self\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        code.push_str("impl fmt::Display for ParseError {\n");
        code.push_str("    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n");
        code.push_str("        write!(f, \"Parse error at position {}: {}\", self.position, self.message)?;\n");
        code.push_str("        if !self.expected.is_empty() {\n");
        code.push_str("            write!(f, \" (expected: {})\", self.expected.join(\", \"))?;\n");
        code.push_str("        }\n");
        code.push_str("        if let Some(ref found) = self.found {\n");
        code.push_str("            write!(f, \" (found: {})\", found)?;\n");
        code.push_str("        }\n");
        code.push_str("        Ok(())\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        code.push_str("impl std::error::Error for ParseError {}\n\n");
        
        // Token type
        code.push_str("/// Token with position information.\n");
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub struct Token {\n");
        code.push_str("    pub kind: TokenKind,\n");
        code.push_str("    pub text: String,\n");
        code.push_str("    pub position: usize,\n");
        code.push_str("}\n\n");
        
        code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        code.push_str("pub enum TokenKind {\n");
        for rule in input.lexer_rules() {
            if !rule.is_fragment {
                code.push_str(&format!("    {},\n", rule.name));
            }
        }
        code.push_str("    Eof,\n");
        code.push_str("}\n\n");
        
        // Generate AST types
        code.push_str(&self.generate_ast_types(input));
        
        // Generate visitor pattern if requested
        if config.generate_visitor {
            code.push_str(&generate_visitor(input));
        }
        
        // Generate listener pattern if requested
        if config.generate_listener {
            code.push_str(&generate_listener(input));
        }
        
        // Generate lexer
        code.push_str(&self.generate_lexer(input));
        
        // Generate parser
        code.push_str(&self.generate_parser_struct(input));
        code.push_str(&self.generate_parser_impl(input));
        
        Ok(code)
    }

    fn target_language(&self) -> &str {
        "rust"
    }
}
