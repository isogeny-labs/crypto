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

## Fresh-holdout failure-rate sample (v3)

280 new W instances (generator seed `e222m221-holdout-v3`, rungs 10 to 16 by the generator's p bits, r up to 2^18), solver seeds 1..3, fb 8, 840 runs per arm, extra = 1, 2, 3 and the 64 baseline. Data: `results/extra_v3/e{1,2,3,64}/rows.jsonl`, `results/extra_v3/analysis.txt`.

| extra | verified / 840 | median trials ratio vs 64 | bootstrap 95% CI (median) | IQR |
|---|---|---|---|---|
| 64 (baseline, median 4542 trials) | 840 | 1 | - | - |
| 3 | 840 | 0.148 | 0.145 to 0.152 | 0.124 to 0.180 |
| 2 | 840 | 0.136 | 0.132 to 0.140 | 0.111 to 0.166 |
| 1 | 840 | 0.123 | 0.120 to 0.126 | 0.099 to 0.149 |

Zero failures in 840 for each of extra = 1, 2, 3 (rule-of-three 95% upper bound 0.36% per arm); extra = 0 failed 14/144 (about 10%) on the first sample, so one relation of surplus is the cliff edge and extra = 2 or 3 is the safer setting. Per-rung medians are flat (0.12 to 0.13 for extra = 1), so the saving does not depend on size within this range. Both holdouts agree (first sample extra = 2: 0.142).

**Reading.** On the `solve-prime-file` producer with fb = 8, the shipped default spends about 7 to 8 times the trials it needs. The retune is reproducible on two fresh holdouts and costs no recoveries. It is still a parameter retune of the existing solver, unvalidated and local, with no matched-rho band, and says nothing about a different producer or factor-base size. Not yet measured: fb 16 and 32 (the surplus may not matter as much when fb grows), wall time including elimination, and the validation-gate evidence. The goal is not met.
