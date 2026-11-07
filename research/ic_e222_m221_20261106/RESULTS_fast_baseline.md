# Table and walk against the strongest implemented sweep (word-sized sweep): fresh tune/eval (local, not yet independently replayed)

**Status:** unvalidated (pre-gate) until the validator replay; local unisolated host, serial runs; toy sizes (group order up to 2^17); no matched-rho band here. The comparisons with rho and BSGS are counted-operation results in `RESULTS_vs_rho.md` and `RESULTS_multi_target.md` and are not changed by this panel.

**Why this panel.** The validator's replay of the previous panel (`results/merged/VALIDATOR_REPORT.md`) noted that the repository's sweep still ran constant-time `BigUint` arithmetic and was not the strongest sweep: a probe with the inner loop on u64 words was about 2.1x lower in solver time. That probe is now an implemented, env-gated arm (`IC_FAST_SWEEP=1`, PR 3628: same candidates in the same order, using the file's own `sqrt_mod_p_u64` and `inv_mod_u64`, a factor-base bitmap built once per solve; unit test `word_sweep_matches_the_biguint_sweep` checks identical relations and trial counts against the BigUint sweep for 80 relations), so the baseline in this panel is the strongest implemented sweep. Applies to prime fields below 2^40.

**Protocol.** `run_fast.py` (same provenance and scrubbing as `run_merged.py`): instance set v7 (generator seed `e222m221-holdout-v7`, W 14, 15, 16 bits, never used before), `tune` on idx 0 to 9 (seed 1, grids wide on both sides; all frozen values interior): shipped default fb 8, hoisted BigUint sweep (Eh) **256**, word sweep (Ef) **96**, table (D) **32**, walk (Dw) **16**. `frozen.json` was committed at 07:28:09Z and verified on the remote (`git ls-remote`) before the evaluation started at 07:28:29Z. `eval` on idx 10 to 29, seeds 1 and 2 (60 instances, 120 pairs, 600 runs), serial, interleaved seeded order; every row records the commit (2fb74a6; the binary was built from the identical source of 8e3e3bc, the merge adding only another session's example and research files), the binary sha256 (78d4fb69dc5a), the argv and the `IC_*` environment. `analyze_fast.py` reproduces every number below.

All 600 runs recovered the planted log; no timeouts or missing timings.

| arm | fb | solver ms (median) | process ms (median) | median trials |
|---|---|---|---|---|
| A shipped default (BigUint sweep, extra 64) | 8 | 300.49 | 324.3 | 18,760 |
| Eh hoisted BigUint sweep, extra 2 | 256 | 21.79 | 46.0 | 262 |
| **Ef word-sized sweep, extra 2 (strongest implemented sweep)** | 96 | 9.76 | 33.8 | 226 |
| D pair-sum table, extra 2 | 32 | 1.48 | 25.2 | 554 |
| Dw table + restarted walk, extra 2 | 16 | 0.50 | 23.9 | 1,177 |

Paired ratios, median [bootstrap 95%, resampling instances]; wins = pairs in which the first arm has the lower solver time:

| ratio | solver | process | wins |
|---|---|---|---|
| Ef / Eh (the word sweep itself) | 0.425 [0.411, 0.455] | 0.716 [0.670, 0.758] | 120/120 |
| **D / Ef (table vs the strongest implemented sweep)** | **0.162 [0.133, 0.179]** | **0.747 [0.694, 0.777]** | 120/120 |
| **Dw / Ef (walk vs the strongest implemented sweep)** | **0.055 [0.047, 0.062]** | **0.703 [0.635, 0.757]** | 120/120 |
| Dw / D | 0.367 [0.345, 0.382] | 0.961 [0.941, 0.981] | 118/120 |
| Ef / A | 0.034 [0.032, 0.036] | 0.110 [0.097, 0.119] | 120/120 |
| Dw / A | 0.002 [0.002, 0.002] | 0.077 [0.066, 0.087] | 120/120 |

By size, Dw / Ef (solver): 0.072, 0.071, 0.049, 0.036 and D / Ef: 0.199, 0.175, 0.128, 0.107 for r bits 14 to 17 (the advantage grows with size in this range).

**How to read this.**
- Against the strongest implemented sweep the table has about 6x and the walk about 18x lower solver time; end to end about 1.3x and 1.4x lower, because a fixed process overhead of about 23 ms (hashing the binary at start, point counting) dominates a sub-millisecond solve. The shipped default configuration is about 600x the walk's solver time and about 13x its end-to-end time.
- Most of the difference between this panel and the earlier ones is the new arms using the repository's fast point path and the baseline using fast field arithmetic; the table and walk are the structural parts (replacing a per-trial O(fb) square-root sweep with a lookup, and the two scalar multiplications per trial with a walk step).
- Counted group operations are unchanged by the arithmetic work, so the distances to the matched rho (single target, about 22x to 49x in counted operations) and to the counted BSGS (multi-target, 1.40x to 1.53x) stand; the improvement here is over the repository's own sweep, not over the generic attacks.

**Limits.** Toy sizes (the table's cost grows with the square of the factor base and the best fb grows with the field); one unisolated host; solver timer and process time; 60 instances x 2 seeds (pairs share an instance); the word sweep applies to p below 2^40; no independent replay of this panel yet.
