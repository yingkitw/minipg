//! Grammar composition and import resolution.

use crate::ast::Grammar;
use crate::core::{Error, Result};
use crate::core::traits::GrammarParser as GrammarParserTrait;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Grammar composition resolver for handling imports and inheritance.
pub struct GrammarComposer {
    grammar_cache: HashMap<String, Grammar>,
    pub search_paths: Vec<PathBuf>,
}

impl GrammarComposer {
    /// Create a new grammar composer.
    pub fn new() -> Self {
        Self {
            grammar_cache: HashMap::new(),
            search_paths: vec![PathBuf::from(".")],
        }
    }

    /// Add a search path for grammar files.
    pub fn add_search_path(&mut self, path: impl AsRef<Path>) {
        self.search_paths.push(path.as_ref().to_path_buf());
    }

    /// Resolve all imports in a grammar.
    pub fn resolve_imports(&mut self, grammar: &mut Grammar) -> Result<()> {
        let imports = grammar.imports.clone();
        
        for import_name in imports {
            self.resolve_import(grammar, &import_name)?;
        }
        
        Ok(())
    }

    /// Resolve a single import.
    fn resolve_import(&mut self, grammar: &mut Grammar, import_name: &str) -> Result<()> {
        // Check if already cached
        if self.grammar_cache.contains_key(import_name) {
            let imported = self.grammar_cache.get(import_name).unwrap().clone();
            self.merge_grammar(grammar, &imported)?;
            return Ok(());
        }

        // Try to find and load the grammar file
        let grammar_file = self.find_grammar_file(import_name)?;
        
        // Parse the imported grammar
        let parser = crate::parser::GrammarParser::new();
        let imported_grammar = parser.parse_file(&grammar_file)?;
        
        // Cache it
        self.grammar_cache.insert(import_name.to_string(), imported_grammar.clone());
        
        // Recursively resolve imports in the imported grammar
        let mut imported = imported_grammar;
        self.resolve_imports(&mut imported)?;
        
        // Merge into current grammar
        self.merge_grammar(grammar, &imported)?;
        
        Ok(())
    }

    /// Find a grammar file in search paths.
    fn find_grammar_file(&self, import_name: &str) -> Result<PathBuf> {
        let filename = format!("{}.g4", import_name);
        
        for search_path in &self.search_paths {
            let full_path = search_path.join(&filename);
            if full_path.exists() {
                return Ok(full_path);
            }
        }
        
        Err(Error::parse(
            "import resolution".to_string(),
            format!("grammar file not found: {} (searched in {:?})", filename, self.search_paths),
        ))
    }

    /// Merge an imported grammar into the current grammar.
    pub fn merge_grammar(&self, target: &mut Grammar, source: &Grammar) -> Result<()> {
        // Check for rule conflicts
        let target_rule_names: HashSet<String> = target.rules.iter().map(|r| r.name.clone()).collect();
        let source_rule_names: HashSet<String> = source.rules.iter().map(|r| r.name.clone()).collect();
        
        let conflicts: Vec<_> = target_rule_names.intersection(&source_rule_names).collect();
        if !conflicts.is_empty() {
            return Err(Error::parse(
                "grammar composition".to_string(),
                format!("conflicting rules in imported grammar: {:?}", conflicts),
            ));
        }
        
        // Merge rules
        for rule in &source.rules {
            target.add_rule(rule.clone());
        }
        
        // Merge options (source options override target)
        for (key, value) in &source.options {
            target.add_option(key.clone(), value.clone());
        }
        
        // Merge named actions (source actions override target)
        for (name, code) in &source.named_actions {
            target.add_named_action(name.clone(), code.clone());
        }
        
        // Merge lexer modes
        for (mode_name, rules) in &source.lexer_modes {
            target.add_lexer_mode(mode_name.clone(), rules.clone());
        }
        
        // Merge channels
        for channel in &source.channels {
            target.add_channel(channel.clone());
        }
        
        Ok(())
    }

    /// Validate grammar composition (check for circular imports, etc.)
    pub fn validate(&self, grammar: &Grammar) -> Result<()> {
        self.check_circular_imports(grammar, &mut HashSet::new())
    }

