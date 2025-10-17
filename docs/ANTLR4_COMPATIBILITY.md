# ANTLR4 Grammar Compatibility

## Goal

**minipg will be fully compatible with ANTLR4 grammar syntax**, allowing users to use existing ANTLR4 grammars without modification.

## ANTLR4 Grammar Features

### ✅ Currently Supported

#### Basic Grammar Structure
- [x] Grammar declaration: `grammar Name;`
- [x] Parser rules (lowercase): `rule: ...;`
- [x] Lexer rules (uppercase): `TOKEN: ...;`
- [x] Comments: `// line` and `/* block */`

#### Rule Elements
- [x] Alternatives: `a | b | c`
- [x] Sequences: `a b c`
- [x] Optional: `a?`
- [x] Zero or more: `a*`
- [x] One or more: `a+`
- [x] Grouping: `(a | b)`
- [x] Character ranges: `[a-z]`, `[0-9]`
- [x] String literals: `'keyword'`, `"string"`
- [x] Negation: `~[abc]`

#### Lexer Features
- [x] Fragment rules: `fragment DIGIT: [0-9];`
- [x] Skip command: `WS: [ \t\r\n]+ -> skip;`

### 🔄 In Progress

#### Labels and References
- [ ] Rule labels: `expr: left=expr op='+' right=expr`
- [ ] Alternative labels: `expr: a=expr '+' b=expr # add`
- [ ] Token labels: `op=('+' | '-')`
- [ ] List labels: `ids+=ID (',' ids+=ID)*`

#### Actions and Predicates
- [ ] Embedded actions: `{...}`
- [ ] Semantic predicates: `{...}?`
- [ ] Validating predicates: `{...}?`

#### Rule Parameters and Returns
- [ ] Rule arguments: `rule[int x, String y]`
- [ ] Return values: `rule returns [Type value]`
- [ ] Local variables: `rule locals [Type var]`

#### Lexer Advanced Features
- [ ] Lexer modes: `mode NAME;`
- [ ] Lexer channels: `-> channel(NAME)`
- [ ] Lexer commands: `-> more`, `-> type(TYPE)`, `-> pushMode(MODE)`
- [ ] Lexer actions: `{...}`

### 📝 Planned

#### Grammar Options
```antlr
options {
    tokenVocab = CommonLexer;
    superClass = MyParser;
    language = Java;  // Ignored by minipg
}
```

#### Grammar Imports
```antlr
grammar Main;
import SubGrammar, AnotherGrammar;
```

#### Grammar Inheritance
```antlr
grammar Derived;
import Base;

// Override rules from Base
rule: ... ;
```

#### Named Actions
```antlr
@header {
    package com.example;
    import java.util.*;
}

@members {
    int count = 0;
}
```

#### Token Specifications
```antlr
tokens {
    INDENT, DEDENT
}
```

## ANTLR4 Grammar Syntax Reference

### Complete Grammar Structure

```antlr
/** Documentation comment */
grammar MyGrammar;

// Options
options {
    tokenVocab = MyLexer;
    superClass = MyBaseParser;
}

// Imports
import CommonRules, Utilities;

// Tokens
tokens {
    INDENT, DEDENT
}

// Named actions
@header {
    // Code for file header
}

@members {
    // Code for parser class members
}

// Parser rules
start
    : statement+ EOF
    ;

statement
    : assignment
    | expression
    ;

assignment
    : ID '=' expression  # assignExpr
    ;

expression
    : left=expression op=('*'|'/') right=expression  # mulDiv
    | left=expression op=('+'|'-') right=expression  # addSub
    | INT                                             # int
    | ID                                              # id
    | '(' expression ')'                              # parens
    ;

// Lexer rules
ID
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

INT
    : [0-9]+
    ;

WS
    : [ \t\r\n]+ -> skip
    ;

// Lexer modes
mode STRING_MODE;
STRING_CONTENT
    : ~["\\\r\n]+
    ;
```

### Rule Elements

#### Quantifiers
```antlr
optional: a?;           // Zero or one
zeroOrMore: a*;        // Zero or more
oneOrMore: a+;         // One or more
```

#### Subrules
```antlr
grouped: (a | b | c);
optional_group: (a b)?;
repeated_group: (a b)*;
```

#### Character Classes
```antlr
LETTER: [a-zA-Z];
DIGIT: [0-9];
NOT_QUOTE: ~["];
UNICODE: [\u0000-\uFFFF];
```

#### Wildcards
```antlr
ANY_CHAR: .;
```

#### Non-greedy Quantifiers
```antlr
COMMENT: '/*' .*? '*/';
```

### Labels

#### Element Labels
```antlr
assignment
    : var=ID '=' val=expression
    ;
```

#### List Labels
```antlr
idList
    : ids+=ID (',' ids+=ID)*
    ;
```

#### Alternative Labels
```antlr
expression
    : expression '*' expression  # multiply
    | expression '+' expression  # add
    | INT                        # integer
    | ID                         # identifier
    ;
```

### Actions

#### Embedded Actions
```antlr
rule
    : {System.out.println("Starting rule");} 
      a b c
      {System.out.println("Ending rule");}
    ;
```

#### Semantic Predicates
```antlr
rule
    : {isKeyword()}? ID
    | ID
    ;
```

### Rule Arguments and Returns

```antlr
add[int x] returns [int result]
locals [int temp]
    : a=INT '+' b=INT
      {
          $temp = $a.int + $b.int;
          $result = $temp + $x;
      }
    ;
```

