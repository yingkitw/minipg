grammar TestCharClass;

// Test negated character class
rule1: WORD+;

// Simple character class
WORD: [a-zA-Z]+;

// Negated character class (simple)
NOTDIGIT: ~[0-9];

// Whitespace
WS: [ \t\r\n]+ -> skip;
