//! TypeScript code generator.

use minipg_ast::Grammar;
use minipg_core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// TypeScript code generator.
pub struct TypeScriptCodeGenerator;

impl TypeScriptCodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    fn generate_lexer(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Lexer", grammar.name);
        
        code.push_str("/**\n");
        code.push_str(" * Lexer with DFA-based tokenization and error recovery.\n");
        code.push_str(" */\n");
        code.push_str(&format!("export class {} {{\n", class_name));
        code.push_str("  private input: string;\n");
        code.push_str("  private position: number;\n\n");
        
        code.push_str("  constructor(input: string) {\n");
        code.push_str("    this.input = input;\n");
        code.push_str("    this.position = 0;\n");
        code.push_str("  }\n\n");
        
        code.push_str("  /**\n");
        code.push_str("   * Get the next token from input.\n");
        code.push_str("   * @returns The next token\n");
        code.push_str("   * @throws {ParseError} If tokenization fails\n");
        code.push_str("   */\n");
        code.push_str("  nextToken(): Token {\n");
        code.push_str("    // Skip whitespace\n");
        code.push_str("    this.skipWhitespace();\n\n");
        code.push_str("    const startPos = this.position;\n\n");
        code.push_str("    // Check for EOF\n");
        code.push_str("    if (this.position >= this.input.length) {\n");
        code.push_str("      return new Token(TokenKind.EOF, '', startPos);\n");
        code.push_str("    }\n\n");
        code.push_str("    // Use DFA for tokenization\n");
        code.push_str("    const token = this.nextTokenDfa();\n");
        code.push_str("    if (token) {\n");
        code.push_str("      token.position = startPos;\n");
        code.push_str("      return token;\n");
        code.push_str("    }\n\n");
        code.push_str("    // Error recovery: skip invalid character\n");
        code.push_str("    const invalidChar = this.input[this.position];\n");
        code.push_str("    this.position++;\n");
        code.push_str("    throw new ParseError(\n");
        code.push_str("      `Unexpected character: '${invalidChar}'`,\n");
        code.push_str("      startPos,\n");
        code.push_str("      [],\n");
        code.push_str("      invalidChar\n");
        code.push_str("    );\n");
        code.push_str("  }\n\n");
        
        code.push_str("  /**\n");
        code.push_str("   * Tokenize all input and collect errors.\n");
        code.push_str("   * @returns Object with tokens and errors arrays\n");
        code.push_str("   */\n");
        code.push_str("  tokenizeAll(): { tokens: Token[]; errors: ParseError[] } {\n");
        code.push_str("    const tokens: Token[] = [];\n");
        code.push_str("    const errors: ParseError[] = [];\n\n");
        code.push_str("    while (true) {\n");
        code.push_str("      try {\n");
        code.push_str("        const token = this.nextToken();\n");
        code.push_str("        tokens.push(token);\n");
        code.push_str("        if (token.kind === TokenKind.EOF) {\n");
        code.push_str("          break;\n");
        code.push_str("        }\n");
        code.push_str("      } catch (err) {\n");
        code.push_str("        if (err instanceof ParseError) {\n");
        code.push_str("          errors.push(err);\n");
        code.push_str("          // Continue tokenizing after error\n");
        code.push_str("          if (this.position >= this.input.length) {\n");
        code.push_str("            break;\n");
        code.push_str("          }\n");
        code.push_str("        } else {\n");
        code.push_str("          throw err;\n");
        code.push_str("        }\n");
        code.push_str("      }\n");
        code.push_str("    }\n\n");
        code.push_str("    return { tokens, errors };\n");
        code.push_str("  }\n\n");
        
        code.push_str("  private skipWhitespace(): void {\n");
        code.push_str("    while (this.position < this.input.length) {\n");
        code.push_str("      const ch = this.input[this.position];\n");
        code.push_str("      if (ch === ' ' || ch === '\\t' || ch === '\\r' || ch === '\\n') {\n");
        code.push_str("        this.position++;\n");
        code.push_str("      } else {\n");
        code.push_str("        break;\n");
        code.push_str("      }\n");
        code.push_str("    }\n");
        code.push_str("  }\n\n");
        
        code.push_str("  private nextTokenDfa(): Token | null {\n");
        code.push_str("    // TODO: Implement DFA logic\n");
        code.push_str("    return null;\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");
        
        code
    }
    
    fn generate_parser(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Parser", grammar.name);
        
        code.push_str(&format!("export class {} {{\n", class_name));
        code.push_str("  private tokens: Token[];\n");
        code.push_str("  private position: number;\n\n");
        
        code.push_str("  constructor(tokens: Token[]) {\n");
        code.push_str("    this.tokens = tokens;\n");
        code.push_str("    this.position = 0;\n");
        code.push_str("  }\n\n");
        
        // Generate parse methods for each rule
        for rule in grammar.parser_rules() {
            let method_name = to_camel_case(&rule.name);
            code.push_str(&format!("  parse{}(): any {{\n", capitalize(&method_name)));
            code.push_str("    // TODO: Implement rule parsing\n");
            code.push_str("    throw new Error('Not implemented');\n");
            code.push_str("  }\n\n");
        }
        
        code.push_str("}\n\n");
        code
    }
}

