grammar parser Calculator;

// Parser rules
expr: term;
term: factor;
factor: NUMBER;

// Lexer rules
NUMBER: DIGIT+;
DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
WS: SPACE+;
SPACE: ' ' | '\t' | '\r' | '\n';
