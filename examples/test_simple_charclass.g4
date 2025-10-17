grammar TestCharClass;

// Simple character class test
LETTER: [a-z];
DIGIT: [0-9];
ALPHANUM: [a-zA-Z0-9];

WS: [ \t\r\n]+ -> skip;
