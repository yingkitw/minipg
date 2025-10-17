# Grammar Syntax Reference

## Overview

minipg uses a syntax similar to ANTLR4 for defining grammars. This document provides a complete reference for the grammar syntax.

## Grammar Structure

```
grammar <type> <name>;

[options { ... }]
[import ...]

<rules>
```

### Grammar Declaration

```
grammar parser MyGrammar;
grammar lexer MyLexer;
grammar MyGrammar;  // defaults to combined
```

## Rules

### Parser Rules

Parser rules define the structure of the language. They start with a lowercase letter.

```
ruleName: alternative1 | alternative2 | ... ;
```

Example:
```
expr: term (('+' | '-') term)*;
```

### Lexer Rules

Lexer rules define tokens. They start with an uppercase letter.

```
TOKEN_NAME: pattern;
```

Example:
```
NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
```

### Fragment Rules

Fragment rules are reusable components that don't produce tokens themselves.

```
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
```

## Alternatives

Alternatives are separated by `|`:

```
value: NUMBER | STRING | BOOLEAN;
```

## Elements

### Rule References

Reference other rules by name:

```
expr: term;
term: factor;
```

### String Literals

Single or double quotes:

```
'+' | '-' | '*' | '/'
"if" | "then" | "else"
```

### Character Ranges

Define ranges of characters (in lexer rules):

```
LETTER: 'a' | 'b' | 'c' | 'd' | 'e';  // explicit
```

## Operators

### Optional (?)

Makes an element optional:

```
returnStatement: 'return' expr? ';';
```

### Zero or More (*)

Matches zero or more occurrences:

```
block: '{' statement* '}';
```

### One or More (+)

Matches one or more occurrences:

```
NUMBER: DIGIT+;
```

### Grouping (())

Groups elements together:

```
expr: term (('+'  | '-') term)*;
```

### Negation (~)

Matches anything except the specified element:

```
CHAR: ~["\\\r\n];  // any character except ", \, \r, \n
```

### Wildcard (.)

Matches any single character:

```
ANY: .;
```

## Labels

Labels allow you to name elements for easier access in code:

```
assignment: id=IDENTIFIER '=' value=expr ';';
```

## Options

Grammar-level options:

```
options {
    language = rust;
    tokenVocab = CommonLexer;
}
```

## Imports

Import rules from other grammars:

```
import CommonRules;
import LexerRules, ParserRules;
```

## Comments

### Line Comments

```
// This is a line comment
```

### Block Comments

```
/*
 * This is a block comment
 * spanning multiple lines
 */
```

## Precedence and Associativity

### Left Recursion

Direct left recursion is detected and warned about:

```
// Warning: direct left recursion
expr: expr '+' term | term;
```

Rewrite using repetition:

```
expr: term ('+' term)*;
```

### Operator Precedence

Define precedence through rule hierarchy:

```
expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';
```

## Best Practices

### Rule Naming

- **Parser rules**: lowercase, descriptive names
  - `expression`, `statement`, `declaration`
- **Lexer rules**: UPPERCASE, descriptive names
  - `IDENTIFIER`, `NUMBER`, `STRING`
- **Fragment rules**: UPPERCASE with `fragment` keyword
  - `fragment DIGIT`, `fragment LETTER`

### Rule Organization

1. Grammar declaration
2. Options (if any)
3. Imports (if any)
4. Parser rules (top-down, most general first)
5. Lexer rules (most specific first)
6. Fragment rules (at the end)

Example:
```
grammar parser Calculator;

options {
    language = rust;
}

// Parser rules
expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

// Lexer rules
NUMBER: DIGIT+;
WS: SPACE+;

// Fragment rules
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
fragment SPACE: ' ' | '\t' | '\r' | '\n';
```

### Avoiding Common Pitfalls

#### 1. Left Recursion

❌ Don't:
```
expr: expr '+' term;
```

✅ Do:
```
expr: term ('+' term)*;
```

#### 2. Ambiguous Alternatives

❌ Don't:
```
statement: 'if' expr statement
         | 'if' expr statement 'else' statement;
```

✅ Do:
```
statement: 'if' expr statement ('else' statement)?;
```

#### 3. Unreachable Rules

❌ Don't define rules that are never used:
```
unusedRule: 'x' 'y' 'z';  // Warning: unreachable
```

✅ Do: Remove or use the rule

## Complete Example

```
grammar parser JSON;

// Parser rules
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

// Lexer rules
STRING: '"' CHAR* '"';
NUMBER: '-'? DIGIT+ ('.' DIGIT+)?;
WS: SPACE+ -> skip;

// Fragment rules
fragment CHAR: ~["\\\r\n];
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
fragment SPACE: ' ' | '\t' | '\r' | '\n';
```

## See Also

- [User Guide](USER_GUIDE.md)
- [API Documentation](API.md)
- [Examples](../examples/)
