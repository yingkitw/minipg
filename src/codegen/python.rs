//! Python code generator.

use crate::ast::Grammar;
use crate::core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

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
        code.push_str("        self.position = 0\n\n");
        
        // Generate parse methods for each rule
        for rule in grammar.parser_rules() {
            code.push_str(&format!("    def parse_{}(self):\n", rule.name));
            code.push_str("        # TODO: Implement rule parsing\n");
            code.push_str("        pass\n\n");
        }
        
        code
    }
    
    fn generate_ast_types(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        
        code.push_str("from dataclasses import dataclass\n");
        code.push_str("from typing import List, Optional\n\n");
        
        for rule in grammar.parser_rules() {
            let class_name = to_pascal_case(&rule.name);
            code.push_str(&format!("@dataclass\n"));
            code.push_str(&format!("class {}:\n", class_name));
            code.push_str("    pass\n\n");
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
        
        // Header
        code.push_str(&format!("# Generated parser for {} grammar\n", input.name));
        code.push_str("# DO NOT EDIT - This file is automatically generated\n\n");
        
        // Imports
        code.push_str("from dataclasses import dataclass\n");
        code.push_str("from enum import Enum\n");
        code.push_str("from typing import List, Optional, Tuple\n\n");
        
        // Error class
        code.push_str("@dataclass\n");
        code.push_str("class ParseError(Exception):\n");
        code.push_str("    \"\"\"Parse error with context information.\"\"\"\n");
        code.push_str("    message: str\n");
        code.push_str("    position: int\n");
        code.push_str("    expected: List[str]\n");
        code.push_str("    found: Optional[str] = None\n\n");
        code.push_str("    def __str__(self) -> str:\n");
        code.push_str("        result = f\"Parse error at position {self.position}: {self.message}\"\n");
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
    use crate::core::types::GrammarType;

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
