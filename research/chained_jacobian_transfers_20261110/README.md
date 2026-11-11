# Chained Jacobian transfers

Current continuation: [report](REPORT_20261111.md), [PDF](report.pdf),
[persistent queue](QUEUE.md), [sources](SOURCES_20261111.md),
[GitHub #3904](https://github.com/isogeny-labs/crypto/issues/3904).

`native/` is the execution path for new development. It is a standalone,
dependency-free Rust crate for the exact public F101 fixture. The historical
Python files, protocol and 2026-11-10 receipt are retained evidence; their
old status paragraphs do not describe the continuation. Do not extend or use
those Python paths for new comparisons.

```sh
cargo test --release --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml -- --nocapture
cargo clippy --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml --all-targets -- -D warnings
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/PolynomialIdentities.lean
```

The cross-repository reference is
`cryptanalysis:experiments/chained-transfer-reference/`. The comparison uses
pre-existing BigUint arithmetic after a faithful monic chart change; both
implementations use Cantor and the runs are by this producer. This is
differential development evidence, not signed independent verification.
All current claim standings must be read through `tools/pcc/pcc.py standing`.
[pcc:unverified]

The fixed fixture now has full rational source maps, a genuine Richelot hop,
its sign-corrected dual, nonzero and zero endpoint controls, and a complete
finite-group deduplication against an elliptic shortcut. The report gives the
precise scope and open geometric proof bridge. No performance run or speed
claim is present. [pcc:unverified]

Benchmark/integration blockers have a
[named record](../../problems/chained-transfer-admission-and-ledger-sync.md).
Root-library checks and the new focused CI are tracked in the session log and
PR. A source checkpoint does not waive their gates.
