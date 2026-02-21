# JavaScript Code Generation Guide

**minipg** generates standalone JavaScript parsers from ANTLR4 grammars with modern ES6+ features.

---

## Overview

The JavaScript code generator produces:
- **Lexer** - Tokenizes input with pattern matching
- **Parser** - Parses tokens into AST
- **Token Types** - Object with token kind constants
- **Error Types** - ParseError class
- **ES6+ Features** - Classes, const/let, arrow functions

---

## Generated Code Structure

```javascript
// Token types
const TokenKind = {
    IDENTIFIER: 'IDENTIFIER',
    NUMBER: 'NUMBER',
    PLUS: 'PLUS',
    // ... other tokens
};

// Token class
class Token {
    constructor(kind, value, position) {
        this.kind = kind;
        this.value = value;
        this.position = position;
    }
}

// Parse error
class ParseError extends Error {
    constructor(message, position) {
        super(`${message} at position ${position}`);
        this.message = message;
        this.position = position;
        this.name = 'ParseError';
    }
}

// Lexer
class Lexer {
    constructor(input) {
        this.input = input;
        this.position = 0;
    }
    
    nextToken() {
        // Tokenization logic
    }
    
    tokenizeAll() {
        const tokens = [];
        while (this.position < this.input.length) {
            tokens.push(this.nextToken());
        }
        return tokens;
    }
}

// Parser
class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
    }
    
    parseRule() {
        // Parsing logic
    }
}
```

---

## Features

### 1. ES6+ Syntax

Modern JavaScript features:

```javascript
class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
    }
    
    parseExpr(precedence = 0) {
        const term = this.parseTerm();
        if (!term) return null;
        
        return {
            type: 'Expr',
            term
        };
    }
    
    // Arrow functions
    hasTokens = () => this.position < this.tokens.length;
    
    // Destructuring
    parseOp() {
        const { kind, value } = this.currentToken();
        return { kind, value };
    }
}
```

### 2. Error Recovery

Exception-based error handling:

```javascript
tokenizeAll() {
    const tokens = [];
    while (this.position < this.input.length) {
        try {
            const token = this.nextToken();
            tokens.push(token);
            if (token.kind === TokenKind.EOF) {
                break;
            }
        } catch (e) {
            if (e instanceof ParseError) {
                console.error(`Lexer error: ${e.message}`);
                this.position++; // Skip invalid character
            } else {
                throw e;
            }
        }
    }
    return tokens;
}
```

### 3. Named Actions

Support for `@header` and `@members`:

```antlr
grammar Calculator;

@header {
    const util = require('util');
}

@members {
    symbolTable = new Map();
}

expr: term (('+' | '-') term)*;
```

Generates:

```javascript
// @header inserted at top
const util = require('util');

class CalculatorParser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
        // @members inserted here
        this.symbolTable = new Map();
    }
}
```

### 4. Template Literals

String interpolation for error messages:

```javascript
throw new ParseError(
    `Expected token: ${expectedKind}, got: ${actualKind}`,
    this.position
);
```

### 5. Spread Operator

Array and object spreading:

```javascript
class Parser {
    constructor(tokens, ...options) {
        this.tokens = tokens;
        this.options = { ...defaultOptions, ...options };
    }
    
    parseList() {
        return [...this.parseItems()];
    }
}
```

### 6. Lexer Modes

```antlr
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_CHAR: ~["\r\n];
STRING_END: '"' -> popMode;
```

Generates mode stack management:

```javascript
class Lexer {
    constructor(input) {
        this.input = input;
        this.position = 0;
        this.modeStack = ['DEFAULT'];
    }
    
    pushMode(mode) {
        this.modeStack.push(mode);
    }
    
    popMode() {
        if (this.modeStack.length > 1) {
            this.modeStack.pop();
        }
    }
    
    currentMode() {
        return this.modeStack[this.modeStack.length - 1];
    }
}
```

---

## Usage Example

### 1. Create Grammar

```antlr
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

### 2. Generate Parser

```bash
minipg generate Calculator.g4 -o src/ -l javascript
```

### 3. Use Generated Parser (Node.js)

```javascript
const { Lexer, Parser } = require('./calculator');

