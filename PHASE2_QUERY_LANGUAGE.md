# Phase 2: Query Language Implementation

## Status: ✅ Complete (v0.1.6)

## What Was Implemented

### 1. Pattern Representation (`src/query/pattern.rs`)

**Pattern** - A complete query pattern:
```rust
pub struct Pattern {
    pub root: PatternNode,
    pub captures: Vec<String>,
}
```

**PatternNode** - Node in pattern tree:
```rust
pub struct PatternNode {
    pub node_type: String,
    pub field: Option<String>,
    pub capture: Option<String>,
    pub children: Vec<PatternNode>,
    pub is_wildcard: bool,
    pub is_negated: bool,
}
```

**Features**:
- ✅ Node type matching
- ✅ Field matching (`name:`)
- ✅ Capture groups (`@name`)
- ✅ Wildcard matching (`_`)
- ✅ Nested patterns
- ✅ Automatic capture extraction

### 2. Query Parser (`src/query/parser.rs`)

**QueryParser** - Parses S-expression queries:
```rust
pub struct QueryParser {
    source: String,
    pos: usize,
}
```

**Supported Syntax**:
```scheme
; Simple pattern
(identifier) @variable

; Pattern with field
(function name: (identifier))

; Pattern with capture
(function name: (identifier)) @function.decl

; Wildcard
(_) @any

; Multiple patterns
(identifier) @variable
(number) @number
(string) @string

; Comments
; This is a comment
(identifier) @var  ; inline comment
```

**Features**:
- ✅ S-expression parsing
- ✅ Field syntax (`name:`)
- ✅ Capture syntax (`@name`)
- ✅ Comment support (`;`)
- ✅ Whitespace handling
- ✅ Multiple patterns per query

### 3. Capture Groups (`src/query/capture.rs`)

**CaptureGroup** - Represents a captured node:
```rust
pub struct CaptureGroup {
    pub name: String,
    pub range: Range,
    pub text: String,
}
```

**Features**:
- ✅ Named captures
- ✅ Position tracking
- ✅ Text extraction

### 4. Pattern Matcher (`src/query/matcher.rs`)

**QueryMatcher** - Matches patterns against AST:
```rust
pub struct QueryMatcher {
    patterns: Vec<Pattern>,
}
```

**Match** - Result of pattern matching:
```rust
pub struct Match {
    pub pattern_index: usize,
    pub captures: Vec<Capture>,
}
```

**Features**:
- ✅ Pattern matching against Grammar AST
- ✅ Rule name matching
- ✅ Element matching
- ✅ Wildcard support
- ✅ Capture extraction

### 5. Integration

**Module Structure**:
```
src/query/
├── mod.rs           # Module exports
├── pattern.rs       # Pattern representation
├── parser.rs        # Query parser
├── capture.rs       # Capture groups
└── matcher.rs       # Pattern matching
```

**Public API** (exported from `lib.rs`):
```rust
pub use query::{
    QueryParser,
    Pattern, PatternNode,
    QueryMatcher,
    CaptureGroup
};
```

## Test Results

**16 tests passing** (100% pass rate):

### Pattern Tests (4 tests)
- ✅ Pattern node creation
- ✅ Pattern with capture
- ✅ Pattern with field
- ✅ Wildcard node
- ✅ Capture extraction

### Parser Tests (7 tests)
- ✅ Simple pattern parsing
- ✅ Pattern with capture
- ✅ Pattern with field
- ✅ Pattern with field and capture
- ✅ Multiple patterns
- ✅ Comments
- ✅ Wildcard

### Matcher Tests (3 tests)
- ✅ Matcher creation
- ✅ Rule name matching
- ✅ Wildcard matching

### Capture Tests (1 test)
- ✅ Capture group creation

### Total: 147 tests (131 existing + 16 new)

## Usage Example

