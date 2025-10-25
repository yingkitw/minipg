// Configuration File Format (INI-like)
// Demonstrates: sections, key-value pairs, comments, various value types

grammar Config;

// Document structure
config: item*;
item: section | property;

// Section header
section: '[' sectionName ']';
sectionName: IDENTIFIER ('.' IDENTIFIER)*;

// Property (key-value pair)
property: key '=' value;
key: IDENTIFIER;
value: stringValue | numberValue | booleanValue | arrayValue;

stringValue: STRING;
numberValue: NUMBER;
booleanValue: BOOLEAN;
arrayValue: '[' value (',' value)* ']';

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
STRING: '"' (~["\\\n] | '\\' .)* '"' | '\'' (~['\\\n] | '\\' .)* '\'';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
BOOLEAN: 'true' | 'false' | 'yes' | 'no' | 'on' | 'off';
COMMENT: (';' | '#') ~[\n]* -> skip;
NEWLINE: ('\n' | '\r\n') -> skip;
WS: [ \t]+ -> skip;
