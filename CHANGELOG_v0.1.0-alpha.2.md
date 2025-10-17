# Changelog for v0.1.0-alpha.2

## Major Improvements

### Lexer State Machine for Character Classes ✅
Implemented a comprehensive lexer state machine to properly handle ANTLR4 character class syntax:

- **CharClass Mode**: Lexer now enters a special mode when parsing character classes `[...]`
- **Context-Aware Tokenization**: Automatically enters CharClass mode after:
  - `:` (rule start)
  - `|` (alternative)
  - `~` (negation)
  - `(` (grouping)
  - `]` (after another character class)
  - `?`, `*`, `+` (after quantifiers)

### Character Class Features
- **Escape Sequences**: Full support for escape sequences in character classes:
  - Simple escapes: `\\`, `\/`, `\n`, `\r`, `\t`, etc.
  - Unicode escapes: `\u0000`, `\u001F`, etc.
- **Special Characters**: Properly handles special characters as literals in CharClass mode:
  - `/` (forward slash) - not treated as comment start
  - `+` (plus) - treated as character, not quantifier
  - Other special characters treated as literals
- **Whitespace Handling**: Skips whitespace but preserves `/` for character matching

### Lexer Commands Support ✅
Added basic parsing support for ANTLR4 lexer commands:
- `-> skip` - skip tokens (e.g., whitespace)
- `-> channel(NAME)` - send tokens to named channel
- Other lexer commands with optional parentheses

### Test Suite Improvements
- **95 tests passing** (up from 65)
- **CompleteJSON.g4 fully supported**: All 5 integration tests now pass
  - Grammar parsing ✅
  - Semantic analysis ✅
  - Rust code generation ✅
  - Python code generation ✅
  - Multi-language code generation ✅
- **4 tests ignored**: SQL.g4 tests (require non-greedy quantifiers `.*?`)

### Bug Fixes
- Fixed character class parsing for complex patterns like `["\\\u0000-\u001F]`
- Fixed tokenization of special characters in character classes
- Fixed comment handling in character class mode
- Fixed CharClass mode entry/exit logic

## Technical Details

### Files Modified
- `src/parser/lexer.rs`: Implemented CharClass mode and state machine
- `src/parser/parser.rs`: Added lexer command parsing
- `tests/integration_grammar_tests.rs`: Removed ignore attributes from passing tests

### New Capabilities
1. Can now parse real-world ANTLR4 grammars like CompleteJSON.g4
2. Proper handling of Unicode escapes in character classes
3. Support for lexer commands (basic parsing)
4. Improved error messages for character class parsing

## Known Limitations
- Non-greedy quantifiers (`.*?`, `.+?`) not yet supported (needed for SQL.g4)
- Lexer commands are parsed but not implemented in code generation
- Some advanced ANTLR4 features still pending

## Next Steps
1. Implement non-greedy quantifiers for SQL.g4 support
2. Implement lexer command code generation
3. Prepare for v0.1.0-alpha.2 release
