# Multi-Language Target Support Plan

## Overview

minipg will generate **standalone, optimized code** for multiple target languages from ANTLR4-compatible grammars.

## Target Languages

### Priority 1: Core Languages (0-3 months)

#### 1. Rust ⭐ (Primary)
**Status**: In progress
**Priority**: Highest
**Features**:
- Generate standalone `.rs` files
- Zero dependencies
- Idiomatic Rust (Result, Option, iterators)
- Pattern matching for parsing
- Inline DFA as match statements
- Const lookup tables
- Full error handling with custom types

**Example Output**:
```rust
pub struct JsonParser {
    input: &'input str,
    pos: usize,
}

impl JsonParser {
    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        self.parse_value()
    }
    
    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        match self.peek()? {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string(),
            // ... optimized inline
        }
    }
}
```

#### 2. Python 🐍
**Status**: Planned
**Priority**: High
**Features**:
- Generate standalone `.py` files
- Type hints (Python 3.10+)
- Dataclasses for AST nodes
- No external dependencies
- PEP 8 compliant

**Example Output**:
```python
from dataclasses import dataclass
from typing import Optional, List, Union

@dataclass
class JsonValue:
    pass

class JsonParser:
    def __init__(self, input: str):
        self.input = input
        self.pos = 0
    
    def parse(self) -> JsonValue:
        return self.parse_value()
    
    def parse_value(self) -> JsonValue:
        ch = self.peek()
        if ch == '{':
            return self.parse_object()
        elif ch == '[':
            return self.parse_array()
        # ... optimized inline
```

#### 3. JavaScript 📜
**Status**: Planned
**Priority**: High
**Features**:
- Generate standalone `.js` files (ES6+)
- Modern JavaScript (classes, arrow functions)
- No dependencies
- Works in Node.js and browsers

**Example Output**:
```javascript
class JsonParser {
    constructor(input) {
        this.input = input;
        this.pos = 0;
    }
    
    parse() {
        return this.parseValue();
    }
    
    parseValue() {
        const ch = this.peek();
        switch (ch) {
            case '{': return this.parseObject();
            case '[': return this.parseArray();
            case '"': return this.parseString();
            // ... optimized inline
        }
    }
}

module.exports = { JsonParser };
```

#### 4. TypeScript 📘
**Status**: Planned
**Priority**: High
**Features**:
- Generate standalone `.ts` files
- Full type safety
- Interfaces for AST nodes
- No dependencies

**Example Output**:
```typescript
interface JsonValue {
    type: 'object' | 'array' | 'string' | 'number' | 'boolean' | 'null';
}

class JsonParser {
    private input: string;
    private pos: number = 0;
    
    constructor(input: string) {
        this.input = input;
    }
    
    parse(): JsonValue {
        return this.parseValue();
    }
    
    private parseValue(): JsonValue {
        const ch = this.peek();
        switch (ch) {
            case '{': return this.parseObject();
            case '[': return this.parseArray();
            // ... optimized inline
        }
    }
}
```

### Priority 2: Systems Languages (3-6 months)

#### 5. Go 🐹
**Status**: Planned
**Priority**: Medium
**Features**:
- Generate standalone `.go` files
- Idiomatic Go (errors, interfaces)
- No external dependencies
- Package structure

**Example Output**:
```go
package parser

type JsonValue interface {
    isJsonValue()
}

type JsonParser struct {
    input string
    pos   int
}

func NewJsonParser(input string) *JsonParser {
    return &JsonParser{input: input, pos: 0}
}

func (p *JsonParser) Parse() (JsonValue, error) {
    return p.parseValue()
}

func (p *JsonParser) parseValue() (JsonValue, error) {
    ch, err := p.peek()
    if err != nil {
        return nil, err
    }
    switch ch {
    case '{':
        return p.parseObject()
    case '[':
        return p.parseArray()
    // ... optimized inline
    }
}
```

#### 6. C 🔧
**Status**: Planned
**Priority**: Medium
**Features**:
- Generate standalone `.c` and `.h` files
- No dependencies (not even libc for parsing)
- Manual memory management helpers
- Struct-based AST

