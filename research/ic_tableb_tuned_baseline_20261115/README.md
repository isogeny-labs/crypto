# Table B incumbent baselines: recorded vs in-session tuned (evidence note, 2026-11-15)

Status: evidence note. No new runs; every number below is copied from the cited source files or is a ratio of two such numbers. All numbers are [pcc:unverified] by this note; they inherit the standing of their sources (binary study: validator report VAL-20261115-8c7c3c, `research/ic_tableb_binary_ksum_20261115/validator/validator-report.json`; prime study: validator report `research/ic_tableb_ksum_20261115/validator/validator-report.json`, status "pass", no VAL id recorded in it; gate files `benchmarks/20261115-tableb-binary-ksum/VALIDATION.json`, `benchmarks/20261115-tableb-ksum/VALIDATION.json`). Counted cost (L0), ecbench GAE per sqrt(r), native work priced at w = 0 / 0.1 / 1 (AGENTS.md section 5a).

## What this is, and is not

Re-tuning the incumbent is an accounting/baseline correction. It is NOT an index-calculus improvement under the success criterion (CLAUDE.md: "Relabelling and accounting changes are not improvements"). No solver got faster. The point is narrow: Table B's recorded IC/rho gaps for the random-binary rows overstate the distance remaining for a tuned pair table, so the distance should be read against the tuned baseline. The k-sum improvement itself is measured in the two source studies, against the tuned incumbent, and is not restated here.

Proposal (for the owner, not a decision): re-baseline Table B's binary rows (and say so in the row) in a future preregistered Table B round, with the pair-table base tuned (smallest-abscissa `binary-multiplier`, size bracketing the pilot optimum) and the rho reference run in the same session and binary. The prime rows need no such correction (below).

## Findings