### Lexer Features

#### Modes
```antlr
DEFAULT_MODE
STRING_START: '"' -> pushMode(STRING_MODE);

mode STRING_MODE;
STRING_END: '"' -> popMode;
STRING_CONTENT: ~["\r\n]+;
```

#### Channels
```antlr
WS: [ \t\r\n]+ -> channel(HIDDEN);
COMMENT: '//' ~[\r\n]* -> channel(COMMENT_CHANNEL);
```

#### Commands
```antlr
WS: [ \t\r\n]+ -> skip;
MORE: '...' -> more;
TYPE: 'type' -> type(KEYWORD);
```

## Implementation Plan

### Phase 1: Core Compatibility (Month 1-2)
- [x] Basic grammar structure
- [x] Parser and lexer rules
- [x] Alternatives and sequences
- [x] Quantifiers
- [x] Character classes
- [x] Fragment rules
- [x] Skip command

### Phase 2: Labels and References (Month 3)
- [ ] Element labels
- [ ] List labels
- [ ] Alternative labels
- [ ] Generate labeled AST nodes

### Phase 3: Actions and Predicates (Month 4)
- [ ] Parse embedded actions
- [ ] Parse semantic predicates
- [ ] Generate action code for each target language
- [ ] Translate Java actions to target language

### Phase 4: Advanced Features (Month 5-6)
- [ ] Rule arguments and returns
- [ ] Local variables
- [ ] Lexer modes
- [ ] Lexer channels
- [ ] Lexer commands

### Phase 5: Grammar Composition (Month 7-8)
- [ ] Grammar imports
- [ ] Grammar inheritance
- [ ] Token vocabularies
- [ ] Grammar options

### Phase 6: Full Compatibility (Month 9-12)
- [ ] Named actions (@header, @members)
- [ ] Token specifications
- [ ] All ANTLR4 options
- [ ] Pass ANTLR4 test suite

## Testing Strategy

### Unit Tests
Test each ANTLR4 feature individually:
```rust
#[test]
fn test_rule_labels() {
    let grammar = "rule: left=ID op='=' right=ID;";
    let ast = parse(grammar).unwrap();
    assert!(ast.has_labels());
}
```

### Integration Tests
Test complete ANTLR4 grammars:
```rust
#[test]
fn test_antlr4_json_grammar() {
    let grammar = include_str!("antlr4/JSON.g4");
    let ast = parse(grammar).unwrap();
    let code = generate_rust(ast).unwrap();
    assert!(code.compiles());
}
```

### Compatibility Tests
Use official ANTLR4 grammars:
- JSON.g4
- Java.g4
- Python3.g4
- C.g4
- SQL.g4

### Test Suite
Run ANTLR4's own test suite:
```bash
# Clone ANTLR4 grammars repository
git clone https://github.com/antlr/grammars-v4.git

# Test each grammar
for grammar in grammars-v4/*/*.g4; do
    minipg validate "$grammar"
done
```

## Differences from ANTLR4

### By Design

1. **No Runtime Library**
   - ANTLR4: Generates code dependent on runtime
   - minipg: Generates standalone code

2. **Multi-Language Generation**
   - ANTLR4: Separate tool for each language
   - minipg: Single tool generates all languages

3. **Action Translation**
   - ANTLR4: Actions in target language
   - minipg: Actions translated from Java-like syntax

### Limitations

1. **Target-Specific Actions**
   - ANTLR4 allows target-specific actions: `{Java code}?<Java>`
   - minipg uses language-agnostic actions

2. **Custom Token Factories**
   - ANTLR4: `@tokenfactory` action
   - minipg: Standard token types only (initially)

## Migration Guide

### From ANTLR4 to minipg

Most ANTLR4 grammars work without changes:

```bash
# ANTLR4
java -jar antlr4.jar -Dlanguage=Rust MyGrammar.g4

# minipg
minipg generate MyGrammar.g4 -l rust
```

### Action Translation

ANTLR4 actions (Java):
```antlr
rule
    : INT {System.out.println($INT.text);}
    ;
```

minipg actions (language-agnostic):
```antlr
rule
    : INT {print($INT.text);}
    ;
```

Generated Rust:
```rust
fn rule(&mut self) -> Result<()> {
    let int_token = self.match_token(INT)?;
    println!("{}", int_token.text);
    Ok(())
}
```

Generated Python:
```python
def rule(self):
    int_token = self.match_token(INT)
    print(int_token.text)
```

## Resources

### ANTLR4 Documentation
- [ANTLR4 Grammar Syntax](https://github.com/antlr/antlr4/blob/master/doc/grammars.md)
- [ANTLR4 Lexer Rules](https://github.com/antlr/antlr4/blob/master/doc/lexer-rules.md)
- [ANTLR4 Parser Rules](https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md)
- [ANTLR4 Actions and Attributes](https://github.com/antlr/antlr4/blob/master/doc/actions.md)

### ANTLR4 Grammars Repository
- [grammars-v4](https://github.com/antlr/grammars-v4) - 300+ grammars

### Testing
- Use grammars-v4 for compatibility testing
- Validate against ANTLR4 test suite
- Compare generated parsers with ANTLR4 output

## Success Criteria

1. ✅ Parse 100% of ANTLR4 grammar syntax
2. ✅ Generate working parsers from ANTLR4 grammars
3. ✅ Pass ANTLR4 test suite
4. ✅ Support all major ANTLR4 features
5. ✅ Provide migration guide for ANTLR4 users
