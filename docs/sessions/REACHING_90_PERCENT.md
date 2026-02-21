# Reaching 90% Completion - Implementation Guide

**Date**: February 21, 2026  
**Current Status**: Rust 75%, Python 70%, JavaScript 70%  
**Target**: 90% for all three languages  
**Estimated Time**: 2-3 hours per language (6-9 hours total)

---

## Current State Summary

### ✅ What's Working (70-75%)

All three generators now support:
- ✅ AST node type generation (structs/dataclasses/classes)
- ✅ Field extraction from labeled elements
- ✅ Rule body generation with alternatives
- ✅ Error handling (Result/Exceptions/Exceptions)
- ✅ Terminal matching with EOF checks
- ✅ Rule references
- ✅ List labels (`+=`)
- ✅ Multiple alternatives with backtracking

### ⏸️ What's Missing (to reach 90%)

1. **String Literal Matching** - Match `'string'` or `"string"`
2. **Optional Elements** - Handle `element?`
3. **Zero-or-More** - Handle `element*`
4. **One-or-More** - Handle `element+`
5. **Groups** - Handle `(alt1 | alt2 | alt3)`

---

## Implementation Plan

### Phase 1: String Literals (30 minutes per language)

**Rust Implementation**:
```rust
Element::StringLiteral { value, label, is_list } => {
    code.push_str(&format!("// Match string literal: \"{}\"\\n", value));
    code.push_str("if self.position >= self.tokens.len() {\\n");
    code.push_str("    return Err(ParseError::new(\\n");
    code.push_str(&format!("        format!(\"Unexpected EOF, expected: \\\\\"{}\\\\\"\"),\\n", value));
    code.push_str("        self.position\\n"));
    code.push_str("    ));\\n");
    code.push_str("}\\n");
    
    code.push_str(&format!("if self.tokens[self.position].text == \"{}\" {{\\n", value));
    if let Some(lbl) = label {
        if *is_list {
            code.push_str(&format!("    {}.push(self.tokens[self.position].clone());\\n", lbl));
        } else {
            code.push_str(&format!("    let {} = self.tokens[self.position].clone();\\n", lbl));
        }
    }
    code.push_str("    self.position += 1;\\n");
    code.push_str("} else {\\n");
    code.push_str("    return Err(ParseError::new(\\n");
    code.push_str(&format!("        format!(\"Expected \\\\\"{}\\\\\", got {{:?}}\", self.tokens[self.position].text),\\n", value));
    code.push_str("        self.position\\n"));
    code.push_str("    ));\\n");
    code.push_str("}\\n");
}
```

**Python Implementation**:
```python
# Match string literal: "+"
if self.position >= len(self.tokens):
    raise ParseError("Unexpected EOF, expected: \"+\"", self.position, [], None)
if self.tokens[self.position].text == "+":
    op = self.tokens[self.position]
    self.position += 1
else:
    raise ParseError(f"Expected \"+\", got {self.tokens[self.position].text}", 
                    self.position, [], None)
```

**JavaScript Implementation**:
```javascript
// Match string literal: "+"
if (this.position >= this.tokens.length) {
  throw new ParseError('Unexpected EOF, expected: "+"', this.position);
}
if (this.tokens[this.position].text === "+") {
  const op = this.tokens[this.position];
  this.position++;
} else {
  throw new ParseError(`Expected "+", got ${this.tokens[this.position].text}`, 
                      this.position);
}
```

### Phase 2: Optional Elements (30 minutes per language)

**Rust Implementation**:
```rust
Element::Optional { element, .. } => {
    code.push_str("// Optional element\\n");
    code.push_str("let saved_pos = self.position;\\n");
    code.push_str("match {\\n");
    // Generate inner element code
    code.push_str(&generate_element_code(element, rule, ctx, lang, 0));
    code.push_str("    Ok(_) => {},\\n");
    code.push_str("    Err(_) => {\\n");
    code.push_str("        self.position = saved_pos;\\n");
    code.push_str("    }\\n");
    code.push_str("}\\n");
}
```

**Python Implementation**:
```python
# Optional element
saved_pos = self.position
try:
    # Generate inner element code
    result = self.parse_term()
except ParseError:
    self.position = saved_pos
```

**JavaScript Implementation**:
```javascript
// Optional element
const savedPos = this.position;
try {
  // Generate inner element code
  const result = this.parseTerm();
} catch (err) {
  if (err instanceof ParseError) {
    this.position = savedPos;
  } else {
    throw err;
  }
}
```

### Phase 3: Zero-or-More (30 minutes per language)

