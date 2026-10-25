# m = 3 curve-vs-presentation with 32 subspaces per curve, n = 31 and n = 23 (2026-10-25)

This note records two arms of the pre-registered curve-vs-presentation design
(`presentation-vs-curve-design-20261003.md`, amended by
`m3-presentation-vs-curve-n31-design-20261012.md`). The model, the estimator, the
metrics and the thresholds are unchanged. The only change against the 4-V arms is the
number of factor-base subspaces V per curve: 32 instead of 4.

Run report, graphs and PDF: `research/koblitz-isogeny/reports/run-20261024T2200Z/`.

Question: within a binary Koblitz isogeny class, does the decomposition cost depend on
the curve, or only on the factor-base presentation (the F2-subspace V)?

## Design

| item | value |
|---|---|
| model | two-way crossed random effects, curves × subspaces; ICC_curve = curve variance share of cell-level variance |
| interval | 95 %, 2000-resample bootstrap over curves |
| rule | **H0** if every metric's 95 % upper bound is < 0.05; **H1** if any cost metric has ICC ≥ 0.10 with lower bound > 0; otherwise inconclusive |
| metrics | yield per \|F\|³ (`yield_per_Fm`); ln µs per call (`log_us_per_call`); work per call (`work_per_call`, S4 roots per call for PinX3); ln projected solve (`log_projected_solve`) |
| decomposition | m = 3, `M3Solver::PinX3`, geometric V |
| probes | 2000 full-group probes per cell |
| sample | 32 curves per class (seeded hash sample of the class, K_a included) × 32 geometric V, 1024 cells per class |
| analysis | `examples/koblitz_presentation_icc.rs` (the JSONs carry the `diag_*` and `power` keys, so the diagnostic and power switches were on) |

Sweep command pattern (from the n = 31 design note, seeds 1–4):
`KOBLITZ_PRES_M3=pin KOBLITZ_PRES_GEOMETRIC=1 KOBLITZ_PRES_FFD_TARGETS=0 koblitz_presentation_sweep 31 0 10 2000 32 1,2,3,4`.
The two new runs used the same pattern with `RAYON_NUM_THREADS=3` / `4` and a
checkpoint directory, resumed across container restarts:
`… koblitz_presentation_sweep 31 0 10 2000 32 9,10,…,32` and
`… koblitz_presentation_sweep 23 1 8 2000 32 1,2,…,32`. The seed sets are also in the
records (`basis_seed`).

### Arms

| arm | class | curves in class | cofactor | l | data (cells) |
|---|---|---|---|---|---|
| A | n = 31, a2 = 0 | 7193 | 1492 | 10 | `pres_31_0_l10_geom_m3pin.jsonl` (seeds 1–4, 128), `pres_31_0_l10_geom_m3pin_rep_s5-8.jsonl` (5–8, 128), `pres_31_0_l10_geom_m3pin_s9-32.jsonl` (9–32, 768) |
| B | n = 23, a2 = 1 | 967 | 2 | 8 | `pres_23_1_l8_geom_m3pin_s1-32.jsonl` (seeds 1–32, 1024) |

All files are in `experiments/koblitz_presentation/`. The class sizes are as given by
the run's operator; they are not recomputed from the files.

Record checks (counted from the JSONL files for this note):

| arm | cells | `bsgs_ok` | `inconsistent` | relations | relations per cell, median (min–max) |
|---|---|---|---|---|---|
| A, seeds 9–32 | 768 | 768 | 0 | 122836 | 159 (109–221) |
| A, seeds 1–8 | 256 | 256 | 0 | 41172 | 160 (115–228) |
| B | 1024 | 1024 | 0 | 583048 | 566 (312–882) |

Factor-base size \|F\| per cell: A 464–562 (median 512 over seeds 1–8 and 511.5 over
9–32), B 104–155 (median 129). B cells are full solves of about 569 relations on
average (583048 / 1024).

## Results

Pooled ICCs: `icc_n31_m3_geom_s1-32.json` (key `n=31 a2=0 l=10 m=3`) and
`icc_n23_m3_geom_s1-32.json` (key `n=23 a2=1 l=8 m=3`). 32 curves, 32 subspaces each.

