# Workstream 5: matched restricted-family GHS frontier package

Status: **implemented, not measured**. This directory freezes the matched
plane-quartic index-calculus versus composed strong-rho experiment. It contains
no timing data and changes no frontier or leaderboard row.

The machine contract is [`protocol.json`](protocol.json). The prospective
four-job envelopes are [`request_templates.json`](request_templates.json).
The standalone native Rust package checks both contracts and refuses
incomplete result manifests; its second binary is a deterministic analysis
skeleton for an independently normalized complete aggregate. Neither binary
submits a job.

## Exact question and boundary

On the intentionally weak family
`E_0 / F_(q^3)` with `#E_0(F_(q^3)) = 4*ell`, does the lowest complete mean
among the frozen historical IC arms `B0,S,L0,L1,L,D,DL1,DL` remain below the
composed `strong-b8-mul5-walker-table` rho reference at every one of the eleven
frozen sizes and every price `w in {0,0.1,1}`?

This is only a restricted-family question. A passing grid would **not** be a
result about generic prime curves, random binary curves, Koblitz curves,
generic prime-order extension-field curves, standardized curves, deployed
keys, or ECC2K-130. No transport to another curve is inferred without an
explicit subgroup-preserving map and its complete cost.

## Cost chain

This chain records what must be measured; it does not assert that any arrow
lowers complete cost.

```text
C(old-neg) = setup + walk + collision/recovery + verification + native work
           = C_old'                         (restructure into frozen phases; equal)
          -> C_mul5                         (replace F_(q^3) multiplication; R17)
          -> C_setup                        (replace initial-walker setup; R36)
           = C(strong-b8-mul5-walker-table) (definition of composed reference)
```

Each replacement must be priced from the composed run's raw counters at the
same `(q, seed)` calibration. Standalone q=223 counter deltas are controls, not
additive cost reductions. R64 counter removal is measured in the separate composition
workstream and cannot replace the counted reference in this grid.

For the cross-method comparison, both arms use the same target and the same
calibration:

```text
M-eq = fq_mul + r_inv*fq_inv + r_lmul*mod_ell_mul
GAE  = 56 + r_inv
C_w  = M-eq + w*GAE*band_ops
S    = C_w / sqrt(ell)
ratio = C_w(IC) / C_w(composed strong rho)
```

The comparison is cold and one-target. Reusable setup and the online-only
interval are reported separately and never replace the cold result.

## Evidence status and blockers

- The source catalog service was unavailable. The fallback source receipt is
  frozen in `protocol.json`: the repository literature review, the original
  GHS protocol/amendment, the R0-R99 report, and the implementation spec.
- No source revision, binary digest, image digest, live budget receipt, pool
  receipt, checkpoint, or result manifest is present here.
- The current serial producer emits complete counted setup/walk/verification
  vectors but not every required cold/online phase interval. The native
  validator therefore requires explicit phase-coverage fields in a future result
  manifest and refuses promotion when any is false.
- The template names the prospective `run_split.sh`, but the runner is absent
  and `request_templates.json` records `present=false`. More importantly, the
  current custom command-job bridge cannot earn L2 with native replay and A/A,
  so `l2_admission_capable=false` is a machine-validated blocker. The request
  objects are planning templates, not launch envelopes, until both fields can
  be prospectively changed with the reviewed bridge and runner in the tree.
- The historical IC archive remains bound to SHA-256
  `aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42`
  and 382,105 bytes. A hydrated file is not accepted without that identity.
- Execution remains behind the repository checkpoint, live budget, all-tenant
  pool, Git-custody, digest-pinned image, replay, independent validation, PCC,
  benchmark-record, and publication gates.

## Local checks

These checks inspect contracts and synthetic fixtures only; they are not
benchmarks.

```sh
cargo test --manifest-path \
  research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml

cargo run --quiet --manifest-path \
  research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml \
  --bin ghs-m3-validate -- protocol \
  research/ghs_frontier_3_7_20261119/ws5_matched_ghs/protocol.json \
  research/ghs_frontier_3_7_20261119/ws5_matched_ghs/request_templates.json
```

An actual result is not admissible until `ghs-m3-validate results` accepts its
sealed manifest. `ghs-m3-analyze` is deliberately only the post-normalization
cost/selection skeleton: the independent raw-record validator must first
recompute the clustered intervals and bind its receipt; supplied intervals are
then checked for census, ordering, and containment of the recomputed point.

