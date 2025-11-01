//! MCP (Model Context Protocol) server for minipg
//!
//! This module exposes minipg functionality via the MCP protocol using rmcp.
//! It allows AI assistants and other tools to interact with minipg programmatically.

use crate::analysis::SemanticAnalyzer;
use crate::codegen::{RustCodeGenerator, PythonCodeGenerator, 
                     JavaScriptCodeGenerator, TypeScriptCodeGenerator, GoCodeGenerator,
                     CCodeGenerator, CppCodeGenerator, JavaCodeGenerator};
use crate::core::{types::CodeGenConfig, GrammarParser, SemanticAnalyzer as SemanticAnalyzerTrait, CodeGenerator as CodeGeneratorTrait};
use crate::parser::GrammarParser as Parser;
use rmcp::{
    ErrorData as McpError,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// MCP server for minipg
#[derive(Clone)]
pub struct MinipgServer {
    tool_router: ToolRouter<Self>,
}

impl MinipgServer {
    /// Create a new minipg MCP server
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for MinipgServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for parse_grammar tool
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ParseGrammarRequest {
    /// The grammar text to parse
    pub grammar_text: String,
    /// Filename for error reporting (default: "grammar.g4")
    #[serde(default = "default_filename")]
    pub filename: String,
}

fn default_filename() -> String {
    "grammar.g4".to_string()
}

/// Request for validate_grammar tool
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ValidateGrammarRequest {
    /// The grammar text to validate
    pub grammar_text: String,
}

/// Request for generate_code tool
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GenerateCodeRequest {
    /// The grammar text to generate code from
    pub grammar_text: String,
    /// Target language (rust, python, javascript, typescript, go, java, c, cpp)
    pub target_language: String,
    /// Package name for generated code
    pub package_name: Option<String>,
    /// Generate visitor pattern (default: false)
    #[serde(default)]
    pub generate_visitor: bool,
    /// Generate listener pattern (default: true)
    #[serde(default = "default_true")]
    pub generate_listener: bool,
}

fn default_true() -> bool {
    true
}

/// Request for get_grammar_info tool
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetGrammarInfoRequest {
    /// The grammar text to analyze
    pub grammar_text: String,
}

/// Response for list_target_languages
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LanguageInfo {
    pub name: String,
    pub description: String,
}

#[tool_router]
impl MinipgServer {
    /// Parse a grammar specification and return the AST
    #[tool(description = "Parse a grammar specification and return the AST")]
    pub async fn parse_grammar(
        &self,
        params: Parameters<ParseGrammarRequest>,
    ) -> Result<CallToolResult, McpError> {
        let parser = Parser::new();
        match parser.parse_string(&params.0.grammar_text, &params.0.filename) {
            Ok(grammar) => {
                let result = serde_json::json!({
                    "success": true,
                    "grammar": {
                        "name": grammar.name,
                        "type": format!("{:?}", grammar.grammar_type),
                        "rules_count": grammar.rules.len(),
                        "parser_rules_count": grammar.parser_rules().count(),
                        "lexer_rules_count": grammar.lexer_rules().count(),
                        "imports": grammar.imports,
                        "options": grammar.options,
                    }
                });
                Ok(CallToolResult::success(vec![Content::json(result)?]))
            }
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("{}", e)
                });
                Ok(CallToolResult::success(vec![Content::json(result)?]))
            }
        }
    }

    /// Validate a grammar specification and return diagnostics
    #[tool(description = "Validate a grammar specification and return diagnostics")]
    pub async fn validate_grammar(
        &self,
        params: Parameters<ValidateGrammarRequest>,
    ) -> Result<CallToolResult, McpError> {
        let parser = Parser::new();
        let grammar = match parser.parse_string(&params.0.grammar_text, "validate.g4") {
            Ok(g) => g,
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": format!("Parse error: {}", e),
                    "diagnostics": []
                });
                return Ok(CallToolResult::success(vec![Content::json(result)?]));
            }
        };
        
        let analyzer = SemanticAnalyzer::new();
        let analysis = analyzer.analyze(&grammar)
            .map_err(|e| {
                let msg = format!("Analysis error: {}", e);
                McpError::internal_error(msg, None)
            })?;
        
        let diagnostics: Vec<serde_json::Value> = analysis.diagnostics.iter().map(|d| {
            serde_json::json!({
                "severity": format!("{:?}", d.severity),
                "message": d.message,
                "location": d.location.as_ref().map(|l| serde_json::json!({
                    "line": l.line,
                    "column": l.column,
                    "file": l.file
                }))
            })
        }).collect();
        
        let has_warnings = analysis.diagnostics.iter()
            .any(|d| matches!(d.severity, crate::core::DiagnosticSeverity::Warning));
        
        let result = serde_json::json!({
            "success": !analysis.has_errors(),
            "has_errors": analysis.has_errors(),
            "has_warnings": has_warnings,
            "diagnostics": diagnostics
        });
        
        Ok(CallToolResult::success(vec![Content::json(result)?]))
    }

    /// Generate parser code from a grammar specification
    #[tool(description = "Generate parser code from a grammar specification")]
    pub async fn generate_code(
        &self,
        params: Parameters<GenerateCodeRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Parse grammar
        let parser = Parser::new();
        let grammar = parser.parse_string(&params.0.grammar_text, "generate.g4")
            .map_err(|e| {
                let msg = format!("Parse error: {}", e);
                McpError::invalid_params(msg, None)
            })?;
        
        // Analyze grammar
        let analyzer = SemanticAnalyzer::new();
        let analysis = analyzer.analyze(&grammar)
            .map_err(|e| McpError::internal_error(format!("Analysis error: {}", e), None))?;
        
        if analysis.has_errors() {
            let diagnostics: Vec<serde_json::Value> = analysis.diagnostics.iter().map(|d| {
                serde_json::json!({
                    "severity": format!("{:?}", d.severity),
                    "message": d.message
                })
            }).collect();
            let result = serde_json::json!({
                "success": false,
                "error": "Grammar has errors, cannot generate code",
                "diagnostics": diagnostics
            });
            return Ok(CallToolResult::success(vec![Content::json(result)?]));
        }
        
        // Generate code
        let config = CodeGenConfig {
            target_language: params.0.target_language.clone(),
            output_directory: ".".to_string(),
            package_name: params.0.package_name.clone(),
            generate_listener: params.0.generate_listener,
            generate_visitor: params.0.generate_visitor,
        };
        
        let code = match params.0.target_language.as_str() {
            "rust" => {
                let generator = RustCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "python" => {
                let generator = PythonCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "javascript" | "js" => {
                let generator = JavaScriptCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "typescript" | "ts" => {
                let generator = TypeScriptCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "go" => {
                let generator = GoCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "java" => {
                let generator = JavaCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "c" => {
                let generator = CCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            "cpp" | "c++" => {
                let generator = CppCodeGenerator::new();
                generator.generate(&grammar, &config)
            }
            _ => return Err(McpError::invalid_params(
                format!("Unsupported target language: {}", params.0.target_language),
                None,
            )),
        }.map_err(|e| {
            let msg = format!("Code generation error: {}", e);
            McpError::internal_error(msg, None)
        })?;
        
        let result = serde_json::json!({
            "success": true,
            "code": code,
            "grammar_name": grammar.name,
            "target_language": params.0.target_language
        });
        
        Ok(CallToolResult::success(vec![Content::json(result)?]))
    }

    /// Get detailed information about a parsed grammar
    #[tool(description = "Get detailed information about a parsed grammar")]
    pub async fn get_grammar_info(
        &self,
        params: Parameters<GetGrammarInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        let parser = Parser::new();
        let grammar = parser.parse_string(&params.0.grammar_text, "info.g4")
            .map_err(|e| {
                let msg = format!("Parse error: {}", e);
                McpError::invalid_params(msg, None)
            })?;
        
        let parser_rules: Vec<String> = grammar.parser_rules()
            .map(|r| r.name.clone())
            .collect();
        
        let lexer_rules: Vec<String> = grammar.lexer_rules()
            .map(|r| r.name.clone())
            .collect();
        
        let result = serde_json::json!({
            "name": grammar.name,
            "type": format!("{:?}", grammar.grammar_type),
            "parser_rules": parser_rules,
            "lexer_rules": lexer_rules,
            "imports": grammar.imports,
            "options": grammar.options,
            "rules_count": grammar.rules.len(),
            "parser_rules_count": parser_rules.len(),
            "lexer_rules_count": lexer_rules.len(),
        });
        
        Ok(CallToolResult::success(vec![Content::json(result)?]))
    }

    /// List all supported target languages for code generation
    #[tool(description = "List all supported target languages for code generation")]
    pub async fn list_target_languages(
        &self,
        _params: Parameters<()>,
    ) -> Result<CallToolResult, McpError> {
        let languages = vec![
            LanguageInfo { name: "rust".to_string(), description: "Rust language".to_string() },
            LanguageInfo { name: "python".to_string(), description: "Python 3 language".to_string() },
            LanguageInfo { name: "javascript".to_string(), description: "JavaScript (ES6+)".to_string() },
            LanguageInfo { name: "typescript".to_string(), description: "TypeScript language".to_string() },
            LanguageInfo { name: "go".to_string(), description: "Go language".to_string() },
            LanguageInfo { name: "java".to_string(), description: "Java language".to_string() },
            LanguageInfo { name: "c".to_string(), description: "C language".to_string() },
            LanguageInfo { name: "cpp".to_string(), description: "C++ language".to_string() },
        ];
        
        let result = serde_json::json!({
            "languages": languages
        });
        
        Ok(CallToolResult::success(vec![Content::json(result)?]))
    }
}

#[tool_handler(router = self.tool_router)]
impl rmcp::ServerHandler for MinipgServer {}

/// Create a new minipg MCP server instance
pub fn create_server() -> MinipgServer {
    MinipgServer::new()
}
