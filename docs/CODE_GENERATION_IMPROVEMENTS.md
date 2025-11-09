# Code Generation Improvements Plan

This document outlines the plan to address the three high-priority TODO items:
1. Code generator produces skeleton code (needs optimization)
2. Generated code needs error recovery  
3. Need more robust ANTLR4 grammar parsing

## Current Status

### 1. Code Generator Skeleton Code ✅ Started

**Problem**: Code generators create function signatures but leave bodies as `unimplemented!()` or `// TODO`.

**Progress**:
- ✅ Created `src/codegen/rule_body.rs` module structure
- ✅ Defined `RuleBodyContext` for code generation state
- ✅ Started implementation of `generate_rust_rule_body()` function
- ⚠️ Compilation errors need fixing
- ⚠️ Syntax generation needs refinement
- ⚠️ Need to extend to all languages

**What's Needed**:
1. Fix compilation errors in `rule_body.rs`
2. Generate proper Rust syntax (correct match expressions, blocks)
3. Handle token access patterns correctly for parser structure
4. Implement for Python, JavaScript, TypeScript, Go
5. Complete C, C++, Java implementations (currently TODOs)

**Implementation Approach**:
- Generate code by recursively walking rule alternatives and elements
- Handle terminals (token matching)
- Handle non-terminals (rule calls)
- Handle quantifiers (?, *, +)
- Handle groups and alternatives
- Handle actions and predicates
- Build proper AST nodes with labels

### 2. Error Recovery ⚠️ Partial

**Problem**: Generated parsers need better error recovery strategies.

**Current State**:
- ✅ Lexer error recovery: Implemented (skip invalid characters, collect errors)
- ❌ Parser error recovery: Not fully implemented
- ❌ Panic mode: Not implemented
- ❌ Synchronization points: Not implemented

**What's Needed**:
1. **Panic Mode**: When parser encounters error, enter panic mode
   - Skip tokens until synchronization point reached
   - Synchronization points: statement boundaries, rule boundaries
   
2. **Better Error Messages**:
   - Show expected tokens at error location
   - Show what was found instead
   - Show context (line, column, surrounding tokens)
   
3. **Error Recovery Strategies**:
   - Single token deletion
   - Token insertion
   - Token substitution
   - Phrase-level recovery

**Implementation Plan**:
1. Add panic mode state to generated parsers
2. Define synchronization tokens per grammar
3. Generate recovery code in parser methods
4. Enhance error types with more context

### 3. Robust ANTLR4 Grammar Parsing ⚠️ Good but Needs Work

**Problem**: Parser needs to handle more edge cases and provide better error messages.

**Current State**:
- ✅ Basic ANTLR4 syntax parsing works
- ✅ Handles most common grammar constructs
- ⚠️ Error messages could be more descriptive
- ⚠️ Edge cases may cause panics
- ⚠️ Recovery from parse errors limited

**What's Needed**:
1. **Better Error Messages**:
   - Show grammar context (which rule, which alternative)
   - Show position with surrounding text
   - Suggest fixes for common errors

2. **Edge Case Handling**:
   - Invalid character sequences
   - Malformed character classes
   - Unterminated strings/comments
   - Nested structures beyond limits

3. **Error Recovery**:
   - Recover from parse errors and continue
   - Collect multiple errors in one pass
   - Validate grammar even with parse errors

**Implementation Plan**:
1. Enhance error reporting in `src/parser/parser.rs`
2. Add more validation in `src/analysis/validator.rs`
3. Implement incremental error collection
4. Add recovery strategies to parser

## Implementation Priority

### Phase 1: Fix Rule Body Generation (Rust) 🔥 High Priority
1. Fix compilation errors in `rule_body.rs`
2. Generate correct Rust syntax
3. Test with simple grammars
4. Verify generated code compiles

### Phase 2: Extend to Other Languages
1. Python rule body generation
2. JavaScript/TypeScript rule body generation  
3. Go rule body generation
4. C/C++/Java implementations

### Phase 3: Error Recovery
1. Implement panic mode
2. Add synchronization points
3. Enhance error messages
4. Test with malformed input

### Phase 4: Robustness
1. Better error messages with context
2. Edge case handling
3. Incremental error collection
4. Comprehensive testing

## Testing Strategy

For each improvement:
1. **Unit Tests**: Test code generation functions
2. **Integration Tests**: Generate code from real grammars
3. **Compilation Tests**: Verify generated code compiles
4. **Runtime Tests**: Test generated parsers with valid/invalid input
5. **Error Tests**: Test error recovery scenarios

## Related Files

- `src/codegen/rule_body.rs` - Rule body generation (in progress)
- `src/codegen/rust.rs` - Rust code generator (needs rule body integration)
- `src/codegen/python.rs` - Python code generator (needs rule body generation)
- `src/codegen/javascript.rs` - JavaScript code generator (needs rule body generation)
- `src/codegen/typescript.rs` - TypeScript code generator (needs rule body generation)
- `src/codegen/go.rs` - Go code generator (needs rule body generation)
- `src/codegen/c.rs` - C code generator (needs full implementation)
- `src/codegen/cpp.rs` - C++ code generator (needs full implementation)
- `src/codegen/java.rs` - Java code generator (needs full implementation)
- `src/parser/parser.rs` - Grammar parser (needs better error handling)
- `src/parser/error_recovery.rs` - Error recovery strategies (needs enhancement)

## Notes

- This is a large refactoring that will improve generated code quality significantly
- Should be done incrementally, testing each language separately
- Consider performance impact of generated code (optimize hot paths)
- Maintain backward compatibility with existing generated code where possible

