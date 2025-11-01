# Fuzzing Setup for minipg

This directory contains fuzzing targets for coverage-guided fuzzing using `cargo-fuzz`.

## Prerequisites

Install `cargo-fuzz`:

```bash
cargo install cargo-fuzz
```

## Running Fuzzing

To run the fuzzing target:

```bash
cd fuzz
cargo fuzz run fuzz_parser
```

This will run the fuzzer indefinitely. Stop it with Ctrl+C.

## Fuzzing Targets

- **fuzz_parser**: Fuzzes the grammar parser with arbitrary byte inputs, converting valid UTF-8 to strings and testing parser robustness.

## Configuration

Fuzzing configuration is in `fuzz/Cargo.toml`. You can customize:
- Timeouts
- Memory limits
- Dictionary files
- Custom mutators

## Corpus

The fuzzer maintains a corpus of interesting inputs in `fuzz/corpus/fuzz_parser/`. You can seed this directory with known valid/invalid inputs to improve fuzzing coverage.

## Output

Fuzzing results are stored in `fuzz/artifacts/fuzz_parser/`. Crash inputs are saved here for analysis.

## Integration with CI

For continuous fuzzing, consider:
- OSS-Fuzz (for open source projects)
- ClusterFuzz
- Local fuzzing in CI pipelines with time limits

## Manual Fuzzing

You can also manually test with specific inputs:

```bash
cargo fuzz run fuzz_parser -- corpus/fuzz_parser/test_input.txt
```

