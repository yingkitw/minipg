# Next Phase: Advanced Features & Language Support

**Status**: Post-Alpha (v0.1.0-alpha.1 published)  
**Timeline**: Next 2-3 months  
**Focus**: Advanced ANTLR4 syntax + Go/C/C++ targets

---

## Phase 1: Advanced ANTLR4 Syntax Support (Weeks 1-3)

### Priority 1: Character Class Improvements
**Current Issue**: Parser doesn't support negated character classes `~[...]` and complex ranges

**Tasks**:
- [ ] Extend lexer to handle `~` operator in character classes
- [ ] Support Unicode escapes: `\uXXXX`, `\u{XXXXXX}`
- [ ] Support character ranges with escapes: `[a-z\u0000-\u001F]`
- [ ] Add tests for all character class variants
- [ ] Update parser to build correct AST for negated classes

**Files to modify**:
- `src/parser/lexer.rs` - Add character class tokenization
- `src/parser/parser.rs` - Parse negated character classes
- `src/ast/element.rs` - Add `NegatedCharClass` variant
- `src/codegen/*.rs` - Generate code for negated classes

**Estimated**: 1 week

### Priority 2: Rule Arguments & Return Values
**Goal**: Support parameterized rules like ANTLR4

**Tasks**:
- [ ] Parse rule arguments: `rule[int x, String name]`
- [ ] Parse return values: `returns [Type value]`
- [ ] Parse local variables: `locals [Type var]`
- [ ] Extend AST Rule struct with these fields
- [ ] Generate code for Rust (use generics/traits)
- [ ] Generate code for Python (use type hints)
- [ ] Generate code for JS/TS (use TypeScript types)
- [ ] Add comprehensive tests

**Files to modify**:
- `src/parser/parser.rs` - Parse rule signatures
- `src/ast/rule.rs` - Add fields for args/returns/locals
- `src/codegen/rust.rs` - Generate parameterized functions
- `src/codegen/python.rs` - Generate typed functions
- `src/codegen/typescript.rs` - Generate typed functions

**Estimated**: 1.5 weeks

### Priority 3: Lexer Modes & Channels
**Goal**: Support advanced lexer features

**Tasks**:
- [ ] Parse `mode NAME;` declarations
- [ ] Parse `-> channel(NAME)` commands
- [ ] Parse `-> more`, `-> skip`, `-> type(TYPE)` commands
- [ ] Parse `pushMode(NAME)`, `popMode`, `mode(NAME)` actions
- [ ] Extend AST to represent modes and channels
- [ ] Generate mode-aware lexers for all targets
- [ ] Add tests with multi-mode grammars

**Files to modify**:
- `src/parser/lexer.rs` - Add mode/channel keywords
- `src/parser/parser.rs` - Parse mode declarations
- `src/ast/grammar.rs` - Add modes field
- `src/ast/element.rs` - Add channel/mode commands
- `src/codegen/*.rs` - Generate mode-switching logic

**Estimated**: 1 week

---

## Phase 2: Go Target Language (Weeks 4-5)

### Go Code Generator
**Goal**: Generate idiomatic Go parsers

**Tasks**:
- [ ] Create `src/codegen/go.rs` module
- [ ] Implement `GoCodeGenerator` struct
- [ ] Generate Go lexer with proper error handling
- [ ] Generate Go parser with interfaces
- [ ] Generate visitor pattern (interfaces)
- [ ] Generate listener pattern (interfaces)
- [ ] Use Go naming conventions (PascalCase for exports)
- [ ] Proper package structure
- [ ] Error handling with Go idioms
- [ ] Add comprehensive tests

**Code Structure**:
```go
// Generated lexer
type CalculatorLexer struct {
    input string
    pos   int
    line  int
    col   int
}

func NewCalculatorLexer(input string) *CalculatorLexer {
    return &CalculatorLexer{input: input, pos: 0, line: 1, col: 1}
}

func (l *CalculatorLexer) NextToken() (*Token, error) {
    // ... implementation
}

// Generated parser
type CalculatorParser struct {
    lexer *CalculatorLexer
}

// Visitor interface
type CalculatorVisitor interface {
    VisitExpr(ctx *ExprContext) interface{}
    VisitTerm(ctx *TermContext) interface{}
}
```

**Files to create**:
- `src/codegen/go.rs` - Main Go generator
- `tests/go_codegen_tests.rs` - Go generation tests

**Estimated**: 1.5 weeks

---

## Phase 3: C Target Language (Weeks 6-7)

### C Code Generator
**Goal**: Generate portable C parsers

**Tasks**:
- [ ] Create `src/codegen/c.rs` module
- [ ] Implement `CCodeGenerator` struct
- [ ] Generate `.h` header file with declarations
- [ ] Generate `.c` implementation file
- [ ] Manual memory management (malloc/free)
- [ ] Error handling with return codes
- [ ] String handling utilities
- [ ] Visitor pattern with function pointers
- [ ] Listener pattern with callbacks
- [ ] Add comprehensive tests

