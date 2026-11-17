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


## Independent validation and required qualifiers (`results/merged/VALIDATOR_REPORT.md`)

Disposition: **validated as a measurement; the headline sentence needs qualifiers.** Reproduced: 480/480 recovered; all medians, paired ratios and by-instance bootstrap intervals recompute; the 480 rows are in the seeded execution order; the frozen values are exactly the argmin of the committed tune medians; the tune finished at 05:53:33Z, the freeze commit is 05:53:44Z and every eval row records the freeze-time commit, the eval started at about 05:53:48Z (the 13 s delay before the freeze reached the remote does not matter because the frozen values are fixed by the committed tune rows and the rows file is written only at the end; the margin between freeze and eval start is only about 4 s); the 60 evaluation instances are disjoint from the tune set and from 1,060 earlier instances; all 90 v6 instances pass independent arithmetic and match the manifest; the binary hash equals the stored hash and a clean rebuild of HEAD is bit-identical; 960 of 960 replays (all 480 rows twice) match in scalar, trials and relations; the new table equals the old table as a map on all 90 curves at fb 24/32/48; `add_vartime` matches `add` on identity, doubling and P + (-P); instruction-count ratios agree with the timings in ordering.

Qualifiers I accept:
1. **"Best hoisted sweep" is not the best sweep.** The sweep still runs constant-time `BigUint` arithmetic (field inversion is 40% of its instructions) and this change gave only the table and the walk the fast path. The validator's exploratory probe, a sweep ported to u64 words with identical relations, trials and scalars (120/120 match), is 2.1x lower in solver time at its own tuned fb (192). Against that probe sweep the table is about 7.6x and the walk about 20x lower in solver time and 1.46x and 1.54x end to end. The 15x and 38x quoted above are against the repository's sweep as implemented.
2. **Attribution.** At identical trials, this change alone takes the table from 6.23 to 1.58 ms and the walk from 3.71 to 0.495 ms (3.4x to 4.2x and 6.5x to 7.5x depending on host drift): most of the improvement over the previous panel is the fast point path in the new arms, not a new algorithm.
3. **End to end** (1.9x) sits at 95% to 97% of the ceiling set by the fixed 23 ms process overhead (ceiling 1.93x), and varies with size (1.4x, 1.8x, 3.1x).
4. **"Counted group operations are unchanged" is false as worded.** Trials and relations are unchanged, but the table build now performs 2 additions per pair instead of 4 (1,024 vs 2,112 at fb 32), so the single-target ledger's 2 fb^2 table charge overcounts and the single-target distance to the matched rho (stated as 25x to 53x) is nearer 22x to 49x; the multi-target 1.40x to 1.53x against BSGS is unaffected (that example has its own table build). The rho and BSGS conclusions stand qualitatively.
5. **Timer resolution** is not the limit (nanoseconds); the limit is run-to-run noise of about 20% to 25% per row. The conclusion rests on 120-pair medians and 120/120 wins.
6. **Two evaluation curves are anomalous** (n = p: W14-11 and W15-26); they behave normally in the solver.
7. **No unit test covers the env-gated table and walk paths** (only the validator's external equality checks); the table's relation point already used `relation_point` on main, so the earlier wording was inexact there.
8. Provenance: `rows.jsonl` (273 KiB) breaches the artifact-store rule (AGENTS.md section 13); the bot moves it.
