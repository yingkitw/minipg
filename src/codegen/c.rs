//! C code generator for minipg.

use crate::ast::Grammar;
use crate::core::{CodeGenerator, Result};
use crate::core::types::CodeGenConfig;

/// C code generator.
pub struct CCodeGenerator;

impl CCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_header(&self, grammar: &Grammar) -> String {
        let guard = format!("{}_H", grammar.name.to_uppercase());
        let mut code = String::new();

        code.push_str(&format!("#ifndef {}\n", guard));
        code.push_str(&format!("#define {}\n\n", guard));

        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <string.h>\n");
        code.push_str("#include <stdint.h>\n");
        code.push_str("#include <stdbool.h>\n\n");

        // Token types enum
        code.push_str("typedef enum {\n");
        let mut token_types: Vec<String> = grammar.lexer_rules()
            .map(|r| r.name.clone())
            .collect();
        token_types.sort();
        
        for (i, token_type) in token_types.iter().enumerate() {
            code.push_str(&format!("    TOKEN_{} = {},\n", token_type.to_uppercase(), i));
        }
        code.push_str("    TOKEN_EOF = 999,\n");
        code.push_str("    TOKEN_ERROR = 1000\n");
        code.push_str("} TokenType;\n\n");

        // Token structure
        code.push_str("typedef struct {\n");
        code.push_str("    TokenType type;\n");
        code.push_str("    char *text;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n");
        code.push_str("    int length;\n");
        code.push_str("} Token;\n\n");

        // Lexer structure
        code.push_str("typedef struct {\n");
        code.push_str("    const char *input;\n");
        code.push_str("    size_t position;\n");
        code.push_str("    size_t length;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n");
        code.push_str("    int mode;\n");
        code.push_str("} Lexer;\n\n");

        // Parser structure
        code.push_str("typedef struct {\n");
        code.push_str("    Lexer *lexer;\n");
        code.push_str("    Token current_token;\n");
        code.push_str("    Token peek_token;\n");
        code.push_str("} Parser;\n\n");

        // Error structure
        code.push_str("typedef struct {\n");
        code.push_str("    int code;\n");
        code.push_str("    char *message;\n");
        code.push_str("    int line;\n");
        code.push_str("    int column;\n");
        code.push_str("} ParseError;\n\n");

        // Function declarations
        code.push_str("// Lexer functions\n");
        code.push_str("Lexer* lexer_new(const char *input);\n");
        code.push_str("void lexer_free(Lexer *lexer);\n");
        code.push_str("Token lexer_next_token(Lexer *lexer);\n");
        code.push_str("Token lexer_peek_token(Lexer *lexer);\n\n");

        code.push_str("// Parser functions\n");
        code.push_str("Parser* parser_new(Lexer *lexer);\n");
        code.push_str("void parser_free(Parser *parser);\n");
        code.push_str("ParseError* parser_parse(Parser *parser);\n\n");

        // Parser rule declarations
        for rule in grammar.parser_rules() {
            code.push_str(&format!("ParseError* parse_{}(Parser *parser);\n", rule.name));
        }

