# minipg Examples

This directory contains example grammars demonstrating various features of minipg.

## Examples

### calculator.g4

A simple arithmetic calculator grammar demonstrating:
- Parser rules
- Lexer rules
- Fragment rules
- Basic operators

**Generate:**
```bash
minipg generate calculator.g4 -o output/
```

### json.g4

A simplified JSON parser demonstrating:
- Value types
- String and number literals
- Boolean and null values

**Generate:**
```bash
minipg generate json.g4 -o output/ --visitor
```

## Running Examples

1. Generate the parser:
   ```bash
   minipg generate <example>.g4 -o output/
   ```

2. The generated code will be in `output/<example>_parser.rs`

3. Use the generated parser in your Rust project

## Learning Path

1. Start with `calculator.g4` - Basic grammar structure
2. Try `json.g4` - More complex value types
3. Create your own grammar!

## See Also

- [User Guide](../docs/USER_GUIDE.md)
- [Grammar Syntax Reference](../docs/GRAMMAR_SYNTAX.md)
- [API Documentation](../docs/API.md)
