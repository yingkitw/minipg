# Grammars-v4 Test Results

## Test Execution Summary

**Date**: 2025-01-09  
**Repository**: https://github.com/antlr/grammars-v4  
**Total Grammars Found**: 1114 .g4 files  
**Test Status**: Partial (test interrupted at ~64/1114)

## Test Setup

- Created symlink: `grammars-v4-cache` → `~/Downloads/grammars-v4-master`
- Used test suite: `tests/test_grammars_v4_all.rs`
- Test mode: `GRAMMARS_V4_SKIP_DOWNLOAD=1` (using cached grammars)

## Initial Findings

### Popular Grammars Test Results

From the `test_popular_grammars_v4` test:
- **Total tested**: 10 popular grammars
- **Passed**: 0 (0.0%)
- **Failed**: 2 (20.0%) - Found and attempted to parse
- **Skipped**: 8 (80.0%) - Not found (path differences)

**Failed Grammars**:
1. `javascript/javascript/JavaScriptParser.g4` - Parse error: "Expected grammar, but found parser ('parser')"
2. `sql/tsql/TSqlParser.g4` - Parse error: "Expected grammar, but found parser ('parser')"

### Full Test Progress (Partial)

From `test_all_grammars_v4` (interrupted at ~64/1114):
- **Passed**: ~9 grammars (✓)
- **Failed**: ~55 grammars (✗)

**Sample of Passing Grammars**:
- `action/action.g4` ✓
- `alef/alef.g4` ✓
- `alpaca/alpaca.g4` ✓
- `angelscript/angelscript.g4` ✓
- `antlr/antlr4/examples/Hello.g4` ✓

## Key Issues Identified

### 1. Parser/Lexer Grammar Keywords

Many grammars use `parser grammar` or `lexer grammar` keywords instead of just `grammar`:
- Example: `JavaScriptParser.g4` starts with `parser grammar JavaScriptParser;`
- Current minipg parser expects: `grammar Name;`

**Impact**: High - affects many grammars that separate lexer and parser definitions

### 2. Path Differences

Some grammars are in different locations than expected:
- Expected: `java/java9/Java9.g4`
- Actual: May be in `java/Java9.g4` or other subdirectories

**Impact**: Medium - test paths need adjustment

### 3. Grammar Format Variations

Some grammars may use ANTLR4 features not yet supported:
- Named actions
- Lexer modes
- Channels
- Advanced rule features

**Impact**: Medium - depends on specific features used

## Recommendations

1. **Add Support for Parser/Lexer Grammar Keywords**
   - Update parser to recognize `parser grammar Name;` and `lexer grammar Name;`
   - These are common in grammars-v4 repository

2. **Update Test Paths**
   - Verify actual paths in grammars-v4-master
   - Update `test_popular_grammars_v4` with correct paths

3. **Continue Full Test Suite**
   - Run complete test against all 1114 grammars
   - Generate detailed failure report
   - Categorize failures by error type

4. **Create Grammar Compatibility Report**
   - Document which ANTLR4 features are supported
   - List known limitations
   - Provide migration guide for unsupported features

## Next Steps

1. Fix parser to support `parser grammar` and `lexer grammar` keywords
2. Re-run full test suite to completion
3. Analyze failure patterns
4. Prioritize feature additions based on real-world usage

