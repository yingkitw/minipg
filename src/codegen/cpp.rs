//! C++ code generator for minipg.

use crate::ast::{Grammar, Rule};
use crate::core::{CodeGenerator, Result};
use crate::core::types::CodeGenConfig;
use super::pattern_match::generate_simple_pattern_match;

/// C++ code generator.
pub struct CppCodeGenerator;

impl CppCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_header(&self, grammar: &Grammar) -> String {
        let guard = format!("{}_HPP", grammar.name.to_uppercase());
        let mut code = String::new();

        code.push_str(&format!("#ifndef {}\n", guard));
        code.push_str(&format!("#define {}\n\n", guard));

        code.push_str("#include <string>\n");
        code.push_str("#include <vector>\n");
        code.push_str("#include <memory>\n");
        code.push_str("#include <stdexcept>\n");
        code.push_str("#include <iostream>\n\n");

        code.push_str(&format!("namespace {} {{\n\n", grammar.name));

        // Token type enum
        code.push_str("enum class TokenType {\n");
        let mut token_types: Vec<String> = grammar.lexer_rules()
            .map(|r| r.name.clone())
            .collect();
        token_types.sort();
        
        for token_type in token_types.iter() {
            code.push_str(&format!("    {},\n", token_type));
        }
        code.push_str("    Eof,\n");
        code.push_str("    Error\n");
        code.push_str("};\n\n");

        // Token class
        code.push_str("class Token {\n");
        code.push_str("public:\n");
        code.push_str("    TokenType type;\n");
        code.push_str("    std::string text;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n");
        code.push_str("    int length;\n\n");
        code.push_str("    Token(TokenType t, const std::string& txt, int l, int c, int len)\n");
        code.push_str("        : type(t), text(txt), line(l), column(c), length(len) {}\n");
        code.push_str("};\n\n");

        // Lexer class
        code.push_str("class Lexer {\n");
        code.push_str("private:\n");
        code.push_str("    std::string input;\n");
        code.push_str("    size_t position;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n");
        code.push_str("    int mode;\n\n");
        code.push_str("public:\n");
        code.push_str("    Lexer(const std::string& src);\n");
        code.push_str("    Token next_token();\n");
        code.push_str("    Token peek_token();\n");
        
        // Generate match method declarations
        let lexer_rules: Vec<_> = grammar.lexer_rules()
            .filter(|r| !r.is_fragment)
            .collect();
        
        if !lexer_rules.is_empty() {
            code.push_str("\n    // Pattern matching helpers\n");
            for rule in &lexer_rules {
                code.push_str(&format!("    bool match_{}();\n", rule.name.to_lowercase()));
            }
        }
        
        code.push_str("};\n\n");

        // Parser exception
        code.push_str("class ParseError : public std::runtime_error {\n");
        code.push_str("public:\n");
        code.push_str("    int code;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n\n");
        code.push_str("    ParseError(int c, const std::string& msg, int l, int col)\n");
        code.push_str("        : std::runtime_error(msg), code(c), line(l), column(col) {}\n");
        code.push_str("};\n\n");

        // Parser class
        code.push_str("class Parser {\n");
        code.push_str("private:\n");
        code.push_str("    std::unique_ptr<Lexer> lexer;\n");
        code.push_str("    Token current_token;\n");
        code.push_str("    Token peek_token;\n\n");
        code.push_str("    void advance();\n\n");
        code.push_str("public:\n");
        code.push_str("    Parser(std::unique_ptr<Lexer> lex);\n");
        code.push_str("    void parse();\n\n");

        // Parser rule declarations
        for rule in grammar.parser_rules() {
            code.push_str(&format!("    void parse_{}();\n", rule.name));
        }

