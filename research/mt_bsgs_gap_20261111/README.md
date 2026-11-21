# Why the multi-target IC stays above BSGS (option 3, 2026-11-11)

Question: the multi-target IC (IC4, `research/mt_fieldops_v2_20261111/`) is 1.18x to 1.80x a paired BSGS per
target (1.49x to 2.04x without the shared verification constant; claim PCC-20261111-fa13c8 [pcc:unverified]).
Can the IC close that gap?

## The mechanism

Both methods build a table of K x-classes at the same cost per class (one shared inversion per two classes) and
then try one point per step. They differ in which logs the table covers:

- **IC:** the table logs are sums of factor-base logs, i.e. effectively random. A trial point Q + w is in the
  table with probability 2K/n, independently of earlier trials, so the expected number of trials is n/(2K).
- **BSGS:** the baby table covers the interval |k| <= Mx and the giant offsets tile the group with disjoint
  windows, so the hit trial is uniform over the windows: expected trials 1 + n/(4 Mx), half as many.

At the optimum (table cost = expected target cost) a factor 2 in trials becomes a factor sqrt(2) in total cost,
before fixed costs. That is the structure of the measured gap.

## Check 1: the models predict the measured trial counts (v2 holdout rows, no new run)

`trials_check.py` on `research/mt_fieldops_v2_20261111/results/holdout/rows.jsonl` (d = fraction of factor-base
logs determined; a hit is usable with probability about d^2):

| bits | T | ic4 measured / n/(2Kd^2) | bsgs2 measured / (1 + n/(2(2Mx+1))) |
|---|---|---|---|
| 22 | 1024 | 1.011 (22 runs) | 1.002 (66) |
| 24 | 1024 | 1.013 | 1.006 |
| 24 | 4096 | 1.011 | 0.996 |
| 24 | 16384 | 1.022 | 1.000 |
| 25 | 4096 | 1.009 | 1.002 |
| 25 | 16384 | 1.013 | 1.001 |

Both models hold to within about 2% [pcc:unverified], so the factor 2 per target is the whole story of the
target phase.

## Check 2: choosing the offsets cannot recover it (combinatorial simulation)

After phase 1 the IC knows every table log, so in principle it could choose its trial offsets w_k to avoid
re-covering logs already covered, as BSGS's giant steps do. `translate_sim.py` measures the exact expected hit
time E = sum_k (1 - |U_k|/n) for a random K-subset of Z_n (n = 2^18, 3 seeds) under three offset strategies:
random; greedy (each step, the best of R random candidates by new coverage); and the tiled interval (BSGS).
Data `translate_sim.json`; figure `gap.svg`/`gap.png`.

| table K | trials (random) | greedy R = 16 | greedy R = 256 | tiled (BSGS) |
|---|---|---|---|---|
| 256 | 1024 | 0.873 | 0.821 | 0.500 |
| 1024 | 256 | 0.930 | 0.896 | 0.500 |
| 4096 | 64 | 0.963 | 0.943 | 0.500 |

(columns 3 to 5: expected trials relative to random offsets; models n/K and (n/K + 1)/2 reproduce columns 2 and
5 exactly.) In the IC's regime (tens of trials per target) the best greedy choice saves 4% to 6%, not 50%, and
computing it costs about R x K table lookups per step: at the §5a price of 1 GAE per unpriced operation that is
far more than it saves. A random set's translates cannot be packed: once the union has density u, a new
translate overlaps it in about 2Ku points whichever candidate is chosen.

## Conclusion

The gap to BSGS comes from the table's structure, not from arithmetic or trial selection. Closing it needs a
table whose logs form an interval (or a sum-set of arithmetic progressions), which requires a factor base with
known logs: that is BSGS itself, a relabelling, and not an index-calculus improvement. Within generic algorithms
the IC therefore cannot do better than tie BSGS, and with random-log tables it cannot even tie in the target
phase. Beating BSGS (and rho by more than constants) needs a non-generic property of the curve, which none of
these methods uses. Status: closed as a negative result for this family of methods [pcc:unverified].

Limits: the simulation is combinatorial (no curve arithmetic) on plain sets; greedy candidates are random, and
a smarter search could do somewhat better, but the packing argument bounds any strategy far above the tiled 0.5
in this regime.
