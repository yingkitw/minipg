# TODO

## Current Sprint

- [x] Create workspace structure with modular crates
- [x] Implement core traits and error handling
- [x] Implement AST definitions with visitor pattern
- [x] Implement lexer for grammar files
- [x] Implement parser for grammar files
- [x] Implement semantic analysis
- [x] Implement basic Rust code generator
- [x] Implement CLI with clap
- [x] Add insta snapshot tests
- [x] Fix parser implementation issues
- [x] Ensure cargo build succeeds
- [x] Ensure cargo test succeeds
- [x] Add comprehensive test coverage (66 tests)

## Next Steps

### Parser Improvements
- [ ] Complete grammar element parsing (all operators)
- [ ] Add support for actions and semantic predicates
- [ ] Add support for labels and rule arguments
- [x] Improve error recovery and reporting

### Analysis Enhancements
- [x] Implement indirect left recursion detection
- [x] Add reachability analysis
- [x] Detect ambiguous alternatives
- [x] Add first/follow set computation

### Code Generation
- [x] Complete Rust parser implementation
- [x] Generate proper AST types
- [x] Add visitor and listener pattern generation
- [ ] Implement error handling in generated code
- [x] Add support for other target languages (Python, JavaScript)

### Testing
- [x] Add comprehensive parser tests (12 lexer + 3 parser tests)
- [x] Add integration tests (4 end-to-end tests)
- [x] Achieve excellent test coverage (78+ total tests)
- [x] Add performance benchmarks (parser, analysis, codegen)
- [x] Add property-based tests with proptest

### Documentation
- [x] Add API documentation
- [x] Create user guide
- [x] Add grammar syntax reference
- [x] Add examples directory

### Features
- [ ] Add grammar imports
- [ ] Add grammar inheritance
- [ ] Support for Unicode character classes
- [ ] Add debugging/visualization tools
- [ ] Create VS Code extension for grammar files

## Known Issues

- Parser needs more robust error handling
- Code generator produces skeleton code only
- Need to handle more grammar edge cases

## Future Considerations

- Performance optimization
- Incremental parsing support
- Language server protocol support
- Web-based grammar playground
