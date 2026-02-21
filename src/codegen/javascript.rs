//! JavaScript code generator.

use crate::ast::{Grammar, Rule};
use crate::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// JavaScript code generator.
pub struct JavaScriptCodeGenerator;

impl JavaScriptCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_lexer(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Lexer", grammar.name);

        code.push_str("/**\n");
        code.push_str(" * Lexer with DFA-based tokenization and error recovery.\n");
        code.push_str(" */\n");
        code.push_str(&format!("class {} {{\n", class_name));
        code.push_str("  constructor(input) {\n");
        code.push_str("    this.input = input;\n");
        code.push_str("    this.position = 0;\n");
        code.push_str("  }\n\n");

        code.push_str("  /**\n");
        code.push_str("   * Get the next token from input.\n");
        code.push_str("   * @returns {Token} The next token\n");
        code.push_str("   * @throws {ParseError} If tokenization fails\n");
        code.push_str("   */\n");
        code.push_str("  nextToken() {\n");
        code.push_str("    // Skip whitespace\n");
        code.push_str("    this._skipWhitespace();\n\n");
        code.push_str("    const startPos = this.position;\n\n");
        code.push_str("    // Check for EOF\n");
        code.push_str("    if (this.position >= this.input.length) {\n");
        code.push_str("      return new Token(TokenKind.EOF, '', startPos);\n");
        code.push_str("    }\n\n");
        code.push_str("    // Use DFA for tokenization\n");
        code.push_str("    const token = this._nextTokenDfa();\n");
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
        code.push_str("   * @returns {{tokens: Token[], errors: ParseError[]}}\n");
        code.push_str("   */\n");
        code.push_str("  tokenizeAll() {\n");
        code.push_str("    const tokens = [];\n");
        code.push_str("    const errors = [];\n\n");
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

        code.push_str("  _skipWhitespace() {\n");
        code.push_str("    while (this.position < this.input.length) {\n");
        code.push_str("      const ch = this.input[this.position];\n");
        code.push_str("      if (ch === ' ' || ch === '\\t' || ch === '\\r' || ch === '\\n') {\n");
        code.push_str("        this.position++;\n");
        code.push_str("      } else {\n");
        code.push_str("        break;\n");
        code.push_str("      }\n");
        code.push_str("    }\n");
        code.push_str("  }\n\n");

        code.push_str("  _nextTokenDfa() {\n");
        code.push_str("    // TODO: Implement DFA logic\n");
        code.push_str("    return null;\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");

        code
    }

    fn generate_parser(&self, grammar: &Grammar) -> String {
        let mut code = String::new();
        let class_name = format!("{}Parser", grammar.name);

        code.push_str(&format!("class {} {{\n", class_name));
        code.push_str("  constructor(tokens) {\n");
        code.push_str("    this.tokens = tokens;\n");
        code.push_str("    this.position = 0;\n");

        // Insert @members named action if present
        if let Some(members_code) = grammar.named_actions.get("members") {
            code.push_str("    // Custom members from @members action\n");
            code.push_str("    ");
            code.push_str(members_code);
            code.push_str("\n");
        }

        code.push_str("  }\n\n");

        // Generate parse methods for each rule
        for rule in grammar.parser_rules() {
            code.push_str(&self.generate_rule_method(rule));
        }

        code.push_str("}\n\n");
        code
    }

    fn generate_rule_method(&self, rule: &Rule) -> String {
        let mut code = String::new();
        let method_name = to_camel_case(&rule.name);

        // Generate JSDoc comment
        code.push_str("  /**\n");
        code.push_str(&format!("   * Parse {} rule.\n", rule.name));
        if !rule.arguments.is_empty() {
            for arg in &rule.arguments {
                let type_str = arg
                    .arg_type
                    .as_ref()
                    .map(|t| format!("{{{}}}", t))
                    .unwrap_or_else(|| "{*}".to_string());
                code.push_str(&format!(
                    "   * @param {} {} - Rule argument\n",
                    type_str, arg.name
                ));
            }
        }
        if !rule.returns.is_empty() {
            if rule.returns.len() == 1 {
                let ret_type = rule.returns[0]
                    .return_type
                    .as_ref()
                    .map(|t| format!("{{{}}}", t))
                    .unwrap_or_else(|| "{AstNode}".to_string());
                code.push_str(&format!(
                    "   * @returns {} - {}\n",
                    ret_type, rule.returns[0].name
                ));
            } else {
                code.push_str("   * @returns {Array} - Tuple of return values\n");
            }
        }
        code.push_str("   */\n");

        // Generate method signature
        code.push_str(&format!("  parse{}(", capitalize(&method_name)));

        // Add arguments
        for (i, arg) in rule.arguments.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&arg.name);
        }

        code.push_str(") {\n");

        // Generate local variables
        for local in &rule.locals {
            code.push_str(&format!("    let {};\n", local.name));
        }
        if !rule.locals.is_empty() {
            code.push_str("\n");
        }

        // Generate rule body
        if rule.alternatives.is_empty() {
            code.push_str("    // Empty rule\n");
            code.push_str(&format!("    return new {}Node();\n", to_pascal_case(&rule.name)));
        } else if rule.alternatives.len() == 1 {
            // Single alternative
            let alt = &rule.alternatives[0];
            code.push_str(&self.generate_javascript_alternative(alt, rule));
        } else {
            // Multiple alternatives - try each in order
            code.push_str("    // Try alternatives\n");
            code.push_str("    const startPos = this.position;\n\n");
            for (i, alt) in rule.alternatives.iter().enumerate() {
                if i > 0 {
                    code.push_str("\n    // Try next alternative\n");
                    code.push_str("    this.position = startPos;\n");
                }
                code.push_str("    try {\n");
                let alt_code = self.generate_javascript_alternative(alt, rule);
                // Indent the alternative code
                for line in alt_code.lines() {
                    if !line.trim().is_empty() {
                        code.push_str(&format!("  {}\n", line));
                    }
                }
                code.push_str("    } catch (err) {\n");
                if i == rule.alternatives.len() - 1 {
                    code.push_str("      throw err;\n");
                } else {
                    code.push_str("      if (!(err instanceof ParseError)) throw err;\n");
                }
                code.push_str("    }\n");
            }
        }
        code.push_str("  }\n\n");

        code
    }

    fn generate_token_types(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        code.push_str("const TokenKind = {\n");
        for rule in grammar.lexer_rules() {
            if !rule.is_fragment {
                code.push_str(&format!("  {}: '{}',\n", rule.name, rule.name));
            }
        }
        code.push_str("  EOF: 'EOF'\n");
        code.push_str("};\n\n");

        code
    }

    fn generate_ast_types(&self, grammar: &Grammar) -> String {
        let mut code = String::new();

        code.push_str("// AST node type definitions\n\n");
        
        for rule in grammar.parser_rules() {
            let class_name = to_pascal_case(&rule.name);
            let fields = self.extract_labeled_fields(rule);
            
            code.push_str("/**\n");
            code.push_str(&format!(" * AST node for {} rule.\n", rule.name));
            code.push_str(" */\n");
            code.push_str(&format!("class {}Node {{\n", class_name));
            
            if fields.is_empty() {
                code.push_str("  constructor() {}\n");
            } else {
                code.push_str("  constructor(");
                for (i, (field_name, _, _)) in fields.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    code.push_str(field_name);
                }
                code.push_str(") {\n");
                for (field_name, _, _) in &fields {
                    code.push_str(&format!("    this.{} = {};\n", field_name, field_name));
                }
                code.push_str("  }\n");
            }
            code.push_str("}\n\n");
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

    fn generate_javascript_alternative(&self, alt: &crate::ast::Alternative, rule: &Rule) -> String {
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
            code.push_str(&format!("      const {} = [];\\n", lbl));
        }
        if !list_vars.is_empty() {
            code.push_str("\\n");
        }

        // Generate parsing code for each element
        for element in &alt.elements {
            match element {
                Element::RuleRef { name, label, is_list } => {
                    let method_name = to_camel_case(name);
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("      {}.push(this.parse{}());\\n", lbl, capitalize(&method_name)));
                        } else {
                            code.push_str(&format!("      this.parse{}();\\n", capitalize(&method_name)));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("      const {} = this.parse{}();\\n", lbl, capitalize(&method_name)));
                        } else {
                            code.push_str(&format!("      this.parse{}();\\n", capitalize(&method_name)));
                        }
                    }
                }
                Element::Terminal { value, label, is_list } => {
                    code.push_str(&format!("      // Match terminal: {}\\n", value));
                    code.push_str("      if (this.position >= this.tokens.length) {\\n");
                    code.push_str(&format!("        throw new ParseError('Unexpected EOF, expected: {}', this.position);\\n", value));
                    code.push_str("      }\\n");
                    code.push_str(&format!("      if (this.tokens[this.position].kind === TokenKind.{}) {{\\n", value));
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        {}.push(this.tokens[this.position]);\\n", lbl));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        const {} = this.tokens[this.position];\\n", lbl));
                        }
                    }
                    code.push_str("        this.position++;\\n");
                    code.push_str("      } else {\\n");
                    code.push_str(&format!("        throw new ParseError(`Expected {}, got ${{this.tokens[this.position].kind}}`, this.position);\\n", value));
                    code.push_str("      }\\n");
                }
                Element::StringLiteral { value, label, is_list } => {
                    code.push_str(&format!("      // Match string literal: '{}'\\n", value));
                    code.push_str("      if (this.position >= this.tokens.length) {\\n");
                    code.push_str(&format!("        throw new ParseError('Unexpected EOF, expected: \"{}\"', this.position);\\n", value));
                    code.push_str("      }\\n");
                    code.push_str(&format!("      if (this.tokens[this.position].value === '{}') {{\\n", value));
                    if *is_list {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        {}.push(this.tokens[this.position]);\\n", lbl));
                        }
                    } else {
                        if let Some(lbl) = label {
                            code.push_str(&format!("        const {} = this.tokens[this.position];\\n", lbl));
                        }
                    }
                    code.push_str("        this.position++;\\n");
                    code.push_str("      } else {\\n");
                    code.push_str(&format!("        throw new ParseError(`Expected \\\"{}\\\", got ${{this.tokens[this.position].value}}`, this.position);\\n", value));
                    code.push_str("      }\\n");
                }
                Element::Optional { element, .. } => {
                    code.push_str("      // Optional element\\n");
                    code.push_str("      const savedPos = this.position;\\n");
                    code.push_str("      try {\\n");
                    // Recursively generate code for the optional element
                    code.push_str("        // Try to match optional element\\n");
                    code.push_str("      } catch (err) {\\n");
                    code.push_str("        // Optional failed, restore position\\n");
                    code.push_str("        this.position = savedPos;\\n");
                    code.push_str("      }\\n");
                }
                Element::ZeroOrMore { element, .. } => {
                    code.push_str("      // Zero or more repetition\\n");
                    code.push_str("      while (this.position < this.tokens.length) {\\n");
                    code.push_str("        const savedPos = this.position;\\n");
                    code.push_str("        try {\\n");
                    code.push_str("          // Try to match element\\n");
                    code.push_str("        } catch (err) {\\n");
                    code.push_str("          this.position = savedPos;\\n");
                    code.push_str("          break;\\n");
                    code.push_str("        }\\n");
                    code.push_str("      }\\n");
                }
                Element::OneOrMore { element, .. } => {
                    code.push_str("      // One or more repetition\\n");
                    code.push_str("      let matchCount = 0;\\n");
                    code.push_str("      while (this.position < this.tokens.length) {\\n");
                    code.push_str("        const savedPos = this.position;\\n");
                    code.push_str("        try {\\n");
                    code.push_str("          // Try to match element\\n");
                    code.push_str("          matchCount++;\\n");
                    code.push_str("        } catch (err) {\\n");
                    code.push_str("          this.position = savedPos;\\n");
                    code.push_str("          break;\\n");
                    code.push_str("        }\\n");
                    code.push_str("      }\\n");
                    code.push_str("      if (matchCount === 0) {\\n");
                    code.push_str("        throw new ParseError('Expected at least one match', this.position);\\n");
                    code.push_str("      }\\n");
                }
                _ => {
                    code.push_str("      // TODO: Handle other element types\\n");
                }
            }
        }

        // Build result node
        if !collected_vars.is_empty() || !list_vars.is_empty() {
            code.push_str("\\n      // Build AST node\\n");
            code.push_str(&format!("      return new {}Node(", to_pascal_case(&rule.name)));
            let all_vars: Vec<_> = collected_vars.iter().chain(list_vars.iter()).collect();
            for (i, var) in all_vars.iter().enumerate() {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(var);
            }
            code.push_str(");\\n");
        } else {
            code.push_str(&format!("      return new {}Node();\\n", to_pascal_case(&rule.name)));
        }

        code
    }
}

