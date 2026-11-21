# Multi-target IC4 vs the cheapest of four rho arms (2026-11-11)

Copies for browsing; the study `research/mt_fieldops_v2_20261111/` is the source of truth (RESULTS.md,
PROTOCOL.md, raw rows, validator report). Claim PCC-20261111-fa13c8 [pcc:unverified].

- `REPORT.pdf`, `ratios.svg`, `ratios.png`: IC4 / reference per target, median and 95% bootstrap, per cell,
  at I = 3, 10, 30 M and h = 0, 0.1, 1.
- `VALIDATION.json`, `replay-certificate.json`, `validator-report.json`: the gate files. The report here is the
  validator's `research/mt_fieldops_v2_20261111/validation/validator-report.json` with its `raw_data` paths
  rebased to this directory (recorded in its `path_rebase` field).

## Correction (2026-11-11)

Superseded as a statement about rho. The rho arms used here restarted walks at sums of a fixed 64-point bank
(bounded distinct walks), which made the reference 1.4-4.8% too expensive; rerun with the fix,
IC4 / rho at T = 16384 is 0.95-0.99 (still below 1 against this harness's rho family). The family also lacks
Kuhn-Struik retention of solved walks; with it (independent validator, 528 runs) IC4 is 1.12-1.17x rho at h = 0 and
the decisions are 0 cheaper, 6 unranked, 12 dearer. `VALIDATION.json` now records `strongest_available: false`, so
this row no longer passes the gate. Details: [`research/mt_rho_start_fix_20261111/`](../../research/mt_rho_start_fix_20261111/),
[`benchmarks/20261111-mt-rho-start-fix/`](../20261111-mt-rho-start-fix/). [pcc:unverified]
