# Remove redundant batch-inversion endpoint products

The CPU residual kernel now skips three modular multiplications per active
batch: the initial product 1*d, the final inverse-times-prefix-one product,
and the final running-inverse update that no later entry consumes. This is a
source-derived operation reduction, not an elapsed-time or machine-instruction
measurement. Branch costs and compiler optimization affect the eventual timing.

For k nonzero denominators, the Montgomery batch inversion scaffolding uses
(k-1) forward and 2(k-1) reverse multiplications, rather than k forward and 2k
reverse multiplications. Both versions perform one field_inverse request;
the affine slope and coordinate products are unchanged. Zero-denominator
exceptional entries do not count toward k. At the first active entry the
prefix is exactly one, even when exceptional entries precede it. When k=1,
no forward/reverse scaffolding product is necessary.

Identity handling, inverse pairs, doubling equations, residual order, table
construction, batch scheduling and inversion-counter semantics are unchanged.
The same kernel serves tuple-table construction and query batches. Both full
certificate replays remain mandatory; their source was not changed here.
No new allocation or persistent cache is introduced.

## Validation

43 standalone release library tests pass, including archived byte-identical
full certificates for all three complete fixtures. Existing exhaustive group
pair and wide-field checks still pass. The new test compares each residual
against checked affine addition and checks the inversion-request counter for
six batch shapes, 24 targets and three fields (432 batch cases, each also
reused with an identity target). It covers empty/all-exceptional batches, one
ordinary residual, doubling, exceptional prefixes/suffixes and scratch reuse.
Strict all-target release Clippy, formatting and the complete-example build
pass. Logs and source/binary hashes are retained beside this document.

Commands, from the repository root:

```sh
export CARGO_TARGET_DIR=/tmp/cpu-projective-relations-target
cargo test --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --lib
cargo clippy --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --check
cargo build --offline --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml --example complete
```

Parent: a6117b9ec7743d879223806b1cc28556ba83903e. T-1022 owns this change.
The existing Montgomery batch-inversion implementation and its independent
checked-affine tests supplied the derivation and correctness reference; no
new group formula or mathematical novelty is claimed. No new cloud attempt
or performance measurement was run. Round19's frozen binaries remain intact.
The 10x further-improvement goal still requires controlled end-to-end evidence.
