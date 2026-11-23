# H1 cofactor projection: fresh holdout (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metric is `trials_total` (a deterministic operation count), not wall time.

**Design.** 144 new instances, never used before (generator seed `e222m221-holdout-v2`; E, M and W families, rungs 10 to 17, 6 per rung), solver seeds 1..3. Same binary and flags as `RESULTS_h1.md`; arms `off` (baseline) and `on` (`--project-cofactor`). 432 runs per arm. Data: `results/holdout/{off,on}/rows.jsonl`.

**Result (paired by instance and seed, on / off trials, runs verified in both arms):**

| family | cofactor | paired n | median ratio | IQR | in-sample median |
|---|---|---|---|---|---|
| E | 4 | 144 | 0.260 | 0.224 to 0.309 | 0.250 |
| M | 8 | 126 | 0.140 | 0.111 to 0.191 | 0.141 |
| W (control) | 1 | 144 | 1.000 | 1.000 to 1.000 | 1.000 |

The in-sample effect replicates on fresh curves and targets.

**Failures.** 18 of 432 runs per arm did not recover the log: all are rung M17, in both arms. The M17 field has p = 1,048,573, above the `ic solve-prime-file` cap of 10^6, so the tool returns `error` before solving (confirmed on M17-0). They are excluded from the M row (n = 126 = 7 rungs x 6 x 3), and the M row therefore covers rungs 10 to 16 only. No run in range failed.

**What this does and does not show.**
- Cost accounting: the projection adds a one-time [h]F per factor-base point (8 points, h doublings or fewer each), which is negligible next to hundreds to tens of thousands of trials. Not separately timed here. Construction, collection, elimination, extraction and verification are otherwise unchanged; no lifting is needed (target and relations both lie in <G>).
- It applies only to curves with cofactor h > 1. It does nothing for prime-order curves (W row 1.000), and brings E and M down to the cofactor-1 cost, not below it. The gap to the matched rho remains (see `RESULTS_rho1.md`).
- Goal status: not met. No validation-gate evidence, no prime-order improvement.
