# 2026-11-15: S4 full-base line sieve against the walk at genus 3 and 4, counted

These are copies for browsing. The source of truth is the study,
[`research/hec_ic_speedup_20261113/`](../../research/hec_ic_speedup_20261113/) (PLAN.md §10,
`S4-RESULTS.md`), and its run report
[`reports/run-20261115T0626Z/`](../../research/hec_ic_speedup_20261113/reports/run-20261115T0626Z/).
The record is
[`records/20261115T0626Z-hec-s4-line-sieve.json`](../records/20261115T0626Z-hec-s4-line-sieve.json).

- [`REPORT.pdf`](REPORT.pdf) is the run and results report.
- The graphs are SVG with 150 dpi PNG renders:
  - `relcost`: field products per relation, walk against sieve;
  - `ratios`: P1, P2 and P3 per row with the pre-registered bars.

**What the run settles.** Every pre-registered expectation (E12–E16) holds. In counted field
products the full-base line sieve's relations are 5.8–16.9× cheaper than the walk's at genus 3 and
8.2–16.1× at genus 4 (p ≥ 127); end to end it wins 5.5–10.9× at m ≤ 506 and loses (0.79) at
(3, 4001), where its solve fills in.

**Unvalidated.** There is no `VALIDATION.json`. This is engineering, counted on a shared host, so it
is not in the admitted table.

**External comparison: not run.** No open hyperelliptic-IC implementation is known.
