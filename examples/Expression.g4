// Expression Language
// Demonstrates: operator precedence, function calls, variables, literals

grammar Expression;

// Expression hierarchy (precedence)
expr: logicalOr;

logicalOr: logicalAnd ('||' logicalAnd)*;
logicalAnd: equality ('&&' equality)*;
equality: comparison (('==' | '!=') comparison)*;
comparison: additive (('<' | '<=' | '>' | '>=') additive)*;
additive: multiplicative (('+' | '-') multiplicative)*;
multiplicative: unary (('*' | '/' | '%') unary)*;
unary: ('!' | '-' | '+') unary | postfix;
postfix: primary ('[' expr ']' | '(' argList? ')')* ;
primary: IDENTIFIER
       | NUMBER
       | STRING
       | BOOLEAN
       | NULL
       | '(' expr ')'
       | arrayLiteral
       | objectLiteral
       ;

arrayLiteral: '[' (expr (',' expr)*)? ']';
objectLiteral: '{' (objectPair (',' objectPair)*)? '}';
objectPair: (IDENTIFIER | STRING) ':' expr;

argList: expr (',' expr)*;

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
STRING: '"' (~["\\\n] | '\\' .)* '"' | '\'' (~['\\\n] | '\\' .)* '\'';
BOOLEAN: 'true' | 'false';
NULL: 'null';
WS: [ \t\n\r]+ -> skip;
