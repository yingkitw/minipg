# Runtime Library vs Standalone Code Generation

## The Question

Should minipg:
1. Generate code that depends on a runtime library (`minipg-runtime`)?
2. Generate fully standalone `.rs` files with no dependencies?

## Comparison

### Option 1: Runtime Library (antlr4rust approach)

**Pros:**
- ✅ **Smaller generated code** - Only grammar-specific logic
- ✅ **Shared optimizations** - DFA, ATN, error recovery in one place
- ✅ **Easier updates** - Fix bugs in runtime, all parsers benefit
- ✅ **Consistent behavior** - All parsers use same algorithms
- ✅ **Less code generation complexity** - Delegate to runtime

**Cons:**
- ❌ **External dependency** - Users must include `minipg-runtime`
- ❌ **Version coupling** - Generated code tied to runtime version
- ❌ **Harder to customize** - Runtime is opaque
- ❌ **Binary size** - Runtime included even if features unused
- ❌ **Compilation time** - Extra dependency to compile

### Option 2: Standalone Generated Code (your idea)

**Pros:**
- ✅ **Zero dependencies** - Self-contained, easy to vendor
- ✅ **Full transparency** - User sees all code
- ✅ **Easy customization** - User can modify generated code
- ✅ **No version conflicts** - Generated code is frozen
- ✅ **Minimal binary size** - Only what's needed
- ✅ **Fast compilation** - No external dependencies

**Cons:**
- ❌ **Large generated files** - Must include all algorithms
- ❌ **Code duplication** - Each parser has same boilerplate
- ❌ **Harder to optimize** - Must generate optimal code
- ❌ **Bug fixes harder** - Must regenerate all parsers
- ❌ **Complex code generation** - Must generate everything

## Real-World Examples

### Standalone Approach
- **Pest** (Rust) - Generates standalone parsers
- **Nom** (Rust) - Parser combinators (no generation)
- **Ragel** (C) - Generates standalone state machines

### Runtime Approach
- **ANTLR4** (all targets) - Runtime library
- **Tree-sitter** - Runtime library
- **Bison/Yacc** - Runtime library (minimal)

## Hybrid Approach (Recommended) 🎯

**Generate standalone code WITH optional runtime optimization**

### How It Works

1. **Default: Standalone**
   ```bash
   minipg generate grammar.g4 --standalone
   ```
   - Generates complete `.rs` file
   - No dependencies
   - Includes all necessary code

2. **Optional: Runtime-optimized**
   ```bash
   minipg generate grammar.g4 --runtime
   ```
   - Generates smaller code
   - Depends on `minipg-runtime`
   - Uses optimized algorithms

3. **User Choice**
   - Embedded systems, WASM → standalone
   - Large projects, multiple grammars → runtime
   - Prototyping, learning → standalone

### Implementation Strategy

```rust
// Generated standalone code structure
pub struct Parser {
    input: String,
    pos: usize,
    // All state inline
}

impl Parser {
    // All parsing logic generated inline
    fn parse_expr(&mut self) -> Result<Expr> {
        // Complete implementation generated
    }
    
    // Error recovery generated inline
    fn recover(&mut self) { /* ... */ }
    
    // DFA logic generated inline
    fn match_token(&mut self) -> Token { /* ... */ }
}
```

```rust
// Generated runtime-dependent code structure
use minipg_runtime::{Parser, Token, DFA, ErrorRecovery};

pub struct MyParser {
    parser: Parser,
    dfa: DFA,
}

impl MyParser {
    fn parse_expr(&mut self) -> Result<Expr> {
        // Thin wrapper, delegates to runtime
        self.parser.parse_rule(RULE_EXPR)
    }
}
```

## Optimization Strategy

### For Standalone Generation

1. **Template-based with optimization passes**
   ```rust
   // Generate optimized code directly
   fn generate_optimized_dfa(grammar: &Grammar) -> String {
       // Compute DFA at generation time
       // Emit optimized match statements
       format!("match state {{ {} }}", generate_transitions())
   }
   ```

