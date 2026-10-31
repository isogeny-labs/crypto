---
title: "R2W: matched sweep of the WDSat published arm against the swept lab arm (2026-10-31)"
---

Pre-freeze; nothing here is a Register-E row. Re-run of R2 (`R2_MATCHED_SWEEP_20261031.md`) with WDSat (R5, `research/ic_r5_wdsat_arm_20261031/README.md`) instead of msolve as the
published arm's solver. Question: does the lab arm's lead over the published-method arm grow with size once the published arm is WDSat, swept over the base dimension l?

## Verdict rule (fixed before any R2W run; this section was committed before the first measurement)

- **Cells**: the five R2 cells, K_0 n=9, K_1 n=9, K_1 n=11, K_0 n=13, K_0 n=15 (n=15 is the only cell R5 did not cover). Session law, targets (`target_seed 202610111901`, 8 public targets), `m = 3`, host and pricing as E1/R2/R5.
- **Published arm** = WDSat on the e-form S4 system (R5 emitter, flags `-x -b`, first-solution mode), swept over l in {3, 4, 5, 6, 7}. Best WDSat config is chosen **separately for each metric and cell** as the config with the lowest median (over the 8 targets) of that metric.
- **Lab arm** = R2's recorded best-l whole-run values (R2 session records, pooled exactly as `r2_analyze.py` does); not re-measured, except that the lab arm at its best l is repeated in the WDSat sessions as an A/A check.
- **Metrics** (both primary): *whole run* = ecbench child Ir + every WDSat process Ir - 3.51e7; *solver-only* = WDSat process Ir less one measured WDSat start-up per call (favourable to the published arm; compared against the lab arm's whole run, as in E1/R2/R5).
- **Cell ratio** = median over the 8 target indices of (WDSat metric / best-lab whole run on the same target), as in `r2_analyze.py`.
- **Eligibility**: a config can be a cell's best point only if it was whole-solved (8/8 verified, `[d]G = Q`). Per-system probes times systems-per-target projections may only *exclude* configs (labelled "probe-excluded"), never supply a best point.
  If a config is neither whole-solved nor excluded by a probe at a cell, that cell is marked *open* and the fit is also reported without it.
- **Fit**: OLS of ln(cell ratio) on n, 95 % t interval with k-2 degrees of freedom. The verdict uses the headline fit of **all 5 cells**, per metric. The k=4 fit without n=15 is reported as a sensitivity row and does not enter the verdict.
- **Per-metric verdict**: *growing* if the 95 % CI of the slope d ln(ratio)/dn lies entirely above 0; *shrinking* if entirely below 0; *flat* only if the whole CI lies inside the band [-0.10, +0.10] per bit of n
  (x1.5 per 4 bits); otherwise *undecided*. (R2's wording was "CI contains 0 and half-width <= 0.1", which is looser; the verdict is also stated under that wording if it differs.)
- **Overall verdict**: the common per-metric verdict if the two agree; otherwise *undecided* (with the direction of the point estimates described, not voted).

## Amendment 1 (written after the l = 3..7 sessions and the l = 8 sessions at n = 9..13 had been measured, before any l >= 9 session was run)

The l = 3..7 grid above turned out not to bracket WDSat's optimum at the larger cells: in first-solution mode a larger base makes the system trivially satisfiable with many solutions, and the
per-system cost falls with l again (n = 11: best at the grid edge or beyond; n = 13: l = 8 beats l = 7 by 15x). The R2 lab sweeps go to an interior minimum, so the WDSat sweep is continued the same way.

- **Fit A (pre-fixed grid, the verdict under the rule above)**: published arm = best of l in {3..7}. Unchanged.
- **Fit B (extended grid, same rule applied unchanged)**: published arm = best over l in {3..L}, where L is extended per cell until the whole-run cost at two consecutive larger l is above the best seen (an interior optimum, as for the lab arm), or l = n.
  Cells whose optimum is already interior inside {3..8} (K_1 n = 9: l = 3 is the lowest valid l; K_0 n = 9: l = 6) are not extended. Fit B is post hoc in its grid and is reported next to A with its own verdict by the same CI rule; the overall verdict
  states both and says which one the pre-fixed rule decides.
- Everything else (cells, targets, pricing, ratio definition, eligibility, band) is unchanged. No l >= 9 brute-force root cross-check is done (2^27 triples); the ecbench `[d]G = Q` verification on every record and the zero-lift-failure count stand in.
