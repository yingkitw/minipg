; Syntax highlighting queries for ANTLR4 grammars
; This file demonstrates the query language for minipg

; Keywords
[
  "grammar"
  "lexer"
  "parser"
  "fragment"
  "import"
  "mode"
  "returns"
  "locals"
  "throws"
  "catch"
  "finally"
  "options"
] @keyword

; Rule definitions
(rule name: (identifier) @function)

; Token references (uppercase identifiers)
(terminal name: (identifier) @constant)

; String literals
(string_literal) @string

; Character classes
(char_class) @string.special

; Operators
[
  "|"
  "*"
  "+"
  "?"
  "~"
  "->"
] @operator

; Delimiters
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
] @punctuation.bracket

; Special symbols
[
  ":"
  ";"
  ","
  "."
  "="
] @punctuation.delimiter

; Comments
(line_comment) @comment
(block_comment) @comment

; Actions (embedded code)
(action) @embedded

; Predicates
(predicate) @function.special

; Captures for specific patterns
(rule_ref name: (identifier) @variable)
(label name: (identifier) @label)
