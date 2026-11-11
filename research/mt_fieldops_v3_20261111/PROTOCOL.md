# Multi-target IC v3: k-term table (ic5) and larger groups (protocol, frozen before tune and holdout data)

Follows `research/mt_fieldops_v2_20261111/` (PR #3873, claim PCC-20261111-fa13c8 [pcc:unverified]): IC4 was
cheaper than the cheapest of four rho arms only at T = 16384 (0.91-0.95), unranked at 4096, dearer at 1024
(1.05-1.19), and 1.18-1.80x a paired BSGS. The v2 phase split shows why IC4 loses at small T: at 24 b,
T = 1024 its relation phase (about 610 relations for m = 305 unknown factor-base logs) costs about 32 GAE per
target, roughly as much as a fifth of the run.

## Questions

1. (option 2) Does `ic5`, a k-term table built from far fewer unknowns, lower the multi-target IC's cost, and
   does it move the crossover against the strongest rho down to T = 1024 or 4096?
2. (option 1) Do the v2 findings hold at 28- and 32-bit groups?

## Arms (one binary: `examples/mt_fieldops.rs` at the commit that adds this file; field layer now u64 with
u128 products for p < 2^60)

- `ic5` (new): factor base = the m smallest-x points; the table holds x(F_i1 + s2 F_i2 + ... + sk F_ik),
  i1 < ... < ik, built as x(P +- F_l) from full (k-1)-term points P (one shared inversion per pair; the full
  points are counted); m is the smallest value with C(m, k) 2^(k-1) >= mu^2 sqrt(nT/2) table classes, matching
  IC4's table size at mu = 1. Factor-base logs: paired relation trials x(V +- W) as in IC4, each new relation
  reduced into an incremental echelon form mod n until rank m (every mod-n product counted as M, every mod-n
  inverse as I; the dedupe lookup counted as a hash). Targets: paired trials x(Q_t +- W_k) as in IC4.
  **k in {3, 4} and mu in {0.7, 0.85, 1.0, 1.2, 1.4} are chosen on the tune set and frozen in
  `frozen_ic5.json` before the holdout.**
- `ic4` unchanged, mu = 1.0, g = 2.0 (frozen in v1, not re-tuned).
- `rho`, `rho2`, `rho2x`, `rhoseq` x kappa {0.5, 0.7, 1.0}; `rhobest` (analysis only) = the cheapest of these
  12 runs per configuration and (I/M, h). `bsgs2` with mf {0.7, 1.0, 1.4}, best per configuration.

## Instances (fresh)

`instances/`: 48 prime-order Weierstrass curves, `gen_instances.py --families W --bits 22,24,28,32 --per 12
--seed mtfo-v3-20261111`, 0 overlap with the 1135 curves elsewhere under `research/`. Index 0 of each size is
the tune set; indices 1..11 are the holdout.

## Tune (chooses k and mu of ic5 only)

Index 0 at 22, 24, 28, 32 bits; T in {1024, 16384}; seeds 101, 102 (16 configurations, 160 runs). Objective:
mean over configurations of cost(I = 10M, h = 0.1) relative to the best grid point of that configuration.

## Holdout

Indices 1..11, seeds 21 and 22, cells {22, 24, 28, 32} bits x T {1024, 4096, 16384}: 12 cells x 22 paired
configurations x 17 runs = 4488 runs. Every target of every run must verify; failures are reported, not dropped.

## Cost unit and decision rule (unchanged)

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T, c_I in {3, 10, 30}, h in {0, 0.1, 1}.
Per (cell, c_I), candidate X vs reference Y: **cheaper** if the bootstrap 95% upper bound of the median
per-configuration ratio X/Y is below 1 at all three h; **dearer** if the lower bound is above 1 at all three;
otherwise **unranked**. Comparisons reported: ic5 vs rhobest, bsgs2 and ic4; ic4 vs rhobest and bsgs2.

## Predictions (from one smoke run per arm, 24 b, T = 1024; not data)

- ic5 / ic4 below 1 in every cell (largest gain at T = 1024, about 0.85; small at T = 16384).
- ic5 / rhobest: cheaper at T = 16384 and 4096 in every size; at T = 1024 about 0.92, cheaper or unranked.
- ic4 / rhobest at 28 and 32 bits: same pattern as v2 (cheaper at 16384, dearer at 1024).
- Both dearer than bsgs2 everywhere.

## Limits known in advance

Groups 2^22 to 2^33; counted, not timed; memory unbounded for every arm; local host (no AWS identity, so no
isolab-eks); the rho family is this harness's (not ecbench's); a verification constant of about 32 to 50 GAE per
target is shared by every arm and compresses ratios toward 1 (reported with and without it).

## Amendment A1 (2026-11-11, before any holdout result was analysed)

The first holdout launch hung at 32 bits on a defect in the rho-family restarts (bounded set of start points;
details and fix in `../mt_rho_start_fix_20261111/PROTOCOL.md`). Six rows had been written (32 b, T = 1024,
idx 1, seed 21); they are discarded unread beyond the hang diagnosis. The holdout reruns in full with the fixed
binary; arms, parameters, instances, seeds and the decision rule are unchanged. The fix makes every rho arm
cheaper or equal (one addition per restart instead of two, no repeated walks), so it can only strengthen the
reference against ic4 and ic5.
