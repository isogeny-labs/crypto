### Table 1. Measured operation counts, every verified run

Unit: group operations (`ecbench.gae`, one affine addition or doubling = 1), every phase charged. `S = mean ops / sqrt(n)`. `ratio to rho` is `sum ops(Cheon) / sum ops(rho)` over the paired targets (same workload), with a 95 % bootstrap interval (2000 resamples of targets, fixed seed). `queries` and `server ops` are the static-DH oracle's resource, outside `ops`.

| family | bits | n | d | arm | verified/requested | mean ops | min | max | S | ratio to rho (95 %) | oracle queries | server ops |
|:--|--:|--:|--:|:--|:--|--:|--:|--:|--:|:--|--:|--:|
| minus | 32 | 3221255537 |  | `rho-neg` | 8/8 | 37,182 | 14,306 | 72,960 | 0.655 | reference | 0 | 0 |
| minus | 32 | 3221255537 | 16 | `cheon-comb` | 8/8 | 57,459 | 42,502 | 68,568 | 1.012 | 1.55 [1.10, 2.40] | 15 | 679 |
| minus | 32 | 3221255537 | 16 | `cheon-naive` | 8/8 | 997,160 | 665,226 | 1,243,458 | 17.569 | 26.82 [18.66, 42.56] | 15 | 679 |
| minus | 36 | 51539573731 |  | `rho-neg` | 8/8 | 291,150 | 67,105 | 635,370 | 1.282 | reference | 0 | 0 |
| minus | 36 | 51539573731 | 30 | `cheon-comb` | 8/8 | 152,928 | 118,228 | 179,279 | 0.674 | 0.53 [0.34, 0.91] | 29 | 1,450 |
| minus | 36 | 51539573731 | 30 | `cheon-naive` | 8/8 | 3,221,598 | 2,347,739 | 3,885,036 | 14.191 | 11.07 [7.00, 19.48] | 29 | 1,450 |
| minus | 40 | 824632004309 |  | `rho-neg` | 8/8 | 600,853 | 71,008 | 1,447,215 | 0.662 | reference | 0 | 0 |
| minus | 40 | 824632004309 | 28 | `cheon-comb` | 8/8 | 796,596 | 552,699 | 1,010,803 | 0.877 | 1.33 [0.94, 2.31] | 27 | 1,529 |
| minus | 40 | 824632004309 | 28 | `cheon-naive` | 8/8 | 14,820,928 | 10,234,088 | 18,848,245 | 16.321 | 24.67 [17.44, 42.94] | 27 | 1,529 |
| minus | 44 | 13194139832747 |  | `rho-neg` | 8/8 | 3,587,651 | 2,167,521 | 4,813,900 | 0.988 | reference | 0 | 0 |
| minus | 44 | 13194139832747 | 74 | `cheon-comb` | 8/8 | 2,008,902 | 1,549,214 | 2,544,160 | 0.553 | 0.56 [0.47, 0.69] | 73 | 4,572 |
| minus | 44 | 13194139832747 | 74 | `cheon-naive` | 8/8 | 41,420,070 | 31,860,597 | 52,553,184 | 11.403 | 11.55 [9.73, 14.15] | 73 | 4,572 |
| minus | 48 | 211106219695673 |  | `rho-neg` | 8/8 | 18,257,647 | 2,318,297 | 32,193,596 | 1.257 | reference | 0 | 0 |
| minus | 48 | 211106219695673 | 56 | `cheon-comb` | 8/8 | 7,692,650 | 6,051,895 | 9,326,855 | 0.529 | 0.42 [0.30, 0.70] | 55 | 3,768 |
| minus | 48 | 211106219695673 | 56 | `cheon-naive` | 8/8 | 174,528,550 | 137,141,584 | 211,767,362 | 12.012 | 9.56 [6.84, 15.82] | 55 | 3,768 |
| minus | 52 | 3377699780661797 |  | `rho-neg` | 8/8 | 63,092,891 | 39,293,404 | 120,295,824 | 1.086 | reference | 0 | 0 |
| minus | 52 | 3377699780661797 | 164 | `cheon-comb` | 8/8 | 22,767,559 | 18,487,349 | 30,042,391 | 0.392 | 0.36 [0.29, 0.47] | 163 | 12,021 |
| minus | 52 | 3377699780661797 | 164 | `cheon-naive` | 8/8 | 423,476,381 | 343,790,499 | 558,913,697 | 7.286 | 6.71 [5.42, 8.68] | 163 | 12,021 |
| minus | 56 | 54043195377297631 |  | `rho-neg` | 8/8 | 150,869,722 | 56,296,249 | 231,679,321 | 0.649 | reference | 0 | 0 |
| minus | 56 | 54043195377297631 | 210 | `cheon-comb` | 8/8 | 97,260,020 | 72,880,495 | 122,760,558 | 0.418 | 0.64 [0.55, 0.81] | 209 | 17,216 |
| plus | 32 | 3221175689 |  | `rho-neg` | 8/8 | 51,226 | 11,357 | 93,796 | 0.903 | reference | 0 | 0 |
| plus | 32 | 3221175689 | 90 | `cheon-comb` | 8/8 | 85,205 | 55,481 | 113,909 | 1.501 | 1.66 [1.25, 2.35] | 179 | 8,301 |
| plus | 36 | 51539736719 |  | `rho-neg` | 8/8 | 213,101 | 114,238 | 346,591 | 0.939 | reference | 0 | 0 |
| plus | 36 | 51539736719 | 240 | `cheon-comb` | 8/8 | 204,430 | 148,050 | 248,849 | 0.900 | 0.96 [0.73, 1.34] | 479 | 23,950 |
| plus | 40 | 824633358973 |  | `rho-neg` | 8/8 | 1,063,868 | 214,874 | 2,075,309 | 1.172 | reference | 0 | 0 |
| plus | 40 | 824633358973 | 398 | `cheon-comb` | 8/8 | 680,969 | 388,117 | 979,888 | 0.750 | 0.64 [0.41, 1.09] | 795 | 44,321 |
| plus | 44 | 13194137032873 |  | `rho-neg` | 8/8 | 2,924,006 | 960,009 | 4,632,503 | 0.805 | reference | 0 | 0 |
| plus | 44 | 13194137032873 | 574 | `cheon-comb` | 8/8 | 2,345,071 | 1,509,216 | 2,845,511 | 0.646 | 0.80 [0.59, 1.21] | 1,147 | 72,261 |
| plus | 48 | 211106244944429 |  | `rho-neg` | 8/8 | 8,796,208 | 504,689 | 19,888,818 | 0.605 | reference | 0 | 0 |
| plus | 48 | 211106244944429 | 990 | `cheon-comb` | 8/8 | 5,802,606 | 3,173,674 | 9,349,644 | 0.399 | 0.66 [0.38, 1.28] | 1,979 | 137,293 |
| plus | 52 | 3377699707176467 |  | `rho-neg` | 8/8 | 61,615,032 | 6,037,131 | 119,465,498 | 1.060 | reference | 0 | 0 |
| plus | 52 | 3377699707176467 | 1836 | `cheon-comb` | 8/8 | 25,348,349 | 12,957,180 | 35,433,734 | 0.436 | 0.41 [0.29, 0.69] | 3,671 | 278,537 |
| plus | 56 | 54043195250336543 |  | `rho-neg` | 8/8 | 347,686,332 | 84,161,122 | 519,640,803 | 1.496 | reference | 0 | 0 |
| plus | 56 | 54043195250336543 | 2016 | `cheon-comb` | 8/8 | 83,618,997 | 49,002,962 | 131,761,087 | 0.360 | 0.24 [0.18, 0.38] | 4,031 | 329,030 |

