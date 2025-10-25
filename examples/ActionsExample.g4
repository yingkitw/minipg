// Example grammar demonstrating action code and semantic predicates
// Actions allow you to embed code that executes during parsing
// Semantic predicates allow you to add conditions to parsing decisions

grammar ActionsExample;

@header {
  // Language-specific imports and setup
  // For Rust: use std::collections::HashMap;
  // For Python: import json
  // For JavaScript: // No imports needed
}

@members {
  // Language-specific member variables
  // For Rust: values: HashMap<String, i32>
  // For Python: values = {}
  // For JavaScript: this.values = {};
}

// Parser rules with actions
program
  : statement+ EOF
  ;

statement
  : assignment
  | expression
  ;

// Assignment with action to store value
assignment
  : ID '=' expr=expression
    {
      // Generic action - will be translated to target language
      // Rust: self.values.insert(ID.to_string(), expr);
      // Python: self.values[ID] = expr
      // JavaScript: this.values[ID] = expr;
    }
  ;

// Expression with semantic predicate
expression
  : term (('+' | '-') term)*
  ;

term
  : factor (('*' | '/') factor)*
  ;

// Factor with semantic predicate for division by zero
factor
  : NUMBER
  | '(' expression ')'
  | ID
  | ID '/' NUMBER {self.position > 0}?  // Semantic predicate
  ;

// Lexer rules
ID
  : [a-zA-Z_][a-zA-Z0-9_]*
  ;

NUMBER
  : [0-9]+
  ;

WS
  : [ \t\r\n]+ -> skip
  ;