**Example Output**:
```c
// json_parser.h
#ifndef JSON_PARSER_H
#define JSON_PARSER_H

typedef struct JsonValue JsonValue;
typedef struct JsonParser JsonParser;

JsonParser* json_parser_new(const char* input);
JsonValue* json_parser_parse(JsonParser* parser);
void json_parser_free(JsonParser* parser);
void json_value_free(JsonValue* value);

#endif

// json_parser.c
#include "json_parser.h"
#include <stdlib.h>

struct JsonParser {
    const char* input;
    size_t pos;
};

JsonParser* json_parser_new(const char* input) {
    JsonParser* p = malloc(sizeof(JsonParser));
    p->input = input;
    p->pos = 0;
    return p;
}

JsonValue* json_parser_parse(JsonParser* p) {
    return parse_value(p);
}
```

#### 7. C++ 🔨
**Status**: Planned
**Priority**: Medium
**Features**:
- Generate standalone `.cpp` and `.hpp` files
- Modern C++ (C++17+)
- RAII, smart pointers
- Template-based AST
- No external dependencies

**Example Output**:
```cpp
// json_parser.hpp
#pragma once
#include <string>
#include <memory>
#include <variant>

class JsonValue {
public:
    using Value = std::variant<
        std::nullptr_t,
        bool,
        double,
        std::string,
        std::vector<JsonValue>,
        std::map<std::string, JsonValue>
    >;
    
    Value value;
};

class JsonParser {
public:
    explicit JsonParser(std::string_view input);
    JsonValue parse();
    
private:
    std::string_view input_;
    size_t pos_ = 0;
    
    JsonValue parseValue();
    JsonValue parseObject();
    JsonValue parseArray();
};
```

### Priority 3: Enterprise Languages (6-12 months)

#### 8. Java ☕
**Status**: Planned
**Priority**: Low
**Features**:
- Generate standalone `.java` files
- No external dependencies
- Interfaces for AST nodes
- Proper package structure

**Example Output**:
```java
package com.example.parser;

public interface JsonValue {
    // Marker interface
}

public class JsonParser {
    private final String input;
    private int pos = 0;
    
    public JsonParser(String input) {
        this.input = input;
    }
    
    public JsonValue parse() throws ParseException {
        return parseValue();
    }
    
    private JsonValue parseValue() throws ParseException {
        char ch = peek();
        switch (ch) {
            case '{': return parseObject();
            case '[': return parseArray();
            case '"': return parseString();
            // ... optimized inline
        }
    }
}
```

## Code Generation Architecture

### Template Structure

```
templates/
├── rust/
│   ├── parser.rs.template
│   ├── lexer.rs.template
│   ├── ast.rs.template
│   └── visitor.rs.template
├── python/
│   ├── parser.py.template
│   ├── lexer.py.template
│   └── ast.py.template
├── javascript/
│   ├── parser.js.template
│   └── lexer.js.template
├── typescript/
│   ├── parser.ts.template
│   └── types.ts.template
├── go/
│   ├── parser.go.template
│   └── ast.go.template
├── c/
│   ├── parser.h.template
│   └── parser.c.template
├── cpp/
│   ├── parser.hpp.template
│   └── parser.cpp.template
└── java/
    ├── Parser.java.template
    └── Ast.java.template
```

### Code Generator Trait

```rust
pub trait CodeGenerator {
    fn target_language(&self) -> &str;
    fn file_extension(&self) -> &str;
    fn generate_parser(&self, grammar: &Grammar) -> Result<String>;
    fn generate_lexer(&self, grammar: &Grammar) -> Result<String>;
    fn generate_ast(&self, grammar: &Grammar) -> Result<String>;
    fn generate_visitor(&self, grammar: &Grammar) -> Option<String>;
}

// Implementations
pub struct RustCodeGenerator;
pub struct PythonCodeGenerator;
pub struct JavaScriptCodeGenerator;
pub struct TypeScriptCodeGenerator;
pub struct GoCodeGenerator;
pub struct CCodeGenerator;
pub struct CppCodeGenerator;
pub struct JavaCodeGenerator;
```

