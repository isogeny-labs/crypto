# Rho restart fix and the v2 multi-target IC4 result (2026-11-11)

Study: [`research/mt_rho_start_fix_20261111/`](../../research/mt_rho_start_fix_20261111/). The rho arms of
`examples/mt_fieldops.rs` restarted at sums of a fixed bank (bounded distinct walks); fixed with chained starts.
Rerunning all 1584 v2 rho jobs makes the cheapest rho 1.4-4.8% cheaper and moves v2 IC4 / rho at T = 16384 from
0.91-0.95 to 0.95-0.99 (still below 1 against this harness's family). Against the same family with Kuhn-Struik
retention (independent validator), IC4 is dearer (1.12-1.17 at h = 0). Gate: does not pass (validator status
`insufficient_evidence`, `strongest_available: false`). Figures `ratios.svg`/`ratios.png`, report `REPORT.pdf`.
[pcc:unverified]
