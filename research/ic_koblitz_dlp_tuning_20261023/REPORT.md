# REPORT: tuned double-large-prime vs the Koblitz compact-orbit baseline (2026-10-28)

Protocol: `PROTOCOL.md` (frozen 2026-10-23, commit 327afb04f; Amendments 1 to 3 before the data they affect).
Binary for every session: SHA-256 `91263a526a9f591f601acf8a9372fd718da16b486fab7271d3f4f8b3d6b83798`
(origin/main a53a7e2bf + the Amendment 1 edit). Host: shared 4-core Xeon VM, isolation L0, one core,
operation counts only; no wall-clock claim. Public synthetic Koblitz targets; no security claim.

## Result (class: null; no improvement is claimed)

**On none of the four curves did the tuned double-LP arm beat the frozen baseline `ic-compact` at every
weight of the §5a band.** By the preregistered rule the verdict is "unresolved inside the band" at
n = 37, 43, 47 and 41 (at n = 41 and n = 47 the arm is worse at w = 0 and 0.1). The best case is
n = 43, where the arm is cheaper at w = 0.1 and 1 and level at w = 0 (interval spans 1).

Holdout (96 fresh targets per curve, seeds disjoint from train, all 96 x 6 runs verified, replay-all OK,
A/A = 1.000000 at every weight with a degenerate interval on all curves, both arms). Paired ratio
`sum S_dlp-selected / sum S_base-frozen` with 95% two-stage cluster bootstrap CI (`ic_paired_band`):

| n (r) | selected cell | w = 0 | w = 0.1 | w = 1 | verdict (protocol §2) |
|:--|:--|:--|:--|:--|:--|
| 37 (2^27.8) | K=1, c=10 | 1.001 [0.945, 1.061] | 0.793 [0.751, 0.838] | 0.570 [0.536, 0.603] | unresolved (w=0 ties) |
| 43 (2^32.1) | K=1, c=20 | 0.985 [0.928, 1.051] | 0.937 [0.885, 0.995] | 0.866 [0.819, 0.919] | unresolved (w=0 interval spans 1) |
| 41 (2^39.0) | K=1, c=160 | 1.183 [1.155, 1.213] | 1.112 [1.084, 1.143] | 0.955 [0.922, 0.989] | unresolved (worse at w=0, 0.1; better at w=1) |
| 47 (2^36.6) | K=3, c=20 | 1.095 [1.060, 1.132] | 1.067 [1.032, 1.102] | 1.015 [0.980, 1.052] | unresolved (worse at w=0, 0.1; ties at w=1) |

n = 37 and 43 are the primary pair; "established improvement" needed both to succeed: not met.
The baseline is `base-frozen` (Phase 1 Kb = 8, 128, 16, 64). The re-tuned baseline equalled the frozen
one on every curve (stage-2 minimax = 1 at Kb), so no `base-retuned` arm exists and the equal-tuning clause is moot.
`ecbench compare` (recorded S) gives 1.007 [0.949, 1.069], 1.184, 0.985 [0.928, 1.051], 1.095 for
w = 0 (n = 37, 41, 43, 47); the n = 37 and 41 values differ from the table in the third digit because the
recorded S carries those curves' pinned-calibration native cost, which §5a says not to set against uncalibrated rho:
the band's w = 0 is the one used. (`ecbench compare --weight` arrived on main after the sessions; the merged
main was not built because the disk was at 98% to 100% full. `ic_paired_band` computes the same statistic
at each weight from the same records and agrees with `ecbench compare` at w = 0 on n = 43 and 47 to all digits.)

Observation behind the w > 0 gain (measured, mechanism not tested here): S at w = 0 ties at n = 37 and 43, but the
double-LP arm leaves fewer unpriced native operations per run than the baseline (band S(w=1) minus S(w=0), in units of
sqrt(r): n = 37 3.72 vs 7.29; n = 43 4.89 vs 5.81; n = 41 5.27 vs 5.99; n = 47 5.60 vs 5.62). The baseline names 17
unpriced counters, the double-LP arm 19, rho-frob 1 (`canonicalisations_uncharged`); every one carries a count, so all arms
are rankable. This is an accounting-weight effect, not a lower count at w = 0, so it is not an improvement in the sense of
the success criterion: the order does not hold across the band.

## Distance remaining (S of the arm over S of the matched `rho-frob`, same weight, holdout; 95% CI)

| n | baseline w=0 | baseline w=0.1 | baseline w=1 | double-LP w=0 | double-LP w=0.1 | double-LP w=1 |
|:--|:--|:--|:--|:--|:--|:--|
| 37 | 4.87 [4.34, 5.47] | 7.76 [6.94, 8.70] | 21.6 [19.3, 24.3] | 4.87 [4.35, 5.46] | 6.15 [5.48, 6.92] | 12.3 [10.9, 14.0] |
| 41 | 14.0 [12.7, 15.5] | 16.6 [15.0, 18.4] | 28.2 [25.6, 31.3] | 16.6 [15.1, 18.3] | 18.4 [16.8, 20.4] | 27.0 [24.4, 29.9] |
| 43 | 7.86 [7.09, 8.71] | 10.8 [9.76, 11.9] | 24.2 [22.0, 26.8] | 7.74 [7.04, 8.53] | 10.1 [9.18, 11.1] | 21.0 [19.0, 23.2] |
| 47 | 10.1 [9.11, 11.2] | 12.9 [11.7, 14.4] | 25.7 [23.2, 28.5] | 11.1 [10.0, 12.3] | 13.8 [12.4, 15.3] | 26.1 [23.5, 29.0] |

