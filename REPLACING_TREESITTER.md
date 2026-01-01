# Replacing Tree-sitter with minipg

## Executive Summary

**Goal**: Use minipg-generated parsers to completely replace Tree-sitter for editor integration, providing a unified solution where one ANTLR4 grammar serves both runtime parsing and editor tooling.

## Current State (v0.1.5)

### What Works Today

✅ **9 Target Languages**: Rust, Python, JavaScript, TypeScript, Go, Java, C, C++, Tree-sitter  
✅ **Runtime Parsers**: Standalone parsers with error recovery  
✅ **Tree-sitter Grammar Generation**: Converts ANTLR4 → grammar.js  
✅ **193+ Tests**: 100% pass rate  
✅ **Production Ready**: Published to crates.io  

### Current Limitation

❌ **Tree-sitter is still needed**: Generated grammar.js files require Tree-sitter to build parsers for editors

## Vision: Direct Editor Integration

### What We're Building

Instead of:
```
ANTLR4 Grammar → minipg → Tree-sitter grammar.js → Tree-sitter → Editor Parser
```

We want:
```
ANTLR4 Grammar → minipg → Editor-Ready Parser (via LSP)
```

### Key Benefits

1. **Single Source of Truth**: One ANTLR4 grammar for everything
2. **No Conversion**: No ANTLR4 → Tree-sitter translation needed
3. **Larger Ecosystem**: Access to 1000+ ANTLR4 grammars vs ~100 Tree-sitter grammars
4. **Full Features**: Actions, predicates, modes work natively
5. **Consistency**: Same parser in runtime and editor
6. **Better Debugging**: One grammar, one parser, easier to debug

## Implementation Roadmap

### Phase 1: Incremental Parsing (v0.2.0)

**Goal**: Parse only changed portions of files for fast updates

**Features**:
- Position tracking (byte offsets, line/column)
- Edit tracking and application
- Subtree reuse for unchanged regions
- Performance: <10ms for typical edits

**Example**:
```rust
let mut parser = IncrementalParser::new(grammar);
let tree = parser.parse(source);

// User types a character
let edit = Edit {
    start_byte: 100,
    old_end_byte: 100,
    new_end_byte: 101,
    // ... position info
};

let new_tree = parser.parse_incremental(&tree, &edit);
// Only re-parses affected subtree, reuses rest
```

### Phase 2: Query Language (v0.3.0)

**Goal**: Pattern matching for syntax highlighting

**Features**:
- S-expression query syntax (Tree-sitter compatible)
- Pattern matching engine
- Capture groups for extracting nodes
- Query files (highlights.scm, injections.scm, etc.)

**Example Query**:
```scheme
; highlights.scm
(function_declaration
  name: (identifier) @function.name
  parameters: (parameter_list) @function.parameters)

(string_literal) @string
(number_literal) @number
(comment) @comment

; Operators
["+" "-" "*" "/"] @operator
```

### Phase 3: Language Server Protocol (v0.4.0)

**Goal**: LSP server for editor integration

**Features**:
- Semantic tokens (syntax highlighting)
- Folding ranges (code folding)
- Document symbols (outline view)
- Diagnostics (errors/warnings)
- Go to definition
- Find references
- Hover information

**Usage**:
```bash
# Start LSP server
minipg-lsp --grammar MyLanguage.g4

# Editor connects via LSP protocol
# Gets all features automatically
```

### Phase 4: Editor Extensions (v0.5.0)

**Goal**: Native extensions for popular editors

**VS Code**:
```json
{
  "name": "vscode-minipg-mylang",
  "contributes": {
    "languages": [{
      "id": "mylang",
      "extensions": [".mylang"]
    }],
    "grammars": [{
      "language": "mylang",
      "scopeName": "source.mylang",
      "path": "./grammar.g4"
    }]
  }
}
```

**Neovim**:
```lua
require('minipg').setup({
  grammar = 'MyLanguage.g4',
  highlights = 'queries/highlights.scm',
})
```

**Emacs**:
```elisp
(use-package minipg-mode
  :config
  (add-to-list 'auto-mode-alist '("\\.mylang\\'" . minipg-mylang-mode)))
```

## Comparison: minipg vs Tree-sitter

| Feature | minipg (Planned) | Tree-sitter |
|---------|------------------|-------------|
| **Grammar Format** | ANTLR4 (industry standard) | Custom JavaScript |
| **Grammar Ecosystem** | 1000+ grammars | ~100 grammars |
| **Incremental Parsing** | ✅ (planned) | ✅ |
| **Error Recovery** | ✅ (ANTLR4-style) | ✅ |
| **Query Language** | ✅ (planned, compatible) | ✅ |
| **Editor Support** | ✅ (planned) | ✅ |
| **Runtime Parsers** | ✅ (9 languages) | ❌ |
| **Semantic Actions** | ✅ | ❌ |
| **Lexer Modes** | ✅ | ❌ |
| **Single Source** | ✅ (one grammar for all) | ❌ (separate grammars) |

