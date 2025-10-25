// CSS Stylesheet Language
// Demonstrates: selector syntax, property declarations, media queries, nested rules

grammar CSS;

// Stylesheet structure
stylesheet: statement* EOF;
statement: rule
         | mediaQuery
         | fontFace
         | keyframes
         | comment
         ;

// CSS Rule
rule: selector '{' declaration* '}';
selector: simpleSelector (combinator simpleSelector)*;

simpleSelector: elementSelector pseudoClass* pseudoElement?
              | classSelector pseudoClass* pseudoElement?
              | idSelector pseudoClass* pseudoElement?
              | universalSelector pseudoClass* pseudoElement?
              ;

elementSelector: IDENTIFIER;
classSelector: '.' IDENTIFIER;
idSelector: '#' IDENTIFIER;
universalSelector: '*';

combinator: ' ' | '>' | '+' | '~';
pseudoClass: ':' IDENTIFIER ('(' IDENTIFIER ')')?;
pseudoElement: '::' IDENTIFIER;

// Declarations
declaration: property ':' value ';';
property: IDENTIFIER;
value: valueItem+;
valueItem: IDENTIFIER | NUMBER | COLOR | STRING | UNIT;

// Media Query
mediaQuery: '@media' mediaFeature '{' rule* '}';
mediaFeature: '(' IDENTIFIER ':' value ')';

// Font Face
fontFace: '@font-face' '{' declaration* '}';

// Keyframes
keyframes: '@keyframes' IDENTIFIER '{' keyframeRule* '}';
keyframeRule: keyframeSelector '{' declaration* '}';
keyframeSelector: PERCENTAGE | 'from' | 'to';

// Comment
comment: COMMENT;

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+ ('.' [0-9]+)?;
PERCENTAGE: NUMBER '%';
COLOR: '#' [0-9a-fA-F]{3,6};
STRING: '"' (~["\\\n] | '\\' .)* '"' | '\'' (~['\\\n] | '\\' .)* '\'';
UNIT: ('px' | 'em' | 'rem' | 'pt' | '%' | 'vh' | 'vw' | 'ms' | 's');
COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\n\r]+ -> skip;
