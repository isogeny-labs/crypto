# EXP-PFDR-2d36f9-R2-FAM: the break-even ladder on three other structured families (preregistered, before data)

Date: 2026-11-01. Branch `claude/pfdr-families-20261101`. Follows `research/pfdr_ladder_rerun_20261101` (PR #3188; reading O-SUB-BE for the
window / rank-d label-GAP lines with valid controls). The user asked to try the other structured families.

## What "the other families" means here (and what it does not)
- The original specification names two deferred items: the **FAM-1 ladder** (the `subgroup`, `small_x` and `dickson` arms of the EXP-PFDR-011cd0 mitm engine,
  EV-PFDR-ad6274) and the **bbc53e-base ladder** (IDEA-20261017-bbc53e: the weighted base F' = {cP : P in F_0, 1 <= |c| <= N^gamma}).
- The bbc53e base is the planted shared-unknown base, which every ladder already runs as the gamma control (gamma 0, 1/6, 1/3, 1/2: recovered slopes
  0.158 / 0.322-0.331 / 0.489-0.499 in the rerun). It is a plant for kappa growth by construction, not a candidate family: no new run.
- The FAM-1 mitm engine is not in this repository; the `small_x` arm is the x-window line already measured; the `subgroup` arm needs a proper subgroup of
  the curve group, and these curves have prime order. So this study does **not** discharge the FAM-1 clause. It runs the structured families that can be built
  on the existing kernel and curves and that the lead's family list suggests: **Dickson** (d = 1), a **geometric progression in x** (d = 2), and a
  **quadratic progression in x** (d = 3). The three are fixed here; no family is chosen after data (winner's curse).

## Families (bases of exactly b points = b/2 liftable abscissae with both signs; symmetric, distinct)
- d = 1, Dickson: x_k = D_k(x_1) = u^k + u^-k, Chebyshev recurrence, u random in F_p^*, k = 1, 2, ... (stream of liftable abscissae).
- d = 2, geometric progression: x_k = x0 * rho^k, x0 and rho random in F_p^*.
- d = 3, quadratic progression: x_k = x0 + k^2, x0 random.
The base sizes b = f(bits, theta) are the original cells' (theta 0.30 and 0.35; bits 16, 20, 24, 28, 32), the sizing rule is the R2 rule (safety 1.5),
the null is 20 random symmetric bases of the same b on the same curve.

## Change relative to R2 (the only one; `implementation.patch`, R2 driver kept as `run.py.R2`)
`job()` builds the structured base with `family_base(E, b, d, rng)` (above) instead of the window / GAP constructors; the experiment id and the seed blocks
(pilot 8,000,000, main 8,100,000, replication 8,200,000, selftest 8,900,000) are new. Everything else is byte-identical to R2: kernel, curve code, cells, the
counting convention, statistics (five-rung OLS slope of log2 kappa on log2 N, curve-cluster bootstrap, 2000), classes, controls C3 (gamma plants), C4 (Z/N),
C5 (A/A lines), C6 (z_cal on the 10 independent null sets, bounds |mean| <= 0.62, sd in [0.548, 1.454]), the break-even rate delta* = 1/3, the replication
block for any ALIVE line, and the readings. Base-construction check before freezing: for bits 16, 20, 24 and b in {28, 60, 200} every family returns exactly b
distinct symmetric points on the curve.

## Claims
- C1 (confirmatory): the six structured lines (3 families x theta 0.30, 0.35) have a five-rung slope below delta* = 1/3 (SUB-BE or SUB-BE-GROWING) with all
  blocking controls valid: reading O-SUB-BE.
- C2: any line classed ALIVE is replicated (block rep) before any BREAK-EVEN-MET statement.
- Class: structural-lead measurement; it prices no end-to-end index calculus and says nothing about rho.

## Execution and decision rule
`scripts/run_all.sh` (local, 4 workers, about 17 minutes). Report analysis-main.json's reading verbatim; if a blocking control fails the reading is O-INVALID
and is reported as such with no redesign in this study. Raw jobs go to the artifact store.
