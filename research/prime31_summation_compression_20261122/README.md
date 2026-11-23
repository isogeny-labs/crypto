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

[Research progress Space](https://chatgpt.com/space/page_729b0b80bdb4819193f457c80ecabe84) is saved and read back. The existing hourly task is attached there; use its exact Page ID for updates.

## Latest continuation

Six iterations are preserved. [The latest mathematical note](iteration6/MATHEMATICAL_NOTE.md) gives exact full-fibre membership for rational S3 trees, an all-node quadratic representation, explicit field guards, and free-order comb completeness. A balanced tree has a separate counterexample. [The illustrated addendum](iteration6/REPORT.pdf), [source freeze](iteration6/SOURCE_FREEZE.json), [independent written review](iteration6/INDEPENDENT_REVIEW.md) and [updated literature audit](iteration6/LITERATURE_AUDIT.md) state the precise scopes. The original-root-lock supplied-certificate correctness workflow is [run45096187034](https://github.com/isogeny-labs/crypto/actions/runs/45096187034); check its retained receipt before quoting its outcome. [pcc:unverified]

The relation-discovery term remains open. In the declared m=4 full-independent-row model with quadratic setup/matrix work, a useful next target is an exact empty-inclusive query exponent below one. Low-degree input equations alone do not supply that bound. Preserve the earlier adverse recurrence comparison, dense-state costs and full-sum-index setup costs. [pcc:unverified]

Companion PR #4924 now has a newer matching original-Rust monic-oracle correctness receipt: [run 45089472283](https://github.com/isogeny-labs/crypto/actions/runs/45089472283) at checkout 96c9101315d9f0c7ca5fee2af18cdb10d16c645d, with the same monic source blob as head 70e04851fae933675dbc4476f22dcea25090551e. This supersedes the earlier stale-source caveat for those bytes but is neither independent signed verification nor an admitted speed benchmark. [pcc:unverified]

The existing hourly task also owns [PR #4924](https://github.com/isogeny-labs/crypto/pull/4924); compare and preserve both lanes.


The companion printed scan enumerates unordered S3 pairs quadratically; its separately checked balanced construction is not the routine used by extract. The stronger maintained O(B) group-complement scan remains a required control. [pcc:unverified]

## Concurrent iteration seven

The [growing-family note](iteration7/SMOOTH_TORUS_FAMILY.md) supplies a Linnik-based smooth-p+1 prime-field family and split Dickson fibre, with ordinary/large-prime-subgroup and solver obligations explicitly separate. It is undergoing independent review; standing remains unverified. [pcc:unverified]
