// Complete JSON Grammar - ANTLR4 Compatible
// Based on RFC 8259 - Full specification

grammar CompleteJSON;

// Parser Rules
json
    : value
    ;

value
    : object
    | array
    | string
    | number
    | 'true'
    | 'false'
    | 'null'
    ;

object
    : '{' '}'
    | '{' pair (',' pair)* '}'
    ;

pair
    : string ':' value
    ;

array
    : '[' ']'
    | '[' value (',' value)* ']'
    ;

string
    : STRING
    ;

number
    : NUMBER
    ;

// Lexer Rules
STRING
    : '"' (ESC | SAFECODEPOINT)* '"'
    ;

fragment ESC
    : '\\' (["\\/bfnrt] | UNICODE)
    ;

fragment UNICODE
    : 'u' HEX HEX HEX HEX
    ;

fragment HEX
    : [0-9a-fA-F]
    ;

fragment SAFECODEPOINT
    : ~ ["\\]
    ;

NUMBER
    : '-'? INT ('.' [0-9]+)? EXP?
    ;

fragment INT
    : '0'
    | [1-9] [0-9]*
    ;

fragment EXP
    : [Ee] [+\-]? [0-9]+
    ;

WS
    : [ \t\n\r]+ -> skip
    ;