### Pre-registered metrics

| metric | A: n = 31 ICC_curve [95 % CI] | A: subspace share | B: n = 23 ICC_curve [95 % CI] | B: subspace share |
|---|---|---|---|---|
| work per call | 0.000 [0, 0.001] | 0.000 | 0.005 [0, 0.022] | 0.000 |
| yield per \|F\|³ | 0.006 [0, 0.020] | 0.010 | 0.015 [0, 0.039] | 0.000 |
| ln µs per call | 0.004 [0, 0.014] | 0.420 | 0.000 [0, 0.007] | 0.033 |
| ln projected solve | 0.002 [0, 0.010] | 0.382 | 0.000 [0, 0.000] | 0.027 |

CI limits are printed to the precision in the JSON files (A work per call:
[0, 0.0008]; B projected solve: [0, 0.0000]). Every lower bound is 0.

**Verdict by the pre-registered rule: H0 on both arms.** The largest upper bound is
0.0205 (A, yield) and 0.0388 (B, yield), each below 0.05. No lower bound is above 0, so
H1 cannot fire.

### Diagnostics (not pre-registered)

| diagnostic | A: ICC_curve [95 % CI] | B: ICC_curve [95 % CI] |
|---|---|---|
| factor-base size \|F\| | 0.000 [0, 0.000] | 0.000 [0, 0.015] |
| PinX3 pins per call | 0.000 [0, 0.001] | 0.003 [0, 0.020] |
| roots per pin | 0.001 [0, 0.014] | 0.019 [0, 0.040] |
| work per call ÷ \|F\| | 0.000 [0, 0.001] | 0.000 [0, 0.010] |

### Number of subspaces, arm A (same 32 curves)

| V per curve | seeds | work per call | yield per \|F\|³ | ln µs per call | ln projected solve |
|---|---|---|---|---|---|
| 4 | 1–4 | 0.228 [0.032, 0.403] | 0.000 [0, 0.116] | 0.048 [0, 0.216] | 0.067 [0, 0.219] |
| 8 | 1–8 | 0.005 [0, 0.064] | 0.000 [0, 0.030] | 0.019 [0, 0.058] | 0.014 [0, 0.044] |
| 32 | 1–32 | 0.000 [0, 0.001] | 0.006 [0, 0.020] | 0.004 [0, 0.014] | 0.002 [0, 0.010] |

The 4-V and 8-V rows are from `icc_n31_m3_geom_diag.json` and
`icc_n31_m3_geom_s1-8_power.json`. The 4-V work-per-call H1 was withdrawn after the
seeds 5–8 replication (n31 design note, "Follow-up (2026-10-18)"). It does not appear at
8 or 32 V.

### Subspace share of the time metrics

At A, 0.38–0.42 of the variance in the two time metrics is attributed to the
subspace. The three seed batches (1–4, 5–8, 9–32) ran at different times on a shared
4-vCPU container, and batch is confounded with subspace. This is read as a load
artefact, not as a presentation effect on cost. It is a reading, not a tested claim.
The subspace share of work per call and of yield is ≤ 0.01. At B the subspace share is
≤ 0.033 on all four metrics. No isolation level was earned for the timing metrics.

## Detection limit

Simulated verdict rates, from the 32 V rows under `power` in both JSONs (200
simulations per point, 32 curves, the fitted additive normal model with the observed
residual and subspace variances and a planted curve share):

| planted true ICC_curve | P(H0), range over the 4 metrics | P(H1), work per call | P(H1), range over the 4 metrics |
|---|---|---|---|
| 0 | 0.995–1.00 (A and B) | 0.00 | 0.00 |
| 0.05 | 0.08–0.13 | 0.015 | 0.005–0.025 |
| 0.10 | 0.00 | 0.42 (A and B) | 0.42–0.495 (A), 0.42–0.49 (B) |
| 0.20 | 0.00 | 0.99 (A and B) | 0.98–1.00 (A), 0.98–0.995 (B) |