2. **Inline common patterns**
   - Generate specialized code for common cases
   - Use const generics where possible
   - Emit lookup tables for lexer

3. **Dead code elimination**
   - Only generate needed features
   - Skip unused error recovery if grammar is simple
   - Omit visitor/listener if not requested

### For Runtime-dependent Generation

1. **Shared optimization in runtime**
   - DFA minimization
   - ATN optimization
   - Prediction caching

2. **Grammar-specific tables**
   - Generate static data
   - Runtime interprets tables

## Recommendation

### Phase 1: Start with Standalone (Now)
- Easier to implement
- No runtime to maintain
- Users can inspect/modify code
- Perfect for learning and prototyping

### Phase 2: Add Runtime Option (Later)
- After standalone is solid
- For users with multiple grammars
- For production use cases

### Phase 3: Optimize Both (Future)
- Profile standalone generation
- Optimize runtime library
- Let users choose based on needs

## Code Generation Approach

### Standalone Template Structure

```rust
// templates/standalone_parser.rs.template
pub struct {parser_name} {
    input: &'input str,
    pos: usize,
    tokens: Vec<Token>,
}

impl {parser_name} {
    // Generate complete lexer
    fn lex(&mut self) -> Result<Vec<Token>> {
        {lexer_implementation}  // Fully generated
    }
    
    // Generate complete parser
    {parser_rules}  // All rules fully generated
    
    // Generate error recovery
    fn recover(&mut self) {
        {error_recovery}  // Fully generated
    }
}

// Generate AST types
{ast_definitions}

// Generate visitor if requested
{visitor_implementation}
```

### Key Optimizations for Standalone

1. **Lexer Optimization**
   - Generate DFA as match statements (compile-time)
   - Use lookup tables for character classes
   - Inline common token patterns

2. **Parser Optimization**
   - Generate specialized code per rule
   - Inline small rules
   - Use const arrays for prediction

3. **Size Optimization**
   - Share common code via private functions
   - Use macros for repetitive patterns
   - Strip debug info in release

## Comparison with antlr4rust

| Aspect | antlr4rust | minipg (standalone) | minipg (runtime) |
|--------|------------|---------------------|------------------|
| Dependencies | `antlr-rust` | None | `minipg-runtime` |
| Generated size | Small (~500 lines) | Large (~2000 lines) | Small (~500 lines) |
| Customization | Hard | Easy | Hard |
| Performance | Excellent | Good (can be excellent) | Excellent |
| Binary size | Medium | Small | Medium |
| Compile time | Medium | Fast | Medium |
| Updates | Easy | Regenerate | Easy |

## Decision ✅

**FINAL DECISION: Standalone code generation only. No runtime library.**

### Rationale

1. **Simpler to implement** - Focus on code generation quality
2. **Better for users** - No dependency management
3. **More transparent** - Users understand generated code
4. **Easier to optimize** - Can see what's generated
5. **Unique value** - Different from antlr4rust
6. **Multi-language support** - Each target gets optimized standalone code
7. **ANTLR4 compatibility** - Generate from standard ANTLR4 grammars

### Implementation Plan

1. **Phase 1**: Core standalone generation
   - Rust target (primary)
   - Python target
   - JavaScript/TypeScript targets
   
2. **Phase 2**: Additional languages
   - Go target
   - Java target
   - C/C++ targets

3. **Phase 3**: ANTLR4 full compatibility
   - Parse all ANTLR4 grammar features
   - Support actions, predicates, labels
   - Grammar imports and inheritance

## Conclusion

**Your instinct is correct!** Standalone generation is a good differentiator and aligns with Rust's philosophy of zero-cost abstractions. The key is to make the generated code:

1. **Efficient** - Optimize at generation time
2. **Readable** - Users should understand it
3. **Maintainable** - Clean structure
4. **Complete** - No hidden dependencies

This approach makes minipg more accessible and easier to adopt than antlr4rust, while still allowing runtime optimization as an advanced option later.
