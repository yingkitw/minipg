//! DFA (Deterministic Finite Automaton) generation for lexer optimization.
//!
//! This module generates optimized DFA code at compile time, which is then
//! inlined into the generated lexer as match statements.

use crate::ast::{Element, Rule};
use std::collections::{HashMap, HashSet};

/// DFA state representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DfaState {
    pub id: usize,
    pub accepting: Option<String>, // Token name if accepting state
    pub transitions: HashMap<CharClass, usize>, // char class -> next state
}

/// Character class for efficient matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharClass {
    Single(char),
    Range(char, char),
    Any,
}

/// DFA builder for lexer rules
pub struct DfaBuilder {
    states: Vec<DfaState>,
    next_state_id: usize,
}

impl DfaBuilder {
    pub fn new() -> Self {
        Self {
            states: vec![DfaState {
                id: 0,
                accepting: None,
                transitions: HashMap::new(),
            }],
            next_state_id: 1,
        }
    }

    /// Build DFA from lexer rules
    pub fn build_from_rules(&mut self, rules: &[&Rule]) -> Vec<DfaState> {
        for rule in rules {
            if !rule.is_fragment {
                self.add_rule(rule);
            }
        }
        self.states.clone()
    }

    fn add_rule(&mut self, rule: &Rule) {
        // Start from initial state
        let mut current_state = 0;

        // Process each element in the rule
        for alt in &rule.alternatives {
            for element in &alt.elements {
                current_state = self.add_element(current_state, element);
            }
        }

        // Mark final state as accepting
        if let Some(state) = self.states.get_mut(current_state) {
            state.accepting = Some(rule.name.clone());
        }
    }

    fn add_element(&mut self, from_state: usize, element: &Element) -> usize {
        match element {
            Element::Terminal { value, .. } | Element::StringLiteral { value, .. } => {
                let mut current = from_state;
                for ch in value.chars() {
                    current = self.add_transition(current, CharClass::Single(ch));
                }
                current
            }
            Element::CharRange { start, end } => {
                self.add_transition(from_state, CharClass::Range(*start, *end))
            }
            Element::Wildcard => {
                self.add_transition(from_state, CharClass::Any)
            }
            _ => from_state, // Handle other elements as needed
        }
    }

    fn add_transition(&mut self, from_state: usize, char_class: CharClass) -> usize {
        // Check if transition already exists
        if let Some(state) = self.states.get(from_state) {
            if let Some(&next_state) = state.transitions.get(&char_class) {
                return next_state;
            }
        }

        // Create new state
        let new_state_id = self.next_state_id;
        self.next_state_id += 1;

        let new_state = DfaState {
            id: new_state_id,
            accepting: None,
            transitions: HashMap::new(),
        };
        self.states.push(new_state);

        // Add transition
        if let Some(state) = self.states.get_mut(from_state) {
            state.transitions.insert(char_class, new_state_id);
        }

        new_state_id
    }

}

impl Default for DfaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate Rust code for DFA as match statements
pub fn generate_dfa_match(states: &[DfaState]) -> String {
    let mut code = String::new();

    code.push_str("    fn next_token_dfa(&mut self) -> Option<Token> {\n");
    code.push_str("        let mut state = 0;\n");
    code.push_str("        let mut token_start = self.position;\n");
    code.push_str("        let mut last_accepting: Option<(usize, &str)> = None;\n\n");

    code.push_str("        loop {\n");
    code.push_str("            // Check if current state is accepting\n");
    code.push_str("            match state {\n");
    
    for state in states {
        if let Some(token_name) = &state.accepting {
            code.push_str(&format!(
                "                {} => last_accepting = Some((self.position, \"{}\")),\n",
                state.id, token_name
            ));
        }
    }
    
    code.push_str("                _ => {}\n");
    code.push_str("            }\n\n");

    code.push_str("            // Get next character\n");
    code.push_str("            let ch = match self.input.get(self.position) {\n");
    code.push_str("                Some(&c) => c,\n");
    code.push_str("                None => break,\n");
    code.push_str("            };\n\n");

    code.push_str("            // Transition to next state\n");
    code.push_str("            state = match (state, ch) {\n");

    for state in states {
        for (char_class, next_state) in &state.transitions {
            let pattern = match char_class {
                CharClass::Single(ch) => format!("'{}'", ch.escape_default()),
                CharClass::Range(start, end) => {
                    format!("'{}' ..= '{}'", start.escape_default(), end.escape_default())
                }
                CharClass::Any => "_".to_string(),
            };
            code.push_str(&format!("                ({}, {}) => {},\n", state.id, pattern, next_state));
        }
    }

    code.push_str("                _ => break, // No valid transition\n");
    code.push_str("            };\n\n");
    code.push_str("            self.position += 1;\n");
    code.push_str("        }\n\n");

    code.push_str("        // Return token if we found an accepting state\n");
    code.push_str("        if let Some((end_pos, token_name)) = last_accepting {\n");
    code.push_str("            let text: String = self.input[token_start..end_pos].iter().collect();\n");
    code.push_str("            Some(Token {\n");
    code.push_str("                position: token_start,\n");
    code.push_str("                kind: match token_name {\n");
    
    // Generate token kind matching
    let token_names: HashSet<String> = states
        .iter()
        .filter_map(|s| s.accepting.clone())
        .collect();
    
    for token_name in token_names {
        code.push_str(&format!("                    \"{}\" => TokenKind::{},\n", token_name, token_name));
    }
    
    code.push_str("                    _ => TokenKind::Eof,\n");
    code.push_str("                },\n");
    code.push_str("                text,\n");
    code.push_str("            })\n");
    code.push_str("        } else {\n");
    code.push_str("            None\n");
    code.push_str("        }\n");
    code.push_str("    }\n");

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfa_builder_simple() {
        let mut builder = DfaBuilder::new();
        assert_eq!(builder.states.len(), 1);
        assert_eq!(builder.next_state_id, 1);
    }

}
