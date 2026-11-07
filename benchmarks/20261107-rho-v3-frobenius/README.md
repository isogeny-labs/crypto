# signed-Frobenius rho v3 (2026-11-07)

`rho.signed_frobenius_v3` uses the ladder-built table, distinguished
points sized from the floor, and a ladder-read check. It was run on six
Koblitz curves from 2^15.4 to 2^32.1, local L0, counted GAE.

- Paired ops v3 / v2: 0.459 [0.418, 0.504], robust across the band.
- Medium tier: 1.076× the floor [0.998, 1.147]; v2 is 1.249×.
- Search phase: 1.03–1.04× the floor from 2^21 up.

Unvalidated: no L2 run, no validator report; not a leaderboard row.
Report: [REPORT.pdf](REPORT.pdf). Evidence: `research/ecbench_rho_v3_frobenius_20261107/`.
