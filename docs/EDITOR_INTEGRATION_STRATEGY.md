# Editor Integration Strategy - Replacing Tree-sitter

## Vision

Enable minipg-generated parsers to **replace Tree-sitter** for editor integration, providing syntax highlighting, code folding, navigation, and semantic analysis directly from ANTLR4 grammars.

## Why Replace Tree-sitter?

### Advantages of minipg Approach

1. **Single Source of Truth**: One ANTLR4 grammar for both runtime parsing AND editor integration
2. **No Conversion**: No need to convert ANTLR4 → Tree-sitter grammar
3. **Full ANTLR4 Features**: Actions, predicates, modes work natively
4. **Multi-Language**: Same grammar generates parsers for 9 languages
5. **Proven Grammars**: Use existing grammars-v4 repository (1000+ grammars)
6. **Better Error Recovery**: ANTLR4-style error recovery
7. **Semantic Actions**: Can execute code during parsing

### Current Tree-sitter Advantages (To Match)

1. **Incremental Parsing**: Only re-parse changed regions
2. **Error Resilience**: Continues parsing after errors
3. **Query Language**: Pattern matching for highlighting
4. **Native Performance**: C library with editor bindings
5. **Established Ecosystem**: Wide editor support

## Implementation Phases

### Phase 1: Core Parser Features ✅ (DONE)

- [x] Generate standalone parsers
- [x] Error recovery
- [x] AST generation
- [x] Multiple target languages
- [x] ANTLR4 compatibility

### Phase 2: Editor-Specific Features (NEXT)

#### 2.1 Incremental Parsing

**Goal**: Only re-parse changed portions of the document

**Approach**:
```rust
pub trait IncrementalParser {
    fn parse_incremental(&mut self, 
                        old_tree: &SyntaxTree,
                        edit: &Edit) -> SyntaxTree;
}

pub struct Edit {
    start_byte: usize,
    old_end_byte: usize,
    new_end_byte: usize,
    start_point: Point,
    old_end_point: Point,
    new_end_point: Point,
}
```

**Implementation**:
- Track parse tree with byte positions
- Invalidate affected nodes on edit
- Re-parse only invalidated subtrees
- Reuse unchanged nodes

#### 2.2 Syntax Tree with Positions

**Goal**: Provide detailed position information for every node

```rust
pub struct SyntaxNode {
    kind: NodeKind,
    start_byte: usize,
    end_byte: usize,
    start_point: Point,
    end_point: Point,
    children: Vec<SyntaxNode>,
    parent: Option<Weak<SyntaxNode>>,
}

pub struct Point {
    row: usize,
    column: usize,
}
```

#### 2.3 Query Language

**Goal**: Pattern matching for syntax highlighting

**Approach**: Implement S-expression query language similar to Tree-sitter

```scheme
; Example query for highlighting
(function_declaration
  name: (identifier) @function.name
  parameters: (parameter_list) @function.parameters)

(string_literal) @string
(number_literal) @number
(comment) @comment
```

**Implementation**:
```rust
pub struct Query {
    patterns: Vec<Pattern>,
}

impl Query {
    pub fn new(source: &str) -> Result<Self>;
    pub fn matches<'a>(&'a self, tree: &'a SyntaxTree) -> impl Iterator<Item = Match<'a>>;
}
```

### Phase 3: Language Server Protocol (LSP)

**Goal**: Provide LSP server using minipg parsers

**Features**:
- Syntax highlighting (via semantic tokens)
- Code folding
- Document symbols
- Go to definition
- Find references
- Hover information
- Diagnostics (syntax errors)

**Implementation**:
```rust
pub struct MinipgLanguageServer {
    parser: Box<dyn IncrementalParser>,
    documents: HashMap<Url, Document>,
}

impl LanguageServer for MinipgLanguageServer {
    fn semantic_tokens(&self, uri: &Url) -> Vec<SemanticToken>;
    fn folding_ranges(&self, uri: &Url) -> Vec<FoldingRange>;
    fn document_symbols(&self, uri: &Url) -> Vec<DocumentSymbol>;
}
```

### Phase 4: Editor Bindings

#### 4.1 VS Code Extension

**Package**: `vscode-minipg`

```typescript
import { LanguageClient } from 'vscode-languageclient/node';

export function activate(context: ExtensionContext) {
    const client = new LanguageClient(
        'minipg',
        'minipg Language Server',
        serverOptions,
        clientOptions
    );
    client.start();
}
```

