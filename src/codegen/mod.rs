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

pub use rust::RustCodeGenerator;
pub use python::PythonCodeGenerator;
pub use javascript::JavaScriptCodeGenerator;
pub use typescript::TypeScriptCodeGenerator;
pub use go::GoCodeGenerator;
pub use c::CCodeGenerator;
pub use cpp::CppCodeGenerator;
pub use java::JavaCodeGenerator;

use crate::analysis::AnalysisResult;
use crate::core::{types::CodeGenConfig, CodeGenerator as CodeGeneratorTrait, Result};

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
            "python" => {
                let generator = PythonCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "javascript" | "js" => {
                let generator = JavaScriptCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "typescript" | "ts" => {
                let generator = TypeScriptCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "go" => {
                let generator = GoCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "c" => {
                let generator = CCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "cpp" | "c++" => {
                let generator = CppCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            "java" => {
                let generator = JavaCodeGenerator::new();
                generator.generate(&analysis.grammar, &self.config)
            }
            _ => Err(crate::core::Error::codegen(format!(
                "unsupported target language: {}",
                self.config.target_language
            ))),
        }
    }
}
