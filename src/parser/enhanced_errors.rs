//! Enhanced error messages for the parser.
//!
//! This module provides utilities for generating better error messages
//! with context, expected tokens, and helpful suggestions.

use super::token::{Token, TokenKind};
use crate::core::Error;

/// Context information for error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub line: usize,
    pub column: usize,
    pub filename: Option<String>,
    pub surrounding_text: Option<String>,
    pub expected: Vec<TokenKind>,
    pub found: Option<TokenKind>,
    pub found_text: Option<String>,
    pub rule_context: Option<String>,
    pub suggestion: Option<String>,
}

impl ErrorContext {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            filename: None,
            surrounding_text: None,
            expected: Vec::new(),
            found: None,
            found_text: None,
            rule_context: None,
            suggestion: None,
        }
    }

    pub fn with_expected(mut self, expected: Vec<TokenKind>) -> Self {
        self.expected = expected;
        self
    }

    pub fn with_found(mut self, found: TokenKind, text: Option<String>) -> Self {
        self.found = Some(found);
        self.found_text = text;
        self
    }

    pub fn with_context(mut self, rule: String, surrounding: String) -> Self {
        self.rule_context = Some(rule);
        self.surrounding_text = Some(surrounding);
        self
    }

    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }

    pub fn generate_message(&self) -> String {
        let mut msg = String::new();

        // Add location
        let location = if let Some(ref filename) = self.filename {
            format!("{}:{}:{}", filename, self.line, self.column)
        } else {
            format!("{}:{}", self.line, self.column)
        };

        // Add rule context if available
        if let Some(ref rule) = self.rule_context {
            msg.push_str(&format!("In rule '{}': ", rule));
        }

        // Main error message
        if !self.expected.is_empty() {
            if self.expected.len() == 1 {
                msg.push_str(&format!("Expected {}, ", self.expected[0]));
            } else {
                let expected_str = self.expected.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                msg.push_str(&format!("Expected one of: {}, ", expected_str));
            }
        }

        if let Some(ref found) = self.found {
            if let Some(ref found_text) = self.found_text {
                msg.push_str(&format!("but found {} ('{}')", found, found_text));
            } else {
                msg.push_str(&format!("but found {}", found));
            }
        } else {
            msg.push_str("but found unexpected token");
        }

        // Add surrounding text if available
        if let Some(ref surrounding) = self.surrounding_text {
            msg.push_str(&format!("\n  Context: {}", surrounding));
        }

        // Add suggestion if available
        if let Some(ref suggestion) = self.suggestion {
            msg.push_str(&format!("\n  Suggestion: {}", suggestion));
        }

        format!("Parse error at {}: {}", location, msg)
    }
}

/// Enhanced error with context
pub fn create_enhanced_error(
    token: &Token,
    expected: Vec<TokenKind>,
    rule_context: Option<&str>,
    source: Option<&str>,
) -> Error {
    let mut context = ErrorContext::new(token.line, token.column)
        .with_expected(expected)
        .with_found(token.kind, Some(token.text.clone()));

    if let Some(rule) = rule_context {
        let surrounding = extract_surrounding_text(token, source);
        context = context.with_context(rule.to_string(), surrounding);
    }

    // Generate suggestion
    let suggestion = generate_suggestion(&context);
    let mut ctx = context;
    if let Some(sugg) = suggestion {
        ctx.suggestion = Some(sugg);
    }

    Error::parse(
        format!("{}:{}", ctx.line, ctx.column),
        ctx.generate_message(),
    )
}

/// Extract surrounding text for error context
fn extract_surrounding_text(token: &Token, source: Option<&str>) -> String {
    if let Some(source_text) = source {
        let lines: Vec<&str> = source_text.lines().collect();
        if token.line > 0 && token.line <= lines.len() {
            let line_text = lines[token.line - 1];
            let start = if token.column > 20 {
                token.column - 20
            } else {
                0
            };
            let end = (token.column + 20).min(line_text.len());
            let snippet = &line_text[start..end];
            
            // Show the line with a caret pointing to the error
            let caret_pos = token.column - start;
            let caret = format!("{}^", " ".repeat(caret_pos));
            format!("{}\n{}", snippet, caret)
        } else {
            format!("line {}", token.line)
        }
    } else {
        format!("line {}, column {}", token.line, token.column)
    }
}

/// Generate helpful suggestions based on error context
fn generate_suggestion(context: &ErrorContext) -> Option<String> {
    // Common typos and suggestions
    if let Some(ref found) = context.found {
        match found {
            TokenKind::Identifier if context.expected.contains(&TokenKind::Semicolon) => {
                Some("Did you forget a semicolon?".to_string())
            }
            TokenKind::Semicolon if context.expected.contains(&TokenKind::RightBrace) => {
                Some("Did you mean '}'?".to_string())
            }
            TokenKind::RightBrace if context.expected.contains(&TokenKind::Semicolon) => {
                Some("Did you mean ';'?".to_string())
            }
            TokenKind::RightParen if context.expected.contains(&TokenKind::RightBrace) => {
                Some("Did you mean '}' instead of ')'?".to_string())
            }
            TokenKind::RightBrace if context.expected.contains(&TokenKind::RightParen) => {
                Some("Did you mean ')' instead of '}'?".to_string())
            }
            _ if context.expected.contains(&TokenKind::Colon) && found == &TokenKind::Equals => {
                Some("Did you mean ':' instead of '='?".to_string())
            }
            _ if context.expected.contains(&TokenKind::Equals) && found == &TokenKind::Colon => {
                Some("Did you mean '=' instead of ':'?".to_string())
            }
            _ => None,
        }
    } else {
        None
    }
}