**Code Structure**:
```c
// calculator_lexer.h
typedef struct Token {
    TokenKind kind;
    char* text;
    int line;
    int column;
} Token;

typedef struct CalculatorLexer {
    const char* input;
    size_t pos;
    int line;
    int column;
} CalculatorLexer;

CalculatorLexer* calculator_lexer_new(const char* input);
Token* calculator_lexer_next_token(CalculatorLexer* lexer);
void calculator_lexer_free(CalculatorLexer* lexer);
void token_free(Token* token);

// Visitor pattern with function pointers
typedef struct CalculatorVisitor {
    void* (*visit_expr)(struct ExprContext* ctx, void* data);
    void* (*visit_term)(struct TermContext* ctx, void* data);
} CalculatorVisitor;
```

**Files to create**:
- `src/codegen/c.rs` - Main C generator
- `tests/c_codegen_tests.rs` - C generation tests

**Estimated**: 1.5 weeks

---

## Phase 4: C++ Target Language (Weeks 8-9)

### C++ Code Generator
**Goal**: Generate modern C++ parsers

**Tasks**:
- [ ] Create `src/codegen/cpp.rs` module
- [ ] Implement `CppCodeGenerator` struct
- [ ] Generate `.hpp` header file
- [ ] Generate `.cpp` implementation file
- [ ] Use C++17 features (std::optional, std::variant)
- [ ] RAII for resource management
- [ ] Smart pointers (std::unique_ptr, std::shared_ptr)
- [ ] Exception-based error handling
- [ ] Visitor pattern with virtual functions
- [ ] Listener pattern with virtual functions
- [ ] Add comprehensive tests

**Code Structure**:
```cpp
// calculator_lexer.hpp
#pragma once
#include <string>
#include <memory>
#include <optional>
#include <variant>

enum class TokenKind {
    Number,
    Plus,
    Minus,
    // ...
};

struct Token {
    TokenKind kind;
    std::string text;
    int line;
    int column;
};

class CalculatorLexer {
public:
    explicit CalculatorLexer(std::string input);
    std::optional<Token> nextToken();
    std::vector<Token> tokenizeAll();
    
private:
    std::string input_;
    size_t pos_;
    int line_;
    int column_;
};

// Visitor pattern with virtual functions
class CalculatorVisitor {
public:
    virtual ~CalculatorVisitor() = default;
    virtual std::any visitExpr(ExprContext* ctx) = 0;
    virtual std::any visitTerm(TermContext* ctx) = 0;
};
```

**Files to create**:
- `src/codegen/cpp.rs` - Main C++ generator
- `tests/cpp_codegen_tests.rs` - C++ generation tests

**Estimated**: 1.5 weeks

---

## Testing Strategy

### Unit Tests
- [ ] Character class parsing tests (20+ test cases)
- [ ] Rule arguments/returns parsing tests
- [ ] Lexer modes parsing tests
- [ ] Go code generation tests
- [ ] C code generation tests
- [ ] C++ code generation tests

### Integration Tests
- [ ] Test grammars with negated character classes
- [ ] Test grammars with parameterized rules
- [ ] Test grammars with lexer modes
- [ ] Test CompleteJSON.g4 with all new targets
- [ ] Test SQL.g4 with all new targets

### End-to-End Tests
- [ ] Generate Go parser and compile it
- [ ] Generate C parser and compile it
- [ ] Generate C++ parser and compile it
- [ ] Parse real-world inputs with generated parsers

**Target**: 100+ new tests

---

## Documentation Updates

- [ ] Update README with new language support
- [ ] Update ANTLR4_COMPATIBILITY.md with new features
- [ ] Create Go code generation guide
- [ ] Create C code generation guide
- [ ] Create C++ code generation guide
- [ ] Update examples with advanced features
- [ ] Create migration guide for advanced features

---

## Success Criteria

### Phase 1 Complete When:
- ✅ All character class variants parse correctly
- ✅ Rule arguments/returns work in all existing targets
- ✅ Lexer modes work in all existing targets
- ✅ 9 ignored tests now pass (CompleteJSON.g4, SQL.g4)
- ✅ 80+ tests passing total

### Phase 2 Complete When:
- ✅ Go code generator implemented
- ✅ Generated Go code compiles
- ✅ Calculator example works in Go
- ✅ 90+ tests passing

### Phase 3 Complete When:
- ✅ C code generator implemented
- ✅ Generated C code compiles (gcc/clang)
- ✅ Calculator example works in C
- ✅ 100+ tests passing

### Phase 4 Complete When:
- ✅ C++ code generator implemented
- ✅ Generated C++ code compiles (g++/clang++)
- ✅ Calculator example works in C++
- ✅ 110+ tests passing

---

## Timeline Summary

| Week | Focus | Deliverable |
|------|-------|-------------|
| 1 | Character classes | Negated classes working |
| 2-3 | Rule args & modes | Advanced syntax working |
| 4-5 | Go target | Go generator complete |
| 6-7 | C target | C generator complete |
| 8-9 | C++ target | C++ generator complete |

**Total**: 9 weeks to complete all phases

---

## Next Steps

1. Start with Phase 1, Priority 1 (Character classes)
2. Fix the 9 ignored tests
3. Move to rule arguments
4. Then tackle new language targets

**Ready to begin!** 🚀
