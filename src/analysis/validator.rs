//! Grammar validation.

use crate::ast::Grammar;
use crate::core::{GrammarValidator as GrammarValidatorTrait, Result};

/// Grammar validator.
pub struct GrammarValidator;

impl GrammarValidator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GrammarValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl GrammarValidatorTrait for GrammarValidator {
    type Input = Grammar;

    fn validate(&self, input: &Self::Input) -> Result<()> {
        // Basic validation
        if input.name.is_empty() {
            return Err(crate::core::Error::invalid_grammar("grammar name cannot be empty"));
        }

        if input.rules.is_empty() {
            return Err(crate::core::Error::invalid_grammar("grammar must have at least one rule"));
        }

        Ok(())
    }
}
