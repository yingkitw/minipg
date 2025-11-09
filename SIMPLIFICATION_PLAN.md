# Codebase Simplification Plan

## Overview

This plan outlines strategies to simplify the minipg codebase while maintaining functionality and improving maintainability. The focus is on reducing redundancy, consolidating similar code, cleaning up documentation, and removing incomplete/unused features.

## Current State Analysis

### Strengths
- ✅ Well-organized modular structure
- ✅ Comprehensive test coverage (369+ tests)
- ✅ Clear separation of concerns
- ✅ Good trait-based architecture

### Areas for Simplification
1. **Documentation Overhead**: 70+ markdown files (many redundant/session-specific)
2. **Code Duplication**: Similar patterns across 8 code generators
3. **Incomplete Features**: TODOs in generated code (C/C++/Java/Go)
4. **Test File Proliferation**: 25+ test files with potential overlap
5. **Unused/Incomplete Modules**: `rule_body.rs` appears incomplete

---

## Phase 1: Documentation Consolidation (High Impact, Low Risk)

### Goal
Reduce documentation files by 50% while maintaining essential information.

### Actions

#### 1.1 Archive Session/Historical Files
**Target**: Move to `docs/archive/` or delete
- `SESSION_SUMMARY_*.md` (3 files)
- `BUILD_FIX_SUMMARY.md`
- `BUILD_VERIFICATION_CHECKLIST.md`
- `CONSOLIDATION_COMPLETE.md`
- `PROJECT_COMPLETION_SUMMARY.md`

**Rationale**: Historical session summaries are not needed for users/developers.

#### 1.2 Consolidate Implementation Guides
**Target**: Merge into single `IMPLEMENTATION.md`
- `ACTION_TRANSLATION_IMPLEMENTATION.md`
- `ANTLR4_FULL_COMPATIBILITY.md`
- `GRAMMAR_COMPOSITION_IMPLEMENTATION.md`
- `LEXER_MODES_IMPLEMENTATION.md`
- `RULE_FEATURES_IMPLEMENTATION.md`
- `GO_CODEGEN_IMPLEMENTATION.md`
- `JAVA_CODEGEN_IMPLEMENTATION.md`
- `C_CPP_CODEGEN_IMPLEMENTATION.md`

**Rationale**: These are implementation details that can be consolidated into one reference document.

#### 1.3 Consolidate Testing Documentation
**Target**: Merge into `docs/TESTING.md`
- `GRAMMARS_V4_ANTLR4_COMPLETION.md`
- `GRAMMARS_V4_ANTLR4_TESTING.md`

**Rationale**: Testing documentation should be centralized.

#### 1.4 Keep Essential Documentation
**Keep in root**:
- `README.md` (main entry point)
- `ARCHITECTURE.md` (system design)
- `TODO.md` (development roadmap)
- `CHANGELOG.md` (version history)
- `LICENSE-APACHE` (legal requirement)
- `KNOWN_LIMITATIONS.md` (user-facing)

**Keep in `docs/`**:
- `USER_GUIDE.md`
- `API.md`
- `GRAMMAR_SYNTAX.md`
- `ANTLR4_COMPATIBILITY.md`
- `MULTI_LANGUAGE_PLAN.md`
- Language-specific guides (Rust, Python, JS, TS, Go)
- `index.html` (landing page)

**Expected Reduction**: ~30 files → ~15 files (50% reduction)

---

## Phase 2: Code Generator Architecture Enhancement (High Impact, Medium Risk)

### Goal
Reduce code duplication across code generators while making it **easier to add new target languages**. Complete incomplete generators and create an extensible architecture.

### Actions

#### 2.1 Create Common Code Generator Infrastructure
**New Module**: `src/codegen/common.rs`

Extract shared functionality into reusable components:
- **Token type generation** - Common enum/struct generation patterns
- **Error type generation** - Standardized error handling patterns
- **Basic lexer structure** - Common lexer scaffolding
- **Basic parser structure** - Common parser scaffolding
- **Mode/channel handling** - Already exists in `modes.rs`, ensure all languages use it
- **Action translation** - Already exists in `actions.rs`, ensure all languages use it

