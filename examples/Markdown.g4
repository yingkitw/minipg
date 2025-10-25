// Markdown Language
// Demonstrates: text processing, special characters, nested structures, code blocks

grammar Markdown;

// Document structure
document: block* EOF;
block: heading
     | paragraph
     | codeBlock
     | blockquote
     | list
     | horizontalRule
     | blankLine
     ;

// Headings
heading: HEADING_MARKER+ text;
HEADING_MARKER: '#';

// Paragraphs
paragraph: inline+ NEWLINE;

// Code blocks
codeBlock: BACKTICK_FENCE language? NEWLINE codeContent BACKTICK_FENCE;
BACKTICK_FENCE: '```';
language: IDENTIFIER;
codeContent: (~[`])* | '`' (~[`])*;

// Block quotes
blockquote: QUOTE_MARKER text;
QUOTE_MARKER: '>';

// Lists
list: listItem+;
listItem: LIST_MARKER text;
LIST_MARKER: '-' | '*' | '+' | [0-9]+ '.';

// Horizontal rule
horizontalRule: HORIZONTAL_RULE;
HORIZONTAL_RULE: ('---' | '***' | '___');

// Blank line
blankLine: NEWLINE;

// Inline content
inline: inlineCode
      | boldText
      | italicText
      | link
      | image
      | plainText
      ;

inlineCode: BACKTICK ~[`]+ BACKTICK;
BACKTICK: '`';

boldText: BOLD_MARKER text BOLD_MARKER;
BOLD_MARKER: '**' | '__';

italicText: ITALIC_MARKER text ITALIC_MARKER;
ITALIC_MARKER: '*' | '_';

link: '[' linkText ']' '(' url ')';
linkText: (~[\[\]])+ ;
url: (~[\)\\])+ ;

image: '!' link;

plainText: TEXT_CONTENT;
TEXT_CONTENT: (~[\n`*_\[\]()#>])+ ;

// Tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NEWLINE: '\n' | '\r\n';
text: (inline | NEWLINE)*;