/// Validate character class ranges
pub fn validate_char_class_range(start: char, end: char) -> Result<(), Error> {
    if start > end {
        return Err(Error::parse(
            format!("character class"),
            format!("Invalid character range: '{}' must be <= '{}'", start, end),
        ));
    }

    // Check for valid Unicode ranges
    if !start.is_ascii() || !end.is_ascii() {
        // For non-ASCII, ensure valid Unicode scalar values
        if !char::from_u32(start as u32).is_some() || !char::from_u32(end as u32).is_some() {
            return Err(Error::parse(
                format!("character class"),
                format!("Invalid Unicode character in range"),
            ));
        }
    }

    Ok(())
}

/// Validate Unicode escape sequence
pub fn validate_unicode_escape(hex: &str) -> Result<char, Error> {
    if hex.len() != 4 {
        return Err(Error::parse(
            format!("unicode escape"),
            format!("Unicode escape must be 4 hex digits, got {} digits", hex.len()),
        ));
    }

    let code = u32::from_str_radix(hex, 16)
        .map_err(|_| Error::parse(
            format!("unicode escape"),
            format!("Invalid hex digits in unicode escape: \\u{}", hex),
        ))?;

    char::from_u32(code)
        .ok_or_else(|| Error::parse(
            format!("unicode escape"),
            format!("Invalid Unicode code point: U+{:04X}", code),
        ))
}

/// Parse Unicode escape sequences (supports both \uXXXX and \u{XXXXXX})
pub fn parse_unicode_escape(text: &str) -> Result<char, Error> {
    if text.starts_with("\\u{") {
        // Extended Unicode escape: \u{XXXXXX}
        let hex = text.strip_prefix("\\u{")
            .and_then(|s| s.strip_suffix('}'))
            .ok_or_else(|| Error::parse(
                format!("unicode escape"),
                "Unclosed Unicode escape sequence".to_string(),
            ))?;

        if hex.len() > 6 {
            return Err(Error::parse(
                format!("unicode escape"),
                format!("Unicode escape too long: max 6 hex digits, got {}", hex.len()),
            ));
        }

        let code = u32::from_str_radix(hex, 16)
            .map_err(|_| Error::parse(
                format!("unicode escape"),
                format!("Invalid hex digits in unicode escape: \\u{{{}}}", hex),
            ))?;

        char::from_u32(code)
            .ok_or_else(|| Error::parse(
                format!("unicode escape"),
                format!("Invalid Unicode code point: U+{:04X}", code),
            ))
    } else if text.starts_with("\\u") {
        // Standard Unicode escape: \uXXXX
        let hex = &text[2..];
        validate_unicode_escape(hex)
    } else {
        Err(Error::parse(
            format!("unicode escape"),
            format!("Invalid unicode escape format: {}", text),
        ))
    }
}

/// Edge case validation for grammar elements
pub fn validate_grammar_edge_cases(
    token: &Token,
    context: &str,
) -> Result<(), Error> {
    // Check for common edge cases
    
    // Empty alternatives
    if context.contains("||") {
        return Err(Error::parse(
            format!("{}:{}", token.line, token.column),
            "Empty alternative detected. Did you mean to add content between '|'?".to_string(),
        ));
    }

    // Unterminated strings
    if context.contains("'") && !context.matches("'").count() % 2 == 0 {
        return Err(Error::parse(
            format!("{}:{}", token.line, token.column),
            "Unterminated string literal".to_string(),
        ));
    }

    // Unmatched brackets
    let open_braces = context.matches('{').count();
    let close_braces = context.matches('}').count();
    if open_braces != close_braces {
        return Err(Error::parse(
            format!("{}:{}", token.line, token.column),
            format!("Unmatched braces: {} opening, {} closing", open_braces, close_braces),
        ));
    }

    let open_parens = context.matches('(').count();
    let close_parens = context.matches(')').count();
    if open_parens != close_parens {
        return Err(Error::parse(
            format!("{}:{}", token.line, token.column),
            format!("Unmatched parentheses: {} opening, {} closing", open_parens, close_parens),
        ));
    }

    let open_brackets = context.matches('[').count();
    let close_brackets = context.matches(']').count();
    if open_brackets != close_brackets {
        return Err(Error::parse(
            format!("{}:{}", token.line, token.column),
            format!("Unmatched brackets: {} opening, {} closing", open_brackets, close_brackets),
        ));
    }

    Ok(())
}