**Pattern**: Create a `BaseCodeGenerator` trait with default implementations that language-specific generators can override.

#### 2.2 Complete Incomplete Generators
**Current TODOs**: C, C++, Java, Go generators have placeholder TODOs

**Action Plan**:
1. **Complete Go generator** (highest priority - already mostly done)
   - Implement actual lexer tokenization logic
   - Implement parser rule body generation
   - Use `rule_body.rs` or create Go-specific implementation

2. **Complete C/C++ generators**
   - Implement lexer tokenization with DFA
   - Implement parser rule body generation
   - Handle memory management properly
   - Add proper error recovery

3. **Complete Java generator**
   - Implement lexer tokenization
   - Implement parser rule body generation
   - Use proper Java idioms (exceptions, packages)

**Strategy**: Leverage existing `rule_body.rs` module or complete it to generate actual parsing logic.

#### 2.3 Create Language Registry System
**New Module**: `src/codegen/registry.rs`

Replace large match statement with extensible registry:

```rust
pub struct LanguageRegistry {
    generators: HashMap<String, Box<dyn CodeGenerator>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut reg = Self { generators: HashMap::new() };
        // Register all languages
        reg.register("rust", RustCodeGenerator::new());
        reg.register("python", PythonCodeGenerator::new());
        // ... etc
        reg
    }
    
    pub fn register(&mut self, name: &str, generator: impl CodeGenerator + 'static) {
        self.generators.insert(name.to_string(), Box::new(generator));
    }
    
    pub fn get(&self, name: &str) -> Option<&dyn CodeGenerator> {
        self.generators.get(name).map(|g| g.as_ref())
    }
}
```

**Benefits**:
- Easy to add new languages (just call `register()`)
- Can support language aliases (e.g., "js" → "javascript")
- Can support plugin-style extensions in future

#### 2.4 Standardize Code Generator Interface
**Enhancement**: Make all generators follow consistent patterns:

1. **Consistent method structure**:
   - `generate_lexer()` - Token type definitions + lexer implementation
   - `generate_parser()` - Parser implementation
   - `generate_error_type()` - Error handling types
   - `generate_visitor()` - Optional visitor pattern (if enabled)

2. **Consistent configuration handling**:
   - All generators accept same `CodeGenConfig`
   - Language-specific options via config extensions

3. **Consistent output format**:
   - Standardized file structure
   - Consistent naming conventions
   - Proper documentation/comments

#### 2.5 Add Language-Specific Guides
**Action**: Complete language guides for all 8 languages:
- ✅ Rust, Python, JavaScript, TypeScript, Go (already exist)
- ⚠️ C, C++, Java (need guides)

**New Guides**: Create `docs/C_GUIDE.md`, `docs/CPP_GUIDE.md`, `docs/JAVA_GUIDE.md`

#### 2.6 Future Language Support
**Make it easy to add**:
- Swift
- Kotlin
- C#
- Ruby
- PHP
- etc.

**Requirements for new language**:
1. Implement `CodeGenerator` trait
2. Register in `LanguageRegistry`
3. Add tests
4. Add language guide

**Expected Reduction**: ~2000 lines of duplicated code → ~1200 lines (40% reduction) while **supporting more languages**

---

## Phase 3: Test File Consolidation (Medium Impact, Low Risk)

### Goal
Reduce test file count by consolidating related tests.

### Actions

#### 3.1 Consolidate Example Tests
**Merge into**: `tests/test_examples.rs`
- `test_all_examples.rs`
- `test_advanced_examples.rs`
- `test_all_g4_files.rs`
- `test_completejson_line.rs`

**Rationale**: All test example grammars in one place.

#### 3.2 Consolidate Feature Tests
**Merge into**: `tests/test_features.rs`
- `test_rule_features.rs`
- `test_list_labels.rs`
- `test_named_actions.rs`
- `test_named_actions_codegen.rs`
- `test_lexer_modes_parsing.rs`
- `test_modes_channels_actions.rs`
- `test_unicode_escape.rs`

**Rationale**: Feature tests grouped by functionality.

