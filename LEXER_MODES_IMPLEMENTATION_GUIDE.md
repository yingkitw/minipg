# Lexer Modes Implementation Guide

**Status**: Foundation Ready, Implementation Pending  
**Estimated Time**: 6-8 hours  
**Priority**: High  
**Complexity**: Medium-High

---

## Overview

Lexer modes allow different tokenization rules in different contexts. This is essential for grammars that need context-sensitive lexing (e.g., string literals, XML tags, embedded languages).

### Example Use Case

```antlr4
lexer grammar StringLexer;

STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["\\\r\n];
STRING_ESC: '\\' [btnfr"'\\];
STRING_END: '"' -> popMode;
```

---

## Current Status

### ✅ What's Already Implemented

1. **LexerCommand Enum** (`src/ast/element.rs`):
   ```rust
   pub enum LexerCommand {
       Skip,
       Channel(String),
       Mode(String),      // -> mode(NAME)
       PushMode(String),  // -> pushMode(NAME)
       PopMode,           // -> popMode
       Type(String),
       More,
   }
   ```

2. **Command Parsing** (`src/parser/parser.rs`):
   - Parser extracts lexer commands from `->` syntax
   - Commands stored in `Alternative.lexer_command`
   - All 7 command types recognized

3. **AST Storage**:
   - Commands attached to alternatives
   - Ready for code generation

### ❌ What's Missing

1. **Mode Declarations**: No `mode NAME;` parsing
2. **Mode Tracking**: Rules don't know which mode they belong to
3. **Code Generation**: No mode-aware lexer generation
4. **Mode Stack**: Generated lexers don't have mode state

---

## Implementation Plan

### Phase 1: AST Extensions (1-2 hours)

#### 1.1 Add Mode Support to Grammar

**File**: `src/ast/grammar.rs`

```rust
pub struct Grammar {
    pub name: String,
    pub grammar_type: GrammarType,
    pub options: HashMap<String, String>,
    pub rules: Vec<Rule>,
    pub imports: Vec<String>,
    pub named_actions: HashMap<String, String>,
    // NEW: Add mode tracking
    pub modes: Vec<String>,  // List of declared modes
    pub mode_rules: HashMap<String, Vec<usize>>,  // mode_name -> rule indices
}

impl Grammar {
    pub fn add_mode(&mut self, mode_name: String) {
        if !self.modes.contains(&mode_name) {
            self.modes.push(mode_name.clone());
            self.mode_rules.insert(mode_name, Vec::new());
        }
    }
    
    pub fn add_rule_to_mode(&mut self, mode_name: &str, rule_index: usize) {
        if let Some(rules) = self.mode_rules.get_mut(mode_name) {
            rules.push(rule_index);
        }
    }
    
    pub fn rules_in_mode(&self, mode_name: &str) -> Vec<&Rule> {
        self.mode_rules
            .get(mode_name)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.rules.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
}
```

#### 1.2 Add Mode to Rule

**File**: `src/ast/rule.rs`

```rust
pub struct Rule {
    pub name: String,
    pub rule_type: RuleType,
    pub alternatives: Vec<Alternative>,
    pub is_fragment: bool,
    pub arguments: Vec<RuleArgument>,
    pub returns: Vec<RuleReturn>,
    pub locals: Vec<RuleLocal>,
    // NEW: Add mode tracking
    pub mode: Option<String>,  // Which mode this rule belongs to
}
```

### Phase 2: Parser Extensions (2-3 hours)

#### 2.1 Parse Mode Declarations

**File**: `src/parser/token.rs`

Add `Mode` keyword:
```rust
pub enum TokenKind {
    // ... existing tokens
    Mode,  // NEW: 'mode' keyword
    // ...
}
```

**File**: `src/parser/lexer.rs`

Add keyword recognition:
```rust
fn scan_identifier(&mut self) -> Token {
    // ... existing code
    let kind = match text.as_str() {
        "grammar" => TokenKind::Grammar,
        "lexer" => TokenKind::Lexer,
        "parser" => TokenKind::Parser,
        "fragment" => TokenKind::Fragment,
        "import" => TokenKind::Import,
        "options" => TokenKind::Options,
        "mode" => TokenKind::Mode,  // NEW
        _ => TokenKind::Identifier,
    };
    // ...
}
```

#### 2.2 Parse Mode Blocks

**File**: `src/parser/parser.rs`

```rust
fn parse(&mut self) -> Result<Grammar> {
    // ... existing grammar header parsing
    
    let mut current_mode: Option<String> = None;
    
    while self.current_token.kind != TokenKind::Eof {
        if self.current_token.kind == TokenKind::Mode {
            current_mode = Some(self.parse_mode_declaration(&mut grammar)?);
        } else if self.current_token.kind == TokenKind::Identifier {
            let mut rule = self.parse_rule()?;
            rule.mode = current_mode.clone();
            grammar.add_rule(rule);
        }
        // ... other cases
    }
    
    Ok(grammar)
}

fn parse_mode_declaration(&mut self, grammar: &mut Grammar) -> Result<String> {
    self.expect(TokenKind::Mode)?;
    let mode_name = self.expect_identifier()?;
    self.expect(TokenKind::Semicolon)?;
    grammar.add_mode(mode_name.clone());
    Ok(mode_name)
}
```