        code.push_str("};\n\n");
        code.push_str("} // namespace\n\n");
        code.push_str("#endif\n");
        code
    }

    fn generate_source(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        code.push_str(&format!("#include \"{}.hpp\"\n\n", grammar.name));
        code.push_str(&format!("namespace {} {{\n\n", grammar.name));

        // Lexer implementation
        code.push_str("// Lexer implementation\n");
        code.push_str("Lexer::Lexer(const std::string& src)\n");
        code.push_str("    : input(src), position(0), line(1), column(1), mode(0) {}\n\n");

        code.push_str("Token Lexer::next_token() {\n");
        code.push_str("    if (position >= input.length()) {\n");
        code.push_str("        return Token(TokenType::Eof, \"\", line, column, 0);\n");
        code.push_str("    }\n");
        code.push_str("    // Try to match lexer rules in order\n");
        code.push_str("    size_t start_pos = position;\n");
        
        // Generate token matching logic for each lexer rule
        let lexer_rules: Vec<_> = grammar.lexer_rules()
            .filter(|r| !r.is_fragment)
            .collect();
        
        if !lexer_rules.is_empty() {
            code.push_str("    // Try each lexer rule\n");
            for (i, rule) in lexer_rules.iter().enumerate() {
                if i == 0 {
                    code.push_str("    if (");
                } else {
                    code.push_str("    } else if (");
                }
                
                code.push_str(&format!("match_{}()", rule.name.to_lowercase()));
                code.push_str(") {\n");
                code.push_str("        std::string text = input.substr(start_pos, position - start_pos);\n");
                code.push_str(&format!("        return Token(TokenType::{}, text, line, column, text.length());\n", rule.name));
            }
            code.push_str("    }\n\n");
        }
        
        code.push_str("    // Error recovery: skip invalid character\n");
        code.push_str("    if (position < input.length()) {\n");
        code.push_str("        char invalid_char = input[position];\n");
        code.push_str("        position++;\n");
        code.push_str("        return Token(TokenType::Error, std::string(1, invalid_char), line, column, 1);\n");
        code.push_str("    }\n\n");
        
        code.push_str("    return Token(TokenType::Eof, \"\", line, column, 0);\n");
        code.push_str("}\n\n");
        
        // Generate match helper methods
        if !lexer_rules.is_empty() {
            code.push_str("// Helper methods for pattern matching\n");
            for rule in lexer_rules {
                code.push_str(&format!("bool Lexer::match_{}() {{\n", rule.name.to_lowercase()));
                code.push_str("    size_t start_pos = position;\n");
                
                if let Some(alt) = rule.alternatives.first() {
                    for element in &alt.elements {
                        code.push_str(&super::pattern_match::generate_element_match_cpp(element, "    "));
                    }
                }
                
                code.push_str("    return position > start_pos;\n");
                code.push_str("}\n\n");
            }
        }

        code.push_str("Token Lexer::peek_token() {\n");
        code.push_str("    size_t saved_pos = position;\n");
        code.push_str("    Token token = next_token();\n");
        code.push_str("    position = saved_pos;\n");
        code.push_str("    return token;\n");
        code.push_str("}\n\n");

        // Parser implementation
        code.push_str("// Parser implementation\n");
        code.push_str("Parser::Parser(std::unique_ptr<Lexer> lex)\n");
        code.push_str("    : lexer(std::move(lex)),\n");
        code.push_str("      current_token(lexer->next_token()),\n");
        code.push_str("      peek_token(lexer->next_token()) {}\n\n");

        code.push_str("void Parser::advance() {\n");
        code.push_str("    current_token = peek_token;\n");
        code.push_str("    peek_token = lexer->next_token();\n");
        code.push_str("}\n\n");

        code.push_str("void Parser::parse() {\n");
        code.push_str("    // TODO: Implement parser logic\n");
        code.push_str("}\n\n");

        // Parser rule implementations
        for rule in grammar.parser_rules() {
            code.push_str(&format!("void Parser::parse_{}() {{\n", rule.name));
            code.push_str("    // TODO: Implement rule parsing\n");
            code.push_str("}\n\n");
        }

        code.push_str("} // namespace\n");
        code
    }
}

impl Default for CppCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator for CppCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, grammar: &Grammar, _config: &CodeGenConfig) -> Result<String> {
        let header = self.generate_header(grammar);
        let source = self.generate_source(grammar);
        
        // Return both header and source combined with a separator
        Ok(format!("// ===== HEADER FILE: {}.hpp =====\n{}\n\n// ===== SOURCE FILE: {}.cpp =====\n{}", 
                   grammar.name, header, grammar.name, source))
    }

    fn target_language(&self) -> &'static str {
        "C++"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::GrammarParser;
    use crate::core::GrammarParser as GrammarParserTrait;

    #[test]
    fn test_cpp_codegen_basic() {
        let grammar_text = r#"
            grammar Calculator;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("HEADER FILE"));
        assert!(code.contains("SOURCE FILE"));
        assert!(code.contains("enum class TokenType"));
        assert!(code.contains("class Token"));
    }

    #[test]
    fn test_cpp_codegen_classes() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("class Lexer"));
        assert!(code.contains("class Parser"));
        assert!(code.contains("class ParseError"));
        assert!(code.contains("class Token"));
    }

    #[test]
    fn test_cpp_codegen_namespace() {
        let grammar_text = r#"
            grammar MyGrammar;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("namespace MyGrammar"));
    }

    #[test]
    fn test_cpp_codegen_methods() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("next_token"));
        assert!(code.contains("peek_token"));
        assert!(code.contains("parse_expr"));
        assert!(code.contains("parse_term"));
    }

    #[test]
    fn test_cpp_codegen_smart_pointers() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("std::unique_ptr"));
        assert!(code.contains("std::move"));
    }

    #[test]
    fn test_cpp_codegen_target_language() {
        let codegen = CppCodeGenerator::new();
        assert_eq!(codegen.target_language(), "C++");
    }

    #[test]
    fn test_cpp_codegen_with_completejson() {
        let grammar_text = r#"
            grammar CompleteJSON;
            
            json: value EOF;
            value: object | array | STRING | NUMBER | 'true' | 'false' | 'null';
            object: '{' (pair (',' pair)*)? '}';
            pair: STRING ':' value;
            array: '[' (value (',' value)*)? ']';
            
            STRING: '"' (~["\\\r\n] | '\\' .)* '"';
            NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [0-9]+)?;
            WS: [ \t\r\n]+ -> skip;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CppCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("STRING"));
        assert!(code.contains("NUMBER"));
        assert!(code.contains("parse_json"));
        assert!(code.contains("parse_value"));
    }
}
