# Restarted r-adding walk on the pair-sum table: frozen tune/eval (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, serial runs. No replay certificate, no validator report on this panel, no matched-rho band. Not a leaderboard row.

**Mechanism.** With the pair-sum table, a trial is one hash lookup, so the cost of a trial is the two scalar multiplications that build R = aG + bQ (about 13 of 16 ms at fb 32 in the earlier panel). `IC_WALK=1` (with `IC_SUM_TABLE=1`) replaces them by an r-adding walk: 32 precomputed points a_k G + b_k Q, the step chosen by x mod 32, one point addition and a running (a, b) per trial. Restart cost (two scalar multiplications per walk start, and the 32 arm points once per solve) is inside `total_ms`. A first version without restarts hung: a walk whose step depends only on x enters a cycle after about sqrt(n) steps, which is shorter than the 500 to 3000 trials a relation can need, and the tuning panel stalled with no data (killed, logged in `docs/session-logs/2026-11-07.md`). The shipped variant restarts every 32 steps (`WALK_RESTART`).

**Protocol.** New instance set v5 (generator seed `e222m221-holdout-v5`, W 14, 15, 16 bits). `run_walk.py tune` on instances 0 to 9 (seed 1, fb in 16, 24, 32, 48, 64, 128; 360 runs, 0 failures) wrote `frozen.json` = {hoisted sweep fb 256 (carried from the v4 tuning), table fb 48, walk fb 16}, committed and pushed at 02:39:17Z (commit 0b345b6) before `run_walk.py eval` ran instances 10 to 29 (60 instances never used before), seeds 1, 2, 360 runs, serial, interleaved seeded order, 120 s per-run timeout. Data: `results/walk/` (`rows.jsonl`, `frozen.json`, `tune_rows.json`, `analysis.txt`).

| arm | fb | solver ms | process ms | max solver ms | median trials |
|---|---|---|---|---|---|
| Eh hoisted sweep, extra 2 | 256 | 26.9 | 50.5 | 91 | 262 |
| D pair-sum table, extra 2 | 48 | 13.9 | 37.8 | 41 | 377.5 |
| Dw table + restarted walk, extra 2 | 16 | 3.3 | 27.1 | 9 | 1113.5 |

All 360 runs recovered the planted log; no timeouts.

| paired ratio (120 pairs) | solver ms | process ms |
|---|---|---|
| Dw / D | 0.249 [0.235, 0.259] | 0.720 [0.689, 0.752] |
| Dw / Eh (vs best hoisted sweep) | 0.124 [0.110, 0.132] | 0.520 [0.492, 0.564] |
| D / Eh | 0.494 [0.457, 0.526] | 0.718 [0.691, 0.774] |

Dw beat D in 120/120 pairs on the solver timer and 114/120 at process level; Dw beat Eh in 120/120 on both. Per-size Dw/D (solver): 0.250, 0.254, 0.251, 0.200 for r bits 14 to 17.

**Reading.** On the solver timer the walk is a further 4x over the table and about 8x over the best hoisted sweep. The process figures are smaller (1.4x and 1.9x) because the process spends about 24 ms outside the solver timer (mostly hashing the binary at start, per the validator), which now dominates a 3 ms solve; the process figure is the end-to-end one and the solver figure is the algorithmic one. The walk needs about 3x more trials than the table at a smaller fb (1,113 vs 378) but each costs a point addition rather than two scalar multiplications.

**Note on rho (added after review):** nothing in this file compares with rho; the IC-vs-rho comparison is in `RESULTS_vs_rho.md` and shows the walk is far from the matched rho in counted operations.

**Caveats.** The walk's frozen fb (16) is the lowest value on the tuning grid, so its true optimum may be lower and the arm may be slightly understated. Walk samples are correlated along a segment (restart every 32 steps); all 720 runs across tuning and evaluation recovered their logs, but the recovery rate under the walk at larger sizes and with extra = 2 has only been sampled here. Toy sizes (r up to 2^17); the exponent argument (cost near p^(2/3) with the table, rho p^(1/2)) is unchanged by a constant factor, so this does not close the gap to rho. No isolab-eks run, no validation gate.