#### 4.2 Neovim Plugin

**Package**: `nvim-minipg`

```lua
local minipg = require('minipg')

minipg.setup({
    grammar = 'path/to/grammar.g4',
    highlights = 'path/to/highlights.scm',
})
```

#### 4.3 Emacs Mode

**Package**: `minipg-mode`

```elisp
(use-package minipg-mode
  :config
  (add-to-list 'auto-mode-alist '("\\.mylang\\'" . minipg-mode)))
```

## Technical Architecture

### Component Stack

```
┌─────────────────────────────────────┐
│         Editor (VS Code, etc)       │
├─────────────────────────────────────┤
│      LSP Client / Extension         │
├─────────────────────────────────────┤
│      minipg Language Server         │
│  ┌───────────────────────────────┐  │
│  │   Incremental Parser          │  │
│  │   Query Engine                │  │
│  │   Semantic Analysis           │  │
│  └───────────────────────────────┘  │
├─────────────────────────────────────┤
│   Generated Parser (from minipg)    │
│  ┌───────────────────────────────┐  │
│  │   Lexer                       │  │
│  │   Parser                      │  │
│  │   AST Builder                 │  │
│  └───────────────────────────────┘  │
├─────────────────────────────────────┤
│      ANTLR4 Grammar (.g4)           │
└─────────────────────────────────────┘
```

### Data Flow

```
1. User edits file
   ↓
2. Editor sends change to LSP server
   ↓
3. Incremental parser updates syntax tree
   ↓
4. Query engine extracts highlights/symbols
   ↓
5. LSP server sends results to editor
   ↓
6. Editor updates UI
```

## Implementation Roadmap

### Milestone 1: Incremental Parsing (2-3 weeks)

**Tasks**:
- [ ] Add position tracking to AST nodes
- [ ] Implement Edit struct and application
- [ ] Add incremental parsing algorithm
- [ ] Benchmark performance vs full re-parse
- [ ] Test with large files (10k+ lines)

**Deliverables**:
- `IncrementalParser` trait
- Position-aware AST
- Performance benchmarks

### Milestone 2: Query Language (2-3 weeks)

**Tasks**:
- [ ] Design query syntax (S-expressions)
- [ ] Implement query parser
- [ ] Implement pattern matching engine
- [ ] Add capture groups (@name)
- [ ] Create highlight queries for example languages

**Deliverables**:
- Query parser and engine
- Example queries (highlights.scm)
- Query documentation

### Milestone 3: LSP Server (3-4 weeks)

**Tasks**:
- [ ] Implement LSP protocol handlers
- [ ] Add semantic tokens support
- [ ] Add folding ranges
- [ ] Add document symbols
- [ ] Add diagnostics
- [ ] Test with multiple editors

**Deliverables**:
- `minipg-lsp` binary
- LSP server implementation
- Multi-editor testing

### Milestone 4: Editor Extensions (2-3 weeks per editor)

**VS Code**:
- [ ] Create extension scaffold
- [ ] Integrate LSP client
- [ ] Add syntax highlighting
- [ ] Add configuration options
- [ ] Publish to marketplace

**Neovim**:
- [ ] Create Lua plugin
- [ ] Integrate with nvim-lspconfig
- [ ] Add Treesitter-style API
- [ ] Publish to package manager

**Emacs**:
- [ ] Create Emacs Lisp package
- [ ] Integrate with lsp-mode
- [ ] Add major mode
- [ ] Publish to MELPA

## Performance Targets

### Parsing Performance

| Metric | Target | Tree-sitter |
|--------|--------|-------------|
| Initial parse (10k lines) | <100ms | ~50ms |
| Incremental edit | <10ms | ~5ms |
| Memory usage | <50MB | ~20MB |
| Startup time | <50ms | ~10ms |

### Optimization Strategies

1. **Lazy Parsing**: Parse only visible regions initially
2. **Caching**: Cache parse results per file
3. **Parallel Parsing**: Parse multiple files concurrently
4. **Native Code**: Use Rust for performance-critical paths
5. **WASM**: Compile to WebAssembly for browser editors

## Comparison: minipg vs Tree-sitter

