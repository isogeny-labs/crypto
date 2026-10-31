---
title: "R2: matched sweep of the published arm against the swept lab arm (2026-10-31)"
---

Pre-freeze; nothing here is a Register-E row. Decides E1 (`E1_SCALING_20261030.md`, undecided). Same arm, session law (`ecbench run --cpus none --wait`, 1 round,
`alternate`, L0, `target_seed 202610111901`, 8 public targets, `m = 3`), host and pricing rule as `PILOT_20261030.md` section 9 and E1: per target, whole run = ecbench child
instructions retired (shim `time -l` file) + every msolve process, minus the 3.51e7 child start-up; "solver-only" = msolve instructions less 3.8e7 per msolve process
(section 9 lower bound, the figure most favourable to the published arm). Ratio = median over the 8 paired targets of published / lab-at-its-best-l.
Binary: `git archive` of `d2cd1ded48` (subset: `src`, `Cargo.lock`, `tools/volcano-cli`, `isogeny_algos`, five `docs/` files) plus the E1 child-time shim; `/usr/bin/time -l` ran with the
sandbox off. Deviation: the root disk had 150 MB free, so `CARGO_TARGET_DIR` was on SSD990-2 (`crypto-worktrees/.r2build-tmp`, 0.8 GB), not `$TMPDIR`; both build trees are deleted.
Everything below was *measured* except the rows marked "projected".

**Verdict rule, fixed before any R2 run** (RESUME, 2026-10-31): *growing* if the 95 % CI of the slope d ln(ratio)/dn lies above 0; *shrinking* if below 0; *flat* only if the CI
contains 0 and its half-width is at most 0.1 per bit (x1.5 per 4 bits); otherwise *undecided*.

## Sweeps run (all arms verified 8/8 unless stated; evidence `tuned_20261030/r2/`)

Published arm is the x-form, hybrid `msolve-s4` engine. Per cell (new R2 points marked *; the rest are E1 / section 9 points, pooled by method id):

| cell | published (l, spec), whole-run median Ir per target | lab l, whole-run median Ir per target |
|:--|:--|:--|
| K_0 n=9 (r=127) | l=3 s0 **failed 0/8** (section 9); l=4: s3* 2.60e10, **s4 2.68e10**, s5* 4.37e10, s6* 7.46e10; l=5: s5* 1.49e11, s6* 1.63e11, s7 2.42e11 | l4 3.52e8, **l5 2.95e8**, l6 3.28e8 |
| K_1 n=9 (r=37) | l=4: s3* **1.68e10**, s4 1.68e10, s5* 2.76e10, s6* 4.66e10; l=5: s5 1.41e11, s6* 1.58e11 | l2* **failed 0/8**, **l3* 2.37e8**, l4 2.47e8, l5 2.52e8 |
| K_1 n=11 (r=991) | l=4: s3* **9.58e10**, s4 1.04e11, s5* 1.70e11; l=5: s5 1.62e11, s6* 2.29e11 | l4 1.18e9, **l5 4.56e8**, l6 5.45e8 |
| K_0 n=13 (r=2003) | l=4: s3* 1.56e11, **s4 1.48e11**, s5* 2.23e11; l=5: s5 4.37e11 | l4 2.03e9, l5 1.88e9, **l6 1.03e9**, l7* 1.40e9, l8* 1.76e9 |
| K_0 n=15 (r=751) | l=4 s4 1.23e12 (E1); everything else **projected, not run** (below) | l5 2.32e10, l6 3.93e9, **l7 1.85e9**, l8* 2.25e9, l9* 2.25e9 |

Lab sweeps now have an interior minimum at every size except K_1 n=9, where l=2 does not find relations and l=3 is the bottom of the valid range (l=3, 4, 5 are within 6 %).
In particular n=15 is no longer on the grid edge: l=7 beats l=8 and l=9 (re-measured, 8/8 each). The published best is l=4 everywhere; spec 3 or 4 on the whole run
(within 8 % of each other), spec 4 to 6 on the solver-only figure, because that figure charges no msolve start-up and so keeps falling with spec while the whole run rises.

**Mechanism checked** (`r2/runs/R2_k0n9`): relation systems per target depend on l and n only, not on spec (l=4, n=9: 15.5 at s3, s5 and s6; msolve calls = systems x 2^spec).
So cost(l, s) = systems(l) x per-system(l, s), and per-system cost can be probed.

**K_0 n=15, published side (projected, not whole-solved).** msolve instructions per relation system, probe (`r2/probes/n15_*`, 3 targets for l=4, 2 for l=5, 1 for l=6):
l=4: s2 2.57e9, s3 1.46e9, **s4 1.11e9**, s5 1.61e9, s6 2.78e9; l=5: s4 1.80e10, s5 5.6e9, s6 6.7e9, s7 9.7e9; l=6: s7 4.6e10, s8 4.7e10, s10 6.7e10.
Systems per target: l=4 577 (8 targets, measured), l=5 341 (1 target, E1 partial). Projected msolve totals per target: l=4 s4 6.4e11 (matches the measured whole run,
msolve about half of 1.23e12); l=5 s5 at least 1.9e12; l=6 at least 4.6e10 x (systems, not measured, at least 100) = 4.6e12 or more; l=4 s3 8.4e11 (+30 % in msolve, within about 10 % on the whole run
by the n=9, 11, 13 pattern). A cell is run only when projected cheaper than the current best; none is, so l=4 s4 stands. The n=15 published point therefore rests on one measured config plus probe-based
exclusion of the rest, not on a whole-run sweep. (n=11 and n=15 probes: `r2/probes/n11_a1_*`, `n15_*`; the n=13 probes are section 9.2.) l=3 (and l=2) are not retried above n=9: their relation yield only falls with n.

