# Tree-sitter Code Generation Guide

## Overview

minipg can generate Tree-sitter grammar files from ANTLR4 grammars. Tree-sitter is a parser generator tool used by modern editors like VS Code, Neovim, Atom, and Emacs for syntax highlighting, code folding, and semantic analysis.

## Features

- **Automatic Conversion**: Converts ANTLR4 grammars to Tree-sitter grammar.js format
- **Package Generation**: Creates complete npm package with package.json and README
- **Editor Integration**: Generated grammars work with all Tree-sitter-enabled editors
- **Zero Runtime**: Tree-sitter parsers are standalone with no runtime dependencies

## Usage

### Generate Tree-sitter Grammar

```bash
minipg generate grammar.g4 -o output/ -l treesitter
```

This generates three files:
- `grammar.js` - Tree-sitter grammar definition
- `package.json` - npm package configuration
- `README.md` - Documentation for the generated grammar

### Building the Parser

After generation, build the Tree-sitter parser:

```bash
cd output/
npm install
tree-sitter generate
tree-sitter build
```

### Testing

Test the generated parser:

```bash
tree-sitter test
tree-sitter parse example.txt
```

## Example

Given an ANTLR4 grammar:

```antlr4
grammar Calculator;

expr: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | '(' expr ')';

NUMBER: [0-9]+;
WS: [ \t\r\n]+ -> skip;
```

minipg generates a Tree-sitter grammar:

```javascript
module.exports = grammar({
  name: 'calculator',

  extras: $ => [
    /\s/,
  ],

  rules: {
    expr: $ => seq(
      $.term,
      repeat(seq(
        choice('+', '-'),
        $.term
      ))
    ),

    term: $ => seq(
      $.factor,
      repeat(seq(
        choice('*', '/'),
        $.factor
      ))
    ),

    factor: $ => choice(
      $.number,
      seq('(', $.expr, ')')
    ),

    number: $ => /[0-9]+/,
  }
});
```

## Editor Integration

### Neovim

Add to your Tree-sitter configuration:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
parser_config.your_language = {
  install_info = {
    url = "path/to/your/grammar",
    files = {"src/parser.c"}
  }
}
```

### VS Code

1. Install the Tree-sitter extension
2. Configure the grammar path in settings
3. Reload VS Code

### Emacs

Add to your configuration:

```elisp
(use-package tree-sitter
  :config
  (add-to-list 'tree-sitter-load-path "path/to/your/grammar"))
```

## Supported Features

### Parser Rules
- ✅ Sequences: `a b c`
- ✅ Alternatives: `a | b | c`
- ✅ Quantifiers: `*`, `+`, `?`
- ✅ Groups: `(a | b)`
- ✅ Rule references

### Lexer Rules
- ✅ String literals: `'keyword'`
- ✅ Character classes: `[a-z0-9]`
- ✅ Negated classes: `~["\r\n]`
- ✅ Quantifiers: `*`, `+`, `?`
- ✅ Wildcards: `.`

### Limitations

Tree-sitter has different semantics than ANTLR4:

- **No Actions**: Semantic actions and predicates are ignored
- **No Modes**: Lexer modes are not directly supported
- **No Channels**: All tokens are in the default stream
- **Different Precedence**: May need manual adjustment for operator precedence

## Best Practices

1. **Test Thoroughly**: Always test generated grammars with real code samples
2. **Manual Tuning**: Some grammars may need manual adjustments for optimal performance
3. **Incremental Parsing**: Tree-sitter excels at incremental parsing - design rules accordingly
4. **Keep Simple**: Simpler grammars work better with Tree-sitter's GLR algorithm

## Troubleshooting

### Grammar Doesn't Parse

Check for:
- Ambiguous rules that need precedence declarations
- Left recursion (Tree-sitter handles it differently)
- Complex lookahead patterns

### Performance Issues

- Simplify character classes
- Reduce nesting depth
- Use `token()` for lexical rules

### Editor Not Recognizing Grammar

- Verify file paths in configuration
- Rebuild parser after changes
- Check Tree-sitter version compatibility

## Resources

- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Tree-sitter Grammar Writing Guide](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [ANTLR4 to Tree-sitter Migration](https://github.com/tree-sitter/tree-sitter/discussions)

## Example Workflow

Complete workflow from ANTLR4 to editor integration:

```bash
# 1. Generate Tree-sitter grammar
minipg generate MyLanguage.g4 -o tree-sitter-mylang/ -l treesitter

# 2. Setup npm package
cd tree-sitter-mylang/
npm install

# 3. Generate parser
tree-sitter generate

# 4. Build native module
tree-sitter build

# 5. Test parser
tree-sitter test

# 6. Install in editor (Neovim example)
mkdir -p ~/.local/share/nvim/site/pack/tree-sitter/start/
cp -r . ~/.local/share/nvim/site/pack/tree-sitter/start/tree-sitter-mylang/

# 7. Configure in editor
# Add to init.lua or init.vim
```

## Advanced Usage

### Custom Extras

Modify `extras` in generated grammar.js:

```javascript
extras: $ => [
  /\s/,           // whitespace
  $.comment,      // comments
  $.preprocessor  // preprocessor directives
],
```

### Precedence

Add precedence for operators:

```javascript
expr: $ => choice(
  prec.left(1, seq($.expr, '+', $.expr)),
  prec.left(1, seq($.expr, '-', $.expr)),
  prec.left(2, seq($.expr, '*', $.expr)),
  prec.left(2, seq($.expr, '/', $.expr)),
),
```

### External Scanner

For complex lexical rules, use external scanner:

```javascript
externals: $ => [
  $.string_content,
  $.heredoc_start,
],
```

## Contributing

Found an issue with Tree-sitter generation? Please report it at:
https://github.com/yingkitw/minipg/issues

## License

Generated grammars inherit the license of the source ANTLR4 grammar.