#### 3.3 Consolidate Compatibility Tests
**Merge into**: `tests/test_compatibility.rs`
- `test_antlr4_compatibility.rs`
- `test_antlr4_test_suite.rs`
- `test_grammars_v4_compatibility.rs`

**Rationale**: All ANTLR4 compatibility tests together.

#### 3.4 Keep Separate (Specialized)
- `integration_test.rs` (core integration)
- `end_to_end_test.rs` (full pipeline)
- `e2e_simple_pipeline.rs` (simple E2E)
- `property_based_tests.rs` (property testing)
- `fuzzing_tests.rs` (fuzzing)
- `large_file_tests.rs` (performance)
- `memory_profiling.rs` (memory)
- `parser_advanced_features.rs` (advanced parser)

**Expected Reduction**: 25 files → ~12 files (52% reduction)

---

## Phase 4: Module Cleanup (Low Impact, Low Risk)

### Goal
Remove unused/incomplete code and simplify module structure.

### Actions

#### 4.1 Evaluate `rule_body.rs`
**Current State**: Appears incomplete with TODOs

**Options**:
- **A**: Complete the implementation
- **B**: Remove if not used
- **C**: Move to experimental/feature flag

**Recommendation**: Check usage first. If unused, remove. If partially used, complete or mark experimental.

#### 4.2 Simplify `codegen/mod.rs` Dispatcher
**Current**: Large match statement with repetitive code

**Improvement**: Use a registry pattern or macro to reduce repetition:

```rust
// Instead of 8 match arms, use:
let generator: Box<dyn CodeGenerator> = match language {
    "rust" => Box::new(RustCodeGenerator::new()),
    "python" => Box::new(PythonCodeGenerator::new()),
    // ... etc
};
generator.generate(&analysis.grammar, &self.config)
```

#### 4.3 Consolidate Analysis Modules
**Current**: 7 separate analysis modules

**Consider**: Some modules might be underutilized:
- `ambiguity.rs` - Check usage
- `first_follow.rs` - Check usage
- `reachability.rs` - Check usage

**Action**: Audit usage, consolidate if minimal usage.

---

## Phase 5: Dependency Cleanup (Low Impact, Low Risk)

### Goal
Review and potentially reduce dependencies.

### Actions

#### 5.1 Review Dev Dependencies
**Current**:
- `insta` - Snapshot testing (keep)
- `proptest` - Property testing (keep)
- `criterion` - Benchmarks (keep if used)
- `tempfile` - Temp files (keep)

**Action**: Verify all are used. Remove unused.

#### 5.2 Review Runtime Dependencies
**Current**:
- `serde` / `serde_json` - Serialization (keep)
- `thiserror` / `anyhow` - Error handling (consider consolidating)
- `clap` - CLI (keep)
- `tracing` / `tracing-subscriber` - Logging (keep)
- `rmcp` / `tokio` / `schemars` - MCP server (evaluate necessity)

**Action**: 
- Consider using only `anyhow` OR `thiserror`, not both
- Evaluate if MCP server is essential or can be optional feature

---

## Phase 6: Configuration Simplification (Low Impact, Low Risk)

### Goal
Simplify configuration and reduce options.

### Actions

#### 6.1 Simplify `CodeGenConfig`
**Current**: May have many options

**Action**: Review and remove unused options, consolidate related options.

#### 6.2 Simplify CLI Commands
**Current**: Multiple commands

**Action**: Ensure commands are essential and not redundant.

---

## Implementation Priority

### High Priority (Do First)
1. **Phase 1**: Documentation Consolidation
   - **Impact**: High (reduces clutter)
   - **Risk**: Low (no code changes)
   - **Effort**: 2-4 hours

2. **Phase 3**: Test File Consolidation
   - **Impact**: Medium (easier navigation)
   - **Risk**: Low (tests remain same)
   - **Effort**: 4-6 hours

### Medium Priority (Do Next)
3. **Phase 2**: Code Generator Architecture Enhancement
   - **Impact**: High (reduces duplication, enables more languages)
   - **Risk**: Medium (requires careful refactoring)
   - **Effort**: 12-16 hours (includes completing incomplete generators)

4. **Phase 4**: Module Cleanup
   - **Impact**: Medium (cleaner codebase)
   - **Risk**: Low (removing unused code)
   - **Effort**: 2-4 hours

