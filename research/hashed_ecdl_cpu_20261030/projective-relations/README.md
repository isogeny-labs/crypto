# Relation checks without uncached field inversions

Collection and each independent replay now sum validated relation points in
Jacobian coordinates when the Curve has no inverse table. Equality to an affine
target is checked by X=x*Z^2 and Y=y*Z^3, with an explicit identity case. The
existing affine path remains for curves with cached inverses. This removes
field inversions from the uncached tuple-sum check without allocating a table.

Point validation, base membership, scalar bounds, dependency replay and final
scalar multiplication still run. Replay independently reconstructs its bases
and fixed-base tables and recomputes every relation target; it does not trust
collection's target or validation results. The collector's target generation
and replay's target generation are unchanged. RNG draws, witness ordering and
certificate serialization remain unchanged.

The derivation uses the coordinate definition and existing mixed-addition
formula from the [Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html),
reviewed 2026-10-30. This is a reuse of established coordinate arithmetic.
The retained round18 phase records motivated inspection of relation-check
costs; that panel does not measure this change or isolate this helper's time.

## Software validation

The standalone CPU library passes 40 release tests. The new test checks all
199-by-199 point pairs in the small fixture, with sequence prefixes of lengths
0 through 8 containing identity, doubling and inverse cancellation. It compares
both cached and uncached paths against affine sums, accepts the expected target,
and rejects a target offset by a nonidentity generator. Existing archived
certificate tests cover all three complete fixtures, including the uncached
65521-element field. Strict all-target release Clippy and rustfmt pass.

Commands and source/binary hashes are retained in validation.json, with logs.
Only the standalone CPU crate is compiled; no root crypto/CUDA build is run.
No new cloud benchmark, timing sample or performance claim is included.

## Next comparison

Freeze a new comparison with the original baseline, all three fixtures, and
the complete setup/discarded-trial/reduction/two-replay/serialization boundary.
Keep the cached-affine and projective-relation changes distinguishable in the
arms. Cloud execution waits for resolution of the account spending concern;
existing round18 artifacts and binaries remain immutable.

## Handoff, 2026-10-30

- Implemented under T-1012; source and this directory are its scoped outputs.
- The previous goal turn produced billing and live-resource evidence that
  changed the execution decision: continue local software work, avoid another
  cloud benchmark while the cost-reduction proposal is pending.
- This component is validated for correctness; its latency effect is unmeasured.
- Shared session-log files were not edited outside this task's reserved scope.
