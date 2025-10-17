# API Documentation

## Core Traits

### GrammarParser

Parse grammar files into AST.

```rust
pub trait GrammarParser {
    type Output;
    
    fn parse_file(&self, path: &Path) -> Result<Self::Output>;
    fn parse_string(&self, source: &str, filename: &str) -> Result<Self::Output>;
}
```

**Example:**
```rust
use minipg_parser::GrammarParser;
use minipg_core::GrammarParser as GrammarParserTrait;

let parser = GrammarParser::new();
let grammar = parser.parse_file("grammar.g4")?;
```

### SemanticAnalyzer

Perform semantic analysis on grammars.

```rust
pub trait SemanticAnalyzer {
    type Input;
    type Output;
    
    fn analyze(&self, input: &Self::Input) -> Result<Self::Output>;
    fn diagnostics(&self) -> &[Diagnostic];
}
```

**Example:**
```rust
use minipg_analysis::SemanticAnalyzer;
use minipg_core::SemanticAnalyzer as SemanticAnalyzerTrait;

let analyzer = SemanticAnalyzer::new();
let result = analyzer.analyze(&grammar)?;

if result.has_errors() {
    for diagnostic in &result.diagnostics {
        eprintln!("{}", diagnostic);
    }
}
```

### CodeGenerator

Generate code for target languages.

```rust
pub trait CodeGenerator {
    type Input;
    type Config;
    
    fn generate(&self, input: &Self::Input, config: &Self::Config) -> Result<String>;
    fn target_language(&self) -> &str;
}
```

**Example:**
```rust
use minipg_codegen::RustCodeGenerator;
use minipg_core::{CodeGenerator as CodeGeneratorTrait, types::CodeGenConfig};

let generator = RustCodeGenerator::new();
let config = CodeGenConfig {
    target_language: "rust".to_string(),
    output_directory: "./output".to_string(),
    generate_visitor: true,
    generate_listener: true,
    ..Default::default()
};

let code = generator.generate(&grammar, &config)?;
```

## AST Types

### Grammar

Root node of the AST.

```rust
pub struct Grammar {
    pub name: String,
    pub grammar_type: GrammarType,
    pub options: HashMap<String, String>,
    pub rules: Vec<Rule>,
    pub imports: Vec<String>,
}
```

**Methods:**
- `new(name: String, grammar_type: GrammarType) -> Self`
- `add_rule(&mut self, rule: Rule)`
- `add_option(&mut self, key: String, value: String)`
- `get_rule(&self, name: &str) -> Option<&Rule>`
- `lexer_rules(&self) -> impl Iterator<Item = &Rule>`
- `parser_rules(&self) -> impl Iterator<Item = &Rule>`

### Rule

Grammar rule definition.

```rust
pub struct Rule {
    pub name: String,
    pub rule_type: RuleType,
    pub alternatives: Vec<Alternative>,
    pub is_fragment: bool,
}
```

**Methods:**
- `parser_rule(name: String) -> Self`
- `lexer_rule(name: String) -> Self`
- `add_alternative(&mut self, alternative: Alternative)`
- `is_lexer_rule(&self) -> bool`
- `is_parser_rule(&self) -> bool`

### Element

Grammar elements (terminals, non-terminals, operators).

```rust
pub enum Element {
    RuleRef { name: String, label: Option<String> },
    Terminal { value: String, label: Option<String> },
    StringLiteral { value: String, label: Option<String> },
    CharRange { start: char, end: char },
    Optional { element: Box<Element> },
    ZeroOrMore { element: Box<Element> },
    OneOrMore { element: Box<Element> },
    Group { alternatives: Vec<Alternative> },
    Not { element: Box<Element> },
    Wildcard,
    Eof,
}
```

**Methods:**
- `rule_ref(name: String) -> Self`
- `terminal(value: String) -> Self`
- `string_literal(value: String) -> Self`
- `optional(element: Element) -> Self`
- `zero_or_more(element: Element) -> Self`
- `one_or_more(element: Element) -> Self`
- `with_label(self, label: String) -> Self`

## Visitor Pattern

### AstVisitor

Immutable visitor for AST traversal.

```rust
pub trait AstVisitor {
    fn visit_grammar(&mut self, grammar: &Grammar);
    fn visit_rule(&mut self, rule: &Rule);
    fn visit_alternative(&mut self, alternative: &Alternative);
    fn visit_element(&mut self, element: &Element);
}
```

**Example:**
```rust
struct RuleCounter {
    count: usize,
}

impl AstVisitor for RuleCounter {
    fn visit_rule(&mut self, rule: &Rule) {
        self.count += 1;
        self.walk_rule(rule);
    }
}

let mut counter = RuleCounter { count: 0 };
counter.visit_grammar(&grammar);
println!("Total rules: {}", counter.count);
```

## Error Handling

### Error Types

```rust
pub enum Error {
    Io(std::io::Error),
    Parse { location: String, message: String },
    Semantic(String),
    CodeGen(String),
    InvalidGrammar(String),
    FileNotFound(String),
    InvalidArgument(String),
    Internal(String),
}
```

### Diagnostic

Rich diagnostic messages with location information.

```rust
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub location: Option<Location>,
    pub message: String,
    pub code: Option<String>,
}
```

**Methods:**
- `error(message: impl Into<String>) -> Self`
- `warning(message: impl Into<String>) -> Self`
- `with_location(self, location: Location) -> Self`
- `with_code(self, code: impl Into<String>) -> Self`

## Configuration

### CodeGenConfig

Configuration for code generation.

```rust
pub struct CodeGenConfig {
    pub target_language: String,
    pub output_directory: String,
    pub package_name: Option<String>,
    pub generate_listener: bool,
    pub generate_visitor: bool,
}
```

## Complete Example

```rust
use minipg_parser::GrammarParser;
use minipg_analysis::SemanticAnalyzer;
use minipg_codegen::RustCodeGenerator;
use minipg_core::{
    GrammarParser as GrammarParserTrait,
    SemanticAnalyzer as SemanticAnalyzerTrait,
    CodeGenerator as CodeGeneratorTrait,
    types::CodeGenConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse grammar
    let parser = GrammarParser::new();
    let grammar = parser.parse_file("grammar.g4")?;
    
    // Analyze
    let analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&grammar)?;
    
    // Check for errors
    if analysis.has_errors() {
        for diagnostic in &analysis.diagnostics {
            eprintln!("{}", diagnostic);
        }
        return Err("Grammar has errors".into());
    }
    
    // Generate code
    let generator = RustCodeGenerator::new();
    let config = CodeGenConfig {
        target_language: "rust".to_string(),
        output_directory: "./output".to_string(),
        generate_visitor: true,
        generate_listener: true,
        ..Default::default()
    };
    
    let code = generator.generate(&grammar, &config)?;
    
    // Write to file
    std::fs::write("output/parser.rs", code)?;
    
    println!("Parser generated successfully!");
    Ok(())
}
```

## See Also

- [User Guide](USER_GUIDE.md)
- [Grammar Syntax Reference](GRAMMAR_SYNTAX.md)
- [Architecture Documentation](../ARCHITECTURE.md)
