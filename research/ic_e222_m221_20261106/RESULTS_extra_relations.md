# Prime-order control: `--extra-relations` retune (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host; no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metric: `trials_total` (deterministic given seed), fb = 8 fixed so the per-trial cost is identical across arms.

**What this is.** A retune of an existing flag of the baseline, not a new factor-base design. The default `--extra-relations 64` makes the solver collect fb + 64 = 72 relations although only fb + 1 = 9 are needed to determine the 8 factor-base logs and the target. The measurement asks how much of the baseline cost is that surplus.

**Design (pre-registered in the session before the runs).** 48 fresh W (cofactor 1, prime-order) instances from the holdout generator seed `e222m221-holdout-v2`, rungs 10 to 17, solver seeds 1..3, arms extra = 64 (baseline), 8, 2, 0. An arm counts only if it recovers the planted log at least as often as the baseline. Data: `results/extra/e{64,8,2,0}/rows.jsonl`, `results/extra/analysis.txt`.

| extra | verified / 144 | paired n | median trials ratio vs 64 (IQR) | median trials | median relations |
|---|---|---|---|---|---|
| 64 (baseline) | 144 | - | 1 | 6900 | 72 |
| 8 | 144 | 144 | 0.225 (0.199 to 0.265) | 1557.5 | 16 |
| 2 | 144 | 144 | 0.142 (0.115 to 0.167) | 933.5 | 10 |
| 0 | 130 | 130 | 0.112 (verified runs only) | - | 8 |

**Failure.** extra = 0 fails 14 of 144 (`bounded_unknown`, spread over rungs 10 to 17): 8 relations cannot determine 9 unknowns, so the system is rank-deficient whenever the pair sums are not independent. That arm is not an improvement. extra = 2 had 0 failures in 144 (upper 95% bound about 2%), which is too few runs to set a default; extra = 1 and 3 and a larger failure-rate sample are not yet measured.

**Cost accounting.** Collection dominates and falls with the trials. Elimination and extraction run on a 10 x 9 system instead of 72 x 9, so they also shrink; they are not separately timed here (trials are the metric). Construction of the factor base, lifting (none: prime-order) and verification (`[k]G = Q` check against the planted log) are identical across arms.

**Limits.** One producer (`solve-prime-file`), fb = 8, toy sizes r <= 2^18, local host. Other producers (tournament, prime-orbit) may already use small extra values, so this says nothing about the leaderboard baselines. It does not beat rho: at these sizes the ratio to matched rho stays well above 1 (`RESULTS_rho1.md`). The goal is not met.
