# Execution adapter for macOS

The frozen parity checks, source, inputs, expected counts, and stopping rule
in `PROTOCOL.md` are unchanged. The root package's Cargo integration-test
command builds the unrelated Linux-only `icprog` binary on this macOS host
and fails before the parity test can execute. The study-local `harness/`
package depends on this checkout's `crypto` library and uses the same
`tests/rootfind_paged_curve_parity.rs` as its test target. Rust 1.99.1 is
selected explicitly for Cargo and rustc. The test source remains bound to
commit `1103716b6d`; this amendment and adapter are committed before their
first execution. [pcc:unverified]