function main() {
    const input = '2 + 3 * 4';
    
    // Tokenize
    const lexer = new Lexer(input);
    const tokens = lexer.tokenizeAll();
    
    // Parse
    const parser = new Parser(tokens);
    try {
        const ast = parser.parseExpr();
        console.log('Parsed:', ast);
    } catch (e) {
        if (e instanceof ParseError) {
            console.error(`Parse error: ${e.message}`);
        } else {
            throw e;
        }
    }
}

main();
```

### 4. Use Generated Parser (Browser)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Calculator Parser</title>
</head>
<body>
    <input id="input" type="text" value="2 + 3 * 4" />
    <button onclick="parse()">Parse</button>
    <pre id="output"></pre>
    
    <script src="calculator.js"></script>
    <script>
        function parse() {
            const input = document.getElementById('input').value;
            const lexer = new Lexer(input);
            const tokens = lexer.tokenizeAll();
            const parser = new Parser(tokens);
            
            try {
                const ast = parser.parseExpr();
                document.getElementById('output').textContent = 
                    JSON.stringify(ast, null, 2);
            } catch (e) {
                document.getElementById('output').textContent = 
                    `Error: ${e.message}`;
            }
        }
    </script>
</body>
</html>
```

---

## Module Systems

### CommonJS (Node.js)

```javascript
// Export
module.exports = {
    Lexer,
    Parser,
    TokenKind,
    ParseError
};

// Import
const { Lexer, Parser } = require('./calculator');
```

### ES Modules

```javascript
// Export
export { Lexer, Parser, TokenKind, ParseError };

// Import
import { Lexer, Parser } from './calculator.js';
```

### UMD (Universal)

```javascript
(function (root, factory) {
    if (typeof define === 'function' && define.amd) {
        // AMD
        define([], factory);
    } else if (typeof module === 'object' && module.exports) {
        // Node
        module.exports = factory();
    } else {
        // Browser globals
        root.Calculator = factory();
    }
}(typeof self !== 'undefined' ? self : this, function () {
    // Parser code
    return { Lexer, Parser };
}));
```

---

## Best Practices

### 1. Use Const/Let

```javascript
const tokens = lexer.tokenizeAll();
let position = 0;

// Never use var
```

### 2. Handle Errors Gracefully

```javascript
try {
    const ast = parser.parseExpr();
    process(ast);
} catch (e) {
    if (e instanceof ParseError) {
        console.error(`Error at position ${e.position}: ${e.message}`);
        process.exit(1);
    }
    throw e;
}
```

### 3. Use JSDoc for Documentation

```javascript
/**
 * Parse an expression.
 * @param {number} precedence - Operator precedence
 * @returns {Expr|null} Parsed expression or null
 */
parseExpr(precedence = 0) {
    // ...
}
```

### 4. Validate Input

```javascript
constructor(tokens) {
    if (!Array.isArray(tokens)) {
        throw new TypeError('tokens must be an array');
    }
    this.tokens = tokens;
    this.position = 0;
}
```

---

## Advanced Features

### Custom AST Nodes

```javascript
class Expr {
    constructor(type, value) {
        this.type = type;
        this.value = value;
    }
}

class BinOp extends Expr {
    constructor(op, left, right) {
        super('BinOp', null);
        this.op = op;
        this.left = left;
        this.right = right;
    }
}
```

### Visitor Pattern

```javascript
class Visitor {
    visitExpr(expr) {
        throw new Error('visitExpr not implemented');
    }
    
    visitTerm(term) {
        throw new Error('visitTerm not implemented');
    }
}

class PrintVisitor extends Visitor {
    visitExpr(expr) {
        console.log('Expression:', expr);
    }
    
    visitTerm(term) {
        console.log('Term:', term);
    }
}
```

### Async Parsing

```javascript
class AsyncParser extends Parser {
    async parseExprAsync() {
        const term = await this.parseTermAsync();
        return { type: 'Expr', term };
    }
    
    async parseTermAsync() {
        // Async parsing logic
        return new Promise(resolve => {
            setTimeout(() => resolve(this.parseTerm()), 0);
        });
    }
}
```

