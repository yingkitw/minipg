// Example grammar demonstrating rule arguments, return values, and local variables
// This grammar shows how to use ANTLR4 rule features in minipg

grammar RuleFeatures;

// Rule with arguments - parameters passed to the rule
program[String version]: statement+ EOF;

// Rule with return values - values returned from the rule
statement returns [String type, int count]: 
    assignment 
    | expression
    ;

// Rule with local variables - variables declared within the rule
assignment locals [String name, String value]:
    ID '=' expression
    ;

// Rule with arguments and return values
expression[boolean allowNegative] returns [int result]:
    term (('+' | '-') term)*
    ;

// Rule with all features - arguments, returns, and locals
term[int precedence] returns [int value] locals [int temp, boolean valid]:
    factor (('*' | '/') factor)*
    ;

// Simple factor rule
factor:
    NUMBER
    | '(' expression[true] ')'
    | ID
    ;

// Lexer rules
ID: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
