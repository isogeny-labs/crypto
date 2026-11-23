# 2026-11-14: S2b dense step-column block in the native hyperelliptic IC solve, genus 2–4

Copies for browsing. The source of truth is the study,
[`research/hec_ic_speedup_20261113/`](../../research/hec_ic_speedup_20261113/) (PLAN.md §8 and
`S2B-RESULTS.md`), and its run report
[`reports/run-20261114T2233Z/`](../../research/hec_ic_speedup_20261113/reports/run-20261114T2233Z/).
The record is
[`records/20261114T2233Z-hec-s2b-dense-solve.json`](../records/20261114T2233Z-hec-s2b-dense-solve.json).

- [`REPORT.pdf`](REPORT.pdf) is the run and results report.
- Graphs as SVG with 150 dpi PNG renders:
  - `ratios`: paired Fast/Dense solve ratio per (repeat, seed), with the median.
  - `shares`: the solve's share of native wall time, both solvers.
  - `scaling`: median solve time against factor-base size.

**Unvalidated.** There is no `VALIDATION.json`, no replay certificate and no matched rho reference.
This is engineering on wall clock only. The relation search is identical; the solve does more
multiplications mod ℓ, not fewer. The host was shared (load average 81–95). It is not in the
admitted table.

**External comparison: not run.** FLINT and CADO-NFS `bwc` on exported matrices are planned under
plan track T3.
