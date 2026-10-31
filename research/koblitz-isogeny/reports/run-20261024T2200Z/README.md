# Run 2026-10-24/25: m = 3 curve effect with 32 subspaces, n = 31 and n = 23

**Result (pre-registered rule): H0 on both arms.** All ICC_curve 95 % upper bounds are
below 0.05 on the four metrics (largest: 0.0205 at n = 31, yield; 0.0388 at n = 23,
yield). Detection limit: a planted curve share of 0.20 is called H1 in 98-100 % of
simulations, 0.10 in 42-50 %.

| metric | n = 31, a2 = 0, l = 10 | n = 23, a2 = 1, l = 8 |
|---|---|---|
| work per call | 0.000 [0, 0.001] | 0.005 [0, 0.022] |
| yield per \|F\|^3 | 0.006 [0, 0.020] | 0.015 [0, 0.039] |
| ln us per call | 0.004 [0, 0.014] | 0.000 [0, 0.007] |
| ln projected solve | 0.002 [0, 0.010] | 0.000 [0, 0.000] |

Setup: PinX3 m = 3 solver, geometric V, 2000 full-group probes per cell, 32 curves
(seeded hash sample, K_a included) x 32 V = 1024 cells per class. n = 31: seeds 1-4, 5-8,
9-32 (`pres_31_0_l10_geom_m3pin.jsonl`, `..._rep_s5-8.jsonl`, `..._s9-32.jsonl`);
n = 23: seeds 1-32 (`pres_23_1_l8_geom_m3pin_s1-32.jsonl`). All 2048 cells `bsgs_ok`,
0 inconsistent relations. Host: shared 4-vCPU container, no isolation level earned.

Scope: geometric V only, m = 3 PinX3 only, two classes, the sampled curves. H0 means no
detected curve share of cell-level variance above the bound, not a proof. The n = 31
subspace share of the two time metrics (0.38-0.42) is confounded with run batch (load).
The n = 31, a2 = 1 class could not be walked (operator report; no record file).

Full write-up: `research/notes/koblitz-isogeny/m3-presentation-vs-curve-32v-20261025.md`.
Earlier context: `run-20261011T2357Z` (4 V, H1 on work per call), `run-20261018T1200Z`
(did not replicate).

Data (`experiments/koblitz_presentation/`):
- `icc_n31_m3_geom_s1-32.json` (key `n=31 a2=0 l=10 m=3`, includes `power`);
- `icc_n23_m3_geom_s1-32.json` (key `n=23 a2=1 l=8 m=3`, includes `power`);
- `icc_n31_m3_geom_diag.json` (4 V), `icc_n31_m3_geom_s1-8_power.json` (8 V).

Graphs (from `plot.py`, recorded values only): `icc_vs_subspaces.svg`,
`diagnostics.svg`, `power.svg`. PDF: `report.pdf`.
Not a speed result: no `benchmarks/` or leaderboard entry.
