//! Python code generator.

use crate::ast::{Grammar, Rule};
use crate::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// Python code generator.
pub struct PythonCodeGenerator;

impl PythonCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_lexer(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Lexer", grammar.name);

        code.push_str(&format!("class {}:\n", class_name));
        code.push_str("    \"\"\"Lexer with DFA-based tokenization and error recovery.\"\"\"\n\n");

        code.push_str("    def __init__(self, input_text: str):\n");
        code.push_str("        self.input = input_text\n");
        code.push_str("        self.position = 0\n\n");

        code.push_str("    def next_token(self) -> Token:\n");
        code.push_str("        \"\"\"Get the next token from input.\n\n");
        code.push_str("        Returns:\n");
        code.push_str("            Token: The next token\n\n");
        code.push_str("        Raises:\n");
        code.push_str("            ParseError: If tokenization fails\n");
        code.push_str("        \"\"\"\n");
        code.push_str("        # Skip whitespace\n");
        code.push_str("        self._skip_whitespace()\n\n");
        code.push_str("        start_pos = self.position\n\n");
        code.push_str("        # Check for EOF\n");
        code.push_str("        if self.position >= len(self.input):\n");
        code.push_str("            return Token(TokenKind.EOF, \"\", start_pos)\n\n");
        code.push_str("        # Use DFA for tokenization\n");
        code.push_str("        token = self._next_token_dfa()\n");
        code.push_str("        if token:\n");
        code.push_str("            token.position = start_pos\n");
        code.push_str("            return token\n\n");
        code.push_str("        # Error recovery: skip invalid character\n");
        code.push_str("        invalid_char = self.input[self.position]\n");
        code.push_str("        self.position += 1\n");
        code.push_str("        raise ParseError(\n");
        code.push_str("            message=f\"Unexpected character: '{invalid_char}'\"\n");
        code.push_str("            position=start_pos,\n");
        code.push_str("            expected=[],\n");
        code.push_str("            found=invalid_char\n");
        code.push_str("        )\n\n");

        code.push_str("    def tokenize_all(self) -> Tuple[List[Token], List[ParseError]]:\n");
        code.push_str("        \"\"\"Tokenize all input and collect errors.\n\n");
        code.push_str("        Returns:\n");
        code.push_str("            Tuple of (tokens, errors)\n");
        code.push_str("        \"\"\"\n");
        code.push_str("        tokens = []\n");
        code.push_str("        errors = []\n\n");
        code.push_str("        while True:\n");
        code.push_str("            try:\n");
        code.push_str("                token = self.next_token()\n");
        code.push_str("                tokens.append(token)\n");
        code.push_str("                if token.kind == TokenKind.EOF:\n");
        code.push_str("                    break\n");
        code.push_str("            except ParseError as err:\n");
        code.push_str("                errors.append(err)\n");
        code.push_str("                # Continue tokenizing after error\n");
        code.push_str("                if self.position >= len(self.input):\n");
        code.push_str("                    break\n\n");
        code.push_str("        return tokens, errors\n\n");

        code.push_str("    def _skip_whitespace(self):\n");
        code.push_str("        while self.position < len(self.input):\n");
        code.push_str("            if self.input[self.position] in ' \\t\\r\\n':\n");
        code.push_str("                self.position += 1\n");
        code.push_str("            else:\n");
        code.push_str("                break\n\n");

        code.push_str("    def _next_token_dfa(self) -> Optional[Token]:\n");
        code.push_str("        \"\"\"DFA-based tokenization (placeholder).\"\"\"\n");
        code.push_str("        # TODO: Implement DFA logic\n");
        code.push_str("        return None\n\n");

        code
    }

    fn generate_parser(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Parser", grammar.name);

        code.push_str(&format!("class {}:\n", class_name));
        code.push_str("    def __init__(self, tokens):\n");
        code.push_str("        self.tokens = tokens\n");
        code.push_str("        self.position = 0\n");

        // Insert @members named action if present
        if let Some(members_code) = grammar.named_actions.get("members") {
            code.push_str("        # Custom members from @members action\n");
            code.push_str("        ");
            code.push_str(members_code);
            code.push_str("\n");
        }
        code.push_str("\n");

        // Generate parse methods for each rule
        for rule in grammar.parser_rules() {
            code.push_str(&self.generate_rule_method(rule));
        }

        code
    }

    fn generate_rule_method(&self, rule: &Rule) -> String {
        let mut code = String::new();

        // Generate method signature
        code.push_str(&format!("    def parse_{}(self", rule.name));

        // Add arguments
        for arg in &rule.arguments {
            code.push_str(", ");
            code.push_str(&arg.name);
            if let Some(arg_type) = &arg.arg_type {
                code.push_str(&format!(": {}", arg_type));
            }
        }

        code.push_str(")");

        // Add return type annotation
        if !rule.returns.is_empty() {
            code.push_str(" -> ");
            if rule.returns.len() == 1 {
                if let Some(ret_type) = &rule.returns[0].return_type {
                    code.push_str(ret_type);
                } else {
                    code.push_str("AstNode");
                }
            } else {
                // Multiple returns - use tuple
                code.push_str("tuple[");
                for (i, ret) in rule.returns.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    if let Some(ret_type) = &ret.return_type {
                        code.push_str(ret_type);
                    } else {
                        code.push_str("AstNode");
                    }
                }
                code.push_str("]");
            }
        }

        code.push_str(":\n");

        // Generate docstring
        code.push_str(&format!("        \"\"\"Parse {} rule.\n", rule.name));
        if !rule.arguments.is_empty() {
            code.push_str("        \n");
            code.push_str("        Args:\n");
            for arg in &rule.arguments {
                let type_str = arg
                    .arg_type
                    .as_ref()
                    .map(|t| format!(" ({})", t))
                    .unwrap_or_default();
                code.push_str(&format!(
                    "            {}{}: Rule argument\n",
                    arg.name, type_str
                ));
            }
        }
        if !rule.returns.is_empty() {
            code.push_str("        \n");
            code.push_str("        Returns:\n");
            for ret in &rule.returns {
                let type_str = ret
                    .return_type
                    .as_ref()
                    .map(|t| t.as_str())
                    .unwrap_or("AstNode");
                code.push_str(&format!("            {}: {}\n", ret.name, type_str));
            }
        }
        code.push_str("        \"\"\"\n");

        // Generate local variables
        for local in &rule.locals {
            let type_str = local
                .local_type
                .as_ref()
                .map(|t| format!(": {}", t))
                .unwrap_or_default();
            code.push_str(&format!("        {}{} = None\n", local.name, type_str));
        }
        if !rule.locals.is_empty() {
            code.push_str("\n");
        }

        // Generate rule body
        if rule.alternatives.is_empty() {
            code.push_str("        # Empty rule\n");
            code.push_str(&format!("        return {}Node()\n\n", to_pascal_case(&rule.name)));
        } else if rule.alternatives.len() == 1 {
            // Single alternative
            let alt = &rule.alternatives[0];
            code.push_str(&self.generate_python_alternative(alt, rule));
        } else {
            // Multiple alternatives - try each in order
            code.push_str("        # Try alternatives\n");
            code.push_str("        start_pos = self.position\n\n");
            for (i, alt) in rule.alternatives.iter().enumerate() {
                if i > 0 {
                    code.push_str("\n        # Try next alternative\n");
                    code.push_str("        self.position = start_pos\n");
                }
                code.push_str("        try:\n");
                let alt_code = self.generate_python_alternative(alt, rule);
                // Indent the alternative code
                for line in alt_code.lines() {
                    if !line.trim().is_empty() {
                        code.push_str(&format!("    {}\n", line));
                    }
                }
                code.push_str("        except ParseError:\n");
                if i == rule.alternatives.len() - 1 {
                    code.push_str("            raise\n");
                } else {
                    code.push_str("            pass\n");
                }
            }
            code.push_str("\n");
        }

        code
    }

    fn generate_ast_types(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        code.push_str("# AST node type definitions\n\n");
        
        for rule in grammar.parser_rules() {
            let class_name = to_pascal_case(&rule.name);
            let fields = self.extract_labeled_fields(rule);
            
            code.push_str(&format!("@dataclass\n"));
            code.push_str(&format!("class {}Node:\n", class_name));
            code.push_str(&format!("    \"\"\"AST node for {} rule.\"\"\"\n", rule.name));
            
            if fields.is_empty() {
                code.push_str("    pass\n\n");
            } else {
                for (field_name, field_type, is_list) in &fields {
                    if *is_list {
                        code.push_str(&format!("    {}: List[{}]\n", field_name, field_type));
                    } else {
                        code.push_str(&format!("    {}: {}\n", field_name, field_type));
                    }
                }
                code.push_str("\n");
            }
        }

        code
    }

    fn extract_labeled_fields(&self, rule: &Rule) -> Vec<(String, String, bool)> {
        let mut fields = Vec::new();
        let mut seen_labels = std::collections::HashSet::new();

        for alt in &rule.alternatives {
            for elem in &alt.elements {
                self.collect_labels_from_element(elem, &mut fields, &mut seen_labels);
            }
        }

        fields
    }

    fn collect_labels_from_element(
        &self,
        element: &crate::ast::Element,
        fields: &mut Vec<(String, String, bool)>,
        seen_labels: &mut std::collections::HashSet<String>,
    ) {
        use crate::ast::Element;

        match element {
            Element::Terminal { label: Some(label), is_list, .. } => {
                if seen_labels.insert(label.clone()) {
                    fields.push((label.clone(), "Token".to_string(), *is_list));
                }
            }
            Element::RuleRef { label: Some(label), is_list, .. } => {
                if seen_labels.insert(label.clone()) {
                    fields.push((label.clone(), "AstNode".to_string(), *is_list));
                }
            }
            Element::StringLiteral { label: Some(label), is_list, .. } => {
                if seen_labels.insert(label.clone()) {
                    fields.push((label.clone(), "Token".to_string(), *is_list));
                }
            }
            Element::Group { alternatives } => {
                for alt in alternatives {
                    for elem in &alt.elements {
                        self.collect_labels_from_element(elem, fields, seen_labels);
                    }
                }
            }
            Element::Optional { element, .. } => {
                self.collect_labels_from_element(element, fields, seen_labels);
            }
            Element::ZeroOrMore { element, .. } => {
                self.collect_labels_from_element(element, fields, seen_labels);
            }
            Element::OneOrMore { element, .. } => {
                self.collect_labels_from_element(element, fields, seen_labels);
            }
            _ => {}
        }
    }

    fn generate_python_alternative(&self, alt: &crate::ast::Alternative, rule: &Rule) -> String {
        use crate::ast::Element;
        let mut code = String::new();

        // Collect labeled variables
        let mut collected_vars = Vec::new();
        let mut list_vars = Vec::new();

        for element in &alt.elements {
            match element {
                Element::RuleRef { label: Some(lbl), is_list, .. }
                | Element::Terminal { label: Some(lbl), is_list, .. }
                | Element::StringLiteral { label: Some(lbl), is_list, .. } => {
                    if *is_list {
                        list_vars.push(lbl.clone());
                    } else {
                        collected_vars.push(lbl.clone());
                    }
                }
                _ => {}
            }
        }

        // Initialize list variables
        for lbl in &list_vars {
            code.push_str(&format!("        {} = []\\n", lbl));
        }
        if !list_vars.is_empty() {
            code.push_str("\\n");
        }

        // Generate parsing code for each element
        for element in &alt.elements {
            match element {
                Element::RuleRef { name, label, is_list } => {
                    let method_name = format!("parse_{}", name);
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        {}.append(self.{}())\\n", lbl, method_name));
                        } else {
                            code.push_str(&format!("        self.{}()\\n", method_name));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        {} = self.{}()\\n", lbl, method_name));
                        } else {
                            code.push_str(&format!("        self.{}()\\n", method_name));
                        }
                    }
                }
                Element::Terminal { value, label, is_list } => {
                    code.push_str(&format!("        # Match terminal: {}\\n", value));
                    code.push_str("        if self.position >= len(self.tokens):\\n");
                    code.push_str(&format!("            raise ParseError('Unexpected EOF, expected: {}', self.position, [], None)\\n", value));
                    code.push_str(&format!("        if self.tokens[self.position].kind == TokenKind.{}:\\n", value));
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("            {}.append(self.tokens[self.position])\\n", lbl));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("            {} = self.tokens[self.position]\\n", lbl));
                        }
                    }
                    code.push_str("            self.position += 1\\n");
                    code.push_str("        else:\\n");
                    code.push_str(&format!("            raise ParseError(f'Expected {}, got {{self.tokens[self.position].kind}}', self.position, [], None)\\n", value));
                }
                Element::StringLiteral { value, label, is_list } => {
                    code.push_str(&format!("        # Match string literal: '{}'\\n", value));
                    code.push_str("        if self.position >= len(self.tokens):\\n");
                    code.push_str(&format!("            raise ParseError('Unexpected EOF, expected: \"{}\"', self.position, [], None)\\n", value));
                    code.push_str(&format!("        if self.tokens[self.position].value == '{}':\\n", value));
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("            {}.append(self.tokens[self.position])\\n", lbl));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("            {} = self.tokens[self.position]\\n", lbl));
                        }
                    }
                    code.push_str("            self.position += 1\\n");
                    code.push_str("        else:\\n");
                    code.push_str(&format!("            raise ParseError(f'Expected \\\"{}\\\", got {{self.tokens[self.position].value}}', self.position, [], None)\\n", value));
                }
                Element::Optional { element, .. } => {
                    code.push_str("        # Optional element\\n");
                    code.push_str("        saved_pos = self.position\\n");
                    code.push_str("        try:\\n");
                    code.push_str("            # Try to match optional element\\n");
                    code.push_str("            pass\\n");
                    code.push_str("        except ParseError:\\n");
                    code.push_str("            # Optional failed, restore position\\n");
                    code.push_str("            self.position = saved_pos\\n");
                }
                Element::ZeroOrMore { element, .. } => {
                    code.push_str("        # Zero or more repetition\\n");
                    code.push_str("        while self.position < len(self.tokens):\\n");
                    code.push_str("            saved_pos = self.position\\n");
                    code.push_str("            try:\\n");
                    code.push_str("                # Try to match element\\n");
                    code.push_str("                pass\\n");
                    code.push_str("            except ParseError:\\n");
                    code.push_str("                self.position = saved_pos\\n");
                    code.push_str("                break\\n");
                }
                Element::OneOrMore { element, .. } => {
                    code.push_str("        # One or more repetition\\n");
                    code.push_str("        match_count = 0\\n");
                    code.push_str("        while self.position < len(self.tokens):\\n");
                    code.push_str("            saved_pos = self.position\\n");
                    code.push_str("            try:\\n");
                    code.push_str("                # Try to match element\\n");
                    code.push_str("                match_count += 1\\n");
                    code.push_str("            except ParseError:\\n");
                    code.push_str("                self.position = saved_pos\\n");
                    code.push_str("                break\\n");
                    code.push_str("        if match_count == 0:\\n");
                    code.push_str("            raise ParseError('Expected at least one match', self.position, [], None)\\n");
                }
                _ => {
                    code.push_str("        # TODO: Handle other element types\\n");
                }
            }
        }

        // Build result node
        if !collected_vars.is_empty() || !list_vars.is_empty() {
            code.push_str("\\n        # Build AST node\\n");
            code.push_str(&format!("        return {}Node(", to_pascal_case(&rule.name)));
            let all_vars: Vec<_> = collected_vars.iter().chain(list_vars.iter()).collect();
            for (i, var) in all_vars.iter().enumerate() {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(&format!("{}={}", var, var));
            }
            code.push_str(")\\n");
        } else {
            code.push_str(&format!("        return {}Node()\\n", to_pascal_case(&rule.name)));
        }

        code
    }
}