Not swept on the published side (as in section 9.6): large primes, m=2 or 4, msolve `-l` variants, e-form hybrid (4x dearer at n=9 spec 4), msolve 0.6.5, l=6 whole solves at n=9 to 13 (probe per-system 8x l=5).

## Result: best published against best lab

| cell | best published (whole run): config, Ir/target | best lab: l, Ir/target | ratio whole run | best published (solver-only): config, Ir/target | ratio solver-only |
|:--|:--|:--|--:|:--|--:|
| K_0 n=9  | x l4 s3, 2.60e10 | l5, 2.95e8 | 86.2 | l4 s6, 4.92e9 | 16.5 |
| K_1 n=9  | x l4 s3, 1.68e10 | l3, 2.37e8 | 70.9 | l4 s6, 3.19e9 | 13.1 |
| K_1 n=11 | x l4 s3, 9.58e10 | l5, 4.56e8 | 206 | l4 s4, 2.39e10 | 51.6 |
| K_0 n=13 | x l4 s4, 1.48e11 | l6, 1.03e9 | 136 | l4 s5, 2.53e10 | 23.2 |
| K_0 n=15 | x l4 s4, 1.23e12 (probe-excluded alternatives) | l7, 1.85e9 | 634 | l4 s4, 2.66e11 | 137 |

Compared with E1's fixed-published rows (88.9, 68.1, 223, 136, 634 whole; 20.8, 16.2, 51.6, 34.6, 137 solver-only): the published sweep moves the ratio by 0 to 8 % on the whole run, and at the
solver-only figure by 20 % to 33 % at n=9 and n=13, none at n=11 and n=15. The lab sweep, not the published one, is what moves the ratio with n.

## Fit: ln(best published / best lab) on n (OLS, 95 % t interval, k cells; `r2/r2_analysis.txt`, `r2/r2_res.json`, `r2/r2_analyze.py`)

| set | whole run: slope per bit | x per 4 bits | solver-only: slope per bit | x per 4 bits |
|:--|:--|:--|:--|:--|
| all 5 cells | **+0.296 +- 0.285** (CI +0.011 to +0.581, p=0.05) | 3.3 (1.04 to 10.2) | **+0.304 +- 0.382** (CI -0.078 to +0.686, p=0.09) | 3.4 (0.73 to 15.6) |
| without n=15 (k=4) | +0.170 +- 0.555 | 2.0 (0.22 to 18) | +0.161 +- 0.819 | 1.9 (0.07 to 50) |
| K_0 only (k=3) | +0.302 +- 2.06 | 3.3 | +0.314 +- 2.53 | 3.5 |

## Verdict: undecided, leaning growing

- Under the pre-stated rule the whole-run slope is growing only by a hair (CI lower end +0.011, p=0.05); the solver-only slope, the figure most favourable to the published arm, has a CI that contains 0
  and is wide, so it is not growing, not flat (half-width 0.38, far over 0.1) and not shrinking. The two metrics give the same point estimate, +0.30 per bit, about x3.3 per 4 bits.
  Combined: **undecided**; the evidence favours growth over flat or shrinking, and nothing in the data supports flat.
- The estimate is carried by the top cell. Without n=15 the CI is +-0.56 to +-0.82 per bit; n=15 alone is a single published point (excluded alternatives are projections), and K_1 n=11 sits well above the K_0 line (206 against 86 and 136).
  Two families and r (37 to 2003) are mixed with 5 cells; the interval is a t interval on 3 degrees of freedom.
- The reason for the growth is visible in the table, not in the fit: the lab's own optimum moves up with n (l = 3, 5, 5, 6, 7) and its cost grows from 2.4e8 to 1.9e9 (x8 over 6 bits) while the swept published cost grows
  from 1.7e10-2.6e10 to 1.2e12 (x47 to x73 over the same 6 bits, whole run) and 3.2e9-4.9e9 to 2.7e11 (x55 to x83, solver-only): the published arm's relation systems per target rise
  (16 at n=9 to 577 at n=15 for l=4) with every system dear, and the l that would cut the system count (5, 6) costs 5x to 40x more per system at every size.
- What would decide it: a published whole-run point at n=17 (projected 5e12 Ir per target, about 3 h for 8 targets: over the cap, so skipped) or a second K_0 cell between 13 and 15 (none has a usable prime-order subgroup in the constructor, E1 note).

## Not done

- No whole-run sweep of the published arm at n=15 (projected cheaper configs: none; l=4 s3 not run; l=5 and l=6 projected 3x to 8x dearer); no n >= 17; e-form, LP, m=2/4, msolve `-l`, msolve 0.6.5 and WDSat not tried.
- The published whole run includes this harness's own Rust work (about half; section 9.1); the solver-only figure removes it.
- K_0 n=9 lab l=3 was started and killed (not needed: l=4 > l=5 already; the session is not in the evidence). Ir per GAE and the rho distance were not re-measured.
- Lab and published sweeps were measured in the same session law as E1 but in different sessions; lab arms in E1 and R2 reproduce within 0.5 % where repeated.

Evidence: `tuned_20261030/r2/` (`specs/`, `runs/` records and per-child `time -l` files, `probes/`, `mkspec.py`, `runcell.sh`, `queue.sh`, `r2_analyze.py`, `r2_res.json`, `r2_analysis.txt`).
