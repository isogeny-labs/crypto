# RESULT: LEAD-ICP-hecke-cm-class-point-rank-completion

## Outcome
- status: ran (kind runnable_local), single run, no retries.
- exit code 0, wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- RESULT.json: one JSON object, verdict field = "falsified".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "max marginal = 0 in every dense cell AND mean marginal >= 1 in at least one sparse cell."
Falsified if any dense cell has marginal > 0, or all sparse cells have mean marginal < 1.

Measured:
- dense cells: 1 (n=4129, m=64; mean collisions 458.6 >= 3m=192). Its marginal is mean 1.0, max 1 in all 20 seeds (pair_free mean 2.0). Dense max marginal = 1 > 0, so the first falsification clause holds.
- sparse cells: 7; max mean marginal 9.0. The second clause (all sparse < 1) is not met; the positive-control side of the prediction held.
- Other cells fall between dense and sparse (collisions between m/2 and 3m, e.g. n=16417 m=64: 154.8 collisions, mean marginal 7.4) and are not covered by the prediction.
- Controls: negative (wrong-eigenvalue relations rejected, all seeds, all cells) pass; positive (Hecke relations valid in all seeds; sparse marginal >= 1) pass; A/A noise floor mean |pair-rank diff| 0.0 to 4.2 across cells (largest at m=64, n=16417 and n=65537, 4.2 and 3.3; 2.8 at n=4129 m=32). controls_ok = true.
- Verdict: falsified (by the single dense cell).

## Reading
- The falsification rests on exactly one cell with a marginal of 1 (every seed gives exactly 1). That looks like a structural residual, not noise: a rank deficiency of the pair collector at pair_free = 2 that the eigenvalue relations fill by one. With n=4129 and m=64 the base (16 orbits) is large relative to n, so pair relations nearly saturate; the leftover dimension of 1 is probably a degenerate linear dependency (for example the orbit closure relation) that Hecke relations always supply. I did not investigate this beyond reading the output (no new code allowed).
- The dense/sparse split is thin: only 1 of 15 cells is dense under the protocol's 3m threshold, and the sparse regime has marginals near m/4+ (5, 9, 17 for m=16, 32, 64) in the saturated-n cells. That is essentially one new relation per orbit, so the sparse result is close to tautological in this stand-in.
- The A/A noise floor is nonzero for m>=32 at small n (up to 4.2), so marginals of 1 in a dense cell are small versus that scale in the A/A arm of other cells, yet the dense cell itself has A/A diff 0.0, so its marginal 1 is well above its own noise floor.
- run.py's verdict field matches the numbers. The toy tests its own density-threshold hypothesis but not the lead (no X_0(11), D_K=-39, real Hecke operators; collisions are labelled by hidden-log oracle; no 0/0.1/1 GAE pricing vs rho).

## Bearing on the sub-sqrt(p) question
none: stand-in toy with a hand-built eigenvalue orbit structure; the dense-regime marginal of 1 does not touch the recorded real-curve 0/10/19/0 cells or any sub-sqrt(p) cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
