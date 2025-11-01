# Publishing Guide for minipg

This document provides instructions for publishing minipg to crates.io.

## Pre-Publishing Checklist

- [x] Version updated in Cargo.toml
- [x] Package has required metadata (description, homepage, documentation, readme)
- [x] LICENSE file exists (Apache-2.0)
- [x] `cargo build` passes
- [x] `cargo test` passes (102+ tests, 100% pass rate)
- [x] `cargo publish --dry-run` succeeds

## Publishing Commands

### 1. Login to crates.io

```bash
cargo login
```

Enter your crates.io API token when prompted. Get your token from: https://crates.io/me

### 2. Verify Package

```bash
cargo publish --dry-run
```

This verifies the package can be published without actually publishing it.

### 3. Publish

```bash
cargo publish
```

## Troubleshooting HTTP2 Errors

If you encounter `Error in the HTTP2 framing layer`:

### Solution 1: Retry
Network issues are often transient. Simply retry:
```bash
cargo publish
```

### Solution 2: Disable HTTP2 (use HTTP/1.1)
Set environment variable to force HTTP/1.1:
```bash
CARGO_HTTP2=false cargo publish
```

### Solution 3: Check Network/Proxy
- Ensure stable internet connection
- Check if behind corporate proxy/firewall
- Try from different network if possible

### Solution 4: Use Alternative Method
If persistent issues, try publishing via web interface:
1. Create the crate on crates.io website first
2. Then use `cargo publish`

## Post-Publishing

After successful publishing:

1. **Verify on crates.io**: https://crates.io/crates/minipg
2. **Test installation**: 
   ```bash
   cargo install minipg
   ```
3. **Create git tag**:
   ```bash
   git tag v0.1.3
   git push origin v0.1.3
   ```
4. **Create GitHub release** with release notes from CHANGELOG.md

## Current Status

- **Current Version**: 0.1.3
- **Status**: Ready for publishing
- **Tests**: 102+ tests passing (100% pass rate)
- **Build**: ✅ Success
- **Dry-run**: ✅ Success

## Notes

- Version numbers should follow semantic versioning (major.minor.patch)
- For alpha/beta releases, use version like `0.1.3-alpha.1`
- Always test with `--dry-run` before actual publish
- Check crates.io status if experiencing persistent issues

---

**Last Updated**: Current
