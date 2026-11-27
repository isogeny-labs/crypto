# Pushed-Git production checkpoint

## Goal

Measure the target-specialized Dickson/Semaev quadratic norm-gcd primitive at
depths 1 through 6, independently verify every fixed public query, and apply
the preregistered correctness, subquadratic-growth, direct-baseline, cold
budget, and concrete-composition gates.

## Frozen inputs

- `PROTOCOL.md` and `PROOF_MODEL.md` in this directory.
- Curve `icv1-fp31-tm3989-ae9db972` and its sealed depth-1--6 support hashes.
- Eleven deterministic queries per depth; `seed=none`.
- Three frozen field-equivalent panels, 27,370 residual queries, and a cold
  ceiling of 1.6 million field-equivalent operations.
- No cross-target amortization and no generic factor-base substitution.

## State before launch

The dependency-free Rust candidate, separately coded pointwise verifier, and
frozen analyzer are implemented. Development was capped at depths 1 and 2.
Formatting, strict Clippy, all 14 unit tests, and a fresh depth-2
candidate/verifier/analyzer replay pass. No depth-3--6 candidate output or
production evidence exists at this checkpoint.

The implemented object is only the two-leaf predicate `R in A_t+A_t`. The
analyzer records the four-leaf composition as absent and cannot promote the
primitive to same-target batching or m=7 on polynomial slope alone.

## Sole launch command

From the repository root:

```sh
research/pplus_dickson_norm_gcd_20261127/run_census.sh
```

The runner refuses an existing `evidence/` directory, dirty or detached Git
state, a local branch not exactly pushed to the same branch on `origin`, a
source-manifest mismatch, any failed stage, or an invalid analyzer record. It
builds only after admission and runs candidate, verifier, and analyzer exactly
once each.

## Expected artifacts

On success, `evidence/` contains candidate, verifier, and analyzer standard
output/error; elapsed-time diagnostics with RSS explicitly unavailable; run
metadata; and `SHA256SUMS`. These artifacts are public deterministic replay,
not a zero-knowledge proof or host attestation.

## Stop and success conditions

Correctness failure stops immediately. Otherwise promotion requires every
frozen gate: strict subquadratic depth transitions and slope, at least a 2x
depth-6 win against the strong one-inversion batched scan in all three panels,
and a written complete four-leaf composition under the cold ceiling in every
panel. The currently absent composition forces a scoped primitive-only stop
even if earlier gates pass. Failure grants no permission to pool unrelated
targets, change the query panel, or claim a general lower bound.