### Table 1b. Failures, wrong answers and timeouts (preserved)

None: every recorded run verified.


### Table 2. Pricing grid (AGENTS.md 5a, extended to oracle queries)

Each cell is `sum (ops + w U + q Q) / sum (ops_rho + w U_rho)` over the paired targets: `U` the arm's unpriced native operations (every `*_uncharged` counter, rho's included), `w` in {0, 0.1, 1} GAE each, the same for both arms; `Q` the oracle queries, priced at `q` in {0, 0.1, 1} GAE each or at the server's own work (`server`); rho has no queries. The order is **resolved** only if every one of the 12 cells is on the same side of 1; `min` and `max` are the extreme cells, the interval is the 95 % bootstrap interval of the extreme cell on the side of 1.

| family | bits | arm | min cell | max cell | w=0,q=0 | w=1,q=1 | w=1,q=server | order | 95 % interval of the extreme cell excludes 1 |
|:--|--:|:--|--:|--:|--:|--:|--:|:--|:--|
| minus | 32 | `cheon-comb` | 1.384 (w=1, q=0) | 1.564 (w=0, q=server) | 1.545 | 1.385 | 1.393 | dearer than rho at all 12 prices | no (best cell lower bound 0.99) |
| minus | 32 | `cheon-naive` | 14.089 (w=1, q=0) | 26.837 (w=0, q=server) | 26.818 | 14.089 | 14.098 | dearer than rho at all 12 prices | yes (best cell lower bound 9.98) |
| minus | 36 | `cheon-comb` | 0.483 (w=1, q=0) | 0.530 (w=0, q=server) | 0.525 | 0.483 | 0.485 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.92) |
| minus | 36 | `cheon-naive` | 5.756 (w=1, q=0) | 11.070 (w=0, q=server) | 11.065 | 5.756 | 5.759 | dearer than rho at all 12 prices | yes (best cell lower bound 3.60) |
| minus | 40 | `cheon-comb` | 1.101 (w=1, q=0) | 1.328 (w=0, q=server) | 1.326 | 1.101 | 1.102 | dearer than rho at all 12 prices | no (best cell lower bound 0.79) |
| minus | 40 | `cheon-naive` | 12.776 (w=1, q=0) | 24.669 (w=0, q=server) | 24.666 | 12.776 | 12.777 | dearer than rho at all 12 prices | yes (best cell lower bound 9.16) |
| minus | 44 | `cheon-comb` | 0.465 (w=1, q=0) | 0.561 (w=0, q=server) | 0.560 | 0.465 | 0.466 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.68) |
| minus | 44 | `cheon-naive` | 5.958 (w=1, q=0) | 11.546 (w=0, q=server) | 11.545 | 5.958 | 5.959 | dearer than rho at all 12 prices | yes (best cell lower bound 4.99) |
| minus | 48 | `cheon-comb` | 0.351 (w=1, q=0) | 0.422 (w=0, q=server) | 0.421 | 0.351 | 0.351 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.69) |
| minus | 48 | `cheon-naive` | 4.920 (w=1, q=0) | 9.559 (w=0, q=server) | 9.559 | 4.920 | 4.920 | dearer than rho at all 12 prices | yes (best cell lower bound 3.49) |
| minus | 52 | `cheon-comb` | 0.271 (w=1, q=0) | 0.361 (w=0, q=server) | 0.361 | 0.271 | 0.271 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.47) |
| minus | 52 | `cheon-naive` | 3.446 (w=1, q=0) | 6.712 (w=0, q=server) | 6.712 | 3.446 | 3.446 | dearer than rho at all 12 prices | yes (best cell lower bound 2.77) |
| minus | 56 | `cheon-comb` | 0.484 (w=1, q=0) | 0.645 (w=0, q=server) | 0.645 | 0.484 | 0.484 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.81) |
| plus | 32 | `cheon-comb` | 1.025 (w=1, q=0) | 1.825 (w=0, q=server) | 1.663 | 1.027 | 1.106 | dearer than rho at all 12 prices | no (best cell lower bound 0.77) |
| plus | 36 | `cheon-comb` | 0.591 (w=1, q=0) | 1.072 (w=0, q=server) | 0.959 | 0.592 | 0.648 | unresolved |  |
| plus | 40 | `cheon-comb` | 0.388 (w=1, q=0) | 0.682 (w=0, q=server) | 0.640 | 0.388 | 0.409 | cheaper than rho at all 12 prices | no (worst cell upper bound 1.21) |
| plus | 44 | `cheon-comb` | 0.486 (w=1, q=0) | 0.827 (w=0, q=server) | 0.802 | 0.486 | 0.499 | cheaper than rho at all 12 prices | no (worst cell upper bound 1.26) |
| plus | 48 | `cheon-comb` | 0.406 (w=1, q=0) | 0.675 (w=0, q=server) | 0.660 | 0.406 | 0.414 | cheaper than rho at all 12 prices | no (worst cell upper bound 1.31) |
| plus | 52 | `cheon-comb` | 0.241 (w=1, q=0) | 0.416 (w=0, q=server) | 0.411 | 0.241 | 0.243 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.65) |
| plus | 56 | `cheon-comb` | 0.142 (w=1, q=0) | 0.241 (w=0, q=server) | 0.241 | 0.142 | 0.142 | cheaper than rho at all 12 prices | yes (worst cell upper bound 0.39) |

