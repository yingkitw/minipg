// SQL Grammar - ANTLR4 Compatible
// Simplified SQL SELECT statement grammar

grammar SQL;

// Parser Rules
statement
    : selectStatement
    | insertStatement
    | updateStatement
    | deleteStatement
    ;

selectStatement
    : SELECT selectList
      FROM tableName
      (WHERE whereClause)?
      (ORDER BY orderByClause)?
      (LIMIT limitClause)?
    ;

insertStatement
    : INSERT INTO tableName
      '(' columnList ')'
      VALUES '(' valueList ')'
    ;

updateStatement
    : UPDATE tableName
      SET setClause (',' setClause)*
      (WHERE whereClause)?
    ;

deleteStatement
    : DELETE FROM tableName
      (WHERE whereClause)?
    ;

selectList
    : '*'
    | columnName (',' columnName)*
    ;

columnList
    : columnName (',' columnName)*
    ;

valueList
    : value (',' value)*
    ;

setClause
    : columnName '=' value
    ;

whereClause
    : condition (AND condition)*
    | condition (OR condition)*
    ;

condition
    : columnName operator value
    ;

operator
    : '='
    | '!='
    | '<'
    | '>'
    | '<='
    | '>='
    | LIKE
    ;

orderByClause
    : columnName (ASC | DESC)?
    ;

limitClause
    : NUMBER
    ;

tableName
    : IDENTIFIER
    ;

columnName
    : IDENTIFIER
    ;

value
    : STRING
    | NUMBER
    | NULL
    ;

// Lexer Rules
SELECT : 'SELECT' | 'select' ;
FROM : 'FROM' | 'from' ;
WHERE : 'WHERE' | 'where' ;
INSERT : 'INSERT' | 'insert' ;
INTO : 'INTO' | 'into' ;
VALUES : 'VALUES' | 'values' ;
UPDATE : 'UPDATE' | 'update' ;
SET : 'SET' | 'set' ;
DELETE : 'DELETE' | 'delete' ;
ORDER : 'ORDER' | 'order' ;
BY : 'BY' | 'by' ;
LIMIT : 'LIMIT' | 'limit' ;
AND : 'AND' | 'and' ;
OR : 'OR' | 'or' ;
ASC : 'ASC' | 'asc' ;
DESC : 'DESC' | 'desc' ;
LIKE : 'LIKE' | 'like' ;
NULL : 'NULL' | 'null' ;

IDENTIFIER
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

STRING
    : '\'' (~['\r\n] | '\'\'')* '\''
    ;

NUMBER
    : [0-9]+ ('.' [0-9]+)?
    ;

WS
    : [ \t\r\n]+ -> skip
    ;

LINE_COMMENT
    : '--' ~[\r\n]* -> skip
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> skip
    ;
