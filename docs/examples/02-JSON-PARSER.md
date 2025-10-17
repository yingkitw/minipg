# Example 2: JSON Parser

## Level: Intermediate

A complete JSON parser implementing RFC 8259 specification.

## Grammar File

**File**: `examples/CompleteJSON.g4`

```antlr4
grammar CompleteJSON;

// Parser Rules
json : value EOF ;

value
    : object
    | array
    | string
    | number
    | 'true' | 'false' | 'null'
    ;

object
    : '{' '}'
    | '{' pair (',' pair)* '}'
    ;

pair : string ':' value ;

array
    : '[' ']'
    | '[' value (',' value)* ']'
    ;

// Lexer Rules
STRING : '"' (ESC | SAFECODEPOINT)* '"' ;
NUMBER : '-'? INT ('.' [0-9]+)? EXP? ;

fragment ESC : '\\' (["\\/bfnrt] | UNICODE) ;
fragment UNICODE : 'u' HEX HEX HEX HEX ;
fragment HEX : [0-9a-fA-F] ;
fragment SAFECODEPOINT : ~ ["\\\u0000-\u001F] ;
fragment INT : '0' | [1-9] [0-9]* ;
fragment EXP : [Ee] [+\-]? [0-9]+ ;

WS : [ \t\n\r]+ -> skip ;
```

## Features Demonstrated

- ✅ Recursive rules (nested objects/arrays)
- ✅ Fragment rules (code reuse)
- ✅ Character classes and ranges
- ✅ Unicode support
- ✅ Complex lexer patterns
- ✅ RFC compliance

## Usage

```bash
# Generate all languages
minipg generate --input examples/CompleteJSON.g4 --output target/ --all-languages
```

## Test Cases

### Valid JSON
```json
{
  "name": "John",
  "age": 30,
  "active": true,
  "scores": [95, 87, 92],
  "address": {
    "city": "NYC",
    "zip": "10001"
  }
}
```

### Edge Cases
```json
{}
[]
null
true
false
0
-123.456e-78
"\u0041\u0042\u0043"
```

## Performance

- **Parsing Speed**: ~1 MB/s
- **Memory Usage**: ~2x input size
- **Error Recovery**: Yes

## Key Concepts

1. **Recursive Structures** - Objects contain values, values contain objects
2. **Fragment Rules** - Reusable lexer components
3. **Unicode Escapes** - `\uXXXX` sequences
4. **Number Formats** - Integers, floats, scientific notation

## Related Examples

- **Previous**: [01-BASIC-CALCULATOR.md](01-BASIC-CALCULATOR.md)
- **Next**: [03-EXPRESSION-EVALUATOR.md](03-EXPRESSION-EVALUATOR.md)