### Table 2b. Gap to the matched rho, pooled over tier medium (36 to 56 bits), at every price

Pooled paired ratio `sum cost(Cheon) / sum cost(rho)` over every verified (curve, target) pair of 36 to 56 bits (the 32-bit toy size is left out), with a 95 % bootstrap interval over the pairs; rows are the price `w` of an unpriced native operation (both arms), columns the price `q` of an oracle query. This is a distance to rho, not a pass or fail.

`cheon-comb` on the `minus` curves, 48 pairs:

| w \ q | 0 | 0.1 | 1 | server |
|--:|--:|--:|--:|--:|
| 0 | 0.552 [0.455, 0.651] | 0.552 [0.458, 0.653] | 0.552 [0.461, 0.653] | 0.552 [0.454, 0.658] |
| 0.1 | 0.528 [0.436, 0.622] | 0.528 [0.438, 0.624] | 0.528 [0.441, 0.625] | 0.528 [0.434, 0.629] |
| 1 | 0.418 [0.347, 0.492] | 0.418 [0.348, 0.494] | 0.418 [0.349, 0.495] | 0.418 [0.346, 0.499] |

`cheon-naive` on the `minus` curves, 40 pairs:

| w \ q | 0 | 0.1 | 1 | server |
|--:|--:|--:|--:|--:|
| 0 | 7.660 [6.332, 9.928] | 7.660 [6.367, 9.706] | 7.660 [6.392, 9.853] | 7.660 [6.407, 9.875] |
| 0.1 | 6.983 [5.772, 9.051] | 6.983 [5.805, 8.848] | 6.983 [5.827, 8.983] | 6.984 [5.841, 9.003] |
| 1 | 3.938 [3.255, 5.104] | 3.938 [3.272, 4.989] | 3.938 [3.286, 5.067] | 3.938 [3.292, 5.078] |

