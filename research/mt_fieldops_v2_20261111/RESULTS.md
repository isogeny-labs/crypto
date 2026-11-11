# IC4 vs the strongest rho on fresh curves (v2, 2026-11-11)

Protocol `PROTOCOL.md` frozen in commit 6b0ac6f192 (2026-11-11T01:38:02Z) before any v2 data; IC parameters
`frozen_ic.json` (mu = 1.0, g = 2.0) carried over unchanged from the v1 tune set. 33 never-used curves
(`instances/`, 0 overlap with the 1031 curves in `research/`), seeds 11 and 12, 2112 runs, 0 errors, every target
of every run recovered and checked by counted scalar multiplication [pcc:unverified]. Bench record
`benchmarks/records/20261111T0142Z-mt-fieldops-ic4-v2.json`. Tables `results/holdout/analysis.txt`, summary
`results/holdout/summary.json`, figure `ratios.svg`/`ratios.png`, report `REPORT.pdf`.

"Strongest rho" = per configuration and per (I/M, h), the cheapest of 12 rho runs: `rho`, `rho2`, `rho2x`,
`rhoseq`, each at kappa 0.5 / 0.7 / 1.0 (this favours rho). In a spot check of three (I/M, h) settings
across the six cells, the x-only sibling rho `rho2x` was the cheapest rho arm in every configuration of 17 of
the 18 (cell, setting) pairs, and `rho2` in the other.

## Headline [pcc:unverified]

Cost per target in GAE (1 GAE = I + 2M + S); median over 22 paired configurations [bootstrap 95%].

| cell | IC4 / strongest rho at I = 3M, h = 0 / 0.1 / 1 | at I = 30M, h = 0 / 1 | §5a vs strongest rho (I = 3 / 10 / 30 M) | IC4 / bsgs2, I = 3M, h = 0 / 1 |
|---|---|---|---|---|
| 22 b, T = 1024 | 1.053 [1.042, 1.058] / 1.064 / 1.116 [1.098, 1.123] | 1.053 / 1.122 | dearer / dearer / dearer | 1.51 / 1.62 |
| 24 b, T = 1024 | 1.094 [1.086, 1.105] / 1.109 / 1.173 [1.164, 1.188] | 1.114 / 1.188 | dearer / dearer / dearer | 1.70 / 1.80 |
| 24 b, T = 4096 | 0.961 [0.957, 0.966] / 0.969 / 1.006 [1.000, 1.012] | 0.985 / 1.021 | unranked / unranked / unranked | 1.36 / 1.47 |
| 24 b, T = 16384 | 0.918 [0.916, 0.922] / 0.920 / 0.933 [0.930, 0.938] | 0.945 / 0.949 | cheaper / cheaper / cheaper | 1.20 / 1.31 |
| 25 b, T = 4096 | 0.972 [0.967, 0.977] / 0.981 / 1.022 [1.017, 1.030] | 0.997 / 1.038 | unranked / unranked / unranked | 1.44 / 1.53 |
| 25 b, T = 16384 | 0.908 [0.906, 0.911] / 0.912 / 0.928 [0.925, 0.932] | 0.936 / 0.944 | cheaper / cheaper / cheaper | 1.25 / 1.35 |
- **IC4 is cheaper than the strongest rho at T = 16384, in both group sizes, at every I/M and every h:
  0.91x to 0.95x (5% to 9% cheaper).** That is the row.
- At T = 4096 the order flips across the band (cheaper at h = 0 and 0.1, dearer at h = 1): unranked.
- At T = 1024 IC4 is dearer (1.05x to 1.19x).
- Against paired BSGS IC4 is dearer everywhere: 1.18x to 1.80x (distance remaining).
- Against the anchored batch rho that the earlier IC campaign was matched to, IC4 stays cheaper in most cells
  (`analysis.txt`, "vs rho"), as in v1; that comparison is superseded by the strongest-rho reference above.

## Prediction vs outcome

Pre-registered: cheaper than the strongest rho at every T >= 4096 cell for every I/M; unranked or dearer at
T = 1024; dearer than bsgs2 everywhere. Outcome: holds at T = 16384 and T = 1024 and for bsgs2; **fails at
T = 4096** (unranked: the h = 1 ratio is 1.006 to 1.038). The v1 sensitivity runs had shown this cell as
cheaper; on fresh curves with the hash dedupe counted it is not.

## Where IC4 wins and why

IC4's target trials need no y-coordinate and its trial progression W_k is computed once for all targets; the
strongest rho (`rho2x`) also gets two x-classes per inversion but must carry y along its walk. The fixed cost of
IC4 (factor base, table, relations) is amortized over T, so its advantage appears only at large T; at T = 1024 the
fixed cost dominates.

## Limits

Toy groups (2^22 to 2^26); I, M, S counted, not timed; memory unbounded for every arm; local host (no
isolab-eks: this container has no AWS identity); one rho family (r-adding walks; no distinguished-point rho);
producer-written harness and analysis, so the row stands only after the independent validator's replay
certificate and report.
