# Exact energy, reduction and scaling controls

Standalone Rust exact arithmetic and finite-control library for the proposed
[four-sum energy gap test](../../research/curve25519_torus_native_20261115/energy-test-v1/PROOF_DRAFT.md).
It is not a decomposition solver or a measured base-sampling campaign.

```sh
cargo test --release --manifest-path tools/torus-energy/Cargo.toml
cargo run --release --manifest-path tools/torus-energy/Cargo.toml -- calibrate /tmp/new-energy-calibration
```

The output directory must be new. Pre-call records bind the executing binary;
the result records exact binomial-tail numerators/denominators and a native
integer plan for the actual Curve25519 torus parameters. It explicitly records
that no base samples or solver were invoked. Failure to meet any arithmetic
threshold is an error, not a silently weakened statistical promise.

`block_has_collision` compares full supplied labels and rejects incomplete
blocks. `accept_blocks` enforces the fixed 8192-block frame and at most 341
collision blocks. Callers must supply independent target samples under the
declared law; these APIs do not prove randomness, correct curve encoding,
tuple generation or group arithmetic. No large-base sampler is run or claimed.

Controls check exact binomial counts against finite exhaustive cases, full
finite-sample collision moments (including heavy atoms), resource bounds,
frame/threshold mutations and exact actual-instance planning arithmetic.
Confidence applies per frozen base; adaptive attempts need a registered cap
and error correction. The probability argument and its torus application
remain unverified pending the named review/formalization obligations.

`examples/rankfree_control.rs` contains dimension-capped dense kernel controls:
all125 F5 tag vectors, the tag-dependent counterexample and two actual-curve
known-secret fixtures. It does not implement the cited sparse kernel bound
or discover an unknown logarithm. See the frozen
[proposal](../../research/curve25519_torus_native_20261115/energy-test-v1/RANKFREE_REDUCTION.md).
PCC-20261116-ab1a1d and f0439a. [pcc:unverified]

`examples/scaling_audit.rs` performs exact rational/integer arithmetic only.
It caps modeled success at one and records numeric examples and proposed
many-coset products, with admissibility and actual algorithm costs left
unchecked. Its output directory must be new and its pre-call record hashes
the immutable executing copy. See the
[model/parameter report](../../research/curve25519_torus_native_20261115/reports/scaling-audit-v1/REPORT.md).
PCC-20261116-ebfd31, 76dc04 and d45270. [pcc:unverified]

Frozen controls bind their version-specific archives, not a mutable live
template. The final template contains an equivalent range-style lint fix;
publication-v1 retains the earlier failure, complete source hashes and a
new source-v4 snapshot. None of these checks is a timing experiment.