`cheon-comb` on the `plus` curves, 48 pairs:

| w \ q | 0 | 0.1 | 1 | server |
|--:|--:|--:|--:|--:|
| 0 | 0.279 [0.216, 0.419] | 0.279 [0.219, 0.416] | 0.279 [0.219, 0.424] | 0.282 [0.221, 0.424] |
| 0.1 | 0.259 [0.200, 0.388] | 0.259 [0.203, 0.385] | 0.259 [0.203, 0.393] | 0.261 [0.204, 0.392] |
| 1 | 0.165 [0.129, 0.247] | 0.165 [0.130, 0.245] | 0.165 [0.130, 0.250] | 0.166 [0.131, 0.248] |


### Table 3. Unpriced counters and the oracle's price share (means over verified runs)

| family | bits | arm | unpriced counters (mean count) | oracle queries | server ops | server ops as % of ops |
|:--|--:|:--|:--|--:|--:|--:|
| minus | 32 | `rho-neg` | canonicalisations_uncharged=36,783 | 0 | 0 | 0.000 |
| minus | 32 | `cheon-comb` | scalar_mulmod_uncharged=22,465, table_insert_uncharged=14,195, table_lookup_uncharged=8,272 | 15 | 679 | 1.181 |
| minus | 32 | `cheon-naive` | scalar_mulmod_uncharged=22,465, table_insert_uncharged=14,195, table_lookup_uncharged=8,272 | 15 | 679 | 0.068 |
| minus | 36 | `rho-neg` | canonicalisations_uncharged=290,741 | 0 | 0 | 0.000 |
| minus | 36 | `cheon-comb` | scalar_mulmod_uncharged=64,002, table_insert_uncharged=41,455, table_lookup_uncharged=22,549 | 29 | 1,450 | 0.948 |
| minus | 36 | `cheon-naive` | scalar_mulmod_uncharged=64,002, table_insert_uncharged=41,455, table_lookup_uncharged=22,549 | 29 | 1,450 | 0.045 |
| minus | 40 | `rho-neg` | canonicalisations_uncharged=600,429 | 0 | 0 | 0.000 |
| minus | 40 | `cheon-comb` | scalar_mulmod_uncharged=263,064, table_insert_uncharged=171,620, table_lookup_uncharged=91,446 | 27 | 1,529 | 0.192 |
| minus | 40 | `cheon-naive` | scalar_mulmod_uncharged=263,064, table_insert_uncharged=171,620, table_lookup_uncharged=91,446 | 27 | 1,529 | 0.010 |
| minus | 44 | `rho-neg` | canonicalisations_uncharged=3,587,216 | 0 | 0 | 0.000 |
| minus | 44 | `cheon-comb` | scalar_mulmod_uncharged=664,446, table_insert_uncharged=422,264, table_lookup_uncharged=242,184 | 73 | 4,572 | 0.228 |
| minus | 44 | `cheon-naive` | scalar_mulmod_uncharged=664,446, table_insert_uncharged=422,264, table_lookup_uncharged=242,184 | 73 | 4,572 | 0.011 |
| minus | 48 | `rho-neg` | canonicalisations_uncharged=18,257,199 | 0 | 0 | 0.000 |
| minus | 48 | `cheon-comb` | scalar_mulmod_uncharged=2,554,000, table_insert_uncharged=1,941,594, table_lookup_uncharged=612,408 | 55 | 3,768 | 0.049 |
| minus | 48 | `cheon-naive` | scalar_mulmod_uncharged=2,554,000, table_insert_uncharged=1,941,594, table_lookup_uncharged=612,408 | 55 | 3,768 | 0.002 |
| minus | 52 | `rho-neg` | canonicalisations_uncharged=63,092,430 | 0 | 0 | 0.000 |
| minus | 52 | `cheon-comb` | scalar_mulmod_uncharged=5,696,858, table_insert_uncharged=4,538,265, table_lookup_uncharged=1,158,594 | 163 | 12,021 | 0.053 |
| minus | 52 | `cheon-naive` | scalar_mulmod_uncharged=5,696,858, table_insert_uncharged=4,538,265, table_lookup_uncharged=1,158,594 | 163 | 12,021 | 0.003 |
| minus | 56 | `rho-neg` | canonicalisations_uncharged=150,869,245 | 0 | 0 | 0.000 |
| minus | 56 | `cheon-comb` | scalar_mulmod_uncharged=24,342,158, table_insert_uncharged=16,042,102, table_lookup_uncharged=8,300,058 | 209 | 17,216 | 0.018 |
| plus | 32 | `rho-neg` | canonicalisations_uncharged=50,826 | 0 | 0 | 0.000 |
| plus | 32 | `cheon-comb` | fp2_mul_uncharged=8,972, scalar_mulmod_uncharged=1,448, table_insert_uncharged=5,993, table_lookup_uncharged=2,981 | 179 | 8,301 | 9.743 |
| plus | 36 | `rho-neg` | canonicalisations_uncharged=212,694 | 0 | 0 | 0.000 |
| plus | 36 | `cheon-comb` | fp2_mul_uncharged=21,749, scalar_mulmod_uncharged=3,848, table_insert_uncharged=14,671, table_lookup_uncharged=7,080 | 479 | 23,950 | 11.715 |
| plus | 40 | `rho-neg` | canonicalisations_uncharged=1,063,446 | 0 | 0 | 0.000 |
| plus | 40 | `cheon-comb` | fp2_mul_uncharged=68,840, scalar_mulmod_uncharged=6,376, table_insert_uncharged=45,539, table_lookup_uncharged=23,303 | 795 | 44,321 | 6.509 |
| plus | 44 | `rho-neg` | canonicalisations_uncharged=2,923,570 | 0 | 0 | 0.000 |
| plus | 44 | `cheon-comb` | fp2_mul_uncharged=244,588, scalar_mulmod_uncharged=9,192, table_insert_uncharged=151,637, table_lookup_uncharged=92,953 | 1,147 | 72,261 | 3.081 |
| plus | 48 | `rho-neg` | canonicalisations_uncharged=8,795,761 | 0 | 0 | 0.000 |
| plus | 48 | `cheon-comb` | fp2_mul_uncharged=660,144, scalar_mulmod_uncharged=15,848, table_insert_uncharged=461,810, table_lookup_uncharged=198,336 | 1,979 | 137,293 | 2.366 |
| plus | 52 | `rho-neg` | canonicalisations_uncharged=61,614,573 | 0 | 0 | 0.000 |
| plus | 52 | `cheon-comb` | fp2_mul_uncharged=2,139,231, scalar_mulmod_uncharged=29,384, table_insert_uncharged=1,356,401, table_lookup_uncharged=782,832 | 3,671 | 278,537 | 1.099 |
| plus | 56 | `rho-neg` | canonicalisations_uncharged=347,685,862 | 0 | 0 | 0.000 |
| plus | 56 | `cheon-comb` | fp2_mul_uncharged=7,497,778, scalar_mulmod_uncharged=32,264, table_insert_uncharged=5,177,607, table_lookup_uncharged=2,320,173 | 4,031 | 329,030 | 0.393 |

