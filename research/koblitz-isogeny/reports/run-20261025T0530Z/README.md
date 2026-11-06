# Run 2026-10-25: m = 2 random-V curve effect at n = 19 with 32 V

**Result (pre-registered rule): H0.** All four ICC_curve 95 % upper bounds are below 0.05
(largest 0.0223, yield). Detection limit at 32 curves x 32 V: a planted curve share of
0.20 is called H1 in 98-99.5 % of simulations, 0.10 in 42-49 %.

| metric | n = 19, a2 = 1, l = 9, m = 2, random V: ICC_curve [95 % CI] | subspace share |
|---|---|---|
| work per call (reductions per call) | 0.003 [0, 0.018] | 0.002 |
| yield per \|F\|^2 | 0.002 [0, 0.022] | 0.000 |
| ln us per call | 0.000 [0, 0.009] | 0.013 |
| ln projected solve | 0.003 [0, 0.020] | 0.002 |

Diagnostic: \|F\| 0.000 [0, 0.012].

Setup: the original n = 19 arm (m = 2 Groebner, random non-geometric V, 400 full-group
probes per cell, l = 9) extended from 4 to 32 V on the first 32 curves of its sample
(sweep's seeded hash order, K_a included; operator statement). Seeds 1-32.
`pres_19_1_l9_s1-32.jsonl` has 1408 rows: the original 512 (128 curves x seeds 1-4) and 896
new cells (32 curves x seeds 5-32). All 1408 cells `bsgs_ok`, 0 inconsistent, none
skipped; 3.372 reductions per call over the file (derived from the JSONL). The ICC uses
the 32 curves that have every seed: 32 x 32 = 1024 cells. Host: shared container, no
isolation level earned.

Scope: one class (n = 19), m = 2 only, l = 9, 400 probes per cell, the sampled curves.
Every recorded first-fall entry is at degree 3, so there is no first-fall variation in the
file. The 128 seed-1-4 cells of the analysed 1024 come from the curve-order run whose
timings an earlier note excluded as drift-confounded; the work and yield metrics are
deterministic. H0 means no curve share detected above the bound, not a proof; a share
between 0.05 and 0.10 is not excluded.

Not run: n = 23, m = 2, random V at 32 V (about 900 s per cell, operator figure).

Comparison on the same graph: `icc_n31_m3_geom_s1-32.json` and `icc_n23_m3_geom_s1-32.json`
(m = 3 PinX3, geometric V; run `run-20261024T2200Z`).

Full write-up: `research/notes/koblitz-isogeny/m3-presentation-vs-curve-32v-20261025.md`,
section "m = 2, random V, n = 19 at 32 V (2026-10-25)".

Data (`experiments/koblitz_presentation/`): `pres_19_1_l9_s1-32.jsonl`,
`icc_n19_m2_rand_s1-32.json` (key `n=19 a2=1 l=9 m=2`, includes `power`).

Graphs (from `plot.py`, recorded values only): `icc_three_arms.svg`, `power_32v.svg`.
PDF: `report.pdf`. Not a speed result: no `benchmarks/` or leaderboard entry, no
cross-method comparison, no validation-gate directory.
