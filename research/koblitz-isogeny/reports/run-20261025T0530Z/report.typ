#set page(width: 21cm, height: auto, margin: 1.5cm)
#set text(size: 10pt)
= m = 2 random-V curve effect at n = 19 with 32 subspaces (2026-10-25)

Run directory `research/koblitz-isogeny/reports/run-20261025T0530Z/`. Full note, section
"m = 2, random V, n = 19 at 32 V": `research/notes/koblitz-isogeny/m3-presentation-vs-curve-32v-20261025.md`.

== Question and design
Does the decomposition cost depend on the curve, or only on the factor-base presentation
(the F2-subspace V), in the Gröbner regime with non-geometric V? The m = 3 arms covered
geometric V with a linear-algebra solver only. Model: two-way crossed random effects,
curves × subspaces; ICC#sub[curve] with a 2000-resample curve bootstrap (95 % CI).
Pre-registered rule, unchanged: *H0* if every metric's upper bound is < 0.05; *H1* if any
cost metric has ICC ≥ 0.10 with lower bound > 0; otherwise inconclusive. Metrics:
`work_per_call` (reductions per call), `yield_per_Fm` (yield per |F|²), `log_us_per_call`,
`log_projected_solve`.

Change against the original n = 19 arm (128 curves × 4 random V): 32 random V per curve
on the first 32 curves of the sample (seeded hash order, K_a included; operator
statement). Class n = 19, a2 = 1, l = 9; m = 2 Gröbner; 400 full-group probes per cell;
seeds 1–32. Host: shared container, no isolation level earned.

Data: `pres_19_1_l9_s1-32.jsonl`, 1408 rows = 512 original (128 curves × seeds 1–4) + 896
new (32 curves × seeds 5–32). Counted from the file: 1408 `bsgs_ok`, 0 inconsistent, none
skipped; 124920 relations; 3.372 reductions per call (sum of `reductions` over sum of
`groebner_calls`; new cells 3.373, original 3.371). The ICC keeps the 32 curves with every
seed: 1024 cells. Every recorded `first_fall_hist` entry (4412) is at degree 3.

== Result (ICC#sub[curve] [95 % CI], 32 curves × 32 V)
#table(
  columns: (auto, auto, auto),
  align: left,
  [*metric*], [*ICC#sub[curve] [95 % CI]*], [*subspace share*],
  [`work_per_call`], [0.003 [0, 0.018]], [0.002],
  [`yield_per_Fm`], [0.002 [0, 0.022]], [0.000],
  [`log_us_per_call`], [0.000 [0, 0.009]], [0.013],
  [`log_projected_solve`], [0.003 [0, 0.020]], [0.002],
)
Diagnostic: |F| 0.000 [0, 0.012]. The pins and roots diagnostics in the JSON are zero
with zero variance (m = 2 has no pin step) and are not plotted.

Verdict by the pre-registered rule: *H0* (largest upper bound 0.0223). The m = 3 arms of
`run-20261024T2200Z` had largest upper bounds 0.0205 (n = 31) and 0.0388 (n = 23).

#figure(
  image("icc_three_arms.svg", width: 100%),
  caption: [ICC#sub[curve] with 95 % CI by metric for the three 32 curves × 32 V arms:
  n = 19, m = 2 Gröbner, random V (this run); n = 31 and n = 23, m = 3 PinX3, geometric V
  (`run-20261024T2200Z`). "yield per |F|^m" is |F|² at m = 2 and |F|³ at m = 3. Sources:
  `icc_n19_m2_rand_s1-32.json`, `icc_n31_m3_geom_s1-32.json`,
  `icc_n23_m3_geom_s1-32.json`. Dashed: 0.05; dotted: 0.10. Arms differ in solver, V
  type, class and metric definition, so the points are placed side by side, not pooled.],
)

== Detection limit (simulated verdict rates)
`power` rows at 32 V; 200 simulations per point, 32 curves, fitted additive normal model
with a planted curve share. For this arm: P(H0) at true ICC 0 is 0.995–1.00 across the four
metrics; P(H1) at 0.10 is 0.42 (work per call) and 0.42–0.49 across metrics; P(H1) at 0.20
is 0.98–0.995 (work per call 0.99). A curve share of 0.20 would almost surely have been
called H1; 0.10 about half the time; a share between 0.05 and 0.10 is not excluded.
Monte Carlo standard error near 0.5 is about 0.035.

#figure(
  image("power_32v.svg", width: 100%),
  caption: [`work_per_call`: recorded P(H0) and P(H1) against the planted ICC at 32 V for
  the three arms (200 simulations per point). Same source files.],
)

== Limits
- One class (n = 19), m = 2 only, l = 9, 400 probes per cell, the 32 sampled curves.
  Other n, solvers and the first-fall quantity as a metric are not covered; the file has no
  first-fall variation (all entries at degree 3).
- H0 means no curve share detected above the stated bound. It is not a proof of curve
  independence.
- Timing is wall clock on a shared container without isolation. 128 of the 1024 analysed
  cells (seeds 1–4) are from the curve-order run whose timings an earlier note excluded as
  drift-confounded. Cell order of the new cells was not checked. Work per call and yield
  are deterministic.
- Additive model: a curve × V interaction is counted in the residual.
- Not run: n = 23, m = 2, random V at 32 V (about 900 s per cell, operator figure).
- Not a speed result; no cross-method comparison, leaderboard or `benchmarks/` entry; no
  validation-gate directory for these data.

== Records
`experiments/koblitz_presentation/`: `pres_19_1_l9_s1-32.jsonl`,
`icc_n19_m2_rand_s1-32.json`, `icc_n31_m3_geom_s1-32.json`,
`icc_n23_m3_geom_s1-32.json`. Graph source: `plot.py`.
