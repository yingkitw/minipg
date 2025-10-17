# Release Notes: minipg v0.1.0-alpha.2

**Release Date**: October 17, 2025

## 🎉 Highlights

This release brings **major improvements** to ANTLR4 grammar compatibility, with all integration tests now passing!

- ✅ **99 tests passing** (up from 65, +52% improvement)
- ✅ **0 tests ignored** (down from 9)
- ✅ **CompleteJSON.g4 fully supported**
- ✅ **SQL.g4 fully supported**
- ✅ **Non-greedy quantifiers** (`.*?`, `.+?`, `.??`)
- ✅ **Lexer state machine** for character classes
- ✅ **Lexer commands** parsed and stored

## 🚀 Major Features

### 1. Lexer State Machine for Character Classes

Implemented a comprehensive state machine to properly handle ANTLR4 character class syntax:

```antlr4
// Now fully supported!
fragment SAFECODEPOINT
    : ~ ["\\\u0000-\u001F]
    ;

fragment HEX
    : [0-9a-fA-F]
    ;

fragment EXP
    : [Ee] [+\-]? [0-9]+
    ;
```

**Features:**
- **Context-Aware Tokenization**: Automatically enters CharClass mode after `:`, `|`, `~`, `(`, `]`, `?`, `*`, `+`
- **Escape Sequences**: Full support for `\\`, `\/`, `\n`, `\r`, `\t`, `\u0000-\uFFFF`
- **Special Characters**: `/`, `+`, and other special chars treated as literals in CharClass mode
- **Comment Handling**: Skips whitespace but preserves `/` for character matching

### 2. Non-Greedy Quantifiers

Full support for non-greedy quantifiers, enabling complex patterns like block comments:

```antlr4
BLOCK_COMMENT
    : '/*' .*? '*/' -> skip
    ;

LINE_COMMENT
    : '//' .*? '\n' -> skip
    ;
```

**Implementation:**
- Added `greedy: bool` field to `Optional`, `ZeroOrMore`, `OneOrMore` variants
- New helper methods: `zero_or_more_non_greedy()`, `one_or_more_non_greedy()`, `optional_non_greedy()`
- Parser automatically detects `?` after quantifiers (`*?`, `+?`, `??`)

### 3. Lexer Commands

Lexer commands are now properly parsed and stored in the AST:

```antlr4
WS
    : [ \t\n\r]+ -> skip
    ;

COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;

STRING_MODE
    : '"' -> pushMode(STRING)
    ;
```

**Supported Commands:**
- `skip` - Skip this token
- `channel(NAME)` - Send to a channel
- `mode(NAME)` - Switch to a mode
- `type(TYPE)` - Change token type
- `pushMode(NAME)` - Push mode
- `popMode` - Pop mode
- `more` - Continue current token

## 📊 Test Suite Improvements

### Before (v0.1.0-alpha.1)
- 65 tests passing
- 9 tests ignored
- CompleteJSON.g4: ❌ Failed
- SQL.g4: ❌ Failed

### After (v0.1.0-alpha.2)
- **99 tests passing** (+34 tests)
- **0 tests ignored** (-9 ignored)
- CompleteJSON.g4: ✅ All 5 tests passing
- SQL.g4: ✅ All 4 tests passing

### Test Breakdown
- 48 unit tests
- 11 E2E tests
- 9 integration tests (CompleteJSON + SQL)
- 31 other tests (character classes, Unicode, etc.)

## 🐛 Bug Fixes

1. **Character Class Parsing**: Fixed parsing of complex patterns like `["\\\u0000-\u001F]`
2. **Special Character Tokenization**: Fixed handling of `/`, `+` in character classes
3. **Comment Handling**: Fixed comment detection in CharClass mode
4. **Mode Entry/Exit**: Fixed CharClass mode state transitions
5. **Pattern Matching**: Updated all pattern matches to handle new `greedy` field

## 🔧 Technical Changes

### AST Changes
- **Element enum**: Added `greedy` field to quantifier variants
- **Alternative struct**: Added `lexer_command: Option<LexerCommand>` field
- **New enum**: `LexerCommand` with 7 variants

### Parser Changes
- Enhanced quantifier parsing to detect non-greedy modifiers
- Added lexer command parsing in `parse_alternative()`
- Improved CharClass mode detection logic

### Lexer Changes
- Implemented `LexerMode` enum with `Normal` and `CharClass` modes
- Added mode stack for nested contexts
- Added `skip_whitespace()` for CharClass mode
- Enhanced escape sequence handling

## 📈 Performance

- **No performance regression**
- Code generation remains sub-millisecond
- All benchmarks still passing
- Memory usage unchanged

## 🔜 What's Next

### Planned for v0.1.0-alpha.3
- Implement lexer command code generation
- Add rule arguments and return values
- Support for lexer modes and channels
- Additional target languages (Go, C, C++)

### Long-term Roadmap
- Full ANTLR4 compatibility
- Grammar composition and imports
- VS Code extension
- Online playground

## 📦 Installation

```bash
cargo install minipg
```

Or from source:
```bash
git clone https://github.com/yingkitw/minipg
cd minipg
cargo install --path .
```

## 🙏 Acknowledgments

Thank you to all contributors and users who provided feedback on alpha.1!

## 📝 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete details.

---

**Questions or Issues?** Please open an issue on [GitHub](https://github.com/yingkitw/minipg/issues).
