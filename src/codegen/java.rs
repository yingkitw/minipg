//! Java code generator for minipg.

use crate::ast::Grammar;
use crate::core::{CodeGenerator, Result};
use crate::core::types::CodeGenConfig;
use super::pattern_match::generate_simple_pattern_match;

/// Java code generator.
pub struct JavaCodeGenerator;

impl JavaCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_lexer(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        // Package declaration
        code.push_str(&format!("package {}.lexer;\n\n", grammar.name.to_lowercase()));

        // Imports
        code.push_str("import java.util.*;\n");
        code.push_str("import java.util.regex.*;\n\n");

        // Token type enum
        code.push_str("public enum TokenType {\n");
        let mut token_types: Vec<String> = grammar.lexer_rules()
            .map(|r| r.name.clone())
            .collect();
        token_types.sort();
        
        for (i, token_type) in token_types.iter().enumerate() {
            if i < token_types.len() - 1 {
                code.push_str(&format!("    {},\n", token_type));
            } else {
                code.push_str(&format!("    {}\n", token_type));
            }
        }
        code.push_str("}\n\n");

        // Token class
        code.push_str("public class Token {\n");
        code.push_str("    public TokenType type;\n");
        code.push_str("    public String text;\n");
        code.push_str("    public int line;\n");
        code.push_str("    public int column;\n");
        code.push_str("    public int length;\n\n");
        code.push_str("    public Token(TokenType type, String text, int line, int column, int length) {\n");
        code.push_str("        this.type = type;\n");
        code.push_str("        this.text = text;\n");
        code.push_str("        this.line = line;\n");
        code.push_str("        this.column = column;\n");
        code.push_str("        this.length = length;\n");
        code.push_str("    }\n\n");
        code.push_str("    @Override\n");
        code.push_str("    public String toString() {\n");
        code.push_str("        return String.format(\"Token(%s, '%s', %d:%d)\", type, text, line, column);\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Lexer class
        code.push_str("public class Lexer {\n");
        code.push_str("    private String input;\n");
        code.push_str("    private int position;\n");
        code.push_str("    private int line;\n");
        code.push_str("    private int column;\n");
        code.push_str("    private int mode;\n\n");
        code.push_str("    public Lexer(String input) {\n");
        code.push_str("        this.input = input;\n");
        code.push_str("        this.position = 0;\n");
        code.push_str("        this.line = 1;\n");
        code.push_str("        this.column = 1;\n");
        code.push_str("        this.mode = 0;\n");
        code.push_str("    }\n\n");
        code.push_str("    public Token nextToken() {\n");
        code.push_str("        if (position >= input.length()) {\n");
        code.push_str("            return new Token(TokenType.EOF, \"\", line, column, 0);\n");
        code.push_str("        }\n");
        code.push_str("        // Try to match lexer rules in order\n");
        code.push_str("        int startPos = position;\n");
        
        // Generate token matching logic for each lexer rule
        let lexer_rules: Vec<_> = grammar.lexer_rules()
            .filter(|r| !r.is_fragment)
            .collect();
        
        if !lexer_rules.is_empty() {
            code.push_str("        // Try each lexer rule\n");
            for (i, rule) in lexer_rules.iter().enumerate() {
                if i == 0 {
                    code.push_str("        if (");
                } else {
                    code.push_str("        } else if (");
                }
                
                code.push_str(&format!("match_{}()", rule.name.to_lowercase()));
                code.push_str(") {\n");
                code.push_str("            String text = input.substring(startPos, position);\n");
                code.push_str(&format!("            return new Token(TokenType.{}, text, line, column, text.length());\n", rule.name));
            }
            code.push_str("        }\n\n");
        }
        
        code.push_str("        // Error recovery: skip invalid character\n");
        code.push_str("        if (position < input.length()) {\n");
        code.push_str("            char invalidChar = input.charAt(position);\n");
        code.push_str("            position++;\n");
        code.push_str("            return new Token(TokenType.ERROR, String.valueOf(invalidChar), line, column, 1);\n");
        code.push_str("        }\n\n");
        
        code.push_str("        return new Token(TokenType.EOF, \"\", line, column, 0);\n");
        code.push_str("    }\n\n");
        
        // Generate match helper methods
        if !lexer_rules.is_empty() {
            code.push_str("    // Helper methods for pattern matching\n");
            for rule in lexer_rules {
                let pattern_code = generate_simple_pattern_match(rule, "java");
                code.push_str(&pattern_code);
                code.push_str("\n");
            }
        }
        code.push_str("    public Token peekToken() {\n");
        code.push_str("        int savedPos = position;\n");
        code.push_str("        Token token = nextToken();\n");
        code.push_str("        position = savedPos;\n");
        code.push_str("        return token;\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        code
    }

    fn generate_parser(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        // Package declaration
        code.push_str(&format!("package {}.parser;\n\n", grammar.name.to_lowercase()));

        // Imports
        code.push_str("import java.util.*;\n");
        code.push_str(&format!("import {}.lexer.*;\n\n", grammar.name.to_lowercase()));

        // Parse exception
        code.push_str("public class ParseException extends Exception {\n");
        code.push_str("    public int code;\n");
        code.push_str("    public int line;\n");
        code.push_str("    public int column;\n\n");
        code.push_str("    public ParseException(int code, String message, int line, int column) {\n");
        code.push_str("        super(message);\n");
        code.push_str("        this.code = code;\n");
        code.push_str("        this.line = line;\n");
        code.push_str("        this.column = column;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Parser class
        code.push_str("public class Parser {\n");
        code.push_str("    private Lexer lexer;\n");
        code.push_str("    private Token currentToken;\n");
        code.push_str("    private Token peekToken;\n\n");
        code.push_str("    public Parser(Lexer lexer) {\n");
        code.push_str("        this.lexer = lexer;\n");
        code.push_str("        this.currentToken = lexer.nextToken();\n");
        code.push_str("        this.peekToken = lexer.nextToken();\n");
        code.push_str("    }\n\n");
        code.push_str("    private void advance() {\n");
        code.push_str("        currentToken = peekToken;\n");
        code.push_str("        peekToken = lexer.nextToken();\n");
        code.push_str("    }\n\n");
        code.push_str("    public void parse() throws ParseException {\n");
        code.push_str("        // TODO: Implement parser logic\n");
        code.push_str("    }\n\n");

        // Parser rule declarations
        for rule in grammar.parser_rules() {
            code.push_str(&format!("    public void parse{}() throws ParseException {{\n", 
                self.capitalize_first(&rule.name)));
            code.push_str("        // TODO: Implement rule parsing\n");
            code.push_str("    }\n\n");
        }

        code.push_str("}\n");

        code
    }

    fn capitalize_first(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Default for JavaCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator for JavaCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, grammar: &Grammar, _config: &CodeGenConfig) -> Result<String> {
        let lexer = self.generate_lexer(grammar);
        let parser = self.generate_parser(grammar);
        
        // Return both lexer and parser combined with a separator
        Ok(format!("// ===== LEXER FILE: {}/lexer/Lexer.java =====\n{}\n\n// ===== PARSER FILE: {}/parser/Parser.java =====\n{}", 
                   grammar.name.to_lowercase(), lexer, grammar.name.to_lowercase(), parser))
    }

    fn target_language(&self) -> &'static str {
        "Java"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::GrammarParser;
    use crate::core::GrammarParser as GrammarParserTrait;

    #[test]
    fn test_java_codegen_basic() {
        let grammar_text = r#"
            grammar Calculator;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("LEXER FILE"));
        assert!(code.contains("PARSER FILE"));
        assert!(code.contains("enum TokenType"));
        assert!(code.contains("NUMBER"));
    }

    #[test]
    fn test_java_codegen_package_structure() {
        let grammar_text = r#"
            grammar MyGrammar;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("package mygrammar.lexer"));
        assert!(code.contains("package mygrammar.parser"));
    }

    #[test]
    fn test_java_codegen_classes() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("public class Token"));
        assert!(code.contains("public class Lexer"));
        assert!(code.contains("public class Parser"));
        assert!(code.contains("public class ParseException"));
    }

    #[test]
    fn test_java_codegen_token_types() {
        let grammar_text = r#"
            grammar Test;
            
            expr: ID NUMBER;
            
            ID: [a-zA-Z_]+;
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("ID,"));
        assert!(code.contains("NUMBER"));
    }

    #[test]
    fn test_java_codegen_parser_methods() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            factor: ID;
            
            ID: [a-zA-Z_]+;
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("parseExpr"));
        assert!(code.contains("parseTerm"));
        assert!(code.contains("parseFactor"));
    }

    #[test]
    fn test_java_codegen_target_language() {
        let codegen = JavaCodeGenerator::new();
        assert_eq!(codegen.target_language(), "Java");
    }

    #[test]
    fn test_java_codegen_with_completejson() {
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
        
        let codegen = JavaCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("STRING"));
        assert!(code.contains("NUMBER"));
        assert!(code.contains("parseJson"));
        assert!(code.contains("parseValue"));
    }
}
