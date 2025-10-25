// GraphQL Schema Definition Language
// Demonstrates: complex grammar with multiple rule types, string handling, comments

grammar GraphQL;

// Document structure
document: definition+ EOF;
definition: schemaDefinition
          | typeDefinition
          | directiveDefinition
          ;

// Schema definition
schemaDefinition: 'schema' '{' operationTypeDefinition* '}';
operationTypeDefinition: operationType ':' namedType;
operationType: 'query' | 'mutation' | 'subscription';

// Type definitions
typeDefinition: scalarTypeDefinition
              | objectTypeDefinition
              | interfaceTypeDefinition
              | unionTypeDefinition
              | enumTypeDefinition
              | inputTypeDefinition
              ;

scalarTypeDefinition: 'scalar' name;
objectTypeDefinition: 'type' name implementsInterfaces? '{' fieldDefinition* '}';
interfaceTypeDefinition: 'interface' name '{' fieldDefinition* '}';
unionTypeDefinition: 'union' name '=' unionMemberTypes;
enumTypeDefinition: 'enum' name '{' enumValueDefinition* '}';
inputTypeDefinition: 'input' name '{' inputValueDefinition* '}';

implementsInterfaces: 'implements' '&'? namedType ('&' namedType)*;
unionMemberTypes: namedType ('|' namedType)*;

fieldDefinition: name argumentsDefinition? ':' type;
inputValueDefinition: name ':' type defaultValue?;
enumValueDefinition: name;

argumentsDefinition: '(' inputValueDefinition (',' inputValueDefinition)* ')';

type: namedType | listType | nonNullType;
namedType: name;
listType: '[' type ']';
nonNullType: type '!';

// Directive definition
directiveDefinition: 'directive' '@' name argumentsDefinition? 'on' directiveLocations;
directiveLocations: IDENTIFIER ('|' IDENTIFIER)*;

// Lexer rules
name: IDENTIFIER;
defaultValue: value;
value: INT | FLOAT | STRING | BOOLEAN | NULL | enumValue | listValue | objectValue;
enumValue: IDENTIFIER;
listValue: '[' value* ']';
objectValue: '{' objectField* '}';
objectField: IDENTIFIER ':' value;

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
DIRECTIVE_LOCATION: 'SCHEMA' | 'SCALAR' | 'OBJECT' | 'FIELD_DEFINITION';
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)?;
STRING: '"' (~["\\\n] | '\\' .)* '"';
BOOLEAN: 'true' | 'false';
NULL: 'null';

// Comments and whitespace
COMMENT: '#' ~[\n]* -> skip;
WS: [ \t\n\r]+ -> skip;
