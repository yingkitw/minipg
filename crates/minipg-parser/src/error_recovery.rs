//! Error recovery strategies for the parser.

use crate::token::TokenKind;

/// Error recovery strategy.
pub trait RecoveryStrategy {
    /// Check if we should attempt recovery at this token.
    fn should_recover(&self, kind: TokenKind) -> bool;
    
    /// Get synchronization tokens for recovery.
    fn sync_tokens(&self) -> &[TokenKind];
}

/// Default recovery strategy that syncs on statement boundaries.
pub struct DefaultRecovery;

impl RecoveryStrategy for DefaultRecovery {
    fn should_recover(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Semicolon | TokenKind::RightBrace | TokenKind::Eof
        )
    }
    
    fn sync_tokens(&self) -> &[TokenKind] {
        &[
            TokenKind::Semicolon,
            TokenKind::Grammar,
            TokenKind::Options,
            TokenKind::Import,
            TokenKind::Fragment,
        ]
    }
}

/// Recovery context for tracking error state.
#[derive(Debug)]
pub struct RecoveryContext {
    pub errors: Vec<String>,
    pub in_panic_mode: bool,
}

impl RecoveryContext {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            in_panic_mode: false,
        }
    }
    
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.in_panic_mode = true;
    }
    
    pub fn exit_panic_mode(&mut self) {
        self.in_panic_mode = false;
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for RecoveryContext {
    fn default() -> Self {
        Self::new()
    }
}
