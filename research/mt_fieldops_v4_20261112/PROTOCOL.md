# Multi-target IC v4: ic5 against rho with retention and distinguished points (protocol, frozen before data)

Follows `../mt_fieldops_v3_20261111/` (PR #3913; claim PCC-20261111-dda51e [pcc:unverified]). There the
independent validator found that the harness rho family lacked the standard multi-target rule (Kuhn-Struik
retention of solved walks): with retention added to `rho2x` ic5 was 1.07-1.34x at h = 0 and 1.05-1.18x at h = 0.1,
and a distinguished-point rho was estimated (not measured) to be cheaper at h = 1 as well. This study measures both
standard multi-target rho forms as pre-registered arms.

## Question

Is the multi-target IC (ic5, or ic5 with retention) cheaper per target than the cheapest standard multi-target rho
(retention rho or distinguished-point rho with retention), in counted field operations, across the section 5a
band, on fresh curves?

## Arms (one binary: `examples/mt_fieldops.rs` at the commit that adds this file)

- `ic5`: unchanged from v3, mu = 1.0, k = 4 (frozen in `../mt_fieldops_v3_20261111/frozen_ic5.json`).
- `ic5ks`: ic5 with retention of solved target trials (the validator's v3 sensitivity arm, now in the harness):
  each solved target's trial classes x(Q_t +- W_k) are inserted (one hash each); a later hit is resolved by
  recomputing the stored point (one counted addition).
- `rho2xks`: `rho2x` (x-only sibling rho, chained starts) with Kuhn-Struik retention: after target t is solved,
  every point its walks computed is inserted with its known log (one hash each). kappa in {0.5, 0.7, 1.0}.
- `rhodp` (new): distinguished-point rho on {P, -P} classes. R = 1024 steps A_i (sums of two of 64 bank points);
  canonical class representative y <= p/2; a fruitless 2-cycle is avoided by switching to step A_(h+1) when the
  next point would reuse index h (the extra addition is counted); a point is distinguished when d hash bits are
  zero, d = round(log2(n/T)/4) + off, off in {-1, 0, 1, 2}; walks longer than 20 * 2^d are abandoned (their
  steps counted). Optional precomputation from known-log chained starts until kappa sqrt(nT) steps, kappa in
  {0, 0.25, 0.5}. Targets walk from chained starts Q + sG; a distinguished point found in the database solves
  the target, one found in the target's own earlier walks with the opposite sign of Q solves it, others are
  kept pending and inserted with known logs once the target is solved. Every lookup and insert is one hash op.
- `bsgs2`: unchanged, mf in {0.7, 1.0, 1.4}, best per configuration (distance remaining).
- `rhobest` (analysis only): per configuration and (c_I, h), the cheapest of the 15 retention rho runs
  (rho2xks x 3, rhodp x 12). `icbest`: the cheaper of ic5 and ic5ks (symmetric favour to the candidate).

## Instances (fresh)

`instances/`: 33 prime-order Weierstrass curves, `../ic_e222_m221_20261106/gen_instances.py --families W
--bits 24,28,32 --per 11 --seed mtfo-v4-20261112` (indices 0..10), 0 overlap with the 1228 curves elsewhere
under `research/`. Group orders about 2^24, 2^28 and 2^32 (one 32-bit-field curve has an order just above 2^32).

## Holdout (no tune phase: every parameter grid is run and the best per configuration is taken)

Seeds 31 and 32; cells {24, 28, 32} bits x T {1024, 4096, 16384}: 9 cells x 22 paired configurations x 20 runs
= 3960 runs. Every target of every run must verify; failures are reported, not dropped.

## Cost unit and decision rule (unchanged)

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T, c_I in {3, 10, 30}, h in {0, 0.1, 1}.
Per (cell, c_I), candidate X vs reference Y: **cheaper** if the bootstrap 95% upper bound of the median
per-configuration ratio X/Y is below 1 at all three h; **dearer** if the lower bound is above 1 at all three;
otherwise **unranked**. Comparisons: icbest, ic5 and ic5ks vs rhobest, rho2xks, rhodp and bsgs2.

## Predictions (from one smoke run per arm at 24 b, T = 1024 and 32 b, T = 16384; not data)

- rhodp is the cheapest rho at h = 1 everywhere (fewest hash operations); rho2xks is the cheapest rho at h = 0.
- icbest vs rhobest: no cell cheaper at all three h. At h = 0 icbest is cheaper at 32 b, T = 16384 and dearer
  at 24 b, T = 1024; at h = 1 dearer everywhere (by more than 1.5x at 32 b). Most cells unranked, some dearer.
- icbest dearer than bsgs2 everywhere.

## Limits known in advance

Groups of about 2^24 to 2^32; counted, not timed; memory unbounded for every arm; local host (no AWS identity,
so not isolab-eks); the rho family is this harness's, not ecbench's `rho.multitarget_pooled`; the verification
constant (about 30 to 50 GAE per target) is shared by every arm and compresses ratios toward 1 (reported with and
without it).

## Errata (producer, from the validator's report, 2026-11-12; arms and data unaffected)

- 7 of the 11 curves at 32 bits have group order just above 2^32, not one.
- `icbest` is not a symmetric favour: the IC side has 2 runs per configuration, the rho side 15. (Fixing rho's
  parameters from the other seed leaves the frozen-rule decisions unchanged.)
- The decision rule prices h on hash-table and solver operations only; section 5a also prices rho's unpriced
  native work (canonicalisations), which this harness did not count. Both readings are reported in RESULTS.
