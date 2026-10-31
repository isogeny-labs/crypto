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

## What was run

Binary: `ecbench` built from R5's build tree (R5 commit 4139a42dd7 glue: emitter `wdsat_s4.rs`, engine `wdsat-s4`, first-solution mode, `-x -b`, `m = 3`) plus the E1 child-time shim, with two build-time constants: `WDSAT_BIN_DEFAULT` (own binaries, below) and
`WDSAT_TMP_DEFAULT` (the emitter's scratch directory; patch `research/ic_r2w_wdsat_sweep_20261031/wdsat_s4_tmpdir.diff`, 1 line, no effect on counted instructions). WDSat `61c6ff3f49` built from the existing clone with R5's `wdsat_cell.py`
(first-solution binaries, `__MAX_ID__` exact per system size, pad 4 to 6) for l = 3..11; 1.4e7 to 1.8e7 start-up per process (`probes/startup.txt`; median per l used for the solver-only figure: l=3 1.390e7, 4 1.430e7, 5 1.397e7, 6 1.431e7, 7 1.474e7, 8 1.473e7, 9 1.556e7, 10 1.643e7, 11 1.761e7).
Hashes: `scripts/binary_hashes.txt`. Evidence: `research/ic_r2w_wdsat_sweep_20261031/` (`specs/`, `runs/R2W_*` with records and per-child `time -l` files, `probes/`, `analysis/`, `scripts/`).

- **Probes** (correctness and per-system cost, first-solution, `probes/probe_first_*.jsonl`): l = 3..8 at the five cells, 320 systems, root sets against brute force over V^3: **320/320 agree**, `inexact` 0.
- **Whole-solve sessions** (ecbench law of E1/R2/R5; 8 targets each, `[d]G = Q` checked by ecbench on every record): l = 3, 4, 5, 6, 7, 8 at all five cells; l = 9, 10 at K_1 n=11 and K_0 n=13; l = 11 at K_0 n=13 (l = 11 = n at K_1 n=11 is refused by the pipeline: "dimension must be smaller than n"). The lab arm at its R2-best l was repeated in the l = 4/5 sessions (A/A); lab l = 3 was run at K_0 n=13.
  336 records: 304 verified (all `matches_target`), 24 `exhausted` (the three structurally failing l = 3 cells), 8 refused. WDSat calls 32,102; `refuted_by_emitter` 0; `lift_failures` 21 (all K_1 n=11, spurious roots, as in R5); `wdsat_inexact` 9 (K_1 n=11 l=9: 5 of 324 calls, K_0 n=13 l=9: 1, l=11: 3 of 652; none in any best or near-best config; a larger-id binary was used and counted).
- **Lab arm**: R2's recorded values (pooled records of E1/T/R2 sessions, loaded by the same loader as `r2_analyze.py`), **not re-measured**. A/A against R2 in these sessions: K_0 n=9 l5 2.96e8 (R2 2.95e8), K_1 n=9 l3 2.34e8 (2.37e8), K_1 n=11 l5 4.61e8 (4.56e8), K_0 n=13 l6 1.04e9 (1.03e9), K_0 n=15 l7 1.86e9 (1.85e9): within 1.3 %. New lab point: K_0 n=13 l3 2.93e9 (worse than l6). Best lab l is unchanged in every cell (5, 3, 5, 6, 7).
- **l = 3 is structurally invalid at K_0 n=9, K_1 n=11 and K_0 n=15**: the base exhausts (calls = r - 1 = 126, 990, 750, status `exhausted`) whatever the solver, as with msolve in R2; it works at K_1 n=9 and K_0 n=13.

## WDSat sweep (median over 8 targets; `x` = failed 0/8; l >= 9 only where needed, "-" not run)


whole run, median Ir/target
| cell | l=3 | l=4 | l=5 | l=6 | l=7 | l=8 | l=9 | l=10 | l=11 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| K0 n=9 | failed 0/8 | 2.05e+09 | 4.41e+09 | 1.01e+09 | 1.42e+09 | 1.96e+09 | - | - | - |
| K1 n=9 | 4.81e+08 | 5.63e+08 | 6.55e+08 | 8.17e+08 | 1.02e+09 | 1.40e+09 | - | - | - |
| K1 n=11 | failed 0/8 | 8.23e+09 | 1.19e+10 | 3.11e+10 | 1.65e+10 | 3.85e+09 | 6.53e+09 | 1.03e+10 | - |
| K0 n=13 | 6.68e+09 | 9.82e+09 | 5.36e+10 | 1.21e+11 | 8.76e+10 | 5.77e+09 | 9.16e+09 | 1.61e+10 | 2.63e+10 |
| K0 n=15 | failed 0/8 | 8.27e+10 | 9.41e+11 | 7.78e+11 | 1.05e+12 | 2.41e+12 | - | - | - |

solver-only, median Ir/target
| cell | l=3 | l=4 | l=5 | l=6 | l=7 | l=8 | l=9 | l=10 | l=11 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| K0 n=9 | failed 0/8 | 1.43e+09 | 3.91e+09 | 1.97e+08 | 3.76e+08 | 6.76e+08 | - | - | - |
| K1 n=9 | 4.41e+07 | 1.08e+08 | 9.39e+07 | 1.47e+08 | 2.49e+08 | 4.56e+08 | - | - | - |
| K1 n=11 | failed 0/8 | 6.41e+09 | 1.11e+10 | 2.99e+10 | 1.43e+10 | 1.36e+09 | 2.73e+09 | 5.00e+09 | - |
| K0 n=13 | 1.23e+09 | 7.43e+09 | 5.20e+10 | 1.19e+11 | 8.46e+10 | 2.08e+09 | 3.96e+09 | 7.66e+09 | 1.39e+10 |
| K0 n=15 | failed 0/8 | 6.37e+10 | 9.21e+11 | 7.76e+11 | 1.05e+12 | 2.40e+12 | - | - | - |

relation systems (WDSat calls) per target, median
| cell | l=3 | l=4 | l=5 | l=6 | l=7 | l=8 | l=9 | l=10 | l=11 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| K0 n=9 | (126) | 16 | 10 | 15 | 16.5 | 17 | - | - | - |
| K1 n=9 | 11.5 | 9.5 | 11 | 11.5 | 11 | 11.5 | - | - | - |
| K1 n=11 | (990) | 56.5 | 15.5 | 23.5 | 33 | 32 | 40.5 | 47.5 | - |
| K0 n=13 | 219.5 | 71 | 36 | 31 | 39.5 | 41 | 49.5 | 66.5 | 81.5 |
| K0 n=15 | (750) | 577 | 471 | 37 | 35.5 | 49 | - | - | - |

## Result: best WDSat against best lab

Ratio = median over the 8 paired targets of WDSat metric / best-lab whole run. Fit A = pre-fixed grid l in {3..7}. Fit B = extended grid (Amendment 1; adds l = 8 everywhere and l = 9..11 at n = 11, 13). The last column is R2 (msolve) for comparison.

| cell | best lab: l, Ir | A: best WDSat whole: l, Ir | ratio | A: best solver-only: l, Ir | ratio | B: best WDSat whole: l, Ir | ratio | B: best solver-only: l, Ir | ratio | R2 msolve whole / solver-only |
|:--|:--|:--|--:|:--|--:|:--|--:|:--|--:|:--|
| K_0 n=9  | l5, 2.95e8 | l6, 1.01e9 | 3.33 | l6, 1.97e8 | 0.66 | l6, 1.01e9 | 3.33 | l6, 1.97e8 | 0.66 | 86 / 16.5 |
| K_1 n=9  | l3, 2.37e8 | l3, 4.81e8 | 2.01 | l3, 4.41e7 | 0.18 | l3, 4.81e8 | 2.01 | l3, 4.41e7 | 0.18 | 71 / 13.1 |
| K_1 n=11 | l5, 4.56e8 | l4, 8.23e9 | 18.29 | l4, 6.41e9 | 14.23 | l8, 3.85e9 | 8.74 | l8, 1.36e9 | 3.10 | 206 / 51.6 |
| K_0 n=13 | l6, 1.03e9 | l3, 6.68e9 | 6.32 | l3, 1.23e9 | 1.16 | l8, 5.77e9 | 5.59 | l3, 1.23e9 | 1.16 | 136 / 23.2 |
| K_0 n=15 | l7, 1.85e9 | l4, 8.27e10 | 42.67 | l4, 6.37e10 | 32.86 | l4, 8.27e10 | 42.67 | l4, 6.37e10 | 32.86 | 634 / 137 |

Every best point is a whole solve (8/8 verified); no cell is *open* and no projection supplies a best point. The K_0 n=15 optimum is interior (l = 5, 6, 7, 8 are 9x to 29x dearer); l >= 9 was not run there (cost rises over four consecutive l).
At l = 3 the solver-only figure is the whole WDSat cost less about 1.39e7 per call, i.e. most of the cost; 7 % uncertainty in the start-up changes it by 15 % to 25 % (K_1 n=9, K_0 n=13), so the 0.18 and 1.16 are soft.

## Fit: ln(best WDSat / best lab) on n (OLS, 95 % t interval, `analysis/r2w_fit*.txt`)

| grid | metric | cells | slope per bit | 95 % CI | x per 4 bits | verdict (rule above) |
|:--|:--|:--|:--|:--|:--|:--|
| A: l 3..7 | whole run | all 5 | **+0.395 +- 0.496** | -0.101 to +0.891 (p=0.085) | 4.9 (0.67 to 35) | undecided |
| A | whole run | without n=15 | +0.292 +- 1.226 | -0.934 to +1.518 | 3.2 | undecided |
| A | solver-only | all 5 | **+0.619 +- 1.023** | -0.404 to +1.642 (p=0.15) | 11.9 (0.20 to 711) | undecided |
| A | solver-only | without n=15 | +0.444 +- 2.572 | -2.128 to +3.016 | 5.9 | undecided |
| B: extended | whole run | all 5 | **+0.398 +- 0.378** | +0.020 to +0.777 (p=0.044) | 4.9 (1.08 to 22) | growing |
| B | whole run | without n=15 | +0.231 +- 0.730 | -0.499 to +0.960 | 2.5 | undecided |
| B | solver-only | all 5 | **+0.641 +- 0.702** | -0.061 to +1.343 (p=0.062) | 13.0 (0.78 to 216) | undecided |
| B | solver-only | without n=15 | +0.374 +- 1.494 | -1.120 to +1.869 | 4.5 | undecided |
| R2 msolve (for comparison) | whole / solver-only | all 5 | +0.296 +- 0.285 / +0.304 +- 0.382 | +0.011 to +0.581 / -0.078 to +0.686 | 3.3 / 3.4 | undecided leaning growing |
| fixed l = 4 (R5-style, no WDSat sweep) | whole / solver-only | all 5 | +0.335 +- 0.453 / +0.463 +- 0.754 | -0.118 to +0.788 / -0.291 to +1.217 | 3.8 / 6.4 | undecided |

## Verdict

- **Under the pre-fixed rule (fit A, the l = 3..7 grid): undecided.** Whole run CI -0.101 to +0.891 and solver-only CI -0.404 to +1.642 both contain 0 and are nowhere near the flat band [-0.10, +0.10]. (Under R2's looser wording of "flat" the verdict is the same.)
- **Under the amended grid (fit B): whole run growing (CI +0.020 to +0.777, lower end barely above 0), solver-only undecided (CI -0.061 to +1.343).** The two metrics disagree, so the overall verdict by the combination rule is **undecided, leaning growing**: the point estimates are +0.40 (whole) and +0.64 (solver-only) per bit, x4.9 and x13 per 4 bits, larger than R2's +0.30 with msolve; nothing supports flat or shrinking.
- It is not a clean "growing". The cell ratios are 3.3, 2.0, 8.7, 5.6, 42.7 and are not smooth in n: they follow the optimum l of each arm, which jumps. Without n=15 (k=4, 2 degrees of freedom) the CI is +-0.7 to +-1.5 per bit and says nothing.
  The slope rests on K_0 n=15, where the WDSat optimum stays at l = 4 with 577 relation systems per target (about 1.4e8 each) while the lab's optimum has moved to l = 7 with 34 systems; K_0 n=9..13 alone give ratios of 2 to 18 with no visible trend.
- **Mechanism visible in the tables, not in the fit.** WDSat in first-solution mode is cheap on satisfiable systems (large l: dense solutions, found in 4e7 to 1e8 Ir at n <= 13) and expensive on unsatisfiable ones (every system costs 1.4e8 at l = 4, 2e9 at l = 5, 3e10 at l = 6 when it must be refuted). At n = 9..13 a large base (l = 6 or 8) makes nearly every system satisfiable
  and WDSat comes within 2x to 9x of the lab (solver-only 0.2x to 3x), a much smaller gap than msolve's 13x to 52x. At n = 15 (r = 751, 2^15 field) a base of l = 5 to 8 no longer makes the systems easy: per-system cost at l = 6 to 8 jumps to 1e10 to 7e10 (K_0 n=13: 1e8), so WDSat falls back to l = 4 and 577 systems.
  So the WDSat/lab gap is small and flat up to n = 13 and opens at n = 15; whether that is a trend in n or a feature of this one curve is exactly what 5 cells cannot separate.
- Compared with R2: the lab's lead over the *best published solver* is far smaller with WDSat (2x to 43x whole run, against 71x to 634x with msolve), but the evidence for growth with n is about as weak as in R2 (CI of similar width, wider on the solver-only metric).

## Not done, departures

- The verdict rule's grid (l 3..7) did not contain the WDSat optimum at K_1 n=11 and K_0 n=13; l = 8 (all cells) and l = 9..11 (n = 11, 13) were added by Amendment 1, written after the l = 3..8 data at n = 9..13 existed and before any l >= 9 run. The pre-fixed verdict is fit A; fit B is post hoc in its grid.
- l = 1, 2 not tried (l < 3 is invalid or failing for the lab at n = 9; WDSat l = 3 already fails at three cells); l >= 9 not run at n = 9 (optimum interior) and l >= 9 not run at n = 15 (cost rising from l = 6 to 8). l = 9..11 sessions were not brute-force cross-checked (only the ecbench `[d]G = Q` check; probes cover l <= 8). 9 of 32,102 WDSat calls used a larger-`__MAX_ID__` binary (l = 9, 11 only).
- First-solution mode only (find-all, R5's l = 4, 5 reference, not re-swept); no `-g` branching order, no hybrid specialisation, no `T`-invariants, `m = 3` only, as in R5. The lab arm is R2's record, swept l = 3..9 only where R2/this run measured it (l = 3 at K_0 n=13 new; l = 3 elsewhere failing or measured by R2).
- No single-session A/A for WDSat arms was repeated; the sweep was run twice end to end (first build, discarded, see below) and the repeat reproduced the first build's numbers where both were valid (for example K_0 n=13 l6 1.21e11 in both).
- Sandbox off for: `git worktree add` (the sandbox denied writing `.mcp.json` in the new worktree), every build, probe, startup measurement and `ecbench run` (`/usr/bin/time -l` needs a sysctl; writes outside `/Volumes/SSD990-2/crypto`). Nothing was refused by a permission check or the safety classifier.
- **First-build sessions discarded.** The first ecbench build wrote the emitter's scratch files to `/tmp` on the root disk, which filled up transiently (another job); WDSat calls then failed with "No space left on device" and the pipeline silently dropped those systems (stderr only). Those sessions (some W6 cells, W_k0n15) were detected from the child stderr, discarded, and every session was re-run on a rebuilt binary with `WDSAT_TMP_DEFAULT` on SSD990-2. The runner now also checks each session's stderr for `wdsat-s4:` errors and re-runs. No discarded data enter any table.
- Build and scratch location: `/Volumes/SSD990-2/r2w-build` (cargo target ~0.8 GB copied from R5's build, tree 123 MB, WDSat binaries 8 MB, ANF dumps 40 MB, runs). The first-build sessions are in `r2w-build/runs_firstbuild`. Not committed; see the final status in the commit message / handback for deletion.
- No push, no PR, no download. WDSat source is not in the repository; the clone `/tmp/claude-501/wdsat_src` was used read-only, its prebuilt binary was never run.
