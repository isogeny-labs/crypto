# Multi-target IC with a bucketed pair table (frozen A/B evaluation)

Status: [pcc:unverified] producer's own measurement; not independently replayed; same machine, 1 thread, wall clock, toy sizes (25 to 31 bits).

Change: the pair table of `x(F_j +- F_k)` was one globally sorted array searched by binary search (about 20 dependent steps per lookup over 1M entries at 27 bits). It is now bucketed by the high bits of x: a counting sort into about entries/2 buckets plus a bucket-offset index, so a lookup reads one bucket (about 2 entries) and filters by exact x; `max_x` is stored separately. The legacy layout stays available in the same binary (`IC_PAIR_SORTED=1`) for A/B. Both layouts pass the 10 `prime_fast` unit tests. The build time of the table did not change (the counting-sort scatter is as slow as the old sort, 30 to 50 ms at 1M entries); the gain is in every lookup (collection, per-target phase).

Protocol: frozen in `results/pf_bucket/frozen.json` (commit 8702ff9b48, pushed and checked on the remote before the evaluation) with the prediction and abandon rule: new c=1 / old c=1 <= 0.75 at T = 1000; new c=2 / old c=2 <= 0.85 at T = 1000; new c=2 / old c=2 <= 0.9 at T = 10 000; abandon if new c=1 / old c=1 >= 1.0 at T = 1000. Same binary for the two layouts; order per (bits, T): new c=1, old c=1, new c=2, old c=2, tuned BSGS; identical planted targets. Curves 25, 29, 31 bits (not used to tune this arm; the layout was explored at 27 bits only). 20 rows per arm, 100 arm-runs, every target of every run verified (`k G = Q`), no failed run. Provenance in `provenance.txt`.

Results (medians of per-row ratios, total time; ranges in brackets):

| T | rows | new c=1 / old c=1 | new c=2 / old c=2 | new c=1 / BSGS | new c=2 / BSGS | old c=1 / BSGS | old c=2 / BSGS |
|---|---|---|---|---|---|---|---|
| 1000 | 10 | 0.47 (0.38 to 0.85) | 0.68 (0.47 to 0.81) | 2.0 | 4.0 | 4.1 | 5.9 |
| 10 000 | 10 | 0.44 (0.26 to 0.60) | 0.45 (0.27 to 0.80) | 3.0 | 2.3 | 7.3 | 5.5 |

By size, best of the two new constants over the tuned BSGS: T = 1000: 1.9 (25 bits), 2.2 (29), 2.4 (31); T = 10 000: 2.2, 2.3, 2.4. New/old by size: 0.77, 0.46, 0.39 (c=1, T = 1000) and 0.56, 0.41, 0.27 (c=1, T = 10 000): the gain grows with size.

Prediction check: all three predictions hold (0.47 <= 0.75, 0.68 <= 0.85, 0.45 <= 0.9); the abandon rule is not triggered.

Finding: replacing the binary search by a bucketed lookup is a reproducible 1.5x to 3.7x speed-up of the multi-target IC in the same binary, the largest single gain of the campaign, and it brings the best IC variant to about 2.0x to 2.4x of the tuned multi-target BSGS (T = 1000 and 10 000, 25 to 31 bits), with a much flatter growth over size than before (1.9, 2.2, 2.4). Earlier distances (3.8x to 4.5x for the previous arm) were inflated by the lookup layout, not only by the algorithm. The remaining gap is not closed: IC is still 2x or more slower than the tuned BSGS, and the BSGS arm itself is 1.07x to 1.31x slower than a compacted variant (validator), so the true distance is about 2.2x to 3x.

Limits: sizes 25 to 31 bits only, so the trend over size is three points; the best constant is chosen per (T, size) from two values, post hoc where the freeze gave no rule; one thread; memory not priced (the pair table is fb^2 entries); BSGS reference arm not compacted; the order of arms within a (bits, T) group is fixed, not randomised; no rho arm, §5a band, certificate file, bench record or `VALIDATION.json`; not independently replayed.
