// Protocol Buffer-like Definition Language
// Demonstrates: message definitions, field types, nested structures, options

grammar Protocol;

// File structure
file: syntax? package? definition* EOF;

syntax: 'syntax' '=' STRING ';';
package: 'package' IDENTIFIER ';';
import: 'import' STRING ';';

// Definitions
definition: messageDefinition
          | enumDefinition
          | serviceDefinition
          ;

// Message definition
messageDefinition: 'message' IDENTIFIER '{' messageBody* '}';
messageBody: fieldDefinition
           | nestedMessage
           | nestedEnum
           | option
           ;

fieldDefinition: fieldModifier? fieldType fieldName '=' fieldNumber fieldOptions? ';';
fieldModifier: 'repeated' | 'optional' | 'required';
fieldType: 'int32' | 'int64' | 'uint32' | 'uint64' | 'sint32' | 'sint64'
         | 'fixed32' | 'fixed64' | 'sfixed32' | 'sfixed64'
         | 'float' | 'double' | 'bool' | 'string' | 'bytes'
         | IDENTIFIER
         ;
fieldName: IDENTIFIER;
fieldNumber: NUMBER;
fieldOptions: '[' fieldOption (',' fieldOption)* ']';
fieldOption: IDENTIFIER '=' (STRING | NUMBER | BOOLEAN);

nestedMessage: messageDefinition;
nestedEnum: enumDefinition;

// Enum definition
enumDefinition: 'enum' IDENTIFIER '{' enumBody* '}';
enumBody: enumValue | option;
enumValue: IDENTIFIER '=' NUMBER ';';

// Service definition
serviceDefinition: 'service' IDENTIFIER '{' serviceBody* '}';
serviceBody: rpcDefinition | option;
rpcDefinition: 'rpc' IDENTIFIER '(' IDENTIFIER ')' 'returns' '(' IDENTIFIER ')' ';';

// Option
option: 'option' IDENTIFIER '=' (STRING | NUMBER | BOOLEAN) ';';

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
STRING: '"' (~["\\\n] | '\\' .)* '"' | '\'' (~['\\\n] | '\\' .)* '\'';
BOOLEAN: 'true' | 'false';
COMMENT: '//' ~[\n]* -> skip;
WS: [ \t\n\r]+ -> skip;