**Rust Implementation**:
```rust
Element::ZeroOrMore { element, .. } => {
    code.push_str("// Zero or more\\n");
    code.push_str("loop {\\n");
    code.push_str("    let saved_pos = self.position;\\n");
    code.push_str("    match {\\n");
    // Generate inner element code
    code.push_str(&generate_element_code(element, rule, ctx, lang, 0));
    code.push_str("        Ok(_) => continue,\\n");
    code.push_str("        Err(_) => {\\n");
    code.push_str("            self.position = saved_pos;\\n");
    code.push_str("            break;\\n");
    code.push_str("        }\\n");
    code.push_str("    }\\n");
    code.push_str("}\\n");
}
```

**Python Implementation**:
```python
# Zero or more
while True:
    saved_pos = self.position
    try:
        # Generate inner element code
        result = self.parse_term()
    except ParseError:
        self.position = saved_pos
        break
```

**JavaScript Implementation**:
```javascript
// Zero or more
while (true) {
  const savedPos = this.position;
  try {
    // Generate inner element code
    const result = this.parseTerm();
  } catch (err) {
    if (err instanceof ParseError) {
      this.position = savedPos;
      break;
    } else {
      throw err;
    }
  }
}
```

### Phase 4: One-or-More (30 minutes per language)

**Rust Implementation**:
```rust
Element::OneOrMore { element, .. } => {
    code.push_str("// One or more (at least one required)\\n");
    // First one is required
    code.push_str(&generate_element_code(element, rule, ctx, lang, 0));
    // Then zero or more
    code.push_str("loop {\\n");
    code.push_str("    let saved_pos = self.position;\\n");
    code.push_str("    match {\\n");
    code.push_str(&generate_element_code(element, rule, ctx, lang, 0));
    code.push_str("        Ok(_) => continue,\\n");
    code.push_str("        Err(_) => {\\n");
    code.push_str("            self.position = saved_pos;\\n");
    code.push_str("            break;\\n");
    code.push_str("        }\\n");
    code.push_str("    }\\n");
    code.push_str("}\\n");
}
```

**Python Implementation**:
```python
# One or more (at least one required)
# First one is required
result = self.parse_term()
# Then zero or more
while True:
    saved_pos = self.position
    try:
        result = self.parse_term()
    except ParseError:
        self.position = saved_pos
        break
```

**JavaScript Implementation**:
```javascript
// One or more (at least one required)
// First one is required
let result = this.parseTerm();
// Then zero or more
while (true) {
  const savedPos = this.position;
  try {
    result = this.parseTerm();
  } catch (err) {
    if (err instanceof ParseError) {
      this.position = savedPos;
      break;
    } else {
      throw err;
    }
  }
}
```

### Phase 5: Groups (1 hour per language)

**Rust Implementation**:
```rust
Element::Group { alternatives } => {
    code.push_str("// Group with alternatives\\n");
    code.push_str("{\\n");
    for (i, alt) in alternatives.iter().enumerate() {
        if i > 0 {
            code.push_str("    // Try next alternative\\n");
        }
        code.push_str("    let saved_pos = self.position;\\n");
        code.push_str("    if {\\n");
        // Generate code for all elements in this alternative
        for elem in &alt.elements {
            code.push_str(&generate_element_code(elem, rule, ctx, lang, 0));
        }
        code.push_str("        true\\n");
        code.push_str("    } {\\n");
        code.push_str("        // Alternative matched\\n");
        if i < alternatives.len() - 1 {
            code.push_str("    } else {\\n");
            code.push_str("        self.position = saved_pos;\\n");
        }
    }
    // Close all blocks
    for _ in 0..alternatives.len() {
        code.push_str("    }\\n");
    }
    code.push_str("}\\n");
}
```

---

## Testing Strategy

### Test with Real Grammars

**1. Calculator Grammar** (Simple):
```antlr
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

**Expected Features**:
- String literals: `'+'`, `'-'`, `'*'`, `'/'`, `'('`, `')'`
- Zero-or-more: `(('+' | '-') term)*`
- Groups: `('+' | '-')`
- Terminals: `NUMBER`

**2. JSON Grammar** (Medium):
```antlr
grammar JSON;

json: value;
value: object | array | STRING | NUMBER | 'true' | 'false' | 'null';
object: '{' (pair (',' pair)*)? '}';
pair: STRING ':' value;
array: '[' (value (',' value)*)? ']';

