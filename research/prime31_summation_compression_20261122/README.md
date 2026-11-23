# Prime31 Dickson composition relation extraction

This research lane develops an exact denominator-free S3 membership oracle on the existing ordinary elliptic curve over `p = 2147483647`. The candidate preserves a composed Dickson factor-base constraint through elimination, rather than expanding the lift-filtered membership polynomial before evaluation. The immediate deliverable is a Rust component and correctness evidence. The asymptotic relation-solving objective remains open. [pcc:unverified]

- Tracking PR: https://github.com/isogeny-labs/crypto/pull/4928
- Branch: `lane/prime31-dickson-quotient-gate`
- [Mathematical derivation and cost analysis](REPORT.md)
- [Prospective benchmark protocol](PROTOCOL.md)
- [Persistent state and next action](STATE.json)
- [Illustrated report](REPORT.pdf)
- [New Rust example](../prime31_ic_20261115/native/examples/prime31_dickson_quotient.rs)
- [Development evidence](evidence/README.md)

The existing arithmetic and factor-base implementation are reused. The reference source snapshot is `26cc445bdc6b0b3518535d5f7b8b4ec6cc08fb19`; this lane was created from `e1111deaaee18ebcb346f978a23f2cfc0fbc9e6b`. Exact imported source hashes and the dependency qualification are recorded with the development receipts. No original source file or original Cargo lock is replaced.

## Reproduce the focused check

```sh
cargo +1.100.0 test --locked --release \
  --manifest-path research/prime31_ic_20261115/native/Cargo.toml \
  --example prime31_dickson_quotient
```

The original locked build needs dependencies unavailable in the current offline cache. Development checks therefore use a separately recorded cached-dependency adapter with the original source bytes. A later [original-lock CI replay](https://github.com/isogeny-labs/crypto/actions/runs/45089569480) passed 41 library tests and all 5 component tests at source commit `109fc024e33f122fd678d2c93a18b2c0438a8668`; the diagnostic JSON matches the local output byte-for-byte. EKS admission for performance work, independent signed execution standing and formal Lean verification remain pending. [pcc:unverified]

## Scope and continuation

The automated continuation must read `STATE.json`, the latest session log, relevant live PRs, and the current repository instructions before acting. It should retain the completed original-lock correctness check and pursue the missing exact sublinear empty-root operation. Any subsequent empirical comparison follows the prospective protocol and admission requirements. A fast evaluation circuit alone is not a fast root finder.

The research Space creation request has an uncertain outcome because the Spaces service returned no receipt. Search for the exact name `Prime Field Summation Polynomial Research` before any further creation; do not make a duplicate.

## Latest continuation

The objective remains open. Three iterations are preserved: an exact tested recurrence that loses the sparse-product diagnostic; the full-curve scalar degree obstruction; and a conditional tree-search bound exposing sublinear empty-root certification as the missing operation. See [STATE.json](STATE.json), [NEXT_ROOT_INTERSECTION.md](NEXT_ROOT_INTERSECTION.md), and [CONTINUATION.md](CONTINUATION.md). Claims remain unsigned and unverified. [pcc:unverified]

The existing hourly task also owns [PR #4924](https://github.com/isogeny-labs/crypto/pull/4924); compare and preserve both lanes.
