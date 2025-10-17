//! Lookup table generation for efficient character class matching.
//!
//! This module generates const lookup tables at compile time for fast
//! character classification without runtime branching.

use crate::ast::Rule;
use std::collections::HashMap;

/// Character class ID for lookup table
pub type CharClassId = u8;

/// Lookup table builder for character classes
pub struct LookupTableBuilder {
    /// Map from character to class ID
    char_to_class: HashMap<char, CharClassId>,
    /// Next available class ID
    next_class_id: CharClassId,
    /// Class names for documentation
    class_names: HashMap<CharClassId, String>,
}

impl LookupTableBuilder {
    pub fn new() -> Self {
        Self {
            char_to_class: HashMap::new(),
            next_class_id: 0,
            class_names: HashMap::new(),
        }
    }

    /// Build lookup table from lexer rules
    pub fn build_from_rules(&mut self, rules: &[&Rule]) {
        for rule in rules {
            if !rule.is_fragment {
                self.analyze_rule(rule);
            }
        }
    }

    fn analyze_rule(&mut self, rule: &Rule) {
        // Analyze each alternative in the rule
        for alt in &rule.alternatives {
            for element in &alt.elements {
                self.analyze_element(element, &rule.name);
            }
        }
    }

    fn analyze_element(&mut self, element: &crate::ast::Element, rule_name: &str) {
        use crate::ast::Element;
        
        match element {
            Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
                // Add each character to the lookup table
                for ch in value.chars() {
                    if !self.char_to_class.contains_key(&ch) {
                        let class_id = self.next_class_id;
                        self.char_to_class.insert(ch, class_id);
                        self.class_names.insert(class_id, format!("{}_{}", rule_name, ch.escape_default()));
                        self.next_class_id += 1;
                    }
                }
            }
            Element::CharRange { start, end } => {
                // Add range to lookup table
                for ch in *start..=*end {
                    if !self.char_to_class.contains_key(&ch) {
                        let class_id = self.next_class_id;
                        self.char_to_class.insert(ch, class_id);
                        self.class_names.insert(class_id, format!("{}_range", rule_name));
                        self.next_class_id += 1;
                    }
                }
            }
            Element::Group { alternatives } => {
                for alt in alternatives {
                    for elem in &alt.elements {
                        self.analyze_element(elem, rule_name);
                    }
                }
            }
            Element::Optional { element, .. } |
            Element::ZeroOrMore { element, .. } |
            Element::OneOrMore { element, .. } |
            Element::Not { element } => {
                self.analyze_element(element, rule_name);
            }
            _ => {}
        }
    }

    /// Generate Rust code for const lookup table
    pub fn generate_lookup_table(&self) -> String {
        let mut code = String::new();

        code.push_str("    /// Character class lookup table.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Maps each character to its character class ID for efficient matching.\n");
        code.push_str("    /// This table is generated at compile time and stored as a const array.\n");
        code.push_str("    const CHAR_CLASS_TABLE: [u8; 256] = [\n");

        // Generate lookup table for ASCII characters (0-255)
        for i in 0..256 {
            let ch = i as u8 as char;
            let class_id = self.char_to_class.get(&ch).copied().unwrap_or(255);
            
            if i % 16 == 0 {
                code.push_str("        ");
            }
            
            code.push_str(&format!("{:3}", class_id));
            
            if i < 255 {
                code.push_str(", ");
            }
            
            if i % 16 == 15 {
                code.push_str(&format!(" // 0x{:02X}-0x{:02X}\n", i - 15, i));
            }
        }

        code.push_str("\n    ];\n\n");

        // Generate helper function to get character class
        code.push_str("    /// Get the character class ID for a given character.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Returns 255 for characters not in any class.\n");
        code.push_str("    #[inline]\n");
        code.push_str("    fn get_char_class(ch: char) -> u8 {\n");
        code.push_str("        if (ch as u32) < 256 {\n");
        code.push_str("            Self::CHAR_CLASS_TABLE[ch as usize]\n");
        code.push_str("        } else {\n");
        code.push_str("            255 // Unknown class for non-ASCII\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        code
    }

    /// Generate token type lookup table
    pub fn generate_token_type_table(&self, token_names: &[String]) -> String {
        let mut code = String::new();

        code.push_str("    /// Token type lookup table.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Maps token name strings to TokenKind for efficient conversion.\n");
        code.push_str("    #[inline]\n");
        code.push_str("    fn token_name_to_kind(name: &str) -> TokenKind {\n");
        code.push_str("        match name {\n");

        for token_name in token_names {
            code.push_str(&format!("            \"{}\" => TokenKind::{},\n", token_name, token_name));
        }

        code.push_str("            _ => TokenKind::Eof,\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        code
    }

    /// Get statistics about the lookup table
    pub fn stats(&self) -> LookupTableStats {
        LookupTableStats {
            total_chars: self.char_to_class.len(),
            total_classes: self.next_class_id as usize,
            table_size: 256, // ASCII table size
        }
    }
}

impl Default for LookupTableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about generated lookup table
#[derive(Debug, Clone)]
pub struct LookupTableStats {
    pub total_chars: usize,
    pub total_classes: usize,
    pub table_size: usize,
}

impl LookupTableStats {
    pub fn memory_bytes(&self) -> usize {
        self.table_size // 1 byte per entry
    }
}

/// Generate optimized character matching using lookup table
pub fn generate_optimized_char_match(_table: &LookupTableBuilder) -> String {
    let mut code = String::new();

    code.push_str("    /// Match character using lookup table.\n");
    code.push_str("    /// \n");
    code.push_str("    /// This is faster than multiple if/else or match statements.\n");
    code.push_str("    #[inline]\n");
    code.push_str("    fn match_char_fast(&self, ch: char, expected_class: u8) -> bool {\n");
    code.push_str("        Self::get_char_class(ch) == expected_class\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Check if character is in a range using lookup table.\n");
    code.push_str("    #[inline]\n");
    code.push_str("    fn is_in_range(&self, ch: char, start_class: u8, end_class: u8) -> bool {\n");
    code.push_str("        let class = Self::get_char_class(ch);\n");
    code.push_str("        class >= start_class && class <= end_class\n");
    code.push_str("    }\n\n");

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_table_builder() {
        let builder = LookupTableBuilder::new();
        assert_eq!(builder.next_class_id, 0);
        assert_eq!(builder.char_to_class.len(), 0);
    }

    #[test]
    fn test_lookup_table_stats() {
        let builder = LookupTableBuilder::new();
        let stats = builder.stats();
        assert_eq!(stats.total_chars, 0);
        assert_eq!(stats.total_classes, 0);
        assert_eq!(stats.table_size, 256);
        assert_eq!(stats.memory_bytes(), 256);
    }

    #[test]
    fn test_generate_lookup_table() {
        let builder = LookupTableBuilder::new();
        let code = builder.generate_lookup_table();
        assert!(code.contains("CHAR_CLASS_TABLE"));
        assert!(code.contains("get_char_class"));
    }

    #[test]
    fn test_generate_token_type_table() {
        let builder = LookupTableBuilder::new();
        let tokens = vec!["NUMBER".to_string(), "IDENTIFIER".to_string()];
        let code = builder.generate_token_type_table(&tokens);
        assert!(code.contains("token_name_to_kind"));
        assert!(code.contains("NUMBER"));
        assert!(code.contains("IDENTIFIER"));
    }
}
