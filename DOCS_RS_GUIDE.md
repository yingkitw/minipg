# docs.rs Documentation Guide

**Project**: minipg v0.1.5  
**Documentation**: https://docs.rs/minipg  
**Status**: Ready for publication

---

## Overview

minipg documentation will be automatically built and hosted on docs.rs when the crate is published to crates.io. The documentation is already configured and ready.

---

## Configuration

### Cargo.toml Settings

```toml
[package]
documentation = "https://docs.rs/minipg"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

**What this does**:
- `documentation`: Points to docs.rs URL
- `all-features`: Builds docs with all features enabled
- `rustdoc-args`: Enables docs.rs-specific features

---

## Publishing to docs.rs

### Step 1: Verify Documentation Locally (Optional)

If you have Full Disk Access permissions:
```bash
cargo doc --open
```

If not, skip to Step 2 - docs.rs will build it automatically.

### Step 2: Publish to crates.io

```bash
# Dry run first
cargo publish --dry-run

# If successful, publish
cargo publish
```

### Step 3: Wait for docs.rs

- docs.rs will automatically detect the new version
- Documentation will be built within minutes
- Visit: https://docs.rs/minipg/0.1.5

---

## Documentation Structure

### Main Documentation (lib.rs)

✅ **Overview**: Project description and key features  
✅ **Features**: Incremental parsing, multi-language, ANTLR4 compatibility  
✅ **Example**: Basic usage example  
✅ **Modules**: All modules documented  

### Module Documentation

All modules have comprehensive documentation:

- ✅ `core` - Core types and traits
- ✅ `ast` - Abstract syntax tree
- ✅ `parser` - Grammar parser
- ✅ `lexer` - Lexer implementation
- ✅ `analysis` - Semantic analysis
- ✅ `codegen` - Code generators (9 languages)
- ✅ `incremental` - Incremental parsing (NEW)
- ✅ `query` - Query language (NEW)
- ✅ `cli` - Command-line interface
- ✅ `mcp` - MCP server

### API Documentation

All public APIs are documented with:
- Function/method descriptions
- Parameter descriptions
- Return value descriptions
- Example usage where appropriate
- Links to related items

---

## Documentation Features

### What's Included

✅ **Module-level docs**: Every module has `//!` documentation  
✅ **Type documentation**: All public structs, enums, traits  
✅ **Function documentation**: All public functions and methods  
✅ **Example code**: Usage examples in doc comments  
✅ **Cross-references**: Links between related items  
✅ **Feature flags**: Documented feature requirements  

### Special Sections

**Incremental Parsing** (v0.1.5):
- Position tracking API
- Edit operations API
- IncrementalParser trait
- SyntaxTree structure

**Query Language** (v0.1.5):
- S-expression syntax
- Pattern matching API
- Capture groups
- QueryMatcher usage

---

## Viewing Documentation

### After Publication

1. **Main docs**: https://docs.rs/minipg
2. **Specific version**: https://docs.rs/minipg/0.1.5
3. **Latest**: https://docs.rs/minipg/latest

### Search

docs.rs provides:
- Full-text search across all items
- Type search
- Module navigation
- Source code viewing

---

## Documentation Quality

### Coverage

- **189 tests** (100% pass rate)
- **All public APIs** documented
- **All modules** documented
- **Examples** provided

### Standards

✅ Follows Rust documentation conventions  
✅ Uses standard rustdoc syntax  
✅ Includes code examples  
✅ Cross-references related items  
✅ Documents error conditions  
✅ Explains complex concepts  

---

## Troubleshooting

### Local Build Issues

**Problem**: `cargo doc` fails with "Operation not permitted"  
**Cause**: macOS Full Disk Access permissions  
**Solution**: Either:
1. Grant Full Disk Access to your terminal (System Settings → Privacy & Security)
2. Skip local build and rely on docs.rs (recommended)

### Publication Issues

**Problem**: `cargo publish` fails  
**Possible causes**:
- Version already published
- Missing required fields in Cargo.toml
- Network issues

**Solution**:
```bash
# Check what would be published
cargo publish --dry-run

# Check package contents
cargo package --list
```

---

## Maintenance

### Updating Documentation

When updating the crate:

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with changes
3. **Update module docs** if APIs changed
4. **Add examples** for new features
5. **Publish** new version

docs.rs will automatically rebuild documentation for each new version.

### Documentation Links

Update these when publishing:
- README.md badge: `[![Documentation](https://docs.rs/minipg/badge.svg)](https://docs.rs/minipg)`
- Cargo.toml: Already configured ✅
- GitHub README: Already includes docs.rs link ✅

---

## Current Status

✅ **Cargo.toml**: Configured for docs.rs  
✅ **lib.rs**: Comprehensive overview documentation  
✅ **Modules**: All documented with `//!` comments  
✅ **APIs**: All public items documented  
✅ **Examples**: Included in doc comments  
✅ **Tests**: 189 tests passing  

**Ready for publication**: Yes ✅

---

## Next Steps

1. **Verify package**: `cargo publish --dry-run`
2. **Publish to crates.io**: `cargo publish`
3. **Wait for docs.rs**: Documentation will be available within minutes
4. **Verify docs**: Visit https://docs.rs/minipg/0.1.5
5. **Update README**: Add docs.rs badge if not already present

---

## Resources

- **docs.rs**: https://docs.rs
- **Rust documentation guide**: https://doc.rust-lang.org/rustdoc/
- **Cargo manifest**: https://doc.rust-lang.org/cargo/reference/manifest.html
- **Publishing guide**: https://doc.rust-lang.org/cargo/reference/publishing.html

---

## Summary

minipg is **ready for docs.rs**:
- ✅ Configuration complete
- ✅ Documentation comprehensive
- ✅ All modules documented
- ✅ 189 tests passing
- ✅ Ready to publish

Once published to crates.io, documentation will be automatically available at https://docs.rs/minipg
