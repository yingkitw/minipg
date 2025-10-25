// YAML Configuration Language (Simplified)
// Demonstrates: indentation-sensitive parsing, multiple string types, nested structures

grammar YAML;

// Document structure
document: content EOF;
content: (mapping | sequence | scalar)?;

// Mapping (key-value pairs)
mapping: pair+;
pair: key ':' value;
key: SCALAR;
value: scalar | mapping | sequence;

// Sequence (list)
sequence: ('-' item)+;
item: scalar | mapping | sequence;

// Scalar values
scalar: SCALAR
      | QUOTED_STRING
      | LITERAL_STRING
      | NUMBER
      ;

// Tokens
SCALAR: [a-zA-Z_][a-zA-Z0-9_]*;
QUOTED_STRING: '"' (~["\\\n] | '\\' .)* '"';
LITERAL_STRING: '\'' (~['\\\n] | '\\' .)* '\'';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
BOOLEAN_VALUE: 'true' | 'false' | 'yes' | 'no' | 'on' | 'off';
NULL_VALUE: 'null' | '~' | '';

// Comments and whitespace
COMMENT: '#' ~[\n]* -> skip;
WS: [ \t]+ -> skip;
NEWLINE: '\n' | '\r\n' -> skip;
