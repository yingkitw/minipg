# Lexer Modes & Channels + Action Code Generation Implementation Plan

## Overview

This document outlines the implementation plan for:
1. **Lexer Modes & Channels Code Generation** - Generate code for lexer mode switching and token channels
2. **Action Code Generation** - Generate code for embedded actions and semantic predicates

## Current Status

### Already Implemented ✅
- **Lexer Commands AST**: LexerCommand enum with Skip, Channel, Mode, PushMode, PopMode, More, Type
- **Named Actions Parsing**: @header, @members, @init actions parsed and stored
- **Named Actions Code Generation**: Partially implemented for Rust (members insertion)

### Not Yet Implemented ⚠️
- **Lexer Mode Code Generation**: Mode switching logic in generated lexers
- **Channel Code Generation**: Token channel routing in generated lexers
- **Action Code Generation**: Embedded action blocks {code} in rules
- **Semantic Predicate Code Generation**: Semantic predicates {code}? in rules
- **Language-Specific Action Translation**: Converting actions to target language syntax

## Implementation Strategy

### Phase 1: Lexer Modes & Channels (Priority 1)

#### 1.1 Extend Grammar AST
```rust
// Add to Grammar struct
pub lexer_modes: HashMap<String, Vec<Rule>>,  // mode_name -> rules in that mode
pub channels: HashSet<String>,                 // channel names used
```

#### 1.2 Parser Enhancement
- Parse `mode NAME;` declarations
- Parse `-> mode(NAME)` commands
- Parse `-> pushMode(NAME)` and `-> popMode` commands
- Parse `-> channel(NAME)` commands
- Store mode information in Grammar

#### 1.3 Rust Code Generation
```rust
// Generate mode stack in lexer
pub struct Lexer {
    input: String,
    position: usize,
    mode_stack: Vec<String>,  // Stack of active modes
    channels: HashMap<String, Vec<Token>>,  // Token channels
}

// Generate mode switching logic
fn switch_mode(&mut self, mode: &str) { ... }
fn push_mode(&mut self, mode: &str) { ... }
fn pop_mode(&mut self) { ... }
```

#### 1.4 Other Language Code Generation
- Python: Similar mode stack with list
- JavaScript: Mode stack with array
- TypeScript: Typed mode stack
- Go: Mode stack with slice

### Phase 2: Action Code Generation (Priority 2)

#### 2.1 Extend Element AST
```rust
pub enum Element {
    // ... existing variants ...
    Action {
        code: String,
        language: Option<String>,  // rust, python, js, etc.
    },
    Predicate {
        code: String,
        language: Option<String>,
    },
}
```

#### 2.2 Parser Enhancement
- Parse `{code}` as Action element
- Parse `{code}?` as Predicate element
- Parse language-specific actions: `{rust: code}`
- Store action code in AST

#### 2.3 Code Generation Strategy

**For Semantic Predicates:**
```rust
// In parser rule
if !self.predicate_code() {
    return Err(ParseError::new(...));
}
```

**For Embedded Actions:**
```rust
// In parser rule
{
    // Embedded action code
}
```

#### 2.4 Language-Specific Translation

Create action translators for each language:
- **Rust**: Direct insertion (already valid Rust)
- **Python**: Convert Rust syntax to Python
- **JavaScript**: Convert Rust syntax to JavaScript
- **TypeScript**: Convert Rust syntax to TypeScript
- **Go**: Convert Rust syntax to Go

### Phase 3: Integration & Testing

#### 3.1 Test Cases
- Create test grammars with modes and channels
- Create test grammars with actions and predicates
- Test mode switching in generated code
- Test channel routing in generated code
- Test action execution in generated code

#### 3.2 Documentation
- Update GRAMMAR_SYNTAX.md with mode/channel syntax
- Update per-language guides with action examples
- Create troubleshooting guide for common issues

## Implementation Priority

### High Priority (Next Session)
1. Extend Grammar AST for modes and channels
2. Implement parser for mode/channel declarations
3. Implement Rust lexer mode code generation
4. Create test cases for modes and channels

### Medium Priority (Following Session)
1. Implement action/predicate AST extensions
2. Implement action parsing
3. Implement Rust action code generation
4. Create test cases for actions

