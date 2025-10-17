//! Error types for minipg.

use thiserror::Error;

/// Result type alias for minipg operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for minipg.
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at {location}: {message}")]
    Parse {
        location: String,
        message: String,
    },

    #[error("Semantic error: {0}")]
    Semantic(String),

    #[error("Code generation error: {0}")]
    CodeGen(String),

    #[error("Invalid grammar: {0}")]
    InvalidGrammar(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid command line argument: {0}")]
    InvalidArgument(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    pub fn parse(location: impl Into<String>, message: impl Into<String>) -> Self {
        Error::Parse {
            location: location.into(),
            message: message.into(),
        }
    }

    pub fn semantic(message: impl Into<String>) -> Self {
        Error::Semantic(message.into())
    }

    pub fn codegen(message: impl Into<String>) -> Self {
        Error::CodeGen(message.into())
    }

    pub fn invalid_grammar(message: impl Into<String>) -> Self {
        Error::InvalidGrammar(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Error::Internal(message.into())
    }
}
