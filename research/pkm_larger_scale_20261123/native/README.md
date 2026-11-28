# Native frozen prime workloads

This Rust tool generates and validates **known-answer development fixtures**.
It uses the repository's original `factor-base` arithmetic/curve validator;
there is no Python execution path and no replacement F4 or IC implementation.

```
cargo +1.99.1 test --release --locked --manifest-path research/pkm_larger_scale_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --manifest-path research/pkm_larger_scale_20261123/native/Cargo.toml
```

The CLI is `pkm-prime-workload generate CURVE.json NEW_WORKLOAD.json COUNT SEED DOMAIN`
or `pkm-prime-workload validate WORKLOAD.json`. A curve file is an existing
`factor_base::arithmetic::CurveSpec`. Outputs refuse to replace existing files.
The committed `fixtures/` are F17 unit fixtures, not a larger-scale experiment.
`qualify-frozen.sh NEW_RECEIPT_DIR` is a thin development-test qualifier; its
source hash is retained before calls and all listed inputs are checked after.

Generation uses the explicitly named SHA256-counter/rejection-v1 stream, with
length-delimited domain, u64 little-endian seed/counter and rejection sampling
without replacement in 1..r. Use the protocol's per-curve domain names. A label
alone does not establish that a curve or scalar panel is an untouched holdout;
the population audit and exclusion manifest remain required.

The schema pins p,a,b,r,h,P, every scalar and canonical Q=[k]P. Imports reject
unknown fields, empty/oversized panels, binary fields, p>=2^32, invalid subgroup
parameters, duplicate/out-of-range scalars, and mismatched target points.
Input bytes are capped at 1 MiB. Exact byte SHA-256 is reported; changing even
whitespace changes that identity. A larger curve's group order can remain
`caller_asserted` in the inherited validator: retain this status and supply the
protocol's order certificate before scientific admission.

The production adapter adds `ic bench --prime-workload FILE`. It replaces the
curve source/repeat count with the immutable document, uses the same scalar
list for the IC and rho references, records its hash and inherited validation
status, and cross-checks every Q with the actual solver's native backend.
`--seed` changes solver randomness without regenerating the curve or targets.
`--rho-runs` must be 0 or the panel size, so a subset reference is not silently
used. Generated curve, family, repeat, auto-planning and sweep flags conflict
with this fixed source. Existing non-workload paths keep their target formula.

The adapter's integration tests are in `src/bin/ic/bench.rs`. They require the
full root crate; standalone parser/generator tests do not establish consumer
qualification. Six shared-module tests and CLI immutability checks passed in
`../native-check-01/`; consult `../CONTINUATION.md` for the root-build result.
No new scale timing, independent-rank comparison or source replay is supplied
by this development fixture.