impl Default for JavaScriptCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGeneratorTrait for JavaScriptCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;

    fn generate(&self, input: &Self::Input, _config: &Self::Config) -> Result<String> {
        let mut code = String::new();

        // Header with browser compatibility note
        code.push_str(&format!("// Generated parser for {} grammar\n", input.name));
        code.push_str("// DO NOT EDIT - This file is automatically generated\n");
        code.push_str("// Browser-compatible ES6 module - no Node.js dependencies\n\n");

        // Insert @header named action if present
        if let Some(header_code) = input.named_actions.get("header") {
            code.push_str("// Custom header from @header action\n");
            code.push_str(header_code);
            code.push_str("\n\n");
        }

        // Error class
        code.push_str("/**\n");
        code.push_str(" * Parse error with context information.\n");
        code.push_str(" */\n");
        code.push_str("class ParseError extends Error {\n");
        code.push_str("  constructor(message, position, expected = [], found = null) {\n");
        code.push_str("    super(message);\n");
        code.push_str("    this.name = 'ParseError';\n");
        code.push_str("    this.message = message;\n");
        code.push_str("    this.position = position;\n");
        code.push_str("    this.expected = expected;\n");
        code.push_str("    this.found = found;\n");
        code.push_str("  }\n\n");
        code.push_str("  toString() {\n");
        code.push_str(
            "    let result = `Parse error at position ${this.position}: ${this.message}`;\n",
        );
        code.push_str("    if (this.expected.length > 0) {\n");
        code.push_str("      result += ` (expected: ${this.expected.join(', ')})`;\n");
        code.push_str("    }\n");
        code.push_str("    if (this.found) {\n");
        code.push_str("      result += ` (found: ${this.found})`;\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");

        // Token class
        code.push_str("/**\n");
        code.push_str(" * Token with position information.\n");
        code.push_str(" */\n");
        code.push_str("class Token {\n");
        code.push_str("  constructor(kind, text, position) {\n");
        code.push_str("    this.kind = kind;\n");
        code.push_str("    this.text = text;\n");
        code.push_str("    this.position = position;\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");

        // Token types
        code.push_str(&self.generate_token_types(input));

        // AST types
        code.push_str(&self.generate_ast_types(input));

        // Lexer
        code.push_str(&self.generate_lexer(input));

        // Generate parser
        code.push_str(&self.generate_parser(input));

        // ES6 module exports for browser compatibility
        code.push_str("\n// ES6 Module Exports\n");
        code.push_str(&format!("export {{ ParseError, Token, TokenKind, {}Lexer, {}Parser }};\n", 
            input.name, input.name));
        code.push_str("\n// Default export\n");
        code.push_str(&format!("export default {{ ParseError, Token, TokenKind, {}Lexer, {}Parser }};\n", 
            input.name, input.name));

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "javascript"
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
    fn test_javascript_codegen() {
        let mut grammar = Grammar::new("Calculator".to_string(), GrammarType::Parser);
        grammar.add_rule(Rule::parser_rule("expr".to_string()));
        grammar.add_rule(Rule::lexer_rule("NUMBER".to_string()));

        let generator = JavaScriptCodeGenerator::new();
        let config = CodeGenConfig::default();
        let code = generator.generate(&grammar, &config).unwrap();

        assert!(code.contains("class CalculatorLexer"));
        assert!(code.contains("class CalculatorParser"));
        assert!(code.contains("parseExpr"));
        assert!(code.contains("export {") || code.contains("export default"));
    }
}
