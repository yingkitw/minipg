//! Code generation for parser generators.

pub mod actions;
pub mod common;
pub mod dfa;
pub mod javascript;
pub mod lookup_table;
pub mod modes;
pub mod pattern_match;
pub mod python;
pub mod registry;
pub mod rule_body;
pub mod rust;
pub mod template;
pub mod visitor_gen;

pub use javascript::JavaScriptCodeGenerator;
pub use python::PythonCodeGenerator;
pub use rust::RustCodeGenerator;

use crate::analysis::AnalysisResult;
use crate::{types::CodeGenConfig, Result};

pub use registry::LanguageRegistry;

/// Main code generator dispatcher.
pub struct CodeGenerator {
    config: CodeGenConfig,
    registry: LanguageRegistry,
}

impl CodeGenerator {
    pub fn new(config: CodeGenConfig) -> Self {
        Self {
            config,
            registry: LanguageRegistry::new(),
        }
    }

    /// Create with a custom registry (for testing or extensions).
    pub fn with_registry(config: CodeGenConfig, registry: LanguageRegistry) -> Self {
        Self { config, registry }
    }

    /// Generate parser code from an analyzed grammar.
    ///
    /// This is the main entry point for code generation. It uses the configured
    /// target language and registry to dispatch to the appropriate code generator.
    ///
    /// # Arguments
    /// * `analysis` - The analysis result containing the validated grammar
    ///
    /// # Returns
    /// * `Result<String>` - The generated parser code, or an error if generation fails
    pub fn generate_from_analysis(&self, analysis: &AnalysisResult) -> Result<String> {
        self.registry.generate(
            &self.config.target_language,
            &analysis.grammar,
            &self.config,
        )
    }
}
