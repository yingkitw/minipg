//! Code generation for parser generators.

pub mod rust;
pub mod python;
pub mod javascript;
pub mod template;
pub mod visitor_gen;

pub use rust::RustCodeGenerator;
pub use python::PythonCodeGenerator;
pub use javascript::JavaScriptCodeGenerator;

use minipg_analysis::AnalysisResult;
use minipg_core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

/// Main code generator dispatcher.
pub struct CodeGenerator {
    config: CodeGenConfig,
}

impl CodeGenerator {
    pub fn new(config: CodeGenConfig) -> Self {
        Self { config }
    }

    pub fn generate_from_analysis(&self, analysis: &AnalysisResult) -> Result<String> {
        match self.config.target_language.as_str() {
            "rust" => {
                let generator = RustCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            _ => Err(minipg_core::Error::codegen(format!(
                "unsupported target language: {}",
                self.config.target_language
            ))),
        }
    }
}