## Migration Path

### For Current Tree-sitter Users

1. **If you have an ANTLR4 grammar**:
   ```bash
   # Generate LSP server
   minipg generate-lsp MyLanguage.g4 -o lsp/
   
   # Install editor extension
   code --install-extension minipg-mylanguage
   
   # Done! No Tree-sitter needed
   ```

2. **If you only have Tree-sitter grammar**:
   - Convert Tree-sitter → ANTLR4 (manual, one-time)
   - Or keep using Tree-sitter until minipg LSP is ready
   - Or write new ANTLR4 grammar (recommended)

### For New Projects

1. **Write ANTLR4 grammar** (or use existing from grammars-v4)
2. **Generate runtime parser**: `minipg generate grammar.g4 -l rust`
3. **Generate LSP server**: `minipg generate-lsp grammar.g4`
4. **Install editor support**: One-time setup
5. **Enjoy**: Both runtime and editor from one grammar

## Timeline

### v0.2.0 (Q1 2026) - Incremental Parsing
- Position tracking
- Edit application
- Subtree reuse
- Performance benchmarks

### v0.3.0 (Q2 2026) - Query Language
- Query parser
- Pattern matching
- Highlight queries
- Documentation

### v0.4.0 (Q3 2026) - LSP Server
- LSP implementation
- Semantic tokens
- All LSP features
- Multi-editor testing

### v0.5.0 (Q4 2026) - Editor Extensions
- VS Code extension
- Neovim plugin
- Emacs mode
- Marketplace publishing

### v1.0.0 (Q1 2027) - Production Release
- All features complete
- Performance optimized
- Comprehensive documentation
- Active community

## Technical Challenges

### 1. Incremental Parsing Performance

**Challenge**: Match Tree-sitter's <5ms incremental parse time

**Solution**:
- Efficient subtree invalidation
- Lazy re-parsing
- Parallel parsing
- Native code (Rust)

### 2. Query Language Compatibility

**Challenge**: Support existing Tree-sitter queries

**Solution**:
- Implement same S-expression syntax
- Compatible capture semantics
- Migration tool for existing queries

### 3. Editor Integration

**Challenge**: Work with multiple editors

**Solution**:
- Standard LSP protocol
- Editor-specific extensions
- Comprehensive testing

### 4. Memory Efficiency

**Challenge**: Keep memory usage low

**Solution**:
- Efficient AST representation
- Incremental updates
- Memory profiling
- Lazy node creation

## Success Criteria

### Performance
- ✅ Initial parse: <100ms for 10k lines
- ✅ Incremental edit: <10ms
- ✅ Memory: <50MB per file
- ✅ Startup: <50ms

### Compatibility
- ✅ Works with existing ANTLR4 grammars
- ✅ Tree-sitter query compatibility
- ✅ LSP protocol compliance
- ✅ Multi-editor support

### User Experience
- ✅ Easy setup (one command)
- ✅ Fast response times
- ✅ Accurate highlighting
- ✅ Comprehensive documentation

## Call to Action

### For Contributors

1. **Review strategy**: `docs/EDITOR_INTEGRATION_STRATEGY.md`
2. **Pick a phase**: Incremental parsing, queries, LSP, or extensions
3. **Start coding**: Follow roadmap and architecture
4. **Submit PRs**: Help build the future

### For Users

1. **Try current version**: Generate parsers with minipg today
2. **Provide feedback**: What features do you need?
3. **Test grammars**: Try your ANTLR4 grammars
4. **Spread the word**: Help grow the community

## Resources

- **Strategy Document**: `docs/EDITOR_INTEGRATION_STRATEGY.md`
- **Roadmap**: `TODO.md` (Ecosystem & Editor Integration section)
- **Specification**: `spec.md` (Future Enhancements section)
- **Repository**: https://github.com/yingkitw/minipg
- **Issues**: https://github.com/yingkitw/minipg/issues

## Conclusion

By implementing incremental parsing, a query language, and LSP support, minipg will **completely replace Tree-sitter** while offering significant advantages:

✅ **Single source of truth**: One ANTLR4 grammar for everything  
✅ **Larger ecosystem**: 1000+ existing grammars  
✅ **More features**: Full ANTLR4 capabilities  
✅ **Better consistency**: Same parser everywhere  
✅ **Easier maintenance**: One grammar to update  

**The future is one grammar, everywhere.**

---

**Status**: Strategy Complete, Implementation Starting  
**Version**: 0.1.5 → 1.0.0  
**Timeline**: Q1 2026 - Q1 2027  
**Join us**: Make it happen!
