# Run 2026-10-24: n = 31, m = 3 curve effect with 32 subspaces (in progress)

**Result:** TODO-RESULTS (the 32-V experiment is still running; fill in once
`icc_n31_m3_geom_s1-32.json` exists, quoting recorded ICC and CI values only).

Context (recorded, from earlier runs):
- 4 V (seeds 1-4): work_per_call ICC 0.228 [0.032, 0.403] (`run-20261011T2357Z`).
- 4 V replication seeds 5-8: not replicated (`run-20261018T1200Z`).
- 8 V (seeds 1-8) and the simulated power table:
  `experiments/koblitz_presentation/icc_n31_m3_geom_s1-8_power.json`.

Sections to complete: spec/command and commit, host and isolation, arm and seeds,
runs requested/completed with failure reasons, ICC table per metric (32 V),
power read-off for 32 V, verdict against the pre-registered 0.05 / 0.10 bounds.

Data:
- `experiments/koblitz_presentation/icc_n31_m3_geom_diag.json` (4 V);
- `experiments/koblitz_presentation/icc_n31_m3_geom_s1-8_power.json` (8 V + power);
- `experiments/koblitz_presentation/icc_n31_m3_geom_s1-32.json` (32 V; TODO-RESULTS, not yet present).

Graphs: `icc_vs_subspaces.svg`, `power.svg` (from `plot.py`, recorded values only).
PDF: `report.pdf` (rebuild after the 32-V file lands; the current one is a scaffold).
