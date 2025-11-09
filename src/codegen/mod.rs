//! Code generation for parser generators.

pub mod rust;
pub mod python;
pub mod javascript;
pub mod typescript;
pub mod go;
pub mod c;
pub mod cpp;
pub mod java;
pub mod template;
pub mod visitor_gen;
pub mod dfa;
pub mod lookup_table;
pub mod modes;
pub mod actions;
pub mod rule_body;
pub mod common;
pub mod registry;
pub mod pattern_match;

pub use rust::RustCodeGenerator;
pub use python::PythonCodeGenerator;
pub use javascript::JavaScriptCodeGenerator;
pub use typescript::TypeScriptCodeGenerator;
pub use go::GoCodeGenerator;
pub use c::CCodeGenerator;
pub use cpp::CppCodeGenerator;
pub use java::JavaCodeGenerator;

use crate::analysis::AnalysisResult;
use crate::core::{types::CodeGenConfig, Result};

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

    pub fn generate_from_analysis(&self, analysis: &AnalysisResult) -> Result<String> {
        self.registry.generate(
            &self.config.target_language,
            &analysis.grammar,
            &self.config,
        )
    }
}
