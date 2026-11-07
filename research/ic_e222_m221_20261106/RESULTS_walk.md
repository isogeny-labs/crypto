# Restarted r-adding walk on the pair-sum table: frozen tune/eval (local, unvalidated)

> **SUPERSEDED for the merged baseline.** The sections above the last two describe the pre-merge binary. On the merged tree (another session's faster relation point) the walk adds nothing beyond a retuned table; see the last section and `results/walk_postmerge/VALIDATOR_REPORT.md`.

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


## Independent validation (2026-11-07)
`results/walk/VALIDATOR_REPORT.md`: frozen-before-eval order confirmed (frozen.json 02:39:17Z, rows.jsonl 02:40:25Z); all 360 rows replayed with 0 mismatches in scalar, trials and relations (solver time 0.94 to 0.97 of recorded, process time 0.74 to 0.84); the 60 evaluation instances are valid and disjoint from every earlier panel; the walk keeps r = aG + bQ (independent Python emulation, 0 violations), cannot return a wrong verified log, and leaves the default path unchanged; the walk's and sweep's frozen fb are within 1% of the optimum of a finer grid. Gaps it lists: the headline compares against an arm added by the producer (the hoisted sweep) and the file has no unmodified-baseline row, rho row or counted-operation unit; the pairs are instances x 2 seeds; the rows do not store scalars or a binary hash.

## Re-measurement against the moved baseline (post-merge binary, same frozen settings)

After this branch was merged with `main` (2026-11-07, 05:12Z) the relation point of the sweep and, evidently, the point arithmetic had become faster (another session's work: `relation_point` defaulting to one interleaved word-Jacobian pass, `Point::double_scalar_mul_vartime`; see `RESULTS_shamir.md` on main). Only the relation-point call path changed, not the point arithmetic in general. The strongest implemented baseline therefore moved. `ic` was rebuilt from the merged tree and `run_walk.py eval` was rerun with **identical frozen settings (Eh fb 256, D fb 48, Dw fb 16), instances (v5 idx 10 to 29) and seeds** (`results/walk_postmerge/`, 360 runs, all 360 recovered, no timeouts; `analysis.txt`). The frozen settings were not re-tuned for the new arithmetic.

| ratio (120 pairs) | before the merge: solver / process | after the merge: solver / process |
|---|---|---|
| Eh (best hoisted sweep) absolute | 26.9 ms / 50.5 ms | 24.8 ms / 51.1 ms |
| D (table) absolute | 13.9 ms / 37.8 ms | 6.2 ms / 30.4 ms |
| Dw (table + walk) absolute | 3.3 ms / 27.1 ms | 3.7 ms / 28.6 ms |
| D / Eh | 0.494 [0.459, 0.526] / 0.718 | 0.260 [0.218, 0.299] / 0.593 [0.534, 0.649] |
| Dw / D | 0.249 [0.235, 0.259] / 0.720 | 0.587 [0.541, 0.659] / 0.937 [0.908, 0.959] |
| **Dw / Eh** | 0.124 [0.110, 0.132] / 0.520 | **0.152 [0.131, 0.170] / 0.537 [0.487, 0.600]** |

**Reading.** The headline against the best hoisted sweep survives the move of the baseline: 0.152x of its solver time and 0.537x of its process time (about 6.6x and 1.9x). But the decomposition changed: the new relation point helps the table (whose cost is the table build and the two scalar multiplications per trial; its relation-point share fell from 64% to 7% of instructions) far more than the sweep (90% factor-base sweep, of which inversions are about 40% and square roots about 14% of instructions; the relation point is 1.5% of its instructions after the merge), so the table alone is now 0.26x of the sweep, and the walk's extra gain over the table fell from 4x to 1.7x in solver time and to 6% end to end. The walk's advantage was largely the cost of scalar multiplications that the new relation point made cheap for the other arms; the walk's own code never uses `relation_point`, so it did not benefit (its absolute time moved from 3.3 to 3.7 ms, which is about 11% environment drift on this host; its counted-operation figures are unchanged, though that was not re-verified). The post-merge panel was independently replayed (`results/walk_postmerge/VALIDATOR_REPORT.md`: 360/360 rows, identical scalars, trials and relations in six replay passes; ratios recompute; pre- and post-merge trials agree in 360/360). **The same validation shows that a table retuned to fb 16 on the merged tree beats the walk: walk / table = 1.45x [1.39, 1.58] in solver time, about 1.01x end to end, the walk winning 19 of 120 pairs (post-hoc probe on the already-evaluated instances, not a new holdout).** So on the merged baseline the supported improvement over the hoisted sweep is the table (with the new relation point), not the walk. The shipped default is about 76x the walk in solver time and 11x end to end on the merged binary (seed 1 only), replacing the pre-merge 200x.
