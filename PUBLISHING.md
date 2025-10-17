# Publishing Guide for minipg v0.1.0-alpha.1

This document provides instructions for publishing minipg to crates.io.

## Pre-Publishing Checklist

- [x] Version updated to `0.1.0-alpha.1` in workspace Cargo.toml
- [x] Pig mascot added to README
- [x] All crates have required metadata (description, homepage, documentation, readme)
- [x] LICENSE file created (Apache-2.0)
- [x] `cargo build --all` passes
- [x] `cargo test --all` passes (68 tests passing)
- [x] `cargo package --allow-dirty` verification passes

## Publishing Order

Crates must be published in dependency order:

1. **minipg-core** (no internal dependencies)
2. **minipg-ast** (depends on minipg-core)
3. **minipg-parser** (depends on minipg-core, minipg-ast)
4. **minipg-analysis** (depends on minipg-core, minipg-ast)
5. **minipg-codegen** (depends on minipg-core, minipg-ast, minipg-analysis)
6. **minipg-cli** (depends on all above)

## Publishing Commands

Before publishing, ensure you're logged in to crates.io:

```bash
cargo login
```

Then publish each crate in order:

```bash
# 1. Publish minipg-core
cd crates/minipg-core
cargo publish

# 2. Publish minipg-ast
cd ../minipg-ast
cargo publish

# 3. Publish minipg-parser
cd ../minipg-parser
cargo publish

# 4. Publish minipg-analysis
cd ../minipg-analysis
cargo publish

# 5. Publish minipg-codegen
cd ../minipg-codegen
cargo publish

# 6. Publish minipg-cli
cd ../minipg-cli
cargo publish
```

## Post-Publishing

After publishing:

1. Verify all crates are visible on crates.io
2. Test installation: `cargo install minipg-cli`
3. Create a git tag: `git tag v0.1.0-alpha.1`
4. Push the tag: `git push origin v0.1.0-alpha.1`
5. Create a GitHub release with release notes

## Notes

- This is an **alpha release** - expect breaking changes
- The benchmarks crate is excluded from publishing (development only)
- All crates share the same version number (0.1.0-alpha.1)
- Repository URL should be updated to your actual GitHub repository before publishing
