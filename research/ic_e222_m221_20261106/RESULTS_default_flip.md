# Default flip: pair-sum table, fb 32, extra 2, rank-deficiency retry (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, serial seeded interleaved order; no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metric: solver `total_ms` (wall, diagnostic) and `trials_total` (operation count). All recovered logs checked against the planted logarithm.

**What changed (defaults of `ic solve-prime` / `solve-prime-file`).**
- Factor-base size 8 to 32 and `--extra-relations` 64 to 2 (CLI defaults).
- Relation collector: the pair-sum table is used automatically when the factor base has at most 64 points (its build is O(fb^2) additions and it loses from fb about 128 in `RESULTS_sum_table.md`). `IC_SUM_TABLE=1` forces it, `IC_SUM_TABLE=0` selects the per-trial sweep.
- Rank-deficiency retry: if elimination does not determine the target log, two more relations are collected and elimination is repeated, up to 8 times. This removes the failure mode of a small `extra_relations` (extra = 0 failed about 10% of runs in `RESULTS_extra_relations.md`).
- Together with the joint word-Jacobian relation point already in `main` (PR 3618). `IC_SHAMIR=0` and `IC_SUM_TABLE=0 --fb-size 8 --extra-relations 64` reproduce the former path.

**Paired panel.** 33 fresh prime-order instances (generator seed `e222m221-default-v1`, rungs 12 to 23 bits), seeds 1, 2. OLD = former shipped defaults (fb 8, extra 64, sweep, `IC_SHAMIR=0`); NEW = no flags. 128 runs, all recovered. Data: `results/default_flip/paired/rows.jsonl`. NEW / OLD total time, median of 64 paired ratios: 0.0015 (bootstrap 95% CI 0.0013 to 0.0016), about 660x faster. Medians by rung:

| r bits | OLD ms | NEW ms | NEW/OLD | trials OLD to NEW |
|---|---|---|---|---|
| 12 | 66.6 | 0.6 | 0.0093 | 2,140 to 78 |
| 16 | 1,728.6 | 2.4 | 0.0014 | 40,598 to 1,191 |
| 20 | 33,671.2 | 36.4 | 0.0011 | 646,130 to 18,355 |
| 22 | 139,372.0 | 164.9 | 0.0012 | 2,354,250 to 75,224 |
| 23 | 136,795.4 | 189.5 | 0.0014 | 2,336,908 to 76,820 |

Relations collected: 72 (OLD) to 34 (NEW, fb 32 + 2). The trial count falls about 31x (a larger factor base: trials scale like p over fb squared, and 34 rather than 72 relations are collected); the remaining factor is the cost per trial (table lookup instead of a factor-base sweep, and the joint relation point).

**Robustness sample.** NEW alone on 240 fresh instances (seed `e222m221-robust-v1`, generator rungs 12, 14, 16 and 18 with 60 each; the realised subgroup order has 12 to 19 bits), seeds 1, 2, 3: 720 runs, 720 recovered, 0 failures (rule-of-three 95% upper bound 0.42%). The retry never fired in these runs (every run collected exactly fb + 2 relations), so the sample does not exercise it; the unit test below does. Data: `results/default_flip/robust/rows.jsonl`.

**Retry test.** `staged_retries_a_rank_deficient_system`: 120 seeds with fb 8 and extra 0 on the 271-point fixture; all return the planted log and at least one seed takes the retry path. Result: passed (1 passed, 0 failed), so the retry path was taken and recovered the log.

**Limits.** One producer (`solve-prime-file`), prime-order toy instances up to r = 2^23, local host. The speedup is against the old shipped defaults of this tool, which were not tuned; against the tuned baselines of the earlier notes (`RESULTS_sum_table.md`: tuned sweep 38.9 ms at the confirmation set) the gain is the table plus the joint relation point (`RESULTS_shamir.md`), not 660x. Larger fb or p changes the optimum. No native E-222 / M-221 size was run, no H1 combination was timed here (H1 composes: it only changes the factor base).
