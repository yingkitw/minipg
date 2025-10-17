//! Template utilities for code generation.

use std::collections::HashMap;

/// Simple template engine for code generation.
pub struct Template {
    template: String,
}

impl Template {
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    pub fn render(&self, vars: &HashMap<String, String>) -> String {
        let mut result = self.template.clone();
        
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_render() {
        let template = Template::new("Hello {{name}}!");
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "World".to_string());
        
        assert_eq!(template.render(&vars), "Hello World!");
    }
}
