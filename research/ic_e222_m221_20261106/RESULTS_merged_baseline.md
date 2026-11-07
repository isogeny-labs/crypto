# Table and walk against the merged baseline: fresh tune/eval with provenance (local, unvalidated)

**Status:** unvalidated (pre-gate) until the validator replay in this PR's follow-up; local unisolated host, serial runs; toy sizes (group order up to 2^17); no matched-rho band here (the rho and BSGS comparisons are counted-operation results in `RESULTS_vs_rho.md` and `RESULTS_multi_target.md`, unaffected by this).

**Why this panel.** On the merged tree the sweep baseline got a faster relation point (another session), which left the earlier table and walk numbers partly stale, and the validator's replay showed that the table and the walk's own code paths still used the slow constant-time point operations. This branch moves `build_pair_sum_table` (now two variable-time additions per pair, negation free), the table's relation point, and the walk's setup, restarts and step to the repository's fast path (`relation_point`, `add_vartime`); the default path is unchanged and all arms are env-gated as before. Then the full protocol was rerun on a new instance set.

**Protocol.** `run_merged.py`: instances v6 (generator seed `e222m221-holdout-v6`, W 14, 15, 16 bits, never used in any earlier panel). `tune` on idx 0 to 9, seed 1, grids wide on both sides (hoisted sweep fb 64 to 512, table 8 to 64, walk 6 to 48); all four frozen values are interior to their grids: shipped default fb 8, hoisted sweep **256**, table **32**, walk **24** (`results/merged/frozen.json`, committed 05:53:44Z). `eval` on idx 10 to 29, seeds 1 and 2 (60 instances, 120 pairs, 480 runs), serial, interleaved seeded order; all `IC_*` variables scrubbed before each run; every row records the commit, the binary sha256, the exact argv and the `IC_*` environment. Order of events: the frozen commit was made at 05:53:44Z, the evaluation started at about 05:53:48Z and the commit reached the remote at 05:54:01Z (a push attempted at 05:53:50Z failed because the remote branch had just been deleted by a merge); the rows file is written only at the end of the run, so nothing was influenced by results. `analyze_merged.py` reproduces every number below from the raw rows.

All 480 runs recovered the planted log; no timeouts or missing timings.

| arm | fb | solver ms (median) | process ms (median) | median trials |
|---|---|---|---|---|
| A shipped default (sweep, extra 64) | 8 | 304.96 | 327.3 | 17,909 |
| Eh hoisted sweep, extra 2 | 256 | 22.21 | 45.1 | 263 |
| D pair-sum table, extra 2 | 32 | 1.50 | 26.2 | 543 |
| Dw table + restarted walk, extra 2 | 24 | 0.59 | 24.0 | 661 |

Paired ratios, median [bootstrap 95%, resampling instances], solver timer | process time | pairs in which the first arm is lower (solver):

| ratio | solver | process | wins |
|---|---|---|---|
| D / Eh | 0.066 [0.058, 0.072] | 0.546 [0.466, 0.595] | 120/120 |
| **Dw / Eh** | **0.026 [0.022, 0.029]** | **0.534 [0.458, 0.577]** | 120/120 |
| Dw / D | 0.374 [0.345, 0.429] | 0.939 [0.920, 0.975] | 120/120 |
| Eh / A | 0.077 [0.074, 0.083] | 0.146 [0.131, 0.161] | 120/120 |
| D / A | 0.005 [0.005, 0.006] | 0.082 [0.062, 0.095] | 120/120 |
| Dw / A | 0.002 [0.002, 0.002] | 0.080 [0.061, 0.091] | 120/120 |

By size, Dw / Eh (solver): 0.041, 0.031, 0.019, 0.014 for r bits 14 to 17, and D / Eh: 0.082, 0.072, 0.058, 0.046.

**How to read this.**
- The table is about 15 times lower in solver time than the best retuned and hoisted sweep and the walk about 38 times, and about 500 times lower than the shipped default; end to end they are about 1.9 times lower than the hoisted sweep and about 12 times lower than the shipped default. The end-to-end figures are small because a fixed process overhead of about 24 ms (hashing the binary at start, point counting) now dominates a sub-millisecond solve; solver-timer ratios of this size are close to what that timer can resolve and should be read with that in mind.
- A large part of the change from the previous panel is my own code catching up with the repository's fast point path (the table and walk had been running the slow constant-time operations while the sweep's relation point had been sped up by another session); it is a real reduction in time but it is engineering of the new arms, not a new algorithmic idea, and the validator's finding that "the walk adds nothing beyond a retuned table" applied to the previous code: with the fast path in the walk's own code the walk is 0.374x of the table in solver time and 0.94x end to end.
- Counted group operations are unchanged by this work (the new code paths perform the same number of additions); the comparisons against the matched rho and against the counted BSGS (single target 25x to 53x and multi-target 1.40x to 1.53x in counted operations) therefore stand, and the improvement documented here is over the repository's own sweep, not over the generic attacks.

**Limits.** Toy sizes (the table's cost grows with the square of the factor base, and the best fb grows with the field), one unisolated host, solver timer plus process time, 60 instances x 2 seeds (pairs share an instance), and no independent replay of this panel yet.
