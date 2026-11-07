# Relation point: Shamir interleaving vs the arithmetic path (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, serial interleaved seeded order; no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metric: solver `total_ms` (wall, diagnostic); trial and relation counts are the equivalence check.

**Question.** After the pair-sum table (`RESULTS_sum_table.md`) a trial is one lookup plus `R = aG + bQ`, so the two scalar multiplications dominate. Does one interleaved (Shamir) double-and-add pass over both scalars with `G + Q` precomputed (about max-bits doublings and 3/4 as many additions) cut that cost? Opt-in `IC_SHAMIR=1`, default unchanged (`double_scalar_mul_vartime`, `relation_point` in `src/cryptanalysis/ec_index_calculus.rs`; unit test against two ladders on 1,600 pairs and the `Q = -G` identity case).

**First panel (two arms, off vs on) and the confound.** Table collector total time, on/off median 0.149 (CI 0.135 to 0.172), about 6.7x. That is larger than interleaving can give (about 1.7x), and the on arm also changes the arithmetic: the default path calls `scalar_mul`/`add`, the Shamir pass calls the variable-time `double_vartime`/`add_vartime` with the machine-word fast path. The two effects were not separated. Data: `results/shamir/v1_two_arm/rows.jsonl`.

**Control added and panel rerun (three arms).** `IC_SHAMIR=2` = two separate `scalar_mul_vartime` ladders plus `add_vartime` (same arithmetic path, no interleaving). 32 fresh prime-order instances (generator seed `e222m221-shamir-v1`, rungs 12 to 23 bits), solver seeds 1, 2, collectors table (fb 32, extra 2) and retuned sweep (fb 128, extra 2): 384 runs, all recovered the planted log; trials and relation counts are equal across the three arms in 128 of 128 groups (same R each trial, as designed). Data: `results/shamir/v2_three_arm/rows.jsonl`. Medians of per-run ratios, bootstrap 95% CI, n = 64 per cell:

| collector | arithmetic path (vartime two-ladder / default) | interleaving alone (interleaved / vartime two-ladder) | both (interleaved / default) |
|---|---|---|---|
| table | 0.125 (0.108 to 0.172) | 1.164 (1.068 to 1.226) | 0.155 (0.136 to 0.186) |
| sweep | 0.849 (0.820 to 0.902) | 1.014 (0.991 to 1.045) | 0.891 (0.833 to 0.907) |

By size, interleaved / control on the table collector: 1.006 for r <= 17, 1.299 for r >= 20.

**Result.**
- The gain is the arithmetic path, not the interleaving. Switching the relation point from `scalar_mul`/`add` to the variable-time pair makes the table collector about 8x faster (0.125) and the sweep about 1.18x faster (0.849), with identical trials, relations and recovered logs. The code comment at that call already says "use the variable-time ladder (public k, fine)"; the call used the other function.
- Interleaving, as implemented (plain binary, no window or NAF), is not an improvement: 1.16x slower on the table collector, 1.30x at r >= 20, level on the sweep. The two ladders presumably use a cheaper recoding than the 3/4-density binary interleave. A windowed or signed-digit interleave was not tried.

**Scope and limits.** One producer (`solve-prime-file`), toy sizes (r up to 2^23), prime-order instances, local host. Wall-time ratios from a single host. The default path is unchanged; `IC_SHAMIR=2` is opt-in. Making the variable-time pair the default would shift the other session's timed baselines (same trials, less time), so it is left as a separate decision. The native E-222 / M-221 operation counts (`h2_edwards`, `h3_montgomery`) price the same step in field operations and are unaffected.
