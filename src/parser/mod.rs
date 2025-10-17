//! Grammar parser implementation.

pub mod lexer;
pub mod parser;
pub mod token;
pub mod error_recovery;

pub use lexer::Lexer;
pub use parser::Parser;
pub use token::{Token, TokenKind};

use crate::ast::Grammar;
use crate::core::{GrammarParser as GrammarParserTrait, Result};
use std::path::Path;

/// Main grammar parser.
pub struct GrammarParser;

impl GrammarParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GrammarParser {
    fn default() -> Self {
        Self::new()
    }
}

impl GrammarParserTrait for GrammarParser {
    type Output = Grammar;

    fn parse_file(&self, path: &Path) -> Result<Self::Output> {
        let source = std::fs::read_to_string(path)?;
        let filename = path.to_string_lossy().to_string();
        self.parse_string(&source, &filename)
    }

    fn parse_string(&self, source: &str, filename: &str) -> Result<Self::Output> {
        let lexer = Lexer::new(source, filename);
        let mut parser = Parser::new(lexer);
        parser.parse()
    }
}