```rust
use minipg::query::{QueryParser, QueryMatcher};
use minipg::parser::GrammarParser;

// Parse query
let query_source = r#"
    ; Match function declarations
    (function name: (identifier)) @function.name
    
    ; Match string literals
    (string) @string
    
    ; Match any identifier
    (identifier) @variable
"#;

let mut query_parser = QueryParser::new(query_source.to_string());
let patterns = query_parser.parse().unwrap();

// Parse grammar
let grammar_parser = GrammarParser::new();
let grammar = grammar_parser.parse_file("grammar.g4").unwrap();

// Find matches
let matcher = QueryMatcher::new(patterns);
let matches = matcher.find_matches(&grammar, source);

// Process captures
for m in matches {
    for capture in m.captures {
        println!("{}: {}", capture.group.name, capture.group.text);
    }
}
```

## Example Highlight Query

Created `queries/highlights.scm`:

```scheme
; Keywords
["grammar" "lexer" "parser" "fragment"] @keyword

; Rule definitions
(rule name: (identifier) @function)

; String literals
(string_literal) @string

; Comments
(line_comment) @comment
(block_comment) @comment

; Operators
["|" "*" "+" "?"] @operator
```

## Query Language Syntax

### Basic Pattern
```scheme
(node_type)
```

### Pattern with Capture
```scheme
(node_type) @capture.name
```

### Pattern with Field
```scheme
(parent field: (child))
```

### Pattern with Field and Capture
```scheme
(parent field: (child)) @capture
```

### Wildcard
```scheme
(_)  ; matches any node
```

### Multiple Patterns
```scheme
(pattern1) @cap1
(pattern2) @cap2
(pattern3) @cap3
```

### Comments
```scheme
; This is a comment
(pattern) @capture  ; inline comment
```

## Comparison with Tree-sitter

| Feature | minipg (Phase 2) | Tree-sitter |
|---------|------------------|-------------|
| S-expression syntax | ✅ | ✅ |
| Capture groups | ✅ | ✅ |
| Field matching | ✅ | ✅ |
| Wildcard | ✅ | ✅ |
| Comments | ✅ | ✅ |
| Predicates | ⏳ (future) | ✅ |
| Quantifiers in queries | ⏳ (future) | ✅ |
| Anchors | ⏳ (future) | ✅ |

**Current**: Basic query language complete  
**Next**: Add advanced features (predicates, quantifiers, anchors)

## Files Created

1. `src/query/mod.rs` - Module definition
2. `src/query/pattern.rs` - Pattern representation (140 lines)
3. `src/query/parser.rs` - Query parser (280 lines)
4. `src/query/capture.rs` - Capture groups (30 lines)
5. `src/query/matcher.rs` - Pattern matching (210 lines)
6. `queries/highlights.scm` - Example highlight query

**Total**: ~660 lines of new code + 16 tests

## Integration with Editors

The query language enables:

1. **Syntax Highlighting**: Match patterns and assign colors
2. **Code Folding**: Identify foldable regions
3. **Symbols**: Extract document symbols
4. **Navigation**: Find definitions and references
5. **Semantic Analysis**: Custom pattern-based analysis

## Next Steps (Phase 3: LSP Server)

With incremental parsing (Phase 1) and query language (Phase 2) complete, we can now build:

1. **LSP Server**: Implement Language Server Protocol
2. **Semantic Tokens**: Use queries for syntax highlighting
3. **Document Symbols**: Extract symbols using queries
4. **Folding Ranges**: Identify foldable regions
5. **Diagnostics**: Report syntax errors

## Performance

Query parsing and matching is designed to be fast:

- **Query Parsing**: O(n) where n is query length
- **Pattern Matching**: O(m*p) where m is AST size, p is pattern count
- **Memory**: Minimal - patterns are lightweight

## Conclusion

Phase 2 is **complete** ✅:

- ✅ S-expression query syntax
- ✅ Query parser with full syntax support
- ✅ Pattern matching engine
- ✅ Capture groups
- ✅ Example highlight queries
- ✅ 16 tests passing (100%)
- ✅ Integrated into minipg library

**Ready for Phase 3**: LSP Server implementation

---

**Status**: Complete  
**Version**: 0.1.6  
**Tests**: 16/16 passing  
**Total Tests**: 147/147 passing  
**Next**: Phase 3 - Language Server Protocol