### Phase 3: Code Generation (3-4 hours)

#### 3.1 Rust Generator

**File**: `src/codegen/rust.rs`

```rust
fn generate_lexer(&self, grammar: &Grammar) -> String {
    let mut code = String::new();
    
    // Add mode enum
    if !grammar.modes.is_empty() {
        code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        code.push_str("pub enum LexerMode {\n");
        code.push_str("    Default,\n");
        for mode in &grammar.modes {
            code.push_str(&format!("    {},\n", mode));
        }
        code.push_str("}\n\n");
    }
    
    // Add mode stack to lexer struct
    code.push_str("pub struct Lexer {\n");
    code.push_str("    input: Vec<char>,\n");
    code.push_str("    position: usize,\n");
    if !grammar.modes.is_empty() {
        code.push_str("    mode_stack: Vec<LexerMode>,\n");
    }
    code.push_str("}\n\n");
    
    // Add mode methods
    if !grammar.modes.is_empty() {
        code.push_str("impl Lexer {\n");
        code.push_str("    fn push_mode(&mut self, mode: LexerMode) {\n");
        code.push_str("        self.mode_stack.push(mode);\n");
        code.push_str("    }\n\n");
        code.push_str("    fn pop_mode(&mut self) {\n");
        code.push_str("        self.mode_stack.pop();\n");
        code.push_str("    }\n\n");
        code.push_str("    fn current_mode(&self) -> LexerMode {\n");
        code.push_str("        self.mode_stack.last().copied()\n");
        code.push_str("            .unwrap_or(LexerMode::Default)\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
    }
    
    // Generate mode-aware tokenization
    code.push_str("fn next_token(&mut self) -> Option<Token> {\n");
    if !grammar.modes.is_empty() {
        code.push_str("    match self.current_mode() {\n");
        code.push_str("        LexerMode::Default => self.tokenize_default(),\n");
        for mode in &grammar.modes {
            code.push_str(&format!("        LexerMode::{} => self.tokenize_{}(),\n", 
                mode, mode.to_lowercase()));
        }
        code.push_str("    }\n");
    } else {
        code.push_str("    self.tokenize_default()\n");
    }
    code.push_str("}\n\n");
    
    // Generate tokenize methods for each mode
    code.push_str(&self.generate_tokenize_for_mode(grammar, "Default", 
        &grammar.rules.iter().filter(|r| r.mode.is_none()).collect::<Vec<_>>()));
    
    for mode in &grammar.modes {
        let mode_rules = grammar.rules_in_mode(mode);
        code.push_str(&self.generate_tokenize_for_mode(grammar, mode, &mode_rules));
    }
    
    code
}

fn generate_tokenize_for_mode(&self, grammar: &Grammar, mode_name: &str, rules: &[&Rule]) -> String {
    let mut code = String::new();
    let method_name = format!("tokenize_{}", mode_name.to_lowercase());
    
    code.push_str(&format!("fn {}(&mut self) -> Option<Token> {{\n", method_name));
    code.push_str("    let start_pos = self.position;\n\n");
    
    // Generate matching logic for rules in this mode
    for rule in rules {
        // ... generate pattern matching for each rule
        // ... handle lexer commands (pushMode, popMode, mode)
    }
    
    code.push_str("    None\n");
    code.push_str("}\n\n");
    
    code
}
```

#### 3.2 Handle Lexer Commands

```rust
fn generate_lexer_command(&self, command: &LexerCommand) -> String {
    match command {
        LexerCommand::Skip => "continue;".to_string(),
        LexerCommand::Channel(name) => {
            format!("token.channel = Channel::{};", name)
        }
        LexerCommand::Mode(name) => {
            format!("self.mode_stack = vec![LexerMode::{}];", name)
        }
        LexerCommand::PushMode(name) => {
            format!("self.push_mode(LexerMode::{});", name)
        }
        LexerCommand::PopMode => {
            "self.pop_mode();".to_string()
        }
        LexerCommand::Type(name) => {
            format!("token.kind = TokenKind::{};", name)
        }
        LexerCommand::More => {
            "// Continue current token".to_string()
        }
    }
}
```

### Phase 4: Testing (1 hour)

#### 4.1 Create Test Grammar

```antlr4
lexer grammar StringLexer;

STRING_START: '"' -> pushMode(STRING_MODE);
WHITESPACE: [ \t\r\n]+ -> skip;

mode STRING_MODE;
STRING_CHAR: ~["\\\r\n];
STRING_ESC: '\\' [btnfr"'\\];
STRING_END: '"' -> popMode;
```

