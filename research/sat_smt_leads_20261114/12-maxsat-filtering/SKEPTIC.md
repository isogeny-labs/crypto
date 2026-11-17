# Skeptic review: idea 12, MaxSAT for relation filtering (2026-11-14)

Independent review of the pilot in this directory. All numbers here are unverified pilot
assertions `[pcc:unverified]`. I did not edit any producer file. My scratch scripts and logs are in
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-12-maxsat-filtering/`
(`chk.py`, `n32.log`, `nbig.log`).

## Result of the review

- **Reproduced:** yes, for everything I re-ran (smallest cell, generator, tiny optima, N = 32 optima at 5% excess).
- **Stated verdict supported:** weaker than stated. The kill criterion is met for what was tested:
  subset selection on uniform random weight-4 matrices, 5 s per arm. The idea as written in the task card
  also covers merges / fill-in and realistic matrices, and neither was tested.
- **Corrected verdict:** kill-criterion-met, limited to that scope. Branch (b) is the solid one.

## 1. Generator and solver correctness (my own code)

The review checklist item about planted targets and curve points does not apply: this pilot has no curve
arithmetic. I checked the equivalent things for a matrix-selection problem.

- Regenerated all 72 instances from the seed formula with my own generator. All 72 seeds are distinct,
  every row has 4 distinct columns in range, and R matches `ceil((1+e)N)`.
- My own singleton removal reproduces `g0_rows` and `g0_cols` on 72/72 instances.
- Column weights at N = 1024, 100% excess: mean 8.0, variance 7.88, as the uniform model predicts.
- 16 new tiny instances (N = 9..12, my seeds): my brute force, my z3 Optimize model, the producer's RC2
  and the producer's ANY arm agree on the optimum in 16/16. The greedy G2 was one column above the optimum on one of them.
- N = 32, 5% excess: z3 Optimize (not a producer arm) proves the optimum on 6/6 instances, and it equals
  the reported G3 and ANY column counts (27, 26, 29, 26, 24, 29).

Not checked: the selected rows of each arm are not stored in `results.json`, so a reported selection can
only be re-verified by re-running, not from the record.

## 2. Re-run of the smallest cell

`python3 filt.py smoke 32 5 5` matches `results.json` (N = 32, 5%, rep 0) in every non-timing field:
G0 31 rows / 29 columns, G1 = G2 = G3 = 27 columns, RC2 timeout, ANY optimal at 27 columns.
Only wall times and the G3 restart count differ (50393 against 53097; G3 is wall-clock bounded).

Recomputed from `results.json`: 66 full instances, 0 skipped, RC2 optimal on 0, ANY optimal on 11,
ANY better than greedy on 0, ANY selection size equal to G3 on 66/66.

## 3. Fairness of the comparison

What holds:

- Same instances and seeds for every arm; arms run back to back per instance.
- The treatment is favoured, not the baseline: ANY starts from the G3 solution and gets 5 s on top of G3's 5 s.
- The baselines named in the task card are all present (singleton removal, weight-scored excess removal,
  greedy Markowitz elimination), plus a stronger restart arm at equal wall budget.
- Every arm that was run is in the table, including RC2's 0/66.
- No slope is claimed for the MaxSAT arms (one size only). The G2 slope uses 8 sizes.
- The enumeration / linear-scan / MITM baselines in the checklist are for decomposition pilots and do not apply.

Problems:

1. **RC2 is a weak exact arm, so "RC2 0/66" overstates the difficulty at N = 32.**
   z3 Optimize solves the N = 32, 5% instances in 0.1 to 0.6 s each, where RC2 times out at 5 s.
   The cause is likely the encoding (one cardinality network over all R + N literals as a hard clause).
   z3 Optimize was named in the task card and not run.
2. **This does not move the exact frontier.** z3 returned no answer in 12 s on 12/12 instances at N = 32
   with 20% and 100% excess, nor in 20 s on 11/11 instances at N = 64, 128, 256 with 5% excess.
   So "exact solving stops near N = 32" survives a second, independent solver.
3. **Only half of the idea was modelled.** The task card says "the subset / the merges that minimise the
   weight or the fill-in". Merges and fill-in are not in the MaxSAT model. Since every row has weight 4,
   the "weight" objective reduces to a column count.
4. **Instances are the uniform random model only.** The task card allowed this if real decompositions were
   not cheap, and the producer says so. But the producer's own planted control gives MaxSAT 7 columns
   against greedy 31, so structure is exactly where a gain could exist.
5. **The task card's primary metric (N x weight after elimination) is not usable as measured.**
   `cost_sge` is degenerate at N = 32 and 64 (cost 2 for every arm). In `analyze.py` the ratio divides the
   MaxSAT arm by the minimum over G1, G2 and G3, so it can only be >= 1 when ANY returns the G3 selection.
   Against G3 alone the ratio is exactly 1.0 on 66/66. Branch (a) is therefore evaluated on column count
   and `cost_sel`, which is reasonable but is not what the criterion names first.
6. **The anytime arm is one design at one budget.** It is a SAT call at bound (G3 - 1) with phase hints.
   No local search, no neighbourhood search, 5 s only. For N >= 64 the optimum is unknown, so
   "no gain" means "no better model in 5 s", as the producer states.
   My probe: 40 s of the producer's restarts at N = 512, 100%, rep 0 (108 restarts against 12) also
   stayed at 228 columns, so I found no sign of large headroom, but this is one instance.
7. **Coverage gaps against the task card:** no MaxSAT arm at N = 2048 and 4096, no 50% excess,
   2 to 6 reps per cell. All are disclosed.

## 4. Was the protocol written before the results?

- `PROTOCOL.md` and `filt.py` have the same mtime to the nanosecond (09:36:48), after `validation.json`
  (09:36:04) and before `results_raw.json` (09:54:23). That is consistent with the protocol and deviation
  D1 being fixed before the measured run. It does not prove it: there is no git history, and identical
  mtimes suggest a batch write.
- D1 (density stop at average row weight 16) is a change made after smoke runs. It affects only `cost_sge`,
  which does not drive the verdict.
- The prediction in the protocol matches the outcome, and the kill criterion text matches the task card.
- `SESSION_LOG_ENTRY.md` is stamped 10:15Z while the last file was written at 09:55. Cosmetic.
- I saw no sign of arms or cells removed after the fact: the run plan in `filt.py` matches the 72 rows.

## 5. Does the verdict follow from the kill criterion?

- **Branch (b): met.** Exact solving stops at N = 32, far below 2^10, and no anytime gain was found at any
  size. Confirmed with a second exact solver. This is the branch to rely on.
- **Branch (a): met as measured, but weak for N >= 64.** The ratio is 1.0000 in all 18 cells with N <= 1024.
  For N >= 64 the treatment simply returned its warm start, so the true MaxSAT reduction was not measured.
- The verdict should be read as: weighted-MaxSAT subset selection, with these encodings and a 5 s budget,
  does not beat a greedy pipeline with restarts on uniform random weight-4 relation matrices.
  It says nothing about merge selection, fill-in, or structured matrices.

## What would change this

- A run on relation matrices from real toy decompositions, or with large-prime or orbit structure.
- A model that includes merges, or a tighter encoding (native pseudo-Boolean, or the reduced formulation
  on the singleton-free core where the excess to remove is small: 2 to 3 rows at N = 32, 5%).