Detection limit: with this design (32 curves × 32 V) a curve share of 0.20 would have
been called H1 in at least 98 % of simulations on every metric. A curve share of 0.10
would have been called H1 in 42–50 % of simulations, which is a coin flip because the
rule's point threshold sits at 0.10. The design therefore excludes a curve share of
about 0.20 and above at the tested cost level. It does not exclude a share between
0.05 and 0.10. With 200 simulations per point the Monte Carlo standard error of a rate
near 0.5 is about 0.035 (derived: sqrt(0.25 / 200)).

For comparison, the 4-V rows of the same file give P(H0) at ICC 0 of 0.22–0.325 across
the four metrics (n = 31 file), which is the built-in limit of the 4-V arms
recorded in the n31 design note ("Power of the design").

## The n = 31, a2 = 1 class could not be walked

A second n = 31 class (a2 = 1, small cofactor) was meant to serve as a small-cofactor
control. Its only isogeny degree is l = 7193, which splits in a large extension. The
walk (`koblitz_isogeny_class_walk 31:1`, classify limit 257) reached 2240 of 7194
kernels at the first vertex after 7232 s, about 3.2 s per kernel, and was stopped; its
log is `experiments/koblitz_presentation/logs/walk_31_1_abandoned.log`. The a2 = 0 class
at the same n walked in 939 s. Arm B (n = 23, a2 = 1, l = 8, cofactor 2) replaces it as
the second class with a small cofactor.

## Scope and caveats

- **Solver and V:** m = 3 `PinX3` only, geometric V only. Non-geometric V at m = 3 is
  not tested.
- **Classes:** two (n = 31 a2 = 0 with cofactor 1492, and n = 23 a2 = 1 with cofactor
  2), and 32 curves from each. The sample is a seeded hash sample of each class, so
  each estimate is for that sample's curves.
- **H0 reading:** no curve share of cell-level variance was detected above the stated
  bounds on four metrics. It is not a proof that the cost is curve-independent, and it
  does not cover other solvers, other n, other metrics, or the first-fall quantity
  (not measured).
- **Model:** the ICC and the power rows assume the additive model. A non-additive
  curve × V interaction is counted in the residual. The power simulation is parametric
  (normal draws).
- **Timing:** wall-clock on a shared 4-vCPU container, no isolation. Time metrics carry
  a batch/load component (see above). Cell order for the seeds 9–32 and n = 23 runs
  was not checked.
- **Depth groups:** one curve (K_a) sits at depth 0 in both samples and the other 31 at
  depth 1. Mean \|F\| is 511.69 (K_a) against 512.04 (others) in A, and 125.75 against
  128.57 in B. Those are single-curve group means, recorded in `diag_F_by_group`, with
  no inference drawn here. The pre-registered ICC of \|F\| is 0.000 in both arms.
- **Cost accounting:** these arms compare decomposition cost across curves and V
  inside one method. No cross-method (rho) comparison is made, so the §5a pricing band
  does not apply, and no row is proposed for the leaderboard or `benchmarks/`.
- **Validation gate:** no `VALIDATION.json` exists for these data.

## What this closes, what remains

Closed, within the scope above: at m = 3 PinX3 with geometric V, the 4-V inconclusive
results at n = 19 and n = 31 are followed by H0 at n = 31 (a2 = 0, cofactor 1492) and
n = 23 (a2 = 1, cofactor 2), at a detection limit of about ICC 0.20. The earlier n = 31
factor-base-size reading and its work-per-call H1 were not reproduced at 8 or 32 V.

Remains:
- Non-geometric V at m = 3 is out of reach with the current solver.
- A curve share between 0.05 and 0.10 is not excluded.
- The m = 2 random-V 32-V arm at n = 19 is running and will be reported separately.
- A small-cofactor n = 31 class is not available (walk dead end above).
- No validation-gate evidence directory exists for these arms.

## Records

- Data and ICC JSONs: `experiments/koblitz_presentation/` (files named above).
- Run report: `research/koblitz-isogeny/reports/run-20261024T2200Z/`
  (`README.md`, `report.typ`, `report.pdf`, `plot.py`, `icc_vs_subspaces.svg`,
  `diagnostics.svg`, `power.svg`).

