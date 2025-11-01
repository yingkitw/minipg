# Known Limitations - Alpha v0.1.0

This document tracks known limitations in the current alpha release.

## Character Class Parsing

**Status**: Partially Implemented  
**Priority**: High  
**Target**: v0.2.0

### Current State

The parser has been updated to support:
- ✅ `CharClass` AST variant with negation support
- ✅ Parser logic for negated classes (`~[...]`)
- ✅ Unicode escape handling (`\uXXXX`)
- ✅ Multiple ranges in character classes

### Issue

The **lexer** needs to be context-aware to properly tokenize character class contents. Currently:

```antlr4
// This fails:
WORD: [a-zA-Z]+;

// Because the lexer tokenizes it as:
// LeftBracket, Identifier("a"), Range, Identifier("zA"), Range, Identifier("Z"), RightBracket
```

### What's Needed

The lexer needs to:
1. Detect `[` and enter "character class mode"
2. Read all content until `]` as a single token or sequence
3. Handle escape sequences within the class
4. Handle nested brackets if needed

### Workaround

For now, use simple character ranges that don't require complex lexing:
```antlr4
// These work:
DIGIT: '0'..'9';  // Range operator
LETTER: 'a' | 'b' | 'c';  // Alternatives
```

### Implementation Plan

**Week 1-2**: Lexer Context-Aware Mode
- [ ] Add lexer state machine (Normal, CharClass, String, Comment)
- [ ] Implement character class content tokenization
- [ ] Handle escape sequences in character classes
- [ ] Add comprehensive tests

**Week 3**: Integration & Testing
- [ ] Update parser to work with new lexer tokens
- [ ] Test with CompleteJSON.g4
- [ ] Test with SQL.g4
- [ ] Unignore the 9 integration tests

### Related Files

- `src/parser/lexer.rs` - Needs state machine
- `src/parser/parser.rs` - Already updated
- `src/ast/element.rs` - Already has CharClass variant
- `tests/integration_grammar_tests.rs` - 9 ignored tests

---

## Other Limitations

### ANTLR4 Features Not Yet Supported

1. **Rule Arguments & Returns** (Planned for v0.2.0)
   - `rule[int x] returns [Type]`
   - `locals [Type var]`

2. **Lexer Modes & Channels** (Planned for v0.2.0)
   - `mode NAME;`
   - `-> channel(NAME)`
   - `pushMode()`, `popMode()`

3. **Named Actions** (Planned for v0.3.0)
   - `@header {...}`
   - `@members {...}`

4. **Grammar Imports** (Planned for v0.3.0)
   - `import X;`
   - Grammar inheritance

### Target Language Support

Currently supported:
- ✅ Rust
- ✅ Python
- ✅ JavaScript
- ✅ TypeScript

Planned:
- 🚧 Go (v0.2.0)
- 🚧 C (v0.2.0)
- 🚧 C++ (v0.2.0)
- 🚧 Java (v0.3.0)

---

## Reporting Issues

If you encounter issues not listed here, please report them at:
https://github.com/yingkitw/minipg/issues

Include:
- Grammar file (or minimal reproduction)
- Error message
- Expected behavior
- minipg version (`minipg --version`)

---

*Last Updated: 2025-10-17*