Both arms stay between 4.9x and 28x the matched rho at every weight. At w = 0 the baseline ratios (4.9, 14.0, 7.9, 10.1)
reproduce Phase 1's (5.2, 15.0, 7.4, 9.9). Against `rho-strong` the n = 37 double-LP arm is 3.69x (w=0), 4.48x (0.1), 7.55x (1).
The old best double-LP cell (17.7x rho at n = 41) is matched, not beaten, by the n = 41 selected cell (16.6x at w = 0).

## Controls

- **Positive control** (n = 41, K = 10 columns, same code path, 8 targets): double-LP / plain `m=3` oracle
  = 0.0316 [0.0267, 0.0374] at w = 0 (0.0379 at w = 0.1, 0.0566 at w = 1), i.e. 31.6x cheaper; the preregistered
  bar was an upper bound below 0.2, so the instrument sees a real double-LP effect. The integration note's 23.8x is
  reproduced in direction and size (different targets and a different count of 4 vs 8).
- **A/A**: base-frozen vs base-frozen-aa and dlp-selected vs dlp-selected-aa are exactly 1.0 with interval [1, 1]
  at w = 0, 0.1, 1 on all four curves.
- **Replay**: `ecbench verify --replay-all` OK on all holdout, stage-1, stage-2 and the r2 positive-control sessions
  (`audits/`; holdout receipts `audits/holdout-n*.audit.json`). The first positive-control session
  (`sessions/poscontrol-n41`) FAILED replay on a wall-clock counter, `lp_filter_wall_ns`, in the double-LP oracle's
  replayed record (Amendment 1); it is kept as evidence and the one-line fix changes no count (`total_gae`, every other
  native counter and `counters` hash identically before and after, record for record). `sessions/train-n37` is the interrupted
  one-stage attempt (Amendment 2), unused.

## Train panel and selection (not a result; for reproducibility)

Stage 1 (65 cells, 8 targets, `stage1/paired-n*.json`) pruned to 10 cells per curve (`survivors.json`); stage 2
(48 fresh targets) selected one cell by the minimax-over-band objective (`selection.json`, SHA-256
`b610fc66…`, frozen before any holdout run). Stage-2 minimax to the baseline of the selected cells: n = 37 1.036,
n = 41 1.155, n = 43 0.893, n = 47 1.132. The only cell that looked better than the baseline on train
(n = 43, 0.893) regressed on holdout to 0.985 [0.928, 1.051] at w = 0, the worst weight: a winner's-curse shrinkage
of about 9 points. The selected cells moved to the minimum-K edge, K = 1 (hard edge; n = 47 picked K = 3): with one base column,
c = 10 to 160 large-prime domain points carry the relations. Figure `reports/fig_stage1_grid.svg` shows the grid.

## What this does and does not show

Shown: on these four curves, with one K_0/K_1 curve per size and parameters tuned per curve on disjoint targets,
the grid K in 1..16 x c in 1..320 (K x c <= 640) of the existing double-LP oracle gives no cell that lowers
S at w = 0, and at w > 0 gives only a partial gain at n = 37 and 43 that disappears at 47. Not shown: anything
about other sizes, other curve families, other seeds of the same curve (tuning is per curve, no family claim), wall time,
asymptotics, a multi-target or amortised setting, or a different filter than the one implemented (the large-prime
collector caps at one relation per trial; at K = 1 the pair table is tiny, so the gain at w > 0 comes from replacing priced
work, which is accounting). The grid edge K = 1 is a hard edge, so a lower-K trend cannot be tested here.
Family-wise: four curves at 95%; no curve reached success, so no multiplicity correction is needed.

## Next thing to try

The selected cells sit at K = 1: the base is nearly empty and the large-prime domain does the work. The un-tried lever is
therefore the domain rather than the base: draw `D` from a structured set (small Z[tau]-norm multipliers, the A1
closure that gave 9.2x at n = 41) instead of the next raw-x orbits, and let the collector return more than one relation per
trial. Measure at n = 43 first (the only curve with a train-side gap), with this protocol and these seeds as the baseline.

## Files

`PROTOCOL.md`, `specs/`, `sessions/`, `audits/`, `bands/`, `stage1/`, `stage2/`, `holdout/paired-n*.json|txt`,
`comparisons/`, `selection.json`, `survivors.json`, `SPEC_HASHES.txt`, `timings.txt` (run and audit seconds; about 5 core-hours
in total against the 12 cap), `reports/` (graphs `fig_*.svg|png`, `plot_results.py`, `REPORT.pdf`), code `examples/ic_paired_band.rs`.
