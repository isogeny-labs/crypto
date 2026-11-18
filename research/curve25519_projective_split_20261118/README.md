# Curve25519 projective split qualification (in progress)

The protocol is frozen and the first native Rust implementation now passes
three release-mode correctness panels on the repository's real
`f4_fp_tower` engine.  No performance cell or exponent fit has run yet, so
there is no optimization or subexponential claim. `[pcc:unverified]`

## Implemented arm

`pkm-semaev:mode=projective,m=3` replaces the affine intermediate abscissa
with normalized homogeneous coordinates `(X:Z)`.  The Boolean rule `Z^2=Z`
selects the affine chart `Z=1` or the identity chart `(1:0)`; the polynomial
`(1-Z)(X-1)` excludes `(0:0)`.  Both adjacent `S3` equations are
bihomogenized by clearing squared denominators.  Solver consistency still
goes through the existing exhaustive extraction, sign lift, and exact group
re-addition before a relation is returned.

## Current evidence

The exact commands and hashes are in
[`raw/qualification-tests.json`](raw/qualification-tests.json).

- The inverse-pair/identity chart and `(0:0)` rejection test passed.
- Six planted decompositions passed in affine-chain, projective-chain, full
  `S4`, and exhaustive modes; every returned row re-added to its target.
- On 40 deterministic random/planted targets, projective F4, affine F4,
  full `S4`, and the exhaustive grid agreed on every verdict.

The task branch itself still encounters the 35 unrelated root errors already
tracked in `docs/ACTIVE.md`.  For isolation, the release test binary used a
disposable worktree at compile-repair PR #4650, the exact projective diff, and
local shims only for those unrelated duplicate/drift failures.  This is
stronger than the earlier source-only diagnostic, but it does not clear the
task branch's root gate. `[pcc:unverified]`

## Next gate

Per-target F4 counters have been added to `curve25519_family f4cost` behind
the opt-in `F4COST_TARGET_TRACE=1`.  The
frozen qualification grid is `k=20`, `m=3`, `t=3..6`, 32 matched random
targets and 8 planted targets per cell.  The candidate advances to a new,
separately frozen `m=4` exponent protocol only if all relations verify, no
cell has worse censoring, and the upper paired 95% bound on its F4-cost ratio
is below 0.80.  The actual subexponential endpoint remains stricter: the
`m=4` cost-slope upper bound must be below 0.5 and the complete attack must
also bound setup, collection, linear algebra, and target recovery by
`r^o(1)`.
