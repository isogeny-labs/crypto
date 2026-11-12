# M4GB end to end: the whole index-calculus pipeline on a binary-curve ladder (2026-11-10)

Copies for browsing of
[`research/m4gb_e2e_ic_ladder_20261110/`](../../research/m4gb_e2e_ic_ladder_20261110/README.md)
(source of truth: protocol, per-instance `ic bench --sweep` JSON, scripts).
Benchmark record `benchmarks/records/20261110T1937Z-m4gb-e2e-ic-ladder.json`.

- `REPORT.pdf`: protocol, results and the full table.
- `results.svg` / `results.png`: S per configuration and instance (markers)
  against the matched counted rho (grey bars), log scale.

Headline `[pcc:unverified]`: 24 end-to-end configurations on six binary
curves (Koblitz 13, 17, 19, 23; random 15, 21), every one recovering its
planted logarithm; every index-calculus row 45 to 2,500 times (or more)
above the matched rho; `inherited-f4` the best row on the Koblitz curves and
the pair table on the random ones; `m4gb-f2` never best, faster per call than
the in-house full-basis F4 from about 22 unknowns (3.7× at 24) and 15 to 25×
slower than `inherited-f4` everywhere. Local container, one thread; a stage
comparison of one oracle, not a leaderboard row.
