# Testing Against grammars-v4 Repository

This document describes how to test minipg against all grammars from the [ANTLR grammars-v4 repository](https://github.com/antlr/grammars-v4).

## Overview

The `test_grammars_v4_all.rs` test suite provides comprehensive testing against real-world ANTLR4 grammars from the official grammars-v4 repository, which contains 200+ grammars for various languages and formats.

## Test Files

- `tests/test_grammars_v4_all.rs` - Comprehensive tests against all grammars-v4 grammars
- `tests/test_grammars_v4_compatibility.rs` - Subset tests for specific grammars

## Running the Tests

### Basic Usage

```bash
# Run all grammars-v4 tests (requires git and network access)
cargo test --test test_grammars_v4_all -- --nocapture

# Run popular grammars only (faster)
cargo test --test test_grammars_v4_all test_popular_grammars_v4 -- --nocapture

# Run by category
cargo test --test test_grammars_v4_all test_grammars_by_category -- --nocapture
```

### Offline Mode

If you've already cloned the repository, you can skip the download:

```bash
# Use cached grammars (no download)
GRAMMARS_V4_SKIP_DOWNLOAD=1 cargo test --test test_grammars_v4_all -- --nocapture
```

### Manual Setup

You can also manually clone the repository:

```bash
git clone --depth 1 https://github.com/antlr/grammars-v4.git grammars-v4-cache
GRAMMARS_V4_SKIP_DOWNLOAD=1 cargo test --test test_grammars_v4_all
```

## Test Modes

### 1. All Grammars Test

Tests all `.g4` files found in the grammars-v4 repository:

```bash
cargo test --test test_grammars_v4_all test_all_grammars_v4 -- --nocapture
```

**Features**:
- Automatically clones/updates grammars-v4 repository
- Recursively finds all `.g4` files
- Tests parsing each grammar
- Provides detailed statistics
- Reports failures with error messages

**Output**:
```
Finding all .g4 files in grammars-v4-cache...
Found 523 grammar files
[1/523] Testing java/java9/Java9.g4... ✓
[2/523] Testing python/python3/Python3.g4... ✓
...
================================================================================
GRAMMARS-V4 TEST SUMMARY
================================================================================
Total grammars tested: 523
Passed: 485 (92.7%)
Failed: 38 (7.3%)
Skipped: 0
================================================================================
```

### 2. Popular Grammars Test

Tests a curated list of popular/important grammars (faster for CI):

```bash
cargo test --test test_grammars_v4_all test_popular_grammars_v4 -- --nocapture
```

**Grammars tested**:
- Java (Java9.g4)
- Python (Python3.g4)
- C++ (CPP14.g4)
- C# (CSharp.g4)
- JavaScript (JavaScriptParser.g4)
- TypeScript (TypeScriptParser.g4)
- SQL (TSqlParser.g4)
- GraphQL (GraphQL.g4)
- JSON (JSON.g4)
- XML (XMLParser.g4)

### 3. Category-Based Testing

Tests grammars organized by category:

```bash
cargo test --test test_grammars_v4_all test_grammars_by_category -- --nocapture
```

**Categories**:
- Programming Languages (Java, Python, C++, C, C#, JavaScript, TypeScript, Go, Rust, Swift)
- Data Formats (JSON, XML, YAML, CSV, TOML)
- Query Languages (SQL, SPARQL, Cypher, GraphQL)
- Markup (HTML, Markdown, CSS)
- Configuration (INI, Properties, Config)

## Test Statistics

The test suite provides detailed statistics:

- **Total**: Number of grammars tested
- **Passed**: Number successfully parsed
- **Failed**: Number that failed to parse
- **Skipped**: Number skipped (not found)
- **Failure Rate**: Percentage of failures

## Understanding Failures

When a grammar fails to parse, the test reports:
- Grammar file path
- Error message from the parser

Common failure reasons:
1. **Unsupported ANTLR4 features**: Some grammars use features not yet implemented
2. **Grammar syntax errors**: Some grammars may have syntax issues
3. **Complex patterns**: Very complex grammars may hit parser limitations
4. **File encoding issues**: Some grammars may have encoding problems

## CI/CD Integration

For CI/CD pipelines, use the popular grammars test:

```yaml
# GitHub Actions example
- name: Test grammars-v4 compatibility
  run: |
    cargo test --test test_grammars_v4_all test_popular_grammars_v4 -- --nocapture
  env:
    GRAMMARS_V4_SKIP_DOWNLOAD: 0
```

Or skip if network access is limited:

```yaml
- name: Test grammars-v4 compatibility (cached)
  run: |
    cargo test --test test_grammars_v4_all test_popular_grammars_v4 -- --nocapture
  env:
    GRAMMARS_V4_SKIP_DOWNLOAD: 1
```

## Performance

- **All grammars**: ~5-10 minutes (depends on network and system)
- **Popular grammars**: ~30 seconds
- **Category-based**: ~2-5 minutes

## Troubleshooting

### Git not found
If git is not available, the test will skip. Install git or use cached grammars.

### Network issues
Use `GRAMMARS_V4_SKIP_DOWNLOAD=1` to use cached grammars.

### Permission errors
Ensure write permissions for the current directory (to create `grammars-v4-cache/`).

### Out of memory
For very large test runs, consider:
- Testing by category
- Testing popular grammars only
- Increasing system memory

## Contributing

When adding support for new ANTLR4 features:

1. Run the full test suite: `cargo test --test test_grammars_v4_all`
2. Check which grammars now pass
3. Update this documentation if needed
4. Consider adding new grammars to the "popular" list

## References

- [grammars-v4 Repository](https://github.com/antlr/grammars-v4)
- [ANTLR4 Documentation](https://github.com/antlr/antlr4)
- [minipg ANTLR4 Compatibility](docs/ANTLR4_COMPATIBILITY.md)

---

**Last Updated**: 2025-01-XX
**Status**: Active testing suite