## Optimization Strategies by Language

### Rust
- Inline DFA as match statements
- Const lookup tables
- Zero-copy parsing with lifetimes
- Enum for AST variants

### Python
- Dictionary-based dispatch
- List comprehensions
- Generator expressions for streaming
- Slots for AST classes

### JavaScript/TypeScript
- Switch statements for DFA
- Object literals for lookup
- Typed arrays for character classes
- Class-based AST

### Go
- Switch statements for DFA
- Map-based lookup
- Interface-based AST
- Error handling with multiple returns

### C
- Switch statements for DFA
- Static arrays for lookup
- Struct-based AST
- Manual memory management

### C++
- Template-based DFA
- Constexpr lookup tables
- Variant-based AST
- RAII for memory

### Java
- Switch expressions (Java 14+)
- HashMap for lookup
- Interface-based AST
- Exception handling

## ANTLR4 Compatibility

### Grammar Features to Support

1. **Basic Syntax** ✅
   - Parser rules (lowercase)
   - Lexer rules (uppercase)
   - Alternatives (`|`)
   - Sequences
   - Quantifiers (`*`, `+`, `?`)

2. **Advanced Syntax** 🔄
   - Labels (`label=rule`)
   - Rule arguments (`rule[int x]`)
   - Return values (`returns [Type value]`)
   - Local variables (`locals [Type var]`)
   - Actions (`{...}`)
   - Semantic predicates (`{...}?`)

3. **Lexer Features** 🔄
   - Modes (`mode NAME`)
   - Channels (`-> channel(NAME)`)
   - Commands (`-> skip`, `-> more`)
   - Fragment rules

4. **Grammar Options** 🔄
   - `tokenVocab`
   - `superClass`
   - `language` (ignored, we generate for all)

5. **Grammar Structure** 📝
   - Grammar imports (`import X;`)
   - Grammar inheritance
   - Multiple grammar files

## Implementation Phases

### Phase 1: Foundation (Month 1-2)
- [ ] Complete Rust standalone generation
- [ ] Optimize Rust output (DFA, lookup tables)
- [ ] Add Python standalone generation
- [ ] Add JavaScript standalone generation

### Phase 2: Expansion (Month 3-4)
- [ ] Add TypeScript standalone generation
- [ ] Add Go standalone generation
- [ ] Improve ANTLR4 compatibility (actions, predicates)
- [ ] Add complex grammar examples

### Phase 3: Systems (Month 5-6)
- [ ] Add C standalone generation
- [ ] Add C++ standalone generation
- [ ] Full ANTLR4 grammar compatibility
- [ ] Grammar imports and inheritance

### Phase 4: Enterprise (Month 7-12)
- [ ] Add Java standalone generation
- [ ] ANTLR4 test suite compatibility
- [ ] Performance optimization across all targets
- [ ] Production-ready release

## Testing Strategy

### Per-Language Testing
Each target language needs:
1. **Unit tests** - Test generated code compiles
2. **Integration tests** - Test generated parser works
3. **Snapshot tests** - Verify generated code structure
4. **Performance tests** - Benchmark parsing speed

### Cross-Language Testing
- Same grammar generates working parsers in all languages
- Consistent AST structure across languages
- Equivalent error handling

### ANTLR4 Compatibility Testing
- Parse official ANTLR4 grammars
- Generate from ANTLR4 grammar repository
- Pass ANTLR4 test suite

## Success Metrics

1. **Completeness**: All 8 languages supported
2. **Compatibility**: 100% ANTLR4 grammar support
3. **Performance**: Within 2x of hand-written parsers
4. **Quality**: Generated code passes language linters
5. **Usability**: Zero dependencies for all targets

## Documentation Per Language

Each language needs:
- Getting started guide
- Generated code walkthrough
- Customization guide
- Performance tips
- Example projects
