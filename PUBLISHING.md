# Publishing Guide for minipg v0.1.0-alpha.1

This document provides instructions for publishing minipg to crates.io.

## Pre-Publishing Checklist

- [x] Version updated to `0.1.0-alpha.1` in Cargo.toml
- [x] Pig mascot added to README
- [x] Package has required metadata (description, homepage, documentation, readme)
- [x] LICENSE file created (Apache-2.0)
- [x] `cargo build` passes
- [x] `cargo test` passes (32 tests passing, 9 ignored)
- [x] Consolidated to single crate for easier publishing
- [x] All documentation updated

## Publishing (Single Crate)

Since minipg is now a single consolidated crate, publishing is simple:

## Publishing Commands

### 1. Login to crates.io

```bash
cargo login
```

### 2. Verify Package

```bash
cargo publish --dry-run
```

### 3. Publish

```bash
cargo publish
```

That's it! Just one command to publish the entire package.

## Post-Publishing

After publishing:

1. Verify the crate is visible on crates.io: https://crates.io/crates/minipg
2. Test installation: `cargo install minipg`
3. Create a git tag: `git tag v0.1.0-alpha.1`
4. Push the tag: `git push origin v0.1.0-alpha.1`
5. Create a GitHub release with release notes from ALPHA_RELEASE_NOTES.md

## Notes

- This is an **alpha release** - expect breaking changes
- Single consolidated crate for easy installation
- Version: 0.1.0-alpha.1
- 32 tests passing (9 ignored due to alpha limitations with complex ANTLR4 features)
- Repository: https://github.com/yingkitw/minipg