### Table 4. Positive control: the registered methods against PR #2422's driver runs

For every (curve, arm, target) the earlier driver recorded, the ecbench method's `ops`, per-phase counts and uncharged counters are compared. Identical: **120**; different: **0**; no counterpart in this session: 6.


Resolved `d` equals the frozen `curves.json` `d` on every verified run: **true** (0 mismatches).

### Table 5. Sealed bounds (tier medium, 36 to 56 bits), read from the bound records

`gae = C r^alpha` fitted on every verified run (least squares, two-stage bootstrap). `ops x floor` is the mean of `S` over the generic floor `sqrt(pi/2A)`. `queries alpha` is the same fit for oracle queries; blank for rho.

| bound | arm | weight | sizes | runs | alpha (95 %) | log2 C | ops x floor (95 %) | queries alpha (95 %) | bound id |
|:--|:--|--:|--:|--:|:--|--:|:--|:--|:--|
| minus-comb | `cheon.pminus1_comb` | charged | 6 | 48 | 0.450 [0.406, 0.473] | 1.39 | 0.648 [0.514, 0.818] | 0.154 [0.092, 0.213] | `ECBND1hc7f1fc29171b` |
| minus-comb-w0.1 | `cheon.pminus1_comb` | 0.1 | 6 | 48 | 0.448 [0.405, 0.471] | 1.57 | 0.690 [0.543, 0.873] | 0.154 [0.092, 0.213] | `ECBND1h99cf9dff484c` |
| minus-comb-w1 | `cheon.pminus1_comb` | 1 | 6 | 48 | 0.437 [0.396, 0.460] | 2.71 | 1.074 [0.809, 1.376] | 0.154 [0.092, 0.213] | `ECBND1h2fdaa885bf06` |
| minus-naive | `cheon.pminus1_naive` | charged | 5 | 40 | 0.441 [0.388, 0.497] | 6.09 | 13.814 [10.619, 16.929] | 0.150 [0.074, 0.221] | `ECBND1hce4f9adaff6b` |
| minus-naive-rho | `rho.negation` | charged | 5 | 40 | 0.524 [0.417, 0.685] | -1.28 | 1.190 [0.904, 1.496] |  | `ECBND1h7ccf77f32c59` |
| minus-naive-w0.1 | `cheon.pminus1_naive` | 0.1 | 5 | 40 | 0.441 [0.387, 0.496] | 6.10 | 13.861 [10.651, 16.987] | 0.150 [0.074, 0.221] | `ECBND1h3b205a6d0540` |
| minus-naive-w1 | `cheon.pminus1_naive` | 1 | 5 | 40 | 0.440 [0.387, 0.496] | 6.19 | 14.278 [10.953, 17.522] | 0.150 [0.074, 0.221] | `ECBND1hb4ad379d43d8` |
| minus-rho | `rho.negation` | charged | 6 | 48 | 0.496 [0.431, 0.611] | -0.16 | 1.114 [0.846, 1.408] |  | `ECBND1hc107cc81f0b3` |
| minus-rho-w0.1 | `rho.negation` | 0.1 | 6 | 48 | 0.496 [0.431, 0.611] | -0.02 | 1.225 [0.931, 1.549] |  | `ECBND1hd4cfe6663583` |
| minus-rho-w1 | `rho.negation` | 1 | 6 | 48 | 0.496 [0.431, 0.611] | 0.84 | 2.227 [1.693, 2.816] |  | `ECBND1ha7a3cdc54301` |
| plus-comb | `cheon.pplus1_comb` | charged | 6 | 48 | 0.428 [0.389, 0.460] | 2.36 | 0.657 [0.483, 0.845] | 0.163 [0.146, 0.189] | `ECBND1hbab44fbc5232` |
| plus-comb-w0.1 | `cheon.pplus1_comb` | 0.1 | 6 | 48 | 0.427 [0.389, 0.459] | 2.41 | 0.670 [0.493, 0.864] | 0.163 [0.146, 0.189] | `ECBND1hc2cfee03ebf6` |
| plus-comb-w1 | `cheon.pplus1_comb` | 1 | 6 | 48 | 0.425 [0.390, 0.455] | 2.76 | 0.795 [0.580, 1.031] | 0.163 [0.146, 0.189] | `ECBND1h952774e21c83` |
| plus-rho | `rho.negation` | charged | 6 | 48 | 0.513 [0.386, 0.636] | -0.94 | 1.143 [0.840, 1.457] |  | `ECBND1he00f1952b198` |
| plus-rho-w0.1 | `rho.negation` | 0.1 | 6 | 48 | 0.513 [0.386, 0.636] | -0.80 | 1.257 [0.924, 1.602] |  | `ECBND1heb8f0d5c8a05` |
| plus-rho-w1 | `rho.negation` | 1 | 6 | 48 | 0.513 [0.386, 0.636] | 0.06 | 2.285 [1.679, 2.913] |  | `ECBND1hcc568213272c` |