### Lower Priority (Future)
1. Implement Python/JS/TS/Go mode code generation
2. Implement action translation for all languages
3. Optimize generated mode switching code
4. Add performance benchmarks

## Code Structure

### Files to Create/Modify

**New Files:**
- `src/codegen/modes.rs` - Lexer mode code generation
- `src/codegen/actions.rs` - Action code generation
- `src/codegen/action_translator.rs` - Language-specific action translation

**Modified Files:**
- `src/ast/grammar.rs` - Add modes and channels fields
- `src/ast/element.rs` - Add Action and Predicate variants
- `src/parser/parser.rs` - Parse mode/channel/action syntax
- `src/codegen/rust.rs` - Generate mode/action code
- `src/codegen/python.rs` - Generate mode/action code
- `src/codegen/javascript.rs` - Generate mode/action code
- `src/codegen/typescript.rs` - Generate mode/action code
- `src/codegen/go.rs` - Generate mode/action code

## Example: Lexer Modes

### Input Grammar
```antlr
lexer grammar MyLexer;

mode DEFAULT_MODE;
ID: [a-zA-Z]+;
WS: [ \t\n]+ -> skip;

mode STRING_MODE;
STRING_CONTENT: ~['"\\]+;
STRING_END: '"' -> mode(DEFAULT_MODE);
```

### Generated Rust Code
```rust
pub struct Lexer {
    input: String,
    position: usize,
    mode_stack: Vec<String>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            mode_stack: vec!["DEFAULT_MODE".to_string()],
        }
    }
    
    fn current_mode(&self) -> &str {
        self.mode_stack.last().unwrap_or(&"DEFAULT_MODE".to_string())
    }
    
    fn switch_mode(&mut self, mode: &str) {
        self.mode_stack.pop();
        self.mode_stack.push(mode.to_string());
    }
    
    fn push_mode(&mut self, mode: &str) {
        self.mode_stack.push(mode.to_string());
    }
    
    fn pop_mode(&mut self) {
        if self.mode_stack.len() > 1 {
            self.mode_stack.pop();
        }
    }
}
```

## Example: Action Code Generation

### Input Grammar
```antlr
parser grammar MyParser;

expr: left=term {self.validate_expr(&left)}? (('+' | '-') right=term {self.combine(&left, &right)})*;
```

### Generated Rust Code
```rust
pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
    let left = self.parse_term()?;
    
    // Semantic predicate
    if !self.validate_expr(&left) {
        return Err(ParseError::new("Predicate failed".to_string(), self.position));
    }
    
    let mut rights = Vec::new();
    let mut ops = Vec::new();
    
    while self.current_token().kind == TokenKind::PLUS || 
          self.current_token().kind == TokenKind::MINUS {
        let op = self.current_token().text.clone();
        self.advance();
        
        let right = self.parse_term()?;
        
        // Embedded action
        self.combine(&left, &right);
        
        ops.push(op);
        rights.push(right);
    }
    
    Ok(Expr { left: Box::new(left), ops, rights })
}
```

## Success Criteria

### Phase 1 Complete When:
- ✅ Mode declarations parsed correctly
- ✅ Mode switching code generated for Rust
- ✅ Channel routing code generated for Rust
- ✅ All mode/channel tests passing
- ✅ No regressions in existing tests

### Phase 2 Complete When:
- ✅ Action/predicate syntax parsed correctly
- ✅ Action code generated for Rust
- ✅ Semantic predicates working in generated code
- ✅ All action tests passing
- ✅ No regressions in existing tests

### Phase 3 Complete When:
- ✅ All languages generate mode/channel code
- ✅ All languages generate action code
- ✅ Action translation working for all languages
- ✅ Comprehensive test coverage
- ✅ Documentation complete

## Timeline Estimate

- **Phase 1**: 2-3 hours (AST + parser + Rust codegen)
- **Phase 2**: 2-3 hours (Action AST + parser + Rust codegen)
- **Phase 3**: 3-4 hours (All languages + translation + testing)
- **Total**: 7-10 hours

## Notes

- Start with Rust as reference implementation
- Other languages follow same pattern
- Keep action translation simple initially (direct insertion)
- Add sophisticated translation in future iterations
- Maintain 100% test pass rate throughout

---

**Created**: October 25, 2025  
**Status**: Planning phase  
**Next Step**: Begin Phase 1 implementation
