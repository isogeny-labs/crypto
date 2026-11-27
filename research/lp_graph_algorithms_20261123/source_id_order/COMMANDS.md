# Reproduce the source-ID regression

This is local correctness testing, not a benchmark. Use Rust 1.100.0 and one build/test thread.
The production fix and four tests are at e7084130837c91ff5c3c2b14dc058048b6a21c5e.
The source-exact harness compiles the original graph/filter modules and uses its existing operation-metering shim.

## Prepare the existing harness

Run the symlink preparation from research/lp_graph_algorithms_20261123/native_gain_dsu/run.sh, or run that original script with CARGO_BUILD_JOBS=1 and RUST_TEST_THREADS=1 for the full candidate suite. No root-crate source replacement is needed.

## Exact baseline command shape

Restore only witness.rs from ceba74bb64854027c4dd16f00428bf676383bfdb in a separate baseline checkout, keeping the four new tests:
```sh
CARGO_BUILD_JOBS=1 timeout 300s cargo test --offline --release \
  --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml \
  --test large_prime_graph m31_nonmonotone_source_ids -- --nocapture --test-threads=1
```

Observed exit 101 and exactly two failed positive regressions. The initial local harness had no committed lock; Cargo resolved cached dependencies, and the resulting exact lock is retained as raw/Cargo.lock.

## Exact candidate command shape

Use the fixed witness source and the same new tests. Restore raw/Cargo.lock to the harness in the private test checkout:
```sh
CARGO_BUILD_JOBS=1 timeout 300s cargo test --offline --locked --release \
  --manifest-path research/lp_graph_algorithms_20261123/native_gain_dsu/Cargo.toml \
  --test large_prime_graph -- --nocapture --test-threads=1
```

Observed exit 0 and 45/45 passing tests. Set CARGO_HOME to an existing compatible cached registry and CARGO_TARGET_DIR to a private test directory. The recorded local calls used explicit Rust/Cargo 1.100.0 binaries from the available toolchain; rustc-version.txt and cargo-version.txt retain exact versions. Do not treat a missing cache as permission for an alternative field, benchmark, paid job or rewritten dependency.