1. Prime rows (fp20/22/24): Table B's recipe base (`inc-tb`, N = |F|/2) is already within 0.94-1.04x of the best other incumbent at every weight (table below, column "inc-tb / best"). Table B's prime IC/rho gaps (8.3x, 10.5x, 13.5x recorded) are not an artefact of an untuned incumbent: in-session `inc-tb`/rho-neg is 8.11, 9.51, 11.80 at w = 0, and best/rho-neg 8.16, 10.10, 11.72.
2. Random-binary rows (n15..n27): `inc-tb` (Table B's own subspace base) is 1.37, 1.92, 2.15, 2.06, 3.44 times the best tuned smallest-abscissa incumbent at w = 0 (1.4-3.4x, as the leaderboard says; at w = 1 the factors are 1.47, 2.32, 2.56, 2.37, 3.86, so "3.4x" holds for w = 0 only). The best tuned incumbent over rho-neg at w = 0 is 6.02, 5.78, 8.16, 9.08, 13.05, against recorded Table B IC/rho of 13.1, 10.6, 18.0, 23.0, 38.2.
3. The recorded Table B S_IC/S_rho come from another harness (`docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`, calibration-priced GAE). They are shown for reference and never divided into the in-session S; the comparison of recorded IC/rho with in-session ratios mixes harnesses. In-session `inc-tb` is not equal to the recorded S_IC (e.g. n15 16.27 vs 26.31; n27 47.91 vs 36.93), in either direction.
4. Distance remaining stays large everywhere: ksum5/rho-neg is 5.6-6.9 at all rows and weights; rho stays ahead (section 5a band applies; see the validator reports).

## Table (all rows, w = 0 / 0.1 / 1)

Columns: "rec." = Table B recorded S_IC / S_rho (another harness); inc-tb = Table B recipe base measured in-session; best non-tb incumbent = lowest-S of the other incumbent arms in the same session (prime: `inc-rec`/`inc-sel`/`inc-w0`/`inc-lo`/`inc-hi`, other sizes of the recipe family; binary: `inc-n*`, tuned smallest-abscissa base); ksum4/ksum5 = the studies' candidates; ratios are ratios of the listed numbers (3 sig. fig. rounding). Note the best non-tb incumbent is not always the study's "strictest incumbent" (inc-tb is strictest where it is lowest; the studies' choice is in each `summary.json`).

| row | w | rec. S_IC / S_rho (ratio) | inc-tb S | best non-tb incumbent S | inc-tb / best | ksum4 S | ksum5 S | rho-neg S | rec. IC/rho | inc-tb/rho-neg | best/rho-neg | ksum5/rho-neg |
|:--|--:|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| fp20 | 0 | 10.72 / 1.29 | 10.29 | inc-sel: 10.36 | 0.99 | 6.64 | 7.11 | 1.27 | 8.3 | 8.11 | 8.16 | 5.60 |
| fp20 | 0.1 | 10.72 / 1.29 | 11.80 | inc-sel: 11.77 | 1.00 | 7.47 | 7.86 | 1.38 | 8.3 | 8.56 | 8.53 | 5.70 |
| fp20 | 1 | 10.72 / 1.29 | 25.39 | inc-sel: 24.49 | 1.04 | 14.96 | 14.59 | 2.37 | 8.3 | 10.69 | 10.31 | 6.14 |
| fp22 | 0 | 12.77 / 1.22 | 11.15 | inc-hi: 11.83 | 0.94 | 6.28 | 6.65 | 1.17 | 10.5 | 9.51 | 10.10 | 5.67 |
| fp22 | 0.1 | 12.77 / 1.22 | 12.89 | inc-hi: 13.45 | 0.96 | 7.09 | 7.39 | 1.28 | 10.5 | 10.08 | 10.51 | 5.78 |
| fp22 | 1 | 12.77 / 1.22 | 28.60 | inc-hi: 28.04 | 1.02 | 14.34 | 14.07 | 2.25 | 10.5 | 12.71 | 12.47 | 6.25 |
| fp24 | 0 | 13.97 / 1.04 | 12.82 | inc-w0: 12.74 | 1.01 | 6.37 | 6.38 | 1.09 | 13.5 | 11.80 | 11.72 | 5.87 |
| fp24 | 0.1 | 13.97 / 1.04 | 14.91 | inc-w0: 14.78 | 1.01 | 7.22 | 7.16 | 1.18 | 13.5 | 12.58 | 12.48 | 6.04 |
| fp24 | 1 | 13.97 / 1.04 | 33.68 | inc-w0: 33.16 | 1.02 | 14.95 | 14.13 | 2.06 | 13.5 | 16.33 | 16.08 | 6.85 |
| f2m15 | 0 | 26.31 / 2.01 | 16.27 | inc-n18: 11.84 | 1.37 | 11.19 | 13.47 | 1.97 | 13.1 | 8.27 | 6.02 | 6.85 |
| f2m15 | 0.1 | 26.31 / 2.01 | 17.71 | inc-n18: 12.75 | 1.39 | 11.89 | 14.19 | 2.11 | 13.1 | 8.41 | 6.05 | 6.74 |
| f2m15 | 1 | 26.31 / 2.01 | 30.65 | inc-n18: 20.92 | 1.47 | 18.17 | 20.64 | 3.37 | 13.1 | 9.11 | 6.22 | 6.13 |
| f2m18 | 0 | 21.30 / 2.01 | 20.82 | inc-n21: 10.85 | 1.92 | 9.79 | 11.26 | 1.88 | 10.6 | 11.09 | 5.78 | 6.00 |
| f2m18 | 0.1 | 21.30 / 2.01 | 23.42 | inc-n21: 11.77 | 1.99 | 10.52 | 11.98 | 2.02 | 10.6 | 11.58 | 5.82 | 5.92 |
| f2m18 | 1 | 21.30 / 2.01 | 46.75 | inc-n21: 20.12 | 2.32 | 17.12 | 18.50 | 3.33 | 10.6 | 14.03 | 6.04 | 5.55 |
| f2m21 | 0 | 24.39 / 1.36 | 22.15 | inc-n62: 10.28 | 2.15 | 6.73 | 7.01 | 1.26 | 18.0 | 17.57 | 8.16 | 5.56 |
| f2m21 | 0.1 | 24.39 / 1.36 | 26.13 | inc-n62: 11.67 | 2.24 | 7.57 | 7.73 | 1.37 | 18.0 | 19.06 | 8.52 | 5.64 |
| f2m21 | 1 | 24.39 / 1.36 | 61.88 | inc-n62: 24.18 | 2.56 | 15.16 | 14.27 | 2.36 | 18.0 | 26.24 | 10.25 | 6.05 |
| f2m24 | 0 | 27.24 / 1.18 | 22.60 | inc-n67: 10.99 | 2.06 | 6.58 | 6.99 | 1.21 | 23.0 | 18.67 | 9.08 | 5.77 |
| f2m24 | 0.1 | 27.24 / 1.18 | 26.55 | inc-n80: 12.52 | 2.12 | 7.33 | 7.87 | 1.32 | 23.0 | 20.11 | 9.49 | 5.96 |
| f2m24 | 1 | 27.24 / 1.18 | 62.07 | inc-n80: 26.14 | 2.37 | 14.14 | 15.81 | 2.30 | 23.0 | 26.95 | 11.35 | 6.86 |
| f2m27 | 0 | 36.93 / 0.97 | 47.91 | inc-n150: 13.95 | 3.44 | 6.70 | 6.41 | 1.07 | 38.2 | 44.83 | 13.05 | 6.00 |
| f2m27 | 0.1 | 36.93 / 0.97 | 57.04 | inc-n150: 16.16 | 3.53 | 7.58 | 7.17 | 1.17 | 38.2 | 48.85 | 13.84 | 6.14 |
| f2m27 | 1 | 36.93 / 0.97 | 139.22 | inc-n178: 36.05 | 3.86 | 15.46 | 13.95 | 2.06 | 38.2 | 67.63 | 17.51 | 6.78 |

Sources, per row: prime fp20/22/24: `research/ic_tableb_ksum_20261115/summary.json` (`rows[].table_b`, `rows[].weights[w].s`) and `results.md`; binary f2m15/18/21/24/27: `research/ic_tableb_binary_ksum_20261115/summary.json` (same keys) and `results.md`; recorded S_IC, S_rho, IC/rho, recipe, columns: same `table_b` fields, whose prime source is `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json` and whose values match `docs/ic/LEADERBOARD.md` section "Table B: Prime and random-binary ladder" (S_IC 10.7, 12.8, 14, 26.3, 21.3, 24.4, 27.2, 36.9; IC/rho 8.3, 10.5, 13.5, 13.1, 10.6, 18, 23, 38.2). Machine-readable copy: `table.json` (regenerate: `make_note.py`, `plot.py`). Figure: `baseline-gap.png` / `.svg`; one-page `REPORT.pdf`.

## Inconsistencies and caveats found between sources

- The leaderboard says inc-tb is "1.4-3.4x the tuned incumbent": true at w = 0 only (1.37-3.44); at w = 1 it is 1.47-3.86.
- The binary summary's `table_b.recipe` for f2m15 is "folded table, walked targets, m = 2" while the other binary rows say "folded, walked, base at the family optimum, m = 2" (as recorded in the ledger/leaderboard; the f2m15 and fp12 rows are the ones with the other wording). The f2m15 and f2m18 rows both record 15 columns / 30 signed points, so Table B's recorded recipe for these two is the same size.
- Recorded vs in-session `inc-tb` differ across harnesses (finding 3); the in-session `inc-tb` is the better reference for "Table B's recipe", the recorded one is only the before mark.
- Prime validator report carries no `VAL-` id (its own schema `validation-gate.validator-report/v1`, status "pass"); the binary one is VAL-20261115-8c7c3c.
- The binary validator reports the producer's pooled/per-replicate upper ends differ slightly from its own recomputation (see its caveats); this note uses the producer's `summary.json` values, not the validator's `own-numbers.json`.
- In the binary rows f2m15 (all w) and f2m18 (w <= 0.1) ksum5 is not below the tuned incumbent (L1 does not hold there); only ksum4 is. This note does not change that.
