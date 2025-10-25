// Query Language (SQL-like)
// Demonstrates: complex statements, joins, aggregations, subqueries

grammar Query;

// Main statement
statement: selectStatement
         | insertStatement
         | updateStatement
         | deleteStatement
         | createStatement
         ;

// SELECT statement
selectStatement: 'SELECT' distinct? selectList 'FROM' tableRef joinClause* whereClause? groupByClause? havingClause? orderByClause? limitClause?;
distinct: 'DISTINCT';
selectList: selectItem (',' selectItem)*;
selectItem: expr (alias)?;
alias: 'AS' IDENTIFIER;

tableRef: tableName (alias)?;
tableName: IDENTIFIER;

joinClause: joinType 'JOIN' tableRef 'ON' expr;
joinType: 'INNER' | 'LEFT' | 'RIGHT' | 'FULL' | 'CROSS';

whereClause: 'WHERE' expr;
groupByClause: 'GROUP' 'BY' expr (',' expr)*;
havingClause: 'HAVING' expr;
orderByClause: 'ORDER' 'BY' orderItem (',' orderItem)*;
orderItem: expr direction?;
direction: 'ASC' | 'DESC';
limitClause: 'LIMIT' NUMBER ('OFFSET' NUMBER)?;

// INSERT statement
insertStatement: 'INSERT' 'INTO' tableName columnList? 'VALUES' valuesList;
columnList: '(' IDENTIFIER (',' IDENTIFIER)* ')';
valuesList: '(' value (',' value)* ')' (',' '(' value (',' value)* ')')*;

// UPDATE statement
updateStatement: 'UPDATE' tableName 'SET' assignment (',' assignment)* whereClause?;
assignment: IDENTIFIER '=' expr;

// DELETE statement
deleteStatement: 'DELETE' 'FROM' tableName whereClause?;

// CREATE statement
createStatement: 'CREATE' 'TABLE' tableName '(' columnDef (',' columnDef)* ')';
columnDef: IDENTIFIER dataType constraint*;
dataType: 'INT' | 'VARCHAR' | 'BOOLEAN' | 'DATE' | 'TIMESTAMP' | 'DECIMAL';
constraint: 'PRIMARY' 'KEY' | 'NOT' 'NULL' | 'UNIQUE' | 'DEFAULT' value;

// Expression
expr: logicalOr;
logicalOr: logicalAnd ('OR' logicalAnd)*;
logicalAnd: comparison ('AND' comparison)*;
comparison: additive (('=' | '!=' | '<' | '<=' | '>' | '>=') additive)*;
additive: multiplicative (('+' | '-') multiplicative)*;
multiplicative: unary (('*' | '/' | '%') unary)*;
unary: ('NOT' | '-') unary | primary;
primary: IDENTIFIER
       | NUMBER
       | STRING
       | 'NULL'
       | functionCall
       | '(' expr ')'
       | '(' selectStatement ')'
       ;

functionCall: IDENTIFIER '(' argList? ')';
argList: expr (',' expr)*;
value: expr;

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+ ('.' [0-9]+)?;
STRING: '\'' (~['\\\n] | '\\' .)* '\'';
WS: [ \t\n\r]+ -> skip;
COMMENT: '--' ~[\n]* -> skip;
