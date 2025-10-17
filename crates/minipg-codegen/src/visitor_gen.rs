//! Visitor pattern code generation.

use minipg_ast::Grammar;

/// Generate visitor trait for grammar.
pub fn generate_visitor(grammar: &Grammar) -> String {
    let mut code = String::new();
    
    code.push_str("/// Visitor trait for traversing the AST.\n");
    code.push_str("pub trait Visitor<T> {\n");
    
    for rule in grammar.parser_rules() {
        let method_name = format!("visit_{}", rule.name.to_lowercase());
        let type_name = to_pascal_case(&rule.name);
        code.push_str(&format!(
            "    fn {}(&mut self, node: &{}) -> T;\n",
            method_name, type_name
        ));
    }
    
    code.push_str("}\n\n");
    code
}

/// Generate listener trait for grammar.
pub fn generate_listener(grammar: &Grammar) -> String {
    let mut code = String::new();
    
    code.push_str("/// Listener trait for AST events.\n");
    code.push_str("pub trait Listener {\n");
    
    for rule in grammar.parser_rules() {
        let enter_method = format!("enter_{}", rule.name.to_lowercase());
        let exit_method = format!("exit_{}", rule.name.to_lowercase());
        let type_name = to_pascal_case(&rule.name);
        
        code.push_str(&format!(
            "    fn {}(&mut self, _node: &{}) {{}}\n",
            enter_method, type_name
        ));
        code.push_str(&format!(
            "    fn {}(&mut self, _node: &{}) {{}}\n",
            exit_method, type_name
        ));
    }
    
    code.push_str("}\n\n");
    code
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use minipg_ast::{Grammar, Rule};
    use minipg_core::types::GrammarType;

    #[test]
    fn test_generate_visitor() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        grammar.add_rule(Rule::parser_rule("expr".to_string()));
        grammar.add_rule(Rule::parser_rule("term".to_string()));
        
        let code = generate_visitor(&grammar);
        
        assert!(code.contains("trait Visitor"));
        assert!(code.contains("visit_expr"));
        assert!(code.contains("visit_term"));
    }

    #[test]
    fn test_generate_listener() {
        let mut grammar = Grammar::new("Test".to_string(), GrammarType::Parser);
        grammar.add_rule(Rule::parser_rule("expr".to_string()));
        
        let code = generate_listener(&grammar);
        
        assert!(code.contains("trait Listener"));
        assert!(code.contains("enter_expr"));
        assert!(code.contains("exit_expr"));
    }
}