impl Default for PythonCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGeneratorTrait for PythonCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;

    fn generate(&self, input: &Self::Input, _config: &Self::Config) -> Result<String> {
        let mut code = String::new();

        // Header with PEP 8 compliance
        code.push_str(&format!("\"\"\"Generated parser for {} grammar.\n\n", input.name));
        code.push_str("DO NOT EDIT - This file is automatically generated by minipg.\n");
        code.push_str("PEP 8 compliant, Python 3.10+ compatible.\n");
        code.push_str("\"\"\"\n\n");

        // Standard library imports (PEP 8: standard lib first)
        code.push_str("from __future__ import annotations\n\n");
        code.push_str("from dataclasses import dataclass\n");
        code.push_str("from typing import Any, List, Optional, Tuple\n");

        // Insert @header named action if present
        if let Some(header_code) = input.named_actions.get("header") {
            code.push_str("\n# Custom header from @header action\n");
            code.push_str(header_code);
            code.push_str("\n");
        }
        code.push_str("\n");

        // Error class
        code.push_str("@dataclass\n");
        code.push_str("class ParseError(Exception):\n");
        code.push_str("    \"\"\"Parse error with context information.\"\"\"\n");
        code.push_str("    message: str\n");
        code.push_str("    position: int\n");
        code.push_str("    expected: List[str]\n");
        code.push_str("    found: Optional[str] = None\n\n");
        code.push_str("    def __str__(self) -> str:\n");
        code.push_str(
            "        result = f\"Parse error at position {self.position}: {self.message}\"\n",
        );
        code.push_str("        if self.expected:\n");
        code.push_str("            result += f\" (expected: {', '.join(self.expected)})\"\n");
        code.push_str("        if self.found:\n");
        code.push_str("            result += f\" (found: {self.found})\"\n");
        code.push_str("        return result\n\n");

        // Token class
        code.push_str("@dataclass\n");
        code.push_str("class Token:\n");
        code.push_str("    \"\"\"Token with position information.\"\"\"\n");
        code.push_str("    kind: 'TokenKind'\n");
        code.push_str("    text: str\n");
        code.push_str("    position: int\n\n");

        // Token enum
        code.push_str("class TokenKind(Enum):\n");
        for rule in input.lexer_rules() {
            if !rule.is_fragment {
                code.push_str(&format!("    {} = '{}'\n", rule.name, rule.name));
            }
        }
        code.push_str("    EOF = 'EOF'\n\n");

        // AST types
        code.push_str(&self.generate_ast_types(input));

        // Lexer
        code.push_str(&self.generate_lexer(input));

        // Parser
        code.push_str(&self.generate_parser(input));

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "python"
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Grammar, Rule};
    use crate::types::GrammarType;

    #[test]
    fn test_python_codegen() {
        let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Parser);
        grammar.add_rule(Rule::parser_rule("expr".to_string()));
        grammar.add_rule(Rule::lexer_rule("NUMBER".to_string()));

        let generator = PythonCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = generator.generate(&grammar, &config).unwrap();

        assert!(code.contains("class CalculatorLexer"));
        assert!(code.contains("class CalculatorParser"));
        assert!(code.contains("def parse_expr"));
    }
}