## m = 2, random V, n = 19 at 32 V (2026-10-25)

This section is additive; the text above is unchanged except that this arm, listed above
as running, is now reported here. It extends the original m = 2 random-V arm at n = 19
(`presentation-vs-curve-design-20261003.md`, `presentation-vs-curve-results-20261004.md`)
from 4 to 32 random V on the first 32 curves of its sample. The model, the estimator,
the metrics and the thresholds are unchanged.

Question: does the curve effect found absent for geometric V at m = 3 (PinX3, linear
algebra) also stay absent in the Gröbner regime with non-geometric V, where the
decomposition algebra (first-fall degree, reduction count) could differ by curve?

Run report, graph and PDF: `research/koblitz-isogeny/reports/run-20261025T0530Z/`.

### Design delta against the original n = 19 arm

| item | original arm (results note) | this extension |
|---|---|---|
| class | n = 19, a2 = 1, l = 9 | same |
| solver | m = 2 Gröbner (S3) | same |
| V | random (non-geometric) F2-subspaces, `basis_seed` | same, seeds 1–32 |
| probes | 400 full-group probes per cell | same |
| curves | 128 | the first 32 of the sample (the sweep's seeded hash order, K_a included; as stated by the operator) |
| V per curve | 4 (seeds 1–4) | 32 (seeds 1–32) |
| cells | 512 | 32 × 32 = 1024 (of 1408 rows in the file) |
| metrics, 95 % bootstrap over curves, rule | as in the design note | unchanged |

Data: `experiments/koblitz_presentation/pres_19_1_l9_s1-32.jsonl`, 1408 rows: the original
512 rows (128 curves × seeds 1–4) and 896 new cells (32 curves × seeds 5–32). The ICC
tool keeps only curves that have every seed, so the analysis in
`icc_n19_m2_rand_s1-32.json` (key `n=19 a2=1 l=9 m=2`) uses exactly the 32 curves × 32 V
= 1024 cells. The other 96 curves' 384 rows are not in the ICC.

Record checks (counted from the JSONL for this note):

| block | cells | `bsgs_ok` | `inconsistent` | skipped | relations | reductions per call (derived: sum of `reductions` / sum of `groebner_calls`) |
|---|---|---|---|---|---|---|
| original, seeds 1–4, 128 curves | 512 | 512 | 0 | none | 45430 | 3.371 |
| new, seeds 5–32, 32 curves | 896 | 896 | 0 | none | 79490 | 3.373 |
| file | 1408 | 1408 | 0 | none | 124920 | 3.372 |

Per cell: factor-base size \|F\| 223–293 (median 257), relations 56–126 (median 88),
reductions per call 3.165–3.625 (derived per cell). Every cell has 400 probes, l = 9 and
n = 19. Every entry in the recorded `first_fall_hist` (4412 entries over the file) is at
degree 3 and every `d_star_hist` entry is at 4. The 32 sampled curves hold 31 at depth 1
and one (K_a) at depth 0.

### Result

ICC_curve and its 95 % interval, 32 curves × 32 V (`icc_n19_m2_rand_s1-32.json`). For
comparison, the 128-curve × 4-V values from the results note, where listed there:

| metric | 32 × 32: ICC_curve [95 % CI] | 32 × 32: subspace share | 128 × 4 (results note) |
|---|---|---|---|
| work per call (reductions per call) | 0.003 [0, 0.018] | 0.002 | 0.000 [0, 0.039] |
| yield per \|F\|² | 0.002 [0, 0.022] | 0.000 | 0.000 [0, 0.045] |
| ln µs per call | 0.000 [0, 0.009] | 0.013 | not comparable (curve-order timing, excluded in the results note) |
| ln projected solve | 0.003 [0, 0.020] | 0.002 | not comparable (same reason) |

Unrounded: work per call 0.00251 [0, 0.01795]; yield 0.00241 [0, 0.02231]; ln µs per
call 0 [0, 0.00912]; ln projected solve 0.00259 [0, 0.01991]. Every lower bound is 0.
Diagnostic (not pre-registered): \|F\| 0.000 [0, 0.012], subspace share 0.000. The
`diag_pins_per_call`, `diag_roots_per_pin` and `diag_work_per_call_per_F` entries in the
JSON are all zero with zero recorded variances (the m = 2 solver has no pin step); they
carry no information and are not plotted.

**Verdict by the pre-registered rule: H0.** The largest upper bound is 0.0223 (yield),
below 0.05, and no lower bound is above 0, so H1 cannot fire.

Next to the other two 32 × 32 arms (m = 3 PinX3, geometric V; tables above), the upper
bounds are 0.0205 (n = 31), 0.0388 (n = 23) and 0.0223 (this arm).

### Detection limit

`power` rows at 32 V in the same JSON (200 simulations per point, 32 curves, fitted
additive normal model with a planted curve share):

| planted true ICC_curve | P(H0), range over the 4 metrics | P(H1), work per call | P(H1), range over the 4 metrics |
|---|---|---|---|
| 0 | 0.995–1.00 | 0.00 | 0.00 |
| 0.05 | 0.08–0.11 | 0.015 | 0.01–0.025 |
| 0.10 | 0.00 | 0.42 | 0.42–0.49 |
| 0.20 | 0.00 | 0.99 | 0.98–0.995 |

At the same 32 curves and 4 V, the same file gives P(H0) at ICC 0 of 0.25 for work per
call. As in the m = 3 arms, a curve share of about 0.20 would have been called H1 in at
least 98 % of simulations, a share of 0.10 in 42–49 % (a coin flip at the rule's
threshold; Monte Carlo standard error about 0.035), and a share between 0.05 and 0.10 is
not excluded.

### Scope and caveats

- **One class:** n = 19, a2 = 1, l = 9, cofactor 2, m = 2 Gröbner, 400 probes per cell,
  32 curves of the sample. Other n, other l, other solvers and the first-fall quantity as
  a separate metric are not covered. The recorded first-fall entries are all at degree 3,
  so the file contains no variation in that quantity to attribute to curves.
- **Not the same sample as before:** the 4-V n = 19 result of the results note
  (128 curves) had upper bounds of 0.039 (reductions) and 0.045 (yield), and the note
  did not establish H0 by the strict rule (the n = 19 timing metrics were excluded
  as confounded there). This extension gives bounds of 0.018 and 0.022 on 32 of those curves. It does not redo
  the 128-curve estimate.
- **H0 reading:** no curve share was detected above the stated bounds on four metrics.
  It is not a proof of curve independence.
- **Timing:** wall clock on a shared container, no isolation. The 512 original cells
  (seeds 1–4) are the curve-order run whose timings the results note excluded as
  confounded by machine drift; 128 of the 1024 analysed cells (4 seeds × 32 curves) come
  from that run. Cell order of the 896 new cells was not checked beyond noting that the
  first rows interleave curves and seeds. The time metrics are therefore weaker evidence
  than work per call and yield, which are deterministic.
- **Model:** additive; a curve × V interaction is counted in the residual.
- **Cost accounting and gates:** the arm compares decomposition cost across curves and V
  inside one method. No cross-method (rho) comparison is made, so the §5a band does not
  apply, no leaderboard or `benchmarks/` row is proposed, and no `VALIDATION.json`
  exists for these data.

### What remains

- n = 23, m = 2, random V at 32 V was not run: a cell costs about 900 s (operator
  figure), so 1024 cells would be about 256 core-hours (derived: 1024 × 900 s / 3600). The
  results note's n = 23 arm (32 or 128 curves × 4 V) remains inconclusive by the strict
  rule (yield upper bound 0.162 at 128 curves).
- The 4-V arms at n = 16, 17 (full classes) are unchanged.
- m = 3 with non-geometric V remains out of reach with the current solver.

### Records for this section

- Data: `experiments/koblitz_presentation/pres_19_1_l9_s1-32.jsonl`.
- ICC: `experiments/koblitz_presentation/icc_n19_m2_rand_s1-32.json`.
- Compared arms: `icc_n31_m3_geom_s1-32.json`, `icc_n23_m3_geom_s1-32.json`.
- Run report: `research/koblitz-isogeny/reports/run-20261025T0530Z/`.