#### 4.2 Test Cases

**File**: `tests/test_lexer_modes.rs`

```rust
#[test]
fn test_mode_declaration_parsing() {
    let grammar = r#"
lexer grammar Test;

STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["];
STRING_END: '"' -> popMode;
"#;
    
    let lexer = Lexer::new(grammar, "test.g4");
    let mut parser = Parser::new(lexer);
    let grammar = parser.parse().expect("Failed to parse");
    
    assert_eq!(grammar.modes.len(), 1);
    assert_eq!(grammar.modes[0], "STRING_MODE");
}

#[test]
fn test_mode_aware_code_generation() {
    // Test that generated lexer has mode stack
    // Test that mode switching works
}
```

---

## Implementation Checklist

### AST Changes
- [ ] Add `modes: Vec<String>` to Grammar
- [ ] Add `mode_rules: HashMap<String, Vec<usize>>` to Grammar
- [ ] Add `mode: Option<String>` to Rule
- [ ] Add helper methods: `add_mode()`, `add_rule_to_mode()`, `rules_in_mode()`

### Parser Changes
- [ ] Add `Mode` token kind
- [ ] Add `mode` keyword recognition in lexer
- [ ] Implement `parse_mode_declaration()`
- [ ] Track current mode while parsing rules
- [ ] Assign mode to rules

### Code Generation Changes
- [ ] Generate `LexerMode` enum
- [ ] Add `mode_stack` to lexer struct
- [ ] Implement `push_mode()`, `pop_mode()`, `current_mode()`
- [ ] Generate separate tokenize methods for each mode
- [ ] Handle mode commands in tokenization
- [ ] Update all 5 generators (Rust, Python, TS, JS, Go)

### Testing
- [ ] Test mode declaration parsing
- [ ] Test rule-to-mode assignment
- [ ] Test mode-aware code generation
- [ ] Test pushMode/popMode/mode commands
- [ ] Test with real-world grammar (e.g., string literals)

---

## Challenges & Solutions

### Challenge 1: DFA Generation
**Problem**: Current DFA is mode-agnostic  
**Solution**: Generate separate DFAs for each mode

### Challenge 2: Mode Stack Management
**Problem**: Need to track mode state during tokenization  
**Solution**: Add mode_stack field to lexer, initialize with Default mode

### Challenge 3: Cross-Language Support
**Problem**: Different languages handle state differently  
**Solution**: 
- Rust: Use Vec<LexerMode>
- Python: Use list
- TypeScript/JavaScript: Use array
- Go: Use slice

### Challenge 4: Performance
**Problem**: Mode checking adds overhead  
**Solution**: Use match/switch statements for fast dispatch

---

## Example Generated Code

### Input Grammar
```antlr4
lexer grammar StringLexer;

TEXT: [a-z]+;
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["];
STRING_END: '"' -> popMode;
```

### Generated Rust Code
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerMode {
    Default,
    STRING_MODE,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            mode_stack: vec![LexerMode::Default],
        }
    }
    
    fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }
    
    fn pop_mode(&mut self) {
        self.mode_stack.pop();
    }
    
    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().copied().unwrap_or(LexerMode::Default)
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        match self.current_mode() {
            LexerMode::Default => self.tokenize_default(),
            LexerMode::STRING_MODE => self.tokenize_string_mode(),
        }
    }
    
    fn tokenize_default(&mut self) -> Option<Token> {
        // Match TEXT and STRING_START rules
        if self.match_text() {
            return Some(Token { kind: TokenKind::TEXT, ... });
        }
        if self.match_string_start() {
            self.push_mode(LexerMode::STRING_MODE);
            return Some(Token { kind: TokenKind::STRING_START, ... });
        }
        None
    }
    
    fn tokenize_string_mode(&mut self) -> Option<Token> {
        // Match STRING_CHAR and STRING_END rules
        if self.match_string_end() {
            self.pop_mode();
            return Some(Token { kind: TokenKind::STRING_END, ... });
        }
        if self.match_string_char() {
            return Some(Token { kind: TokenKind::STRING_CHAR, ... });
        }
        None
    }
}
```

---

## Next Steps

1. **Start with AST changes** - Add mode tracking
2. **Update parser** - Parse mode declarations
3. **Test parsing** - Verify modes are recognized
4. **Implement Rust codegen** - Get one language working
5. **Add tests** - Verify mode switching works
6. **Extend to other languages** - Python, TS, JS, Go
7. **Optimize** - Profile and improve performance

---

## Estimated Timeline

- **Day 1** (3-4 hours): AST + Parser changes
- **Day 2** (3-4 hours): Rust code generation + testing
- **Total**: 6-8 hours for basic implementation

---

## Status: Ready to Implement

All the design work is done. The implementation is straightforward but time-consuming. This guide provides everything needed to complete the feature.

**When you're ready to implement, start with Phase 1 (AST Extensions).**
