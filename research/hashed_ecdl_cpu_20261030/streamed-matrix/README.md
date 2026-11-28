# Streamed relation incidence rows

The reducer now validates and stores one column index per factor-base slot,
then fills a reusable dense row for deterministic elimination. It previously
materialized every dense incidence row and cloned each consumed row. All rows
are validated before elimination, including rows after the first dependency.
Duplicate points across different slots still occupy distinct columns.

For N columns, R relations and m slots, input payload changes from 8RN bytes
of u64 entries to sizeof(usize)*Rm bytes of indices; both versions require a
dense working row. On the supported 64-bit host this is 8Rm bytes. This is a
source-derived storage bound, not measured RSS or a latency improvement.
Vector metadata and allocator overhead are excluded. Pivot tails and deferred
provenance factors retain their previous storage and may still be quadratic.
The implementation also replaces per-consumed-row dense allocation/cloning
with one working allocation and reset. Resetting that row remains O(N).

Column traversal, pivot normalization, first dependent row, back-substitution,
RNG draws and certificate format are unchanged. Both full certificate replays
remain mandatory and retain independent base/target reconstruction. The
collector still collects exactly N+1 relations under its existing policy.
This change does not reuse state across the replays.

## Evidence

42 standalone release library tests pass. Existing eager-provenance reference
comparisons check exact coefficients across primes and matrix densities; the
new streamed-incidence test checks 147 slot/size/modulus combinations against
independently constructed dense input rows. A late malformed row is rejected
even after an earlier dependency exists. Archived full certificates for all
three complete fixtures still match byte-for-byte, with replay succeeding.
Strict all-target release Clippy, formatting and the complete-example build
pass. These are software validation results, not new timing observations.

Commands (repository root; target directory outside the checkout):

```sh
export CARGO_TARGET_DIR=/tmp/cpu-projective-relations-target
cargo test --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --lib
cargo clippy --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --check
cargo build --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --example complete
```

Source ancestry: c3799d02d545198dd9d42a3550c34db3b3a13b7f. The retained round18
records and the existing deferred-provenance elimination reference test informed
this change. No new mathematical algorithm or novelty claim is made; the
change removes redundant staging in the existing exact elimination routine.
T-1021 owns the implementation and this validation directory. Round19 is
terminal and its frozen binaries were not replaced. No new cloud job was
submitted. A performance comparison requires a new frozen candidate panel;
no 10x improvement is established by this software validation.
