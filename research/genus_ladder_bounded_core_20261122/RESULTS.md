# Weight-bounded core before Wiedemann: results (2026-11-22) [pcc:unverified]

Protocol and predictions frozen before data (`PROTOCOL.md`, `frozen.sha256`, binary `binary.sha256`). Genus 3, p = 32771,
65537, 131101, 262147, 3 curves x 2 seeds each (the earlier studies' curves), arm c, five arms per instance. 120 runs, all
certified by the existing `mul`/`mul_rtl` check; the 48 plain and unmerged rows equal the earlier studies' rows in every
field except wall time (B1). Full per-prime tables (rows, columns, non-zeros, weights, probes, E): `tables.md`. Figure:
`fig_la_mul_exponent.svg`. Raw rows: `run_P.jsonl`; analysis: `analysis.json`; hashes: `data.sha256`.

## What was built
`genus_ladder.rs`: `BoundedParams`, `select_bounded_rows` (greedy heaviest-row drop keeping COVER live rows per column,
down to cols + margin, additions only), `run_ic_ext_bounded` (adaptive excess: collect in steps of STEP x N0 until the
selected core's mean weight <= W or EMAX; probe work counted in `la`); `run_ic_ext_excess` calls it with `None`. New
`IcResult` fields `bd_n0`, `bd_probes`, `bd_extra`, `bd_capped`. Driver: `--bounded W[,STEP[,EMAX[,COVER]]]`.
Here W = 8, STEP 0.1, EMAX 3.0; cover 2 is the task's "covered at least twice", cover 1 is R4's selection with adaptive E.

## Per-prime summary (cell means; FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul)
Mean row weight: peeled core -> after the step -> after SGE (nnz_out / rows_out). `la_mul` and FPM3 ratios are to plain SGE(8,3).

| p | arm | E | core rows x cols (peeled) | after step | weight peeled / step / SGE | la_mul | relation phase | FPM3 | FPM3 / plain | la_mul / plain |
|--:|---|--:|--|--|--|--:|--:|--:|--:|--:|
| 32771 | plain | 0 | 2.00e3 x 1.93e3 | - | 9.55 / - / 20.28 | 1.80e7 | 1.24e8 | 1.78e8 | 1 | 1 |
| 32771 | R4 ex(0.3) | 0.3 | 4.71e3 x 2.04e3 | 1.87e3 x 1.84e3 | 9.14 / 7.13 / 18.85 | 1.03e7 | 1.61e8 | 1.92e8 | 1.077 | 0.573 |
| 32771 | bd cover 2 | 0.2 | 3.74e3 x 2.03e3 | 2.06e3 x 2.03e3 | 9.29 / 7.69 / 19.54 | 1.30e7 | 1.49e8 | 1.88e8 | 1.056 | 0.725 |
| 32771 | bd cover 1 | 0.2 | 3.74e3 x 2.03e3 | 1.88e3 x 1.84e3 | 9.29 / 7.53 / 19.17 | 1.13e7 | 1.49e8 | 1.83e8 | 1.027 | 0.630 |
| 65537 | plain | 0 | 3.20e3 x 3.12e3 | - | 10.47 / - / 21.27 | 5.19e7 | 3.08e8 | 4.64e8 | 1 | 1 |
| 65537 | R4 ex(0.3) | 0.3 | 8.12e3 x 3.25e3 | 3.01e3 x 2.96e3 | 9.91 / 7.59 / 19.35 | 2.95e7 | 4.01e8 | 4.89e8 | 1.054 | 0.568 |
| 65537 | bd cover 2 | 0.3 | 8.12e3 x 3.25e3 | 3.29e3 x 3.25e3 | 9.91 / 7.73 / 19.61 | 3.36e7 | 4.01e8 | 5.01e8 | 1.081 | 0.648 |
| 65537 | bd cover 1 | 0.3 | 8.12e3 x 3.25e3 | 3.01e3 x 2.96e3 | 9.91 / 7.59 / 19.35 | 2.95e7 | 4.01e8 | 4.89e8 | 1.054 | 0.568 |
| 131101 | plain | 0 | 5.12e3 x 5.02e3 | - | 11.58 / - / 22.27 | 1.50e8 | 7.64e8 | 1.21e9 | 1 | 1 |
| 131101 | R4 ex(0.3) | 0.3 | 1.41e4 x 5.16e3 | 4.86e3 x 4.80e3 | 10.71 / 8.11 / 19.95 | 8.54e7 | 9.92e8 | 1.25e9 | 1.029 | 0.570 |
| 131101 | bd cover 2 | 0.4 | 1.77e4 x 5.16e3 | 5.22e3 x 5.16e3 | 10.44 / 7.78 / 19.91 | 8.55e7 | 1.07e9 | 1.33e9 | 1.093 | 0.571 |
| 131101 | bd cover 1 | 0.4 | 1.77e4 x 5.16e3 | 4.82e3 x 4.74e3 | 10.44 / 7.67 / 19.82 | 7.68e7 | 1.07e9 | 1.30e9 | 1.071 | 0.513 |
| 262147 | plain | 0 | 8.16e3 x 8.03e3 | - | 12.77 / - / 22.98 | 4.27e8 | 1.85e9 | 3.13e9 | 1 | 1 |
| 262147 | R4 ex(0.3) | 0.3 | 2.44e4 x 8.19e3 | 7.79e3 x 7.69e3 | 11.57 / 8.57 / 20.43 | 2.41e8 | 2.40e9 | 3.12e9 | 0.998 | 0.563 |
| 262147 | bd cover 2 | 0.5 | 3.83e4 x 8.19e3 | 8.28e3 x 8.19e3 | 10.92 / 7.84 / 19.77 | 2.24e8 | 2.77e9 | 3.44e9 | 1.100 | 0.523 |
| 262147 | bd cover 1 | 0.5 | 3.83e4 x 8.19e3 | 7.71e3 x 7.61e3 | 10.92 / 7.76 / 19.60 | 2.04e8 | 2.77e9 | 3.39e9 | 1.082 | 0.478 |

Intervals (95 % bootstrap over curves and seeds) for the ratios are in `tables.md`; the FPM3 ratios of the bd arms at 262147 are
1.100 [1.096, 1.104] (cover 2) and 1.082 [1.077, 1.086] (cover 1). The adaptive E rose 0.2, 0.3, 0.4, 0.5 with p (mean over the six
runs, 1 to 4 probes, never capped). The bd arms' selection additions (`la_sel_add`) are 0.09 to 2.9 million against 1e8 to 1e9 additions in
Wiedemann.

## Exponents over the four primes (95 % bootstrap interval)
| arm | la_mul | FPM3 | relation phase |
|---|--|--|--|
| plain SGE(8,3) | 1.524 [1.516, 1.533] | 1.380 [1.377, 1.383] | 1.300 [1.297, 1.304] |
| R4 ex(0.3,0.01) | 1.517 [1.499, 1.531] | 1.343 [1.341, 1.345] | 1.301 [1.299, 1.304] |
| bd(8) cover 2 | 1.365 [1.351, 1.378] | 1.399 [1.396, 1.402] | 1.408 [1.405, 1.411] |
| bd(8) cover 1 | 1.390 [1.376, 1.402] | 1.405 [1.402, 1.407] | 1.408 [1.405, 1.411] |

Paired differences of the `la_mul` exponent against plain: -0.159 [-0.169, -0.150] (cover 2), -0.134 [-0.145, -0.123] (cover 1);
of the FPM3 exponent: +0.019 and +0.025 (both intervals above 0). FPM9 exponents (as in R2 to R4): plain 1.440, bd cover 2 1.390,
cover 1 1.401. FPM1: plain 1.335, bd 1.405 to 1.407.

## Scorecard
| id | result |
|---|---|
| B1 | **holds**: 120 runs certified; 48 plain and unmerged rows equal the earlier rows (0 mismatches), instance lines equal |
| B2 | **holds** for both arms: mean weight after SGE max / min over the four primes 1.019 [1.011, 1.034] (cover 2), 1.034 [1.020, 1.055] (cover 1); plain 1.133, R4 1.084. Weight after the step 7.1 to 7.8 (target 8) |
| B3 | **holds** for both arms: `la_mul` exponent 1.365 (upper 1.378) and 1.390 (upper 1.402); plain 1.524 |
| B4 | **fails** for both arms. (i) FPM3 at 262147 is 1.100 and 1.082 of plain; (ii) FPM3 exponent 1.399 and 1.405 against plain's 1.380 |

## What this says
- The linear algebra's exponent falls from about 1.52 to 1.37 to 1.39 when the core is held near a mean row weight of 8 and E is chosen per
  instance: the mechanism predicted in the brief (bounded weight keeps SGE's merge share, and so the column count, from drifting up) is
  visible in the counts. The merged system's mean weight is flat at 19.2 to 19.9; its columns after SGE grow as p^0.70 to 0.71
  (plain 0.78).
- It does not carry to the total. The price of the bounded weight is a larger relation excess, E rising with p (0.2 to 0.5), which raises the
  relation-phase exponent from 1.30 to 1.41 over this range, and the relation phase is 0.59 of plain's FPM3 at 262147 and 0.82 of the bd arms' (kappa = 3). Net: the
  bounded arms are 3 to 10 % above plain at kappa = 3 (bd cover 1 at 32771 and 262147: 1.027, 1.082) and their FPM3 exponent is higher.
  They are ahead only where the linear algebra is weighted more: at kappa = 9 the FPM9 exponent is lower (1.39 to 1.40 against 1.44); R4's
  fixed E = 0.3 arm is the better kappa = 3 arm here (FPM3 exponent 1.343, 0.998 of plain at 262147) because it pays a constant E.
- Since E must keep rising to hold W fixed (a fixed weight needs cycles of bounded length, and the number of relations needed grows
  polynomially in p for that), a fixed W does not yield a lower total exponent in this range; whether a W that grows slowly with p
  (so that E stays near constant) would balance the two phases was not measured.
- Cover 1 (re-peel) has the lower `la_mul` at every prime (0.87 to 0.91 of cover 2's from 65537 up, 0.87 at 32771) and the lower FPM3; cover 2 has the
  lower fitted exponent (1.365 against 1.390) only because its level at 32771 is relatively high (0.725 of plain against 0.630), which flattens its slope.
  Both are reported; neither passes B4.

## Not an improvement claim
At kappa = 3 neither bounded arm beats plain (B4 fails); the la_mul exponent result is a count of one phase, not wall time, over
four primes and three curves each, with a large relation excess that is part of the cost. No matched-rho comparison was run (the earlier
rho rows exist only for 65537 to 262147 and are unchanged); no replay, no isolated host, no `VALIDATION.json`, no leaderboard or
`benchmarks/` row. Every figure `[pcc:unverified]`.

## Limits and deviations
- Candidates (a) and (c) were not built. Selection and probes cost only additions, which FPM does not price; they are small
  (`tables.md`).
- Development pilot on other curves before the freeze is disclosed in `PROTOCOL.md`; its B4 outcome was already negative.
- Three curves per prime; exponent intervals reflect that sample only. Shared 4-vCPU VM; the clippy run and the run overlapped in time (counts unaffected).
- The CLAUDE.md session log and the Conductor task were not touched: this agent was confined to the owned paths and no git commands.
