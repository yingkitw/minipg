// Example grammar demonstrating lexer modes and channels
// This grammar shows how to use ANTLR4 lexer modes for context-sensitive lexing

grammar LexerModes;

// Default mode - handles normal code
program: statement+ EOF;

statement: assignment | expression;

assignment: ID '=' expression;

expression: term (('+' | '-') term)*;

term: factor (('*' | '/') factor)*;

factor: NUMBER | ID | '(' expression ')' | STRING;

// Lexer rules in default mode
ID: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
QUOTE: '"' -> mode(STRING_MODE);
LBRACE: '{' -> pushMode(NESTED_MODE);
COMMENT: '//' ~[\r\n]* -> channel(COMMENTS);
WS: [ \t\r\n]+ -> skip;

// String mode - handles string content
mode STRING_MODE;
STRING_CHAR: ~["\\\r\n];
STRING_ESCAPE: '\\' .;
END_QUOTE: '"' -> mode(DEFAULT_MODE);

// Nested mode - handles nested braces
mode NESTED_MODE;
NESTED_LBRACE: '{' -> pushMode(NESTED_MODE);
NESTED_RBRACE: '}' -> popMode;
NESTED_ID: [a-zA-Z_]+;
NESTED_WS: [ \t\r\n]+ -> skip;
