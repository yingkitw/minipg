# Example 5: SQL Parser

## Level: Advanced

A SQL parser supporting SELECT, INSERT, UPDATE, and DELETE statements.

## Grammar File

**File**: `examples/SQL.g4`

```antlr4
grammar SQL;

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

// ... (see full grammar in examples/SQL.g4)
```

## Features Demonstrated

- ✅ Multiple statement types
- ✅ Optional clauses
- ✅ Case-insensitive keywords
- ✅ Complex expressions
- ✅ Comments (line and block)
- ✅ Real-world syntax

## Usage

```bash
minipg generate --input examples/SQL.g4 --output target/sql_parser --language rust
```

## Test Queries

### SELECT
```sql
SELECT * FROM users WHERE age > 18 ORDER BY name LIMIT 10;
SELECT id, name FROM products WHERE price < 100;
```

### INSERT
```sql
INSERT INTO users (name, email) VALUES ('John', 'john@example.com');
```

### UPDATE
```sql
UPDATE users SET active = true WHERE id = 1;
```

### DELETE
```sql
DELETE FROM users WHERE created_at < '2020-01-01';
```

## Performance

- **Parsing Speed**: ~500 KB/s
- **Grammar Size**: 140 lines
- **Rules**: 17 parser + 19 lexer
- **Complexity**: High

## Key Concepts

1. **Optional Clauses** - `(WHERE ...)?`
2. **Case-Insensitive** - `'SELECT' | 'select'`
3. **Comments** - `--` and `/* */`
4. **Keywords** - Reserved words

## Related Examples

- **Previous**: [04-PROGRAMMING-LANGUAGE.md](04-PROGRAMMING-LANGUAGE.md)
- **Advanced**: [06-CUSTOM-ACTIONS.md](06-CUSTOM-ACTIONS.md)