impl Default for TypeScriptCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGeneratorTrait for TypeScriptCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, input: &Self::Input, _config: &Self::Config) -> Result<String> {
        let mut code = String::new();
        
        // Header
        code.push_str(&format!("// Generated parser for {} grammar\n", input.name));
        code.push_str("// DO NOT EDIT - This file is automatically generated\n\n");
        
        // Error class
        code.push_str("/**\n");
        code.push_str(" * Parse error with context information.\n");
        code.push_str(" */\n");
        code.push_str("export class ParseError extends Error {\n");
        code.push_str("  public readonly position: number;\n");
        code.push_str("  public readonly expected: string[];\n");
        code.push_str("  public readonly found: string | null;\n\n");
        code.push_str("  constructor(\n");
        code.push_str("    message: string,\n");
        code.push_str("    position: number,\n");
        code.push_str("    expected: string[] = [],\n");
        code.push_str("    found: string | null = null\n");
        code.push_str("  ) {\n");
        code.push_str("    super(message);\n");
        code.push_str("    this.name = 'ParseError';\n");
        code.push_str("    this.position = position;\n");
        code.push_str("    this.expected = expected;\n");
        code.push_str("    this.found = found;\n");
        code.push_str("  }\n\n");
        code.push_str("  toString(): string {\n");
        code.push_str("    let result = `Parse error at position ${this.position}: ${this.message}`;\n");
        code.push_str("    if (this.expected.length > 0) {\n");
        code.push_str("      result += ` (expected: ${this.expected.join(', ')})`;\n");
        code.push_str("    }\n");
        code.push_str("    if (this.found) {\n");
        code.push_str("      result += ` (found: ${this.found})`;\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");
        
        // Token enum
        code.push_str("/**\n");
        code.push_str(" * Token kinds.\n");
        code.push_str(" */\n");
        code.push_str("export enum TokenKind {\n");
        for rule in input.lexer_rules() {
            if !rule.is_fragment {
                code.push_str(&format!("  {} = '{}',\n", rule.name, rule.name));
            }
        }
        code.push_str("  EOF = 'EOF'\n");
        code.push_str("}\n\n");
        
        // Token class
        code.push_str("/**\n");
        code.push_str(" * Token with position information.\n");
        code.push_str(" */\n");
        code.push_str("export class Token {\n");
        code.push_str("  constructor(\n");
        code.push_str("    public kind: TokenKind,\n");
        code.push_str("    public text: string,\n");
        code.push_str("    public position: number\n");
        code.push_str("  ) {}\n");
        code.push_str("}\n\n");
        
        // Lexer
        code.push_str(&self.generate_lexer(input));
        
        // Parser
        code.push_str(&self.generate_parser(input));
        
        Ok(code)
    }
    
    fn target_language(&self) -> &str {
        "typescript"
    }
}

fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }
    
    let mut result = parts[0].to_lowercase();
    for part in &parts[1..] {
        result.push_str(&capitalize(part));
    }
    result
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minipg_ast::{Grammar, Rule};
    use minipg_core::types::GrammarType;

    #[test]
    fn test_typescript_codegen() {
        let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Parser);
        grammar.add_rule(Rule::parser_rule("expr".to_string()));
        grammar.add_rule(Rule::lexer_rule("NUMBER".to_string()));
        
        let generator = TypeScriptCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = generator.generate(&grammar, &config).unwrap();
        
        assert!(code.contains("export class CalculatorLexer"));
        assert!(code.contains("export class CalculatorParser"));
        assert!(code.contains("parseExpr"));
        assert!(code.contains("export class ParseError"));
        assert!(code.contains("export enum TokenKind"));
    }
}
