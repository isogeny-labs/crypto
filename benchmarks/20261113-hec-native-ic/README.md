# 2026-11-13: native hyperelliptic IC engine and solver, genus 2–4

Copies for browsing. The source of truth is the study,
[`research/hec_ic_speedup_20261113/`](../../research/hec_ic_speedup_20261113/), and its run report
[`reports/run-20261114T0356Z/`](../../research/hec_ic_speedup_20261113/reports/run-20261114T0356Z/).
The record is [`records/20261114T0356Z-hec-native-ic.json`](../records/20261114T0356Z-hec-native-ic.json).

- [`REPORT.pdf`](REPORT.pdf) is the run and results report.
- Graphs as SVG with 150 dpi PNG renders:
  - `solve_times`: the native solve, S1 solver vs S2a solver.
  - `end_to_end`: BigUint vs native engine.
  - `phases`: where native time goes after S2a.

**Unvalidated.** There is no `VALIDATION.json`, no replay certificate and no matched rho reference.
This is engineering on wall clock only, with the counted work identical, so it is not in the
admitted table.

**External comparison: not run.** No open hyperelliptic-IC implementation is known. The GTTD and
Nagao 2LP implementations are published, not reproduced.