STRING: '"' (~["\r\n])* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
WS: [ \t\r\n]+ -> skip;
```

**Expected Features**:
- String literals: `'{'`, `'}'`, `'['`, `']'`, `':'`, `','`, `'true'`, `'false'`, `'null'`
- Optional: `(pair (',' pair)*)?`
- Zero-or-more: `(',' pair)*`
- Groups: Multiple

**3. Expression Grammar** (Complex):
```antlr
grammar Expression;

expr: term (op=('+'|'-') term)*;
term: factor (op=('*'|'/') factor)*;
factor: NUMBER | ID | '(' expr ')';

NUMBER: [0-9]+;
ID: [a-zA-Z_][a-zA-Z0-9_]*;
WS: [ \t\r\n]+ -> skip;
```

**Expected Features**:
- All previous features
- Labeled alternatives: `op=('+'|'-')`

---

## Implementation Order

### Recommended Approach

**Week 1 - Rust (3 hours)**:
1. String literals (30 min)
2. Optional elements (30 min)
3. Zero-or-more (30 min)
4. One-or-more (30 min)
5. Groups (1 hour)
6. Test with Calculator (30 min)

**Week 2 - Python (3 hours)**:
1. Apply same patterns from Rust
2. Test with Calculator
3. Fix issues

**Week 3 - JavaScript (3 hours)**:
1. Apply same patterns from Rust
2. Test with Calculator
3. Fix issues

**Week 4 - Integration Testing**:
1. Test all three with JSON grammar
2. Test all three with Expression grammar
3. Fix issues found
4. Document limitations

---

## File Locations

### Where to Add Code

**Rust**: `minipg-antlr/src/codegen/rule_body.rs`
- Function: `generate_element_code()`
- Add cases for each Element type

**Python**: `minipg-antlr/src/codegen/python.rs`
- Function: `generate_python_alternative()`
- Add handling for each element type

**JavaScript**: `minipg-antlr/src/codegen/javascript.rs`
- Function: `generate_javascript_alternative()`
- Add handling for each element type

---

## Success Criteria

### For 90% Completion

**Must Have**:
- ✅ String literal matching working
- ✅ Optional elements working
- ✅ Zero-or-more working
- ✅ One-or-more working
- ✅ Groups with alternatives working
- ✅ Calculator grammar generates and compiles
- ✅ All tests passing

**Nice to Have**:
- JSON grammar working
- Expression grammar working
- Character classes
- Negation (~)
- Wildcards (.)

---

## Known Challenges

### 1. Nested Structures

**Challenge**: Groups within repetitions within optionals  
**Solution**: Recursive `generate_element_code()` handles this naturally

### 2. Backtracking

**Challenge**: Need to save/restore position for alternatives  
**Solution**: Already implemented with `saved_pos` pattern

### 3. List Labels in Repetitions

**Challenge**: `items+=element*` needs to accumulate  
**Solution**: Initialize `Vec`/`list`/`Array` before loop, push in each iteration

### 4. Error Messages

**Challenge**: Generic "parse error" not helpful  
**Solution**: Already have specific messages with expected vs actual

---

## Estimated Timeline

### Conservative Estimate

| Task | Rust | Python | JavaScript | Total |
|------|------|--------|------------|-------|
| String Literals | 30 min | 30 min | 30 min | 1.5 hours |
| Optional | 30 min | 30 min | 30 min | 1.5 hours |
| Zero-or-More | 30 min | 30 min | 30 min | 1.5 hours |
| One-or-More | 30 min | 30 min | 30 min | 1.5 hours |
| Groups | 1 hour | 1 hour | 1 hour | 3 hours |
| Testing | 30 min | 30 min | 30 min | 1.5 hours |
| **Total** | **3.5 hours** | **3.5 hours** | **3.5 hours** | **10.5 hours** |

### Optimistic Estimate

If patterns work well: **6-8 hours total**

---

## Next Session Plan

### Immediate Actions (Next Session)

1. **Implement String Literals in Rust** (30 min)
   - Add case in `generate_element_code()`
   - Test with simple grammar
   - Verify generated code compiles

2. **Implement Optional in Rust** (30 min)
   - Add case for `Element::Optional`
   - Test with `element?` pattern
   - Verify backtracking works

3. **Test with Calculator Grammar** (30 min)
   - Generate Rust parser
   - Compile generated code
   - Fix any issues

**Total**: 1.5 hours for significant progress

---

## Conclusion

**Current State**: Strong foundation at 70-75%  
**Path to 90%**: Clear and achievable  
**Estimated Time**: 6-10 hours total  
**Recommendation**: Implement incrementally, test frequently  

The project is in excellent shape. The patterns are established, the infrastructure is solid, and reaching 90% is a matter of systematic implementation following the proven patterns.

**Status**: Ready for next phase of implementation! 🚀