| Feature | minipg (Proposed) | Tree-sitter |
|---------|-------------------|-------------|
| Grammar Format | ANTLR4 (industry standard) | Custom JavaScript |
| Incremental Parsing | ✅ (planned) | ✅ |
| Error Recovery | ✅ (ANTLR4-style) | ✅ |
| Query Language | ✅ (planned) | ✅ |
| Editor Support | ✅ (planned) | ✅ |
| Runtime Parsers | ✅ (9 languages) | ❌ |
| Semantic Actions | ✅ | ❌ |
| Grammar Ecosystem | ✅ (1000+ ANTLR4 grammars) | ~100 grammars |
| Single Source | ✅ (one grammar for all) | ❌ (separate grammars) |

## Migration Path

### For Users Currently Using Tree-sitter

1. **Keep existing ANTLR4 grammar** (if you have one)
2. **Generate minipg LSP server**: `minipg generate-lsp grammar.g4`
3. **Install editor extension**: `vscode-minipg`, `nvim-minipg`, etc.
4. **Configure highlights**: Use query language or auto-generate
5. **Test and tune**: Adjust queries for optimal highlighting

### For New Projects

1. **Write ANTLR4 grammar** (or use existing from grammars-v4)
2. **Generate runtime parser**: `minipg generate grammar.g4 -l rust`
3. **Generate LSP server**: `minipg generate-lsp grammar.g4`
4. **Install editor support**: One-time setup
5. **Enjoy**: Both runtime parsing and editor integration from one grammar

## Example: Complete Workflow

### 1. Create Grammar

```antlr4
grammar MyLanguage;

program: statement* EOF;

statement
    : assignment
    | expression
    ;

assignment: ID '=' expression ';';

expression
    : expression ('*' | '/') expression
    | expression ('+' | '-') expression
    | NUMBER
    | ID
    | '(' expression ')'
    ;

ID: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

### 2. Generate Everything

```bash
# Generate runtime parser (Rust)
minipg generate MyLanguage.g4 -l rust -o src/

# Generate LSP server
minipg generate-lsp MyLanguage.g4 -o lsp/

# Generate highlight queries (auto)
minipg generate-queries MyLanguage.g4 -o queries/
```

### 3. Install Editor Support

```bash
# VS Code
code --install-extension minipg-mylanguage

# Neovim
:Lazy install nvim-minipg
```

### 4. Use Everywhere

- **Runtime**: Use generated Rust parser in your application
- **Editor**: Get syntax highlighting, folding, symbols automatically
- **CI/CD**: Validate syntax in pipelines
- **Tools**: Build linters, formatters, analyzers

## Benefits Summary

### For Language Designers

✅ **One grammar, multiple uses**: Runtime + editor integration  
✅ **Proven ecosystem**: Use ANTLR4 grammars from grammars-v4  
✅ **Full features**: Actions, predicates, modes work everywhere  
✅ **Easy updates**: Change grammar once, regenerate all  

### For Tool Builders

✅ **Consistent parsing**: Same parser in runtime and editor  
✅ **Multiple languages**: Generate for Rust, Python, JS, etc.  
✅ **No conversion**: No ANTLR4 → Tree-sitter translation  
✅ **Better debugging**: Same grammar, same behavior  

### For Editor Users

✅ **More languages**: Access to 1000+ ANTLR4 grammars  
✅ **Better accuracy**: Grammar matches runtime parser  
✅ **Faster updates**: Language updates propagate immediately  
✅ **Richer features**: Semantic actions enable smarter highlighting  

## Next Steps

### Immediate (v0.2.0)

1. Add position tracking to generated parsers
2. Implement basic incremental parsing
3. Create proof-of-concept LSP server
4. Test with Calculator grammar

### Short-term (v0.3.0)

1. Implement query language
2. Add semantic tokens support
3. Create VS Code extension
4. Benchmark performance

### Long-term (v1.0.0)

1. Full LSP implementation
2. Multi-editor support
3. Query language documentation
4. Migration guides from Tree-sitter

## Conclusion

By implementing incremental parsing, a query language, and LSP support, minipg can **completely replace Tree-sitter** while offering significant advantages:

- **Single source of truth**: One ANTLR4 grammar for everything
- **Larger ecosystem**: 1000+ existing grammars
- **More features**: Full ANTLR4 capabilities
- **Better consistency**: Same parser everywhere

This positions minipg as the **universal parser generator** for both runtime and editor integration.
