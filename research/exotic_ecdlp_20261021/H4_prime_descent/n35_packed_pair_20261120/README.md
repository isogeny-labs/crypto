# H4 n=35 direct-integer pair-index diagnostic

The frozen n35 instance-1 curve has order 34,359,876,606 = 66 × 520,604,191. Clearing its exact twelve-dimensional lifted x-subspace produced **4,096 distinct nonidentity prime-subgroup points**. A direct-integer first-pair index processed **16,777,216 ordered pairs** into **8,321,091 distinct sums** in **79.805 seconds**, with **826,359,808 bytes peak process RSS** on the checked local Sage host. All **32 frozen fresh subgroup targets** returned group-verified three-point witnesses; the median first-hit scan length was **36 points**. The independent audit rebuilt the base and used column-major pair traversal with an explicit minimum-pair rule, reproducing the sum count and every witness.

| Measured stage or result | n35 selected instance 1 |
| --- | ---: |
| Prime subgroup order / cofactor | 520,604,191 / 66 |
| Raw lifts / directly eligible points | 4,097 / 50 |
| Cleared nonidentity points / identity images / duplicate images | 4,096 / 1 / 0 |
| Ordered pair attempts / distinct pair sums | 16,777,216 / 8,321,091 |
| Instance / base construction | 0.033 / 8.340 s |
| Direct-integer index construction | 79.805 s |
| Verified / exhausted fresh target lookups | 32 / 0 |
| Median first-hit scan | 36 third points |
| Process elapsed / peak RSS | 88.204 s / 826,359,808 bytes |

The original five-instance n35 screen selected no curve at its precommitted `2^30` prime-subgroup gate. This separately frozen component experiment used a `2^28` gate and selected the first qualifying previously screened instance, index 1; it does not alter the screen's null result. The exact screen receipt and source hashes, ordered-pair rule, target law, 600-second process cap, and 8-GiB observed RSS stop were frozen in [protocol.json](protocol.json) and [run.py](run.py) before execution in `d0d5d43856f`.

[result.json](result.json) retains the curve and subspace identity, base digest, stage times, all 32 public target points, witnesses, and statuses. [sage-runtime-info.json](sage-runtime-info.json) identifies the checked build. [audit.py](audit.py) independently reconstructs the curve and base, checks all base coordinates against the polynomial-bit encoding, traverses all ordered pairs by second index first, and replays the exact first-pair witnesses; [audit.json](audit.json) passed. The [Lean key proof](formal/PROOF_MAP.md) establishes 35-bit packing injectivity and separation from the identity sentinel under the stated coordinate bounds.

The 32/32 target hit count is a frozen sample result. Under an independent uniform-target sampling model, its descriptive 95% Wilson interval is approximately 89.3% to 100%; no full-subgroup three-sum census is measured. The n31 and n35 curves, subgroup orders, cofactors, and bases differ, so their index times and yields are separate component measurements. A complete n35 IC candidate would need relation collection, full-rank factor-log recovery, fresh-target descent, and matched same-point rho before an online comparison.
