//! Language registry for code generators.

use std::collections::HashMap;
use crate::ast::Grammar;
use crate::core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// Language registry for managing code generators.
pub struct LanguageRegistry {
    generators: HashMap<String, Box<dyn CodeGeneratorTrait<Input = Grammar, Config = CodeGenConfig>>>,
    aliases: HashMap<String, String>, // alias -> canonical name
}

impl LanguageRegistry {
    /// Create a new language registry with all built-in generators registered.
    pub fn new() -> Self {
        let mut reg = Self {
            generators: HashMap::new(),
            aliases: HashMap::new(),
        };
        
        // Register all built-in generators
        reg.register("rust", RustCodeGenerator::new());
        reg.register("python", PythonCodeGenerator::new());
        reg.register("javascript", JavaScriptCodeGenerator::new());
        reg.register("typescript", TypeScriptCodeGenerator::new());
        reg.register("go", GoCodeGenerator::new());
        reg.register("c", CCodeGenerator::new());
        reg.register("cpp", CppCodeGenerator::new());
        reg.register("java", JavaCodeGenerator::new());
        
        // Register aliases
        reg.register_alias("js", "javascript");
        reg.register_alias("ts", "typescript");
        reg.register_alias("c++", "cpp");
        
        reg
    }
    
    /// Register a code generator for a language.
    pub fn register(&mut self, name: &str, generator: impl CodeGeneratorTrait<Input = Grammar, Config = CodeGenConfig> + 'static) {
        self.generators.insert(name.to_string(), Box::new(generator));
    }
    
    /// Register a language alias.
    pub fn register_alias(&mut self, alias: &str, canonical: &str) {
        self.aliases.insert(alias.to_string(), canonical.to_string());
    }
    
    /// Get a code generator by name (supports aliases).
    pub fn get(&self, name: &str) -> Option<&dyn CodeGeneratorTrait<Input = Grammar, Config = CodeGenConfig>> {
        // Resolve alias if present
        let canonical_name = self.aliases.get(name).map(|s| s.as_str()).unwrap_or(name);
        self.generators.get(canonical_name).map(|g| g.as_ref())
    }
    
    /// Check if a language is supported.
    pub fn is_supported(&self, name: &str) -> bool {
        let canonical_name = self.aliases.get(name).map(|s| s.as_str()).unwrap_or(name);
        self.generators.contains_key(canonical_name)
    }
    
    /// Get list of all supported languages.
    pub fn supported_languages(&self) -> Vec<String> {
        self.generators.keys().cloned().collect()
    }
    
    /// Generate code using the registered generator.
    pub fn generate(&self, language: &str, grammar: &Grammar, config: &CodeGenConfig) -> Result<String> {
        match self.get(language) {
            Some(generator) => generator.generate(grammar, config),
            None => Err(crate::core::Error::codegen(format!(
                "unsupported target language: {} (supported: {})",
                language,
                self.supported_languages().join(", ")
            ))),
        }
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Import all generators
use super::{RustCodeGenerator, PythonCodeGenerator, JavaScriptCodeGenerator, TypeScriptCodeGenerator,
            GoCodeGenerator, CCodeGenerator, CppCodeGenerator, JavaCodeGenerator};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_creation() {
        let reg = LanguageRegistry::new();
        assert!(reg.is_supported("rust"));
        assert!(reg.is_supported("python"));
        assert!(reg.is_supported("javascript"));
    }
    
    #[test]
    fn test_alias_resolution() {
        let reg = LanguageRegistry::new();
        assert!(reg.is_supported("js"));
        assert!(reg.is_supported("ts"));
        assert!(reg.is_supported("c++"));
    }
    
    #[test]
    fn test_unsupported_language() {
        let reg = LanguageRegistry::new();
        assert!(!reg.is_supported("swift"));
    }
}

