// Python Subset Grammar - ANTLR4 Compatible
// Simplified Python 3 grammar

grammar PythonSubset;

// Parser Rules
file_input
    : (NEWLINE | statement)* EOF
    ;

statement
    : simple_stmt
    | compound_stmt
    ;

simple_stmt
    : small_stmt (';' small_stmt)* ';'? NEWLINE
    ;

small_stmt
    : expr_stmt
    | del_stmt
    | pass_stmt
    | flow_stmt
    | import_stmt
    | global_stmt
    | nonlocal_stmt
    | assert_stmt
    ;

expr_stmt
    : test ('=' test)*
    | test augassign test
    ;

augassign
    : '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^='
    | '<<=' | '>>=' | '**=' | '//='
    ;

del_stmt
    : 'del' exprlist
    ;

pass_stmt
    : 'pass'
    ;

flow_stmt
    : break_stmt
    | continue_stmt
    | return_stmt
    | raise_stmt
    ;

break_stmt
    : 'break'
    ;

continue_stmt
    : 'continue'
    ;

return_stmt
    : 'return' testlist?
    ;

raise_stmt
    : 'raise' (test ('from' test)?)?
    ;

import_stmt
    : 'import' dotted_as_names
    | 'from' dotted_name 'import' ('*' | import_as_names)
    ;

import_as_names
    : import_as_name (',' import_as_name)*
    ;

import_as_name
    : NAME ('as' NAME)?
    ;

dotted_as_names
    : dotted_as_name (',' dotted_as_name)*
    ;

dotted_as_name
    : dotted_name ('as' NAME)?
    ;

dotted_name
    : NAME ('.' NAME)*
    ;

global_stmt
    : 'global' NAME (',' NAME)*
    ;

nonlocal_stmt
    : 'nonlocal' NAME (',' NAME)*
    ;

assert_stmt
    : 'assert' test (',' test)?
    ;

compound_stmt
    : if_stmt
    | while_stmt
    | for_stmt
    | try_stmt
    | with_stmt
    | funcdef
    | classdef
    ;

if_stmt
    : 'if' test ':' suite ('elif' test ':' suite)* ('else' ':' suite)?
    ;

while_stmt
    : 'while' test ':' suite ('else' ':' suite)?
    ;

for_stmt
    : 'for' exprlist 'in' testlist ':' suite ('else' ':' suite)?
    ;

try_stmt
    : 'try' ':' suite
      (except_clause ':' suite)+
      ('else' ':' suite)?
      ('finally' ':' suite)?
    | 'try' ':' suite 'finally' ':' suite
    ;

except_clause
    : 'except' (test ('as' NAME)?)?
    ;

with_stmt
    : 'with' with_item (',' with_item)* ':' suite
    ;

with_item
    : test ('as' test)?
    ;

suite
    : simple_stmt
    | NEWLINE INDENT statement+ DEDENT
    ;

funcdef
    : 'def' NAME '(' parameters? ')' ('->' test)? ':' suite
    ;

parameters
    : parameter (',' parameter)*
    ;

parameter
    : NAME (':' test)? ('=' test)?
    ;

classdef
    : 'class' NAME ('(' arglist? ')')? ':' suite
    ;

test
    : or_test ('if' or_test 'else' test)?
    | lambdef
    ;

lambdef
    : 'lambda' parameters? ':' test
    ;

or_test
    : and_test ('or' and_test)*
    ;

and_test
    : not_test ('and' not_test)*
    ;

not_test
    : 'not' not_test
    | comparison
    ;

comparison
    : expr (comp_op expr)*
    ;

comp_op
    : '<' | '>' | '==' | '>=' | '<=' | '!=' | 'in' | 'not' 'in' | 'is' | 'is' 'not'
    ;

expr
    : xor_expr ('|' xor_expr)*
    ;

xor_expr
    : and_expr ('^' and_expr)*
    ;

and_expr
    : shift_expr ('&' shift_expr)*
    ;

shift_expr
    : arith_expr (('<<'|'>>') arith_expr)*
    ;

arith_expr
    : term (('+' | '-') term)*
    ;

term
    : factor (('*' | '/' | '%' | '//') factor)*
    ;

factor
    : ('+' | '-' | '~') factor
    | power
    ;

power
    : atom_expr ('**' factor)?
    ;

atom_expr
    : atom trailer*
    ;

atom
    : '(' testlist? ')'
    | '[' testlist? ']'
    | '{' dictorsetmaker? '}'
    | NAME
    | NUMBER
    | STRING+
    | 'None'
    | 'True'
    | 'False'
    ;

trailer
    : '(' arglist? ')'
    | '[' test ']'
    | '.' NAME
    ;

testlist
    : test (',' test)*
    ;

exprlist
    : expr (',' expr)*
    ;

dictorsetmaker
    : test ':' test (',' test ':' test)*
    | test (',' test)*
    ;

arglist
    : argument (',' argument)*
    ;

argument
    : test ('=' test)?
    | '**' test
    | '*' test
    ;

// Lexer Rules
NAME
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

NUMBER
    : INTEGER
    | FLOAT_NUMBER
    | IMAG_NUMBER
    ;

INTEGER
    : DECIMAL_INTEGER
    | OCT_INTEGER
    | HEX_INTEGER
    | BIN_INTEGER
    ;

DECIMAL_INTEGER
    : [1-9] [0-9]*
    | '0'+
    ;

OCT_INTEGER
    : '0' [oO] [0-7]+
    ;

HEX_INTEGER
    : '0' [xX] [0-9a-fA-F]+
    ;

BIN_INTEGER
    : '0' [bB] [01]+
    ;

FLOAT_NUMBER
    : [0-9]+ '.' [0-9]+
    | [0-9]+ ('.' [0-9]+)? [eE] [+\-]? [0-9]+
    ;

IMAG_NUMBER
    : ([0-9]+ | FLOAT_NUMBER) [jJ]
    ;

STRING
    : STRING_LITERAL
    | BYTES_LITERAL
    ;

STRING_LITERAL
    : '"' (~["\\\r\n] | '\\' .)* '"'
    | '\'' (~['\\\r\n] | '\\' .)* '\''
    | '"""' .*? '"""'
    | '\'\'\'' .*? '\'\'\''
    ;

BYTES_LITERAL
    : [bB] STRING_LITERAL
    ;

NEWLINE
    : '\r'? '\n'
    ;

INDENT
    : '    ' // Simplified: 4 spaces
    ;

DEDENT
    : // Handled by lexer logic
    ;

WS
    : [ \t]+ -> skip
    ;

LINE_COMMENT
    : '#' ~[\r\n]* -> skip
    ;
