grammar parser JSON;

json: value;

value: STRING | NUMBER | TRUE | FALSE | NULL;

STRING: '"' CHAR '"';
NUMBER: DIGIT+;
TRUE: 't' 'r' 'u' 'e';
FALSE: 'f' 'a' 'l' 's' 'e';
NULL: 'n' 'u' 'l' 'l';

fragment CHAR: 'a' | 'b' | 'c';
fragment DIGIT: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
