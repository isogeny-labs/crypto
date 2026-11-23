# 2026-11-14: S3 exact inversion hygiene in the native hyperelliptic IC relation phase, genus 2–4

These are copies for browsing. The source of truth is the study,
[`research/hec_ic_speedup_20261113/`](../../research/hec_ic_speedup_20261113/) (PLAN.md §9,
`S3-RESULTS.md`, `LEADS-20261114.md`), and its run report
[`reports/run-20261115T0426Z/`](../../research/hec_ic_speedup_20261113/reports/run-20261115T0426Z/).
The record is
[`records/20261115T0426Z-hec-s3-inversion-hygiene.json`](../records/20261115T0426Z-hec-s3-inversion-hygiene.json).

- [`REPORT.pdf`](REPORT.pdf) is the run and results report.
- The graphs are SVG with 150 dpi PNG renders:
  - `inversions`: field inversions per walk step and per oracle call, old against new.
  - `ratios`: the paired per-trial ratio and the A/A floor, both sets.

**What the run settles.**

- The outputs are identical.
- Inversions fall, by count.
- E9 is falsified on two genus-4 rows.

**What it leaves open.** The timing verdicts E10 and E11 are *not determined*. The A/A floor failed
in both sets, at load average 50–401.

**Unvalidated.** There is no `VALIDATION.json`. This is engineering on a shared host, so it is not in
the admitted table.

**External comparison: not run.** No open hyperelliptic-IC implementation is known.