    /// Check for circular imports.
    fn check_circular_imports(&self, grammar: &Grammar, visited: &mut HashSet<String>) -> Result<()> {
        if visited.contains(&grammar.name) {
            return Err(Error::parse(
                "grammar composition".to_string(),
                format!("circular import detected: {}", grammar.name),
            ));
        }
        
        visited.insert(grammar.name.clone());
        
        for import_name in &grammar.imports {
            if let Some(imported) = self.grammar_cache.get(import_name) {
                self.check_circular_imports(imported, visited)?;
            }
        }
        
        visited.remove(&grammar.name);
        Ok(())
    }
}

impl Default for GrammarComposer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_composer_new() {
        let composer = GrammarComposer::new();
        assert_eq!(composer.search_paths.len(), 1);
        assert_eq!(composer.grammar_cache.len(), 0);
    }

    #[test]
    fn test_grammar_composer_add_search_path() {
        let mut composer = GrammarComposer::new();
        composer.add_search_path("/path/to/grammars");
        assert_eq!(composer.search_paths.len(), 2);
    }

    #[test]
    fn test_merge_grammar_rules() {
        let mut target = Grammar::new("Target".to_string(), crate::core::types::GrammarType::Combined);
        let mut source = Grammar::new("Source".to_string(), crate::core::types::GrammarType::Combined);
        
        let rule1 = crate::ast::Rule::new("expr".to_string(), crate::ast::RuleType::Parser);
        let rule2 = crate::ast::Rule::new("term".to_string(), crate::ast::RuleType::Parser);
        
        target.add_rule(rule1);
        source.add_rule(rule2);
        
        let composer = GrammarComposer::new();
        composer.merge_grammar(&mut target, &source).expect("merge failed");
        
        assert_eq!(target.rules.len(), 2);
    }

    #[test]
    fn test_merge_grammar_options() {
        let mut target = Grammar::new("Target".to_string(), crate::core::types::GrammarType::Combined);
        let mut source = Grammar::new("Source".to_string(), crate::core::types::GrammarType::Combined);
        
        target.add_option("key1".to_string(), "value1".to_string());
        source.add_option("key2".to_string(), "value2".to_string());
        
        let composer = GrammarComposer::new();
        composer.merge_grammar(&mut target, &source).expect("merge failed");
        
        assert_eq!(target.options.len(), 2);
        assert_eq!(target.options.get("key1").map(|s| s.as_str()), Some("value1"));
        assert_eq!(target.options.get("key2").map(|s| s.as_str()), Some("value2"));
    }

    #[test]
    fn test_merge_grammar_named_actions() {
        let mut target = Grammar::new("Target".to_string(), crate::core::types::GrammarType::Combined);
        let mut source = Grammar::new("Source".to_string(), crate::core::types::GrammarType::Combined);
        
        target.add_named_action("header".to_string(), "// target header".to_string());
        source.add_named_action("members".to_string(), "// members".to_string());
        
        let composer = GrammarComposer::new();
        composer.merge_grammar(&mut target, &source).expect("merge failed");
        
        assert_eq!(target.named_actions.len(), 2);
    }

    #[test]
    fn test_merge_grammar_channels() {
        let mut target = Grammar::new("Target".to_string(), crate::core::types::GrammarType::Combined);
        let mut source = Grammar::new("Source".to_string(), crate::core::types::GrammarType::Combined);
        
        target.add_channel("COMMENTS".to_string());
        source.add_channel("HIDDEN".to_string());
        
        let composer = GrammarComposer::new();
        composer.merge_grammar(&mut target, &source).expect("merge failed");
        
        assert_eq!(target.channels.len(), 2);
        assert!(target.channels.contains("COMMENTS"));
        assert!(target.channels.contains("HIDDEN"));
    }

    #[test]
    fn test_merge_grammar_conflict() {
        let mut target = Grammar::new("Target".to_string(), crate::core::types::GrammarType::Combined);
        let mut source = Grammar::new("Source".to_string(), crate::core::types::GrammarType::Combined);
        
        let rule1 = crate::ast::Rule::new("expr".to_string(), crate::ast::RuleType::Parser);
        let rule2 = crate::ast::Rule::new("expr".to_string(), crate::ast::RuleType::Parser);
        
        target.add_rule(rule1);
        source.add_rule(rule2);
        
        let composer = GrammarComposer::new();
        let result = composer.merge_grammar(&mut target, &source);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_grammar_composer_default() {
        let composer = GrammarComposer::default();
        assert_eq!(composer.search_paths.len(), 1);
    }
}
