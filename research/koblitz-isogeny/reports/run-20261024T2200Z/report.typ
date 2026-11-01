#set page(width: 21cm, height: auto, margin: 1.5cm)
#set text(size: 10pt)
= m = 3 curve effect with 32 subspaces: n = 31 and n = 23 (2026-10-25)

Run directory `research/koblitz-isogeny/reports/run-20261024T2200Z/`. Full note:
`research/notes/koblitz-isogeny/m3-presentation-vs-curve-32v-20261025.md`.

== Question and design
Within a binary Koblitz isogeny class, does the index-calculus decomposition cost depend
on the curve, or only on the factor-base presentation (the F2-subspace V)? Two-way
crossed random effects, curves × subspaces; ICC#sub[curve] with a 2000-resample curve
bootstrap (95 % CI). Pre-registered rule: *H0* if every metric's upper bound is < 0.05;
*H1* if any cost metric has ICC ≥ 0.10 with lower bound > 0; otherwise inconclusive.
Metrics: `yield_per_Fm`, `log_us_per_call`, `work_per_call` (S4 roots per call),
`log_projected_solve`.

The only change against the 4-V arms is the number of V per curve (32).
Solver `PinX3`, m = 3, geometric V, 2000 full-group probes per cell, 32 curves (seeded
hash sample, K_a included) × 32 V = 1024 cells per class. Host: shared 4-vCPU
container, no isolation level earned.

#table(
  columns: (auto, auto, auto, auto, auto),
  align: left,
  [*arm*], [*class*], [*cells*], [*`bsgs_ok` / inconsistent*], [*relations*],
  [A], [n = 31, a2 = 0, l = 10 (7193 curves, cofactor 1492); seeds 1–4, 5–8, 9–32], [1024], [1024 / 0], [164008 (122836 in seeds 9–32)],
  [B], [n = 23, a2 = 1, l = 8 (967 curves, cofactor 2); seeds 1–32], [1024], [1024 / 0], [583048 (mean 569 per cell)],
)

Counts are from the JSONL records; class sizes are as stated by the operator.

== Results (ICC#sub[curve] [95 % CI], 32 curves × 32 V)
#table(
  columns: (auto, auto, auto),
  align: left,
  [*metric*], [*A: n = 31*], [*B: n = 23*],
  [`work_per_call`], [0.000 [0, 0.001]], [0.005 [0, 0.022]],
  [`yield_per_Fm`], [0.006 [0, 0.020]], [0.015 [0, 0.039]],
  [`log_us_per_call`], [0.004 [0, 0.014]], [0.000 [0, 0.007]],
  [`log_projected_solve`], [0.002 [0, 0.010]], [0.000 [0, 0.000]],
)
Verdict by the pre-registered rule: *H0* on both arms (largest upper bound 0.0205 for
A and 0.0388 for B).

Diagnostics (not pre-registered): \|F\| 0.000 [0, 0.000] (A) and 0.000 [0, 0.015] (B);
pins per call 0.000 [0, 0.001] and 0.003 [0, 0.020]; roots per pin 0.001 [0, 0.014]
and 0.019 [0, 0.040].

Subspace share of variance: A, time metrics 0.42 (µs per call) and 0.38 (projected
solve), work and yield ≤ 0.01; B, ≤ 0.033 on all four. The A time shares are confounded
with run batch: the three seed batches ran at different times on a shared container.

#figure(
  image("icc_vs_subspaces.svg", width: 100%),
  caption: [ICC#sub[curve] with 95 % CI by metric. Left: n = 31 at 4 V (seeds 1–4),
  8 V (seeds 1–8) and 32 V (seeds 1–32). Right: n = 23 at 32 V. Sources:
  `icc_n31_m3_geom_diag.json`, `icc_n31_m3_geom_s1-8_power.json`,
  `icc_n31_m3_geom_s1-32.json`, `icc_n23_m3_geom_s1-32.json`. Dashed: 0.05; dotted: 0.10.],
)

#figure(
  image("diagnostics.svg", width: 100%),
  caption: [32-V diagnostics (factor-base size, pins per call, roots per pin), same files.],
)

== Detection limit (simulated verdict rates)
Rows `power` of the two 32-V JSONs; 200 simulations per point, 32 curves, fitted additive
normal model with a planted curve share. At 32 V: P(H0) at true ICC 0 is 0.995–1.00
across metrics and arms; P(H1) at true ICC 0.10 is 0.42–0.495 (work per call 0.42);
P(H1) at true ICC 0.20 is 0.98–1.00 (work per call 0.99). A curve share of 0.20 would
have been called H1 almost surely; 0.10 about half the time. At 4 V the same file gives
P(H0) at ICC 0 of 0.22–0.325 (n = 31), the built-in limit of the 4-V arms.

#figure(
  image("power.svg", width: 85%),
  caption: [`work_per_call`: recorded P(H0) and P(H1) against the planted ICC, for 4, 8
  and 32 V. Top: n = 31; bottom: n = 23. 200 simulations per point.],
)

== Limits
- Geometric V and m = 3 PinX3 only; two classes; the sampled curves only.
- H0 means no detected curve share of cell-level variance above the bound on these four
  metrics. It is not a proof of curve independence. A share between 0.05 and 0.10 is
  not excluded.
- Additive model: a non-additive curve × V interaction is counted in the residual.
- Timing is wall clock on a shared container without isolation.
- The n = 31, a2 = 1 class could not be walked (its only isogeny degree 7193 splits in a
  large extension; operator report of 3.3 s per kernel and more than 6 h without the
  first vertex; no record file). Non-geometric V at m = 3 is out of reach. The m = 2
  random-V 32-V arm at n = 19 is running and reported separately.
- Not a speed result and no cross-method comparison: no leaderboard or `benchmarks/`
  entry, and no validation-gate directory for these data.

== Records
`experiments/koblitz_presentation/`: `pres_31_0_l10_geom_m3pin.jsonl`,
`pres_31_0_l10_geom_m3pin_rep_s5-8.jsonl`, `pres_31_0_l10_geom_m3pin_s9-32.jsonl`,
`pres_23_1_l8_geom_m3pin_s1-32.jsonl`, `icc_n31_m3_geom_s1-32.json`,
`icc_n23_m3_geom_s1-32.json`, `icc_n31_m3_geom_diag.json`,
`icc_n31_m3_geom_s1-8_power.json`. Graph source: `plot.py`.