        code.push_str("\n#endif\n");
        code
    }

    fn generate_source(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        code.push_str(&format!("#include \"{}.h\"\n\n", grammar.name));

        // Memory management helpers
        code.push_str("// Memory management helpers\n");
        code.push_str("static void* safe_malloc(size_t size) {\n");
        code.push_str("    void *ptr = malloc(size);\n");
        code.push_str("    if (!ptr && size > 0) {\n");
        code.push_str("        fprintf(stderr, \"Memory allocation failed\\n\");\n");
        code.push_str("        exit(1);\n");
        code.push_str("    }\n");
        code.push_str("    return ptr;\n");
        code.push_str("}\n\n");

        code.push_str("static char* safe_strdup(const char *str) {\n");
        code.push_str("    if (!str) return NULL;\n");
        code.push_str("    char *copy = safe_malloc(strlen(str) + 1);\n");
        code.push_str("    strcpy(copy, str);\n");
        code.push_str("    return copy;\n");
        code.push_str("}\n\n");

        // Lexer implementation
        code.push_str("// Lexer implementation\n");
        code.push_str("Lexer* lexer_new(const char *input) {\n");
        code.push_str("    Lexer *lexer = (Lexer*)safe_malloc(sizeof(Lexer));\n");
        code.push_str("    lexer->input = input;\n");
        code.push_str("    lexer->position = 0;\n");
        code.push_str("    lexer->length = strlen(input);\n");
        code.push_str("    lexer->line = 1;\n");
        code.push_str("    lexer->column = 1;\n");
        code.push_str("    lexer->mode = 0;\n");
        code.push_str("    return lexer;\n");
        code.push_str("}\n\n");

        code.push_str("void lexer_free(Lexer *lexer) {\n");
        code.push_str("    if (lexer) free(lexer);\n");
        code.push_str("}\n\n");

        code.push_str("static Token make_token(TokenType type, const char *text, int line, int column, int length) {\n");
        code.push_str("    Token token;\n");
        code.push_str("    token.type = type;\n");
        code.push_str("    token.text = safe_strdup(text);\n");
        code.push_str("    token.line = line;\n");
        code.push_str("    token.column = column;\n");
        code.push_str("    token.length = length;\n");
        code.push_str("    return token;\n");
        code.push_str("}\n\n");

        code.push_str("static void free_token(Token *token) {\n");
        code.push_str("    if (token && token->text) {\n");
        code.push_str("        free(token->text);\n");
        code.push_str("        token->text = NULL;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code.push_str("Token lexer_next_token(Lexer *lexer) {\n");
        code.push_str("    if (!lexer || lexer->position >= lexer->length) {\n");
        code.push_str("        return make_token(TOKEN_EOF, \"\", lexer->line, lexer->column, 0);\n");
        code.push_str("    }\n");
        code.push_str("    // TODO: Implement lexer logic\n");
        code.push_str("    return make_token(TOKEN_EOF, \"\", lexer->line, lexer->column, 0);\n");
        code.push_str("}\n\n");

        code.push_str("Token lexer_peek_token(Lexer *lexer) {\n");
        code.push_str("    size_t saved_pos = lexer->position;\n");
        code.push_str("    Token token = lexer_next_token(lexer);\n");
        code.push_str("    lexer->position = saved_pos;\n");
        code.push_str("    return token;\n");
        code.push_str("}\n\n");

        // Parser implementation
        code.push_str("// Parser implementation\n");
        code.push_str("Parser* parser_new(Lexer *lexer) {\n");
        code.push_str("    Parser *parser = (Parser*)safe_malloc(sizeof(Parser));\n");
        code.push_str("    parser->lexer = lexer;\n");
        code.push_str("    parser->current_token = lexer_next_token(lexer);\n");
        code.push_str("    parser->peek_token = lexer_next_token(lexer);\n");
        code.push_str("    return parser;\n");
        code.push_str("}\n\n");

        code.push_str("void parser_free(Parser *parser) {\n");
        code.push_str("    if (parser) {\n");
        code.push_str("        free_token(&parser->current_token);\n");
        code.push_str("        free_token(&parser->peek_token);\n");
        code.push_str("        free(parser);\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code.push_str("static void parser_advance(Parser *parser) {\n");
        code.push_str("    free_token(&parser->current_token);\n");
        code.push_str("    parser->current_token = parser->peek_token;\n");
        code.push_str("    parser->peek_token = lexer_next_token(parser->lexer);\n");
        code.push_str("}\n\n");

        code.push_str("ParseError* parser_parse(Parser *parser) {\n");
        code.push_str("    // TODO: Implement parser logic\n");
        code.push_str("    return NULL;\n");
        code.push_str("}\n\n");

        // Parser rule implementations
        for rule in grammar.parser_rules() {
            code.push_str(&format!("ParseError* parse_{}(Parser *parser) {{\n", rule.name));
            code.push_str("    // TODO: Implement rule parsing\n");
            code.push_str("    return NULL;\n");
            code.push_str("}\n\n");
        }

        code
    }
}

impl Default for CCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator for CCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, grammar: &Grammar, _config: &CodeGenConfig) -> Result<String> {
        let header = self.generate_header(grammar);
        let source = self.generate_source(grammar);
        
        // Return both header and source combined with a separator
        Ok(format!("// ===== HEADER FILE: {}.h =====\n{}\n\n// ===== SOURCE FILE: {}.c =====\n{}", 
                   grammar.name, header, grammar.name, source))
    }

    fn target_language(&self) -> &'static str {
        "C"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::GrammarParser;
    use crate::core::GrammarParser as GrammarParserTrait;

    #[test]
    fn test_c_codegen_basic() {
        let grammar_text = r#"
            grammar Calculator;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("HEADER FILE"));
        assert!(code.contains("SOURCE FILE"));
        assert!(code.contains("typedef enum"));
        assert!(code.contains("TOKEN_NUMBER"));
    }

    #[test]
    fn test_c_codegen_token_types() {
        let grammar_text = r#"
            grammar Test;
            
            rule: ID NUMBER;
            
            ID: [a-zA-Z_]+;
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("TOKEN_ID"));
        assert!(code.contains("TOKEN_NUMBER"));
        assert!(code.contains("TOKEN_EOF"));
    }

    #[test]
    fn test_c_codegen_structures() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("typedef struct"));
        assert!(code.contains("Token"));
        assert!(code.contains("Lexer"));
        assert!(code.contains("Parser"));
        assert!(code.contains("ParseError"));
    }

    #[test]
    fn test_c_codegen_functions() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("lexer_new"));
        assert!(code.contains("lexer_free"));
        assert!(code.contains("parser_new"));
        assert!(code.contains("parser_free"));
        assert!(code.contains("parser_parse"));
    }

    #[test]
    fn test_c_codegen_memory_helpers() {
        let grammar_text = r#"
            grammar Test;
            
            expr: term;
            term: NUMBER;
            
            NUMBER: [0-9]+;
        "#;
        
        let parser = crate::parser::GrammarParser::new();
        let grammar = parser.parse_string(grammar_text, "test.g4").expect("Failed to parse");
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("safe_malloc"));
        assert!(code.contains("safe_strdup"));
        assert!(code.contains("free_token"));
    }

    #[test]
    fn test_c_codegen_target_language() {
        let codegen = CCodeGenerator::new();
        assert_eq!(codegen.target_language(), "C");
    }

    #[test]
    fn test_c_codegen_with_completejson() {
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
        
        let codegen = CCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = codegen.generate(&grammar, &config).expect("Failed to generate");
        
        assert!(code.contains("TOKEN_STRING"));
        assert!(code.contains("TOKEN_NUMBER"));
        assert!(code.contains("parse_json"));
        assert!(code.contains("parse_value"));
    }
}
