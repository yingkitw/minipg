// Java Subset Grammar - ANTLR4 Compatible
// Simplified Java grammar for classes, methods, and basic statements

grammar JavaSubset;

// Parser Rules
compilationUnit
    : packageDeclaration? importDeclaration* typeDeclaration* EOF
    ;

packageDeclaration
    : PACKAGE qualifiedName ';'
    ;

importDeclaration
    : IMPORT STATIC? qualifiedName ('.' '*')? ';'
    ;

typeDeclaration
    : classDeclaration
    | interfaceDeclaration
    ;

classDeclaration
    : modifier* CLASS IDENTIFIER
      (EXTENDS qualifiedName)?
      (IMPLEMENTS qualifiedName (',' qualifiedName)*)?
      classBody
    ;

interfaceDeclaration
    : modifier* INTERFACE IDENTIFIER
      (EXTENDS qualifiedName (',' qualifiedName)*)?
      interfaceBody
    ;

classBody
    : '{' classMember* '}'
    ;

interfaceBody
    : '{' interfaceMember* '}'
    ;

classMember
    : fieldDeclaration
    | methodDeclaration
    | constructorDeclaration
    ;

interfaceMember
    : methodSignature
    | fieldDeclaration
    ;

fieldDeclaration
    : modifier* type IDENTIFIER ('=' expression)? ';'
    ;

methodDeclaration
    : modifier* type IDENTIFIER '(' parameterList? ')' methodBody
    ;

constructorDeclaration
    : modifier* IDENTIFIER '(' parameterList? ')' block
    ;

methodSignature
    : modifier* type IDENTIFIER '(' parameterList? ')' ';'
    ;

methodBody
    : block
    | ';'
    ;

parameterList
    : parameter (',' parameter)*
    ;

parameter
    : type IDENTIFIER
    ;

block
    : '{' statement* '}'
    ;

statement
    : block
    | variableDeclaration
    | expressionStatement
    | ifStatement
    | whileStatement
    | forStatement
    | returnStatement
    | breakStatement
    | continueStatement
    ;

variableDeclaration
    : type IDENTIFIER ('=' expression)? ';'
    ;

expressionStatement
    : expression ';'
    ;

ifStatement
    : IF '(' expression ')' statement (ELSE statement)?
    ;

whileStatement
    : WHILE '(' expression ')' statement
    ;

forStatement
    : FOR '(' forInit? ';' expression? ';' forUpdate? ')' statement
    ;

forInit
    : variableDeclaration
    | expressionList
    ;

forUpdate
    : expressionList
    ;

returnStatement
    : RETURN expression? ';'
    ;

breakStatement
    : BREAK ';'
    ;

continueStatement
    : CONTINUE ';'
    ;

expression
    : primary
    | expression '.' IDENTIFIER
    | expression '(' expressionList? ')'
    | expression '[' expression ']'
    | expression ('++' | '--')
    | ('++' | '--') expression
    | ('+' | '-' | '!' | '~') expression
    | expression ('*' | '/' | '%') expression
    | expression ('+' | '-') expression
    | expression ('<' | '>' | '<=' | '>=') expression
    | expression ('==' | '!=') expression
    | expression '&&' expression
    | expression '||' expression
    | expression '?' expression ':' expression
    | expression '=' expression
    | NEW type ('(' expressionList? ')')?
    ;

primary
    : '(' expression ')'
    | THIS
    | SUPER
    | literal
    | IDENTIFIER
    ;

expressionList
    : expression (',' expression)*
    ;

literal
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | BOOLEAN_LITERAL
    | NULL_LITERAL
    ;

type
    : primitiveType ('[' ']')*
    | qualifiedName ('[' ']')*
    ;

primitiveType
    : BOOLEAN
    | BYTE
    | SHORT
    | INT
    | LONG
    | CHAR
    | FLOAT
    | DOUBLE
    | VOID
    ;

qualifiedName
    : IDENTIFIER ('.' IDENTIFIER)*
    ;

modifier
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | STATIC
    | FINAL
    | ABSTRACT
    | SYNCHRONIZED
    ;

// Lexer Rules
// Keywords
ABSTRACT : 'abstract' ;
BOOLEAN : 'boolean' ;
BREAK : 'break' ;
BYTE : 'byte' ;
CHAR : 'char' ;
CLASS : 'class' ;
CONTINUE : 'continue' ;
DOUBLE : 'double' ;
ELSE : 'else' ;
EXTENDS : 'extends' ;
FINAL : 'final' ;
FLOAT : 'float' ;
FOR : 'for' ;
IF : 'if' ;
IMPLEMENTS : 'implements' ;
IMPORT : 'import' ;
INT : 'int' ;
INTERFACE : 'interface' ;
LONG : 'long' ;
NEW : 'new' ;
PACKAGE : 'package' ;
PRIVATE : 'private' ;
PROTECTED : 'protected' ;
PUBLIC : 'public' ;
RETURN : 'return' ;
SHORT : 'short' ;
STATIC : 'static' ;
SUPER : 'super' ;
SYNCHRONIZED : 'synchronized' ;
THIS : 'this' ;
VOID : 'void' ;
WHILE : 'while' ;

// Literals
BOOLEAN_LITERAL
    : 'true'
    | 'false'
    ;

NULL_LITERAL
    : 'null'
    ;

INTEGER_LITERAL
    : [0-9]+
    | '0x' [0-9a-fA-F]+
    | '0b' [01]+
    ;

FLOAT_LITERAL
    : [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)? [fFdD]?
    | [0-9]+ [eE] [+-]? [0-9]+ [fFdD]?
    | [0-9]+ [fFdD]
    ;

STRING_LITERAL
    : '"' (~["\\\r\n] | '\\' .)* '"'
    ;

// Identifiers
IDENTIFIER
    : [a-zA-Z_$] [a-zA-Z0-9_$]*
    ;

// Whitespace and Comments
WS
    : [ \t\r\n]+ -> skip
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> skip
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> skip
    ;
