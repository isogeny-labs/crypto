# Curve25519 projective split: current status

The `m = 3` projective split implementation now passes its native correctness
panels directly on current `main`. The frozen performance qualification grid
has not run, no cost slope has been fitted, and this study makes no
subexponential Curve25519 ECDLP claim. `[pcc:unverified]`

## Implemented arm

`pkm-semaev:mode=projective,m=3` replaces the affine intermediate abscissa
with normalized homogeneous coordinates `(X:Z)`. The Boolean quotient rule
`Z^2 = Z` selects the affine chart `Z = 1` or the identity chart `(1:0)`;
`(1-Z)(X-1) = 0` excludes `(0:0)`. Both adjacent `S3` equations are
bihomogenized by clearing squared denominators. A solver result becomes a
relation only after signed lifts re-add to the target in the elliptic-curve
group.

The current-source reconciliation also distinguishes projective chart
selectors from factor-base tower coordinates during algebraic extraction.
That path was exercised above the tiny-grid crossover by a one-planted
`k = 20, t = 4` smoke check.

## Current-main evidence

The additive receipt
[`raw/current-main-qualification-20261125.json`](raw/current-main-qualification-20261125.json)
records revision, commands, hashes, toolchains, limits, and exclusions.

- The direct inverse-pair/identity-chart and `(0:0)` normalization test passed.
- Six planted decompositions per presentation passed for affine chain,
  projective chain, full `S4`, and exhaustive modes; every returned relation
  re-added to its target.
- On 40 deterministic random/planted targets, projective F4, affine F4, full
  `S4`, the exhaustive grid, and the exact decomposable set agreed on every
  verdict.
- The exact Curve25519 model passed 9/9 release tests. The pinned Lean gate
  audited five theorems with zero failures and zero trusted native decisions.
- Standalone certificate `IDC1h32739202ad9d0c15` accepted 32 primary and 32
  extra points with `false_accept_log2 = -3802.562399953846`.
- The `t = 4` algebraic-descent smoke recovered and exactly re-summed its one
  planted relation. Its counted 44,400,671,897 multiply-adds and 15.01-second
  wall time are retained only as path evidence, not as an estimator.

These observations are current-machine evidence but remain
`[pcc:unverified]`.

## Historical overlay evidence

[`raw/qualification-tests.json`](raw/qualification-tests.json) is the frozen
2026-11-18 repair-overlay receipt. It is retained byte-for-byte as historical
evidence and is not represented as a current-main run.

## Performance and exponent decision

The frozen protocol requires matched `t = 3..6` cells, eight planted
relations per measured cell, paired uncertainty, and an upper cost-ratio bound
below 0.80 before opening a separately frozen `m = 4` stage. Those gates are
not satisfied by the correctness panels or the one-planted descent smoke.

The newer solving-degree frontier analysis in
[`../solving_degree_tower_frontier_20261124/README.md`](../solving_degree_tower_frontier_20261124/README.md)
records reachable `m <= 4` measurements as non-decisive for the exponent bar.
Accordingly, this continuation does not spend the frozen grid or open the
`m = 4` stage. The decision-relevant remaining problem is a proved bound on
the satisfiable-solve cost/degree growth (including the narrow two-free-block
route), together with end-to-end relation collection and linear algebra.
`[pcc:unverified]`
