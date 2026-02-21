grammar SimpleCalc;

// Parser rules
expr: left=term op=('+' | '-') right=term;
term: value=NUMBER;

// Lexer rules
NUMBER: [0-9]+;
PLUS: '+';
MINUS: '-';
WS: [ \t\r\n]+ -> skip;
