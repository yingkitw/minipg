# Migration Guide from ANTLR4

## Overview

minipg is designed to be compatible with ANTLR4 grammars, making migration straightforward. This guide covers the differences and migration steps.

## Quick Comparison

| Feature | ANTLR4 | minipg | Status |
|---------|--------|--------|--------|
| Grammar Syntax | ✅ | ✅ | Compatible |
| Lexer Rules | ✅ | ✅ | Compatible |
| Parser Rules | ✅ | ✅ | Compatible |
| Fragment Rules | ✅ | ✅ | Compatible |
| Labels | ✅ | ✅ | Compatible |
| Actions | ✅ | ✅ | Compatible |
| Predicates | ✅ | ✅ | Compatible |
| Listeners | ✅ | ⚠️ | Partial |
| Visitors | ✅ | ⚠️ | Partial |
| Tree Construction | ✅ | ⚠️ | Manual |
| Runtime | Java | Multi-language | Different |

## Migration Steps

### Step 1: Test Your Grammar

```bash
# Test if your grammar parses
minipg validate --input YourGrammar.g4
```

### Step 2: Generate Code

```bash
# Generate for your target language
minipg generate --input YourGrammar.g4 --output parser.rs --language rust
```

### Step 3: Update Code

#### ANTLR4 (Java)
```java
CharStream input = CharStreams.fromString(text);
YourLexer lexer = new YourLexer(input);
CommonTokenStream tokens = new CommonTokenStream(lexer);
YourParser parser = new YourParser(tokens);
ParseTree tree = parser.startRule();
```

#### minipg (Rust)
```rust
let mut lexer = YourLexer::new(text);
let (tokens, errors) = lexer.tokenize_all();
let parser = YourParser::new(tokens);
// Manual tree construction or use generated methods
```

## Supported Features

### ✅ Fully Supported

1. **Basic Grammar Syntax**
   ```antlr4
   grammar MyGrammar;
   
   rule : element+ ;
   LEXER_RULE : [a-z]+ ;
   ```

2. **Operators**
   - `*` (zero or more)
   - `+` (one or more)
   - `?` (optional)
   - `|` (alternative)

3. **Character Classes**
   ```antlr4
   LETTER : [a-zA-Z] ;
   DIGIT : [0-9] ;
   ANY : . ;
   ```

4. **Fragment Rules**
   ```antlr4
   fragment HEX : [0-9a-fA-F] ;
   NUMBER : '0x' HEX+ ;
   ```

5. **Labels**
   ```antlr4
   expr : left=expr '+' right=expr ;
   rule : element # labeledAlt ;
   ```

6. **Actions**
   ```antlr4
   rule : TOKEN { action code } ;
   ```

7. **Predicates**
   ```antlr4
   rule : {predicate}? element ;
   ```

### ⚠️ Partially Supported

1. **Listeners/Visitors**
   - Can be generated
   - Manual implementation required
   - No automatic tree walking

2. **Tree Construction**
   - Not automatic
   - Manual AST building
   - Or use actions

### ❌ Not Yet Supported

1. **Import/Tokenize**
   ```antlr4
   import CommonLexer;  // Not supported
   ```

2. **Modes**
   ```antlr4
   mode STRING_MODE;  // Not supported
   ```

3. **Channels**
   ```antlr4
   WS : [ \t]+ -> channel(HIDDEN);  // Partial
   ```

## Key Differences

### 1. Error Handling

**ANTLR4**:
```java
parser.removeErrorListeners();
parser.addErrorListener(new CustomErrorListener());
```

**minipg**:
```rust
let (tokens, errors) = lexer.tokenize_all();
for error in errors {
    // Handle error
}
```

### 2. Tree Construction

**ANTLR4**: Automatic
```java
ParseTree tree = parser.expr();
```

**minipg**: Manual or via actions
```rust
// Use actions to build AST
// Or manually construct from tokens
```

### 3. Runtime

**ANTLR4**: Single runtime (Java)
**minipg**: Native to each language

### 4. Performance

**ANTLR4**: Seconds (JVM-based)
**minipg**: Sub-millisecond generation (native Rust)

## Migration Examples

### Example 1: Simple Grammar

**ANTLR4 Grammar** (unchanged):
```antlr4
grammar Expr;

expr : expr '+' expr
     | NUMBER
     ;

NUMBER : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
```

**ANTLR4 Usage**:
```java
ExprLexer lexer = new ExprLexer(input);
ExprParser parser = new ExprParser(new CommonTokenStream(lexer));
ParseTree tree = parser.expr();
```

**minipg Usage**:
```rust
let mut lexer = ExprLexer::new(input);
let (tokens, _) = lexer.tokenize_all();
let parser = ExprParser::new(tokens);
// Manual parsing or use generated methods
```

### Example 2: With Actions

**Grammar**:
```antlr4
grammar Calc;

expr returns [int value]
    : left=expr '+' right=expr { $value = $left.value + $right.value; }
    | NUMBER { $value = $NUMBER.int; }
    ;
```

**minipg**: Actions are preserved in generated code

### Example 3: With Listeners

**ANTLR4**:
```java
ParseTreeWalker.DEFAULT.walk(new MyListener(), tree);
```

**minipg**: Implement visitor pattern manually
```rust
// Implement your own visitor
trait ExprVisitor {
    fn visit_expr(&mut self, node: &Expr);
}
```

## Best Practices

### 1. Start Simple
- Migrate simple grammars first
- Test thoroughly
- Add complexity gradually

### 2. Use Actions
- For AST construction
- For semantic validation
- For custom behavior

### 3. Handle Errors
- Use tokenize_all() for error collection
- Report all errors at once
- Continue parsing when possible

### 4. Test Thoroughly
- Compare outputs with ANTLR4
- Test edge cases
- Benchmark performance

## Common Issues

### Issue 1: Missing Tree Construction
**Solution**: Use actions or manual AST building

### Issue 2: Different Error Messages
**Solution**: Customize error messages in your code

### Issue 3: Performance Differences
**Solution**: Profile and optimize (usually faster!)

## Performance Benefits

After migration, you'll see:
- **Fast code generation** - sub-millisecond for typical grammars
- **Efficient parsing** - optimized native code
- **Low memory usage** - <100 KB for generation
- **No JVM overhead** - native Rust implementation

## Gradual Migration

You can migrate incrementally:

1. **Phase 1**: Test grammar compatibility
2. **Phase 2**: Generate code, keep ANTLR4 runtime
3. **Phase 3**: Switch to minipg for new features
4. **Phase 4**: Full migration

## Getting Help

- **Documentation**: [docs/INDEX.md](../INDEX.md)
- **Examples**: [examples/](../../examples/)
- **Comparison**: [COMPARISON_WITH_ANTLR4RUST.md](../../COMPARISON_WITH_ANTLR4RUST.md)

---

**Next**: [Language-Specific Guides](#) | [Troubleshooting](TROUBLESHOOTING.md)