### Low Priority (Do Later)
5. **Phase 5**: Dependency Cleanup
   - **Impact**: Low (smaller dependencies)
   - **Risk**: Low (if done carefully)
   - **Effort**: 1-2 hours

6. **Phase 6**: Configuration Simplification
   - **Impact**: Low (simpler API)
   - **Risk**: Low (if backward compatible)
   - **Effort**: 2-3 hours

---

## Success Metrics

### Quantitative
- **Documentation files**: 70+ → ~35 (50% reduction)
- **Test files**: 25 → ~12 (52% reduction)
- **Code duplication**: Reduce by ~40% in code generators (while supporting more languages)
- **Supported languages**: 8 → 8+ (complete incomplete ones, enable easy addition)
- **Build time**: Maintain or improve (no regression)
- **Test count**: Maintain or increase (no test removal, only consolidation + new language tests)

### Qualitative
- Easier to navigate codebase
- Clearer documentation structure
- Less cognitive overhead for new contributors
- Maintained functionality (no regressions)

---

## Risk Mitigation

### Before Starting
1. **Backup**: Ensure git is clean, create backup branch
2. **Test**: Run full test suite to establish baseline
3. **Document**: Document current state

### During Implementation
1. **Incremental**: Make changes in small, testable increments
2. **Test**: Run tests after each change
3. **Review**: Review changes before committing

### After Implementation
1. **Verify**: Full test suite passes
2. **Benchmark**: Ensure no performance regression
3. **Document**: Update README/ARCHITECTURE if needed

---

## Timeline Estimate

- **Phase 1** (Documentation): 1 day
- **Phase 2** (Code Generators): 3-4 days (includes completing incomplete generators)
- **Phase 3** (Tests): 1 day
- **Phase 4** (Modules): 0.5 day
- **Phase 5** (Dependencies): 0.5 day
- **Phase 6** (Configuration): 0.5 day

**Total**: ~6-8 days of focused work (updated for completing generators)

---

## Adding New Target Languages

### Quick Start Guide

After Phase 2 implementation, adding a new language will be straightforward:

1. **Create generator module**: `src/codegen/{language}.rs`
2. **Implement CodeGenerator trait**:
   ```rust
   pub struct {Language}CodeGenerator;
   
   impl CodeGenerator for {Language}CodeGenerator {
       type Input = Grammar;
       type Config = CodeGenConfig;
       
       fn generate(&self, grammar: &Grammar, config: &CodeGenConfig) -> Result<String> {
           // Use family helpers or implement from scratch
       }
       
       fn target_language(&self) -> &'static str {
           "{language}"
       }
   }
   ```

3. **Register in LanguageRegistry**: Add to `registry.rs`
4. **Add tests**: Create test file or add to existing
5. **Add documentation**: Create `docs/{LANGUAGE}_GUIDE.md`

### Example: Adding Swift

```rust
// src/codegen/swift.rs
pub struct SwiftCodeGenerator;

impl CodeGenerator for SwiftCodeGenerator {
    type Input = Grammar;
    type Config = CodeGenConfig;
    
    fn generate(&self, grammar: &Grammar, config: &CodeGenConfig) -> Result<String> {
        // Implement Swift-specific code generation
        // Use common helpers from common.rs where applicable
    }
    
    fn target_language(&self) -> &'static str {
        "swift"
    }
}
```

**Estimated effort**: 4-8 hours for a new language (after Phase 2)

---

## Notes

- This plan prioritizes **maintainability** and **extensibility** - making it easier to add new languages
- Focus on **consolidation** and **abstraction** rather than **removal** where possible
- **Complete incomplete generators** (C, C++, Java, Go) rather than marking as experimental
- Create **extensible architecture** for future language additions
- Keep **backward compatibility** where feasible
- Maintain **test coverage** throughout, add tests for completed generators
- Document **decisions** in ARCHITECTURE.md or TODO.md
- **Goal**: Support 8+ languages with less code duplication

---

**Last Updated**: 2025-01-XX
**Status**: Planning Phase
**Next Step**: Review and prioritize phases based on current needs