---

## Performance Tips

### 1. Object Pooling

```javascript
class TokenPool {
    constructor() {
        this.pool = [];
    }
    
    acquire(kind, value, position) {
        const token = this.pool.pop() || new Token();
        token.kind = kind;
        token.value = value;
        token.position = position;
        return token;
    }
    
    release(token) {
        this.pool.push(token);
    }
}
```

### 2. String Building

```javascript
function readString() {
    const chars = [];
    while (this.currentChar() !== '"') {
        chars.push(this.currentChar());
        this.advance();
    }
    return chars.join('');
}
```

### 3. Memoization

```javascript
class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
        this.cache = new Map();
    }
    
    parseExpr() {
        const key = `expr:${this.position}`;
        if (this.cache.has(key)) {
            return this.cache.get(key);
        }
        
        const result = this._parseExpr();
        this.cache.set(key, result);
        return result;
    }
}
```

---

## Complete Example

```antlr
grammar JSON;

json: value;

value
    : object
    | array
    | STRING
    | NUMBER
    | 'true'
    | 'false'
    | 'null'
    ;

object: '{' pair (',' pair)* '}' | '{' '}';
pair: STRING ':' value;

array: '[' value (',' value)* ']' | '[' ']';

STRING: '"' (~["\r\n])* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
WS: [ \t\r\n]+ -> skip;
```

Generated usage:

```javascript
const { Lexer, Parser, ParseError } = require('./json-parser');

function parseJSON(input) {
    const lexer = new Lexer(input);
    const tokens = lexer.tokenizeAll();
    const parser = new Parser(tokens);
    return parser.parseJson();
}

function main() {
    const jsonText = '{"name": "minipg", "version": "0.2.0"}';
    try {
        const value = parseJSON(jsonText);
        console.log('Parsed:', value);
    } catch (e) {
        if (e instanceof ParseError) {
            console.error(`Error: ${e.message}`);
        } else {
            throw e;
        }
    }
}

main();
```

---

## Testing

### Jest

```javascript
const { Lexer, Parser } = require('./calculator');

describe('Calculator Parser', () => {
    test('parse number', () => {
        const lexer = new Lexer('42');
        const tokens = lexer.tokenizeAll();
        const parser = new Parser(tokens);
        const result = parser.parseFactor();
        expect(result).not.toBeNull();
    });
    
    test('parse expression', () => {
        const lexer = new Lexer('2 + 3');
        const tokens = lexer.tokenizeAll();
        const parser = new Parser(tokens);
        const result = parser.parseExpr();
        expect(result).not.toBeNull();
    });
});
```

### Mocha

```javascript
const assert = require('assert');
const { Lexer, Parser } = require('./calculator');

describe('Calculator', () => {
    it('should parse numbers', () => {
        const lexer = new Lexer('42');
        const tokens = lexer.tokenizeAll();
        const parser = new Parser(tokens);
        const result = parser.parseFactor();
        assert.notStrictEqual(result, null);
    });
});
```

---

## Troubleshooting

### Issue: Module Not Found

**Problem**: Cannot find module

**Solution**: Check module system:
```javascript
// CommonJS
const { Lexer } = require('./calculator');

// ES Modules
import { Lexer } from './calculator.js';
```

### Issue: Slow Parsing

**Problem**: Parser is slow in browser

**Solution**:
- Use Web Workers for parsing
- Implement streaming parsing
- Profile with Chrome DevTools

### Issue: Memory Leaks

**Problem**: High memory usage

**Solution**:
- Clear caches periodically
- Use WeakMap for caching
- Implement object pooling

---

## Summary

minipg's JavaScript code generator produces:
- ✅ **Modern** - ES6+ with classes and arrow functions
- ✅ **Standalone** - No runtime dependencies
- ✅ **Universal** - Works in Node.js and browsers
- ✅ **Idiomatic** - Follows JavaScript conventions
- ✅ **Complete** - Full ANTLR4 feature support

For more information, see:
- [User Guide](USER_GUIDE.md)
- [ANTLR4 Compatibility](ANTLR4_COMPATIBILITY.md)
- [Examples](../examples/)
