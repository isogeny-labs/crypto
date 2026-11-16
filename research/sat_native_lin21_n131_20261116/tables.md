# Tables (written by summary.rs; every number [pcc:unverified])

## Small-field series (curve y^2 + xy = x^3 + x^2 + 1, n = 3l + 2, non-decomposable uniform targets)

Arms: `merged` = fastcdcl.rs unmodified; `mw` = the copy with W = 1 and both switches off (control); `del` = keep the 64 newest learnt clauses; `leaf` = learning-free leaf; `delleaf` = both; `merged-b` = second run of `merged` (A/A). `scan` = the lin21 harness's plain scan; `fastleaf` = generic-width port of the baseline leaf of scan_leaf.rs (n odd only).

| l | n | targets | arm | median wall ms | median conflicts | wall / scan | wall / fastleaf | clause-prop share | conflicts = merged (targets) |
|---|---|---|---|---|---|---|---|---|---|
| 5 | 17 | 16 | scan | 0.012 | - | 1 | 2.171 | - | - |
| 5 | 17 | 16 | fastleaf | 0.006 (195.9 ns/leaf) | - | 0.461 | 1 | - | - |
| 5 | 17 | 16 | merged | 0.795 | 46 | 64.44 | 139.9 | 5.1 % | 16 of 16 |
| 5 | 17 | 16 | mw | 0.779 | 46 | 63.13 | 137.1 | 5.2 % | 16 of 16 |
| 5 | 17 | 16 | del | 0.771 | 46 | 62.48 | 135.7 | 5.3 % | 16 of 16 |
| 5 | 17 | 16 | leaf | 0.658 | 46 | 53.32 | 115.8 | 3.6 % | 16 of 16 |
| 5 | 17 | 16 | delleaf | 0.697 | 46 | 56.50 | 122.7 | 3.9 % | 16 of 16 |
| 5 | 17 | 16 | merged-b | 0.771 | 46 | 62.48 | 135.7 | 5.4 % | 16 of 16 |
| 6 | 20 | 16 | scan | 0.058 | - | 1 | n/a | - | - |
| 6 | 20 | 16 | merged | 1.354 | 99 | 23.22 | n/a | 4.4 % | 16 of 16 |
| 6 | 20 | 16 | mw | 1.376 | 99 | 23.61 | n/a | 4.5 % | 16 of 16 |
| 6 | 20 | 16 | del | 1.295 | 99 | 22.22 | n/a | 4.3 % | 16 of 16 |
| 6 | 20 | 16 | leaf | 1.205 | 99 | 20.67 | n/a | 2.7 % | 16 of 16 |
| 6 | 20 | 16 | delleaf | 1.259 | 99 | 21.59 | n/a | 2.6 % | 16 of 16 |
| 6 | 20 | 16 | merged-b | 1.342 | 99 | 23.03 | n/a | 4.5 % | 16 of 16 |
| 7 | 23 | 16 | scan | 0.171 | - | 1 | 10.80 | - | - |
| 7 | 23 | 16 | fastleaf | 0.016 (148.3 ns/leaf) | - | 0.093 | 1 | - | - |
| 7 | 23 | 16 | merged | 2.745 | 182 | 16.02 | 173.0 | 4.8 % | 16 of 16 |
| 7 | 23 | 16 | mw | 2.627 | 182 | 15.33 | 165.5 | 5.0 % | 16 of 16 |
| 7 | 23 | 16 | del | 2.667 | 182 | 15.57 | 168.1 | 4.5 % | 16 of 16 |
| 7 | 23 | 16 | leaf | 2.418 | 182 | 14.11 | 152.4 | 2.7 % | 16 of 16 |
| 7 | 23 | 16 | delleaf | 2.484 | 182 | 14.50 | 156.5 | 2.7 % | 16 of 16 |
| 7 | 23 | 16 | merged-b | 2.941 | 182 | 17.17 | 185.4 | 5.1 % | 16 of 16 |
| 8 | 26 | 16 | scan | 0.453 | - | 1 | n/a | - | - |
| 8 | 26 | 16 | merged | 7.310 | 382 | 16.14 | n/a | 4.6 % | 16 of 16 |
| 8 | 26 | 16 | mw | 7.335 | 382 | 16.19 | n/a | 4.5 % | 16 of 16 |
| 8 | 26 | 16 | del | 7.419 | 382 | 16.38 | n/a | 3.4 % | 16 of 16 |
| 8 | 26 | 16 | leaf | 7.056 | 382 | 15.58 | n/a | 2.0 % | 16 of 16 |
| 8 | 26 | 16 | delleaf | 6.889 | 382 | 15.21 | n/a | 2.1 % | 16 of 16 |
| 8 | 26 | 16 | merged-b | 7.340 | 382 | 16.20 | n/a | 4.6 % | 16 of 16 |
| 9 | 29 | 16 | scan | 1.090 | - | 1 | 8.807 | - | - |
| 9 | 29 | 16 | fastleaf | 0.124 (239.5 ns/leaf) | - | 0.114 | 1 | - | - |
| 9 | 29 | 16 | merged | 15.45 | 774 | 14.17 | 124.8 | 5.6 % | 16 of 16 |
| 9 | 29 | 16 | mw | 15.62 | 774 | 14.32 | 126.1 | 5.6 % | 16 of 16 |
| 9 | 29 | 16 | del | 15.48 | 774 | 14.20 | 125.0 | 3.3 % | 16 of 16 |
| 9 | 29 | 16 | leaf | 14.62 | 774 | 13.41 | 118.1 | 2.0 % | 16 of 16 |
| 9 | 29 | 16 | delleaf | 14.82 | 774 | 13.59 | 119.7 | 2.0 % | 16 of 16 |
| 9 | 29 | 16 | merged-b | 15.32 | 774 | 14.05 | 123.8 | 5.5 % | 16 of 16 |
| 10 | 32 | 16 | scan | 2.470 | - | 1 | n/a | - | - |
| 10 | 32 | 16 | merged | 39.42 | 1527 | 15.96 | n/a | 6.2 % | 16 of 16 |
| 10 | 32 | 16 | mw | 38.99 | 1527 | 15.78 | n/a | 6.3 % | 16 of 16 |
| 10 | 32 | 16 | del | 38.31 | 1527 | 15.51 | n/a | 2.7 % | 16 of 16 |
| 10 | 32 | 16 | leaf | 36.00 | 1527 | 14.57 | n/a | 1.6 % | 16 of 16 |
| 10 | 32 | 16 | delleaf | 36.73 | 1527 | 14.87 | n/a | 1.7 % | 16 of 16 |
| 10 | 32 | 16 | merged-b | 39.22 | 1527 | 15.88 | n/a | 6.4 % | 16 of 16 |
| 11 | 35 | 16 | scan | 5.760 | - | 1 | 8.701 | - | - |
| 11 | 35 | 16 | fastleaf | 0.662 (329.5 ns/leaf) | - | 0.115 | 1 | - | - |
| 11 | 35 | 16 | merged | 93.22 | 3066 | 16.19 | 140.8 | 8.4 % | 16 of 16 |
| 11 | 35 | 16 | mw | 91.10 | 3066 | 15.82 | 137.6 | 8.3 % | 16 of 16 |
| 11 | 35 | 16 | del | 85.66 | 3066 | 14.87 | 129.4 | 2.5 % | 16 of 16 |
| 11 | 35 | 16 | leaf | 83.56 | 3066 | 14.51 | 126.2 | 1.6 % | 16 of 16 |
| 11 | 35 | 16 | delleaf | 83.54 | 3066 | 14.50 | 126.2 | 1.6 % | 16 of 16 |
| 11 | 35 | 16 | merged-b | 94.82 | 3066 | 16.46 | 143.2 | 8.2 % | 16 of 16 |
| 12 | 38 | 16 | scan | 12.84 | - | 1 | n/a | - | - |
| 12 | 38 | 16 | merged | 217.8 | 6150 | 16.97 | n/a | 11.8 % | 16 of 16 |
| 12 | 38 | 16 | mw | 219.9 | 6150 | 17.13 | n/a | 11.6 % | 16 of 16 |
| 12 | 38 | 16 | del | 197.9 | 6150 | 15.42 | n/a | 2.2 % | 16 of 16 |
| 12 | 38 | 16 | leaf | 190.0 | 6150 | 14.80 | n/a | 1.4 % | 16 of 16 |
| 12 | 38 | 16 | delleaf | 196.5 | 6150 | 15.30 | n/a | 1.4 % | 16 of 16 |
| 12 | 38 | 16 | merged-b | 218.9 | 6150 | 17.05 | n/a | 11.5 % | 16 of 16 |
| 13 | 41 | 6 | scan | 28.63 | - | 1 | 8.814 | - | - |
| 13 | 41 | 6 | fastleaf | 3.248 (396.4 ns/leaf) | - | 0.113 | 1 | - | - |
| 13 | 41 | 6 | merged | 530.3 | 12351 | 18.52 | 163.3 | 20.8 % | 6 of 6 |
| 13 | 41 | 6 | mw | 520.2 | 12351 | 18.17 | 160.2 | 21.1 % | 6 of 6 |
| 13 | 41 | 6 | del | 415.2 | 12351 | 14.50 | 127.8 | 2.2 % | 6 of 6 |
| 13 | 41 | 6 | leaf | 401.9 | 12351 | 14.04 | 123.8 | 1.5 % | 6 of 6 |
| 13 | 41 | 6 | delleaf | 396.8 | 12351 | 13.86 | 122.2 | 1.5 % | 6 of 6 |
| 13 | 41 | 6 | merged-b | 526.9 | 12351 | 18.41 | 162.2 | 20.6 % | 6 of 6 |

A/A band, per-target wall ratio `merged-b` / `merged`:

| l | median | min | max |
|---|---|---|---|
| 5 | 0.991 | 0.887 | 1.295 |
| 6 | 0.998 | 0.661 | 1.401 |
| 7 | 0.971 | 0.891 | 1.573 |
| 8 | 0.983 | 0.358 | 1.041 |
| 9 | 0.990 | 0.798 | 1.147 |
| 10 | 0.988 | 0.788 | 1.208 |
| 11 | 1.012 | 0.796 | 1.068 |
| 12 | 0.993 | 0.937 | 1.152 |
| 13 | 0.998 | 0.983 | 1.112 |

Conflict count equal to `merged` on ALL targets (decomposable ones included), per arm: del 176 of 176; delleaf 174 of 176; leaf 174 of 176; merged-b 176 of 176; mw 176 of 176.

ECC2K-130 solver rows on uniform targets: 144 rows, 0 with a learnt clause; largest clause-propagation share 1.27 %.

## Slopes of log2(solver wall / reference wall) per unit of l (least squares on cell medians)

| arm | vs fastleaf, l = 9, 11, 13 | ratio at l = 13 (fastleaf) | vs scan, l = 9..13 | ratio at l = 13 (scan) |
|---|---|---|---|---|
| merged | +0.097 | 163.3 | +0.086 | 18.52 |
| mw | +0.086 | 160.2 | +0.080 | 18.17 |
| del | +0.008 | 127.8 | +0.005 | 14.50 |
| leaf | +0.017 | 123.8 | +0.015 | 14.04 |
| delleaf | +0.007 | 122.2 | +0.010 | 13.86 |

**Small-field decision by the pre-registered rule: inconclusive.**

Checks, small-field: 1232 scan and solver rows compared with the pair oracle, 0 verdicts differ; 252 SAT models, 0 fail a model check; fast-leaf port verdict equals the pair oracle on 90 of 90 rows.

## ECC2K-130 series (y^2 + xy = x^3 + 1 over F_2[z]/(z^131 + z^8 + z^3 + z^2 + 1), V = degree < d)

Uniform (non-decomposable) targets. `A` = nonzero abscissae in V that lift; pair count = C(A, 2) (the panel's unit); `pairs` enumerates point pairs (both signs, i <= j).

| d | targets | A | C(A,2) | median conflicts | conflicts / A | conflicts / C(A,2) | solver ms | fastleaf ms (ns/leaf) | pairs ms (point pairs) | solver / fastleaf | solver / pairs | A/A solver-b / solver median [min, max] |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 6 | 8 | 34 | 561 | 98 | 2.882 | 0.17469 | 22.57 | 0.083 (1206) | 1.381 (2415; 8 targets) | 271.1 | 16.34 | 0.944 [0.904, 1.287] |
| 7 | 8 | 74 | 2701 | 202 | 2.730 | 0.07479 | 43.02 | 0.177 (1191) | 4.147 (11175; 8 targets) | 242.4 | 10.37 | 0.980 [0.946, 1.061] |
| 8 | 8 | 131 | 8515 | 387 | 2.954 | 0.04545 | 78.70 | 0.335 (1272) | 11.60 (34716; 8 targets) | 235.2 | 6.785 | 1.002 [0.953, 1.234] |
| 9 | 8 | 252 | 31626 | 764 | 3.032 | 0.02416 | 163.0 | 0.667 (1322) | 40.25 (127765; 8 targets) | 244.3 | 4.051 | 0.932 [0.798, 1.004] |
| 10 | 8 | 519 | 134421 | 1543 | 2.973 | 0.01148 | 315.3 | 1.480 (1425) | 164.5 (540280; 8 targets) | 213.0 | 1.916 | 0.947 [0.893, 1.128] |
| 11 | 8 | 1023 | 522753 | 3071 | 3.002 | 0.00587 | 640.6 | 3.097 (1513) | 645.8 (2096128; 8 targets) | 206.9 | 0.992 | 1.007 [0.888, 1.213] |
| 12 | 8 | 2002 | 2003001 | 6098 | 3.046 | 0.00304 | 1268 | 6.451 (1611) | 2430 (8022015; 8 targets) | 196.6 | 0.522 | 1.030 [0.882, 1.121] |
| 13 | 8 | 4137 | 8555316 | 12329 | 2.980 | 0.00144 | 2605 | 13.94 (1685) | 10463 (34241950; 8 targets) | 186.9 | 0.249 | 1.002 [0.904, 1.050] |
| 14 | 8 | 8175 | 33411225 | 24559 | 3.004 | 0.00074 | 5779 | 30.41 (1860) | 41889 (133685776; 1 targets) | 190.0 | 0.138 | 0.992 [0.852, 1.080] |

Slopes of log2(median wall) per unit of d over the cells above: solver +0.996, fastleaf +1.062, pairs +1.881.

Checks, ECC2K-130: 90 targets; 0 targets with an arm whose verdict differs from the expected one (uniform: UNSAT on every arm, planted: SAT on every arm); planted targets SAT on all arms: 18 of 18; 36 solver models, 0 fail the lift-and-sum check; rows with an unverified scan/pair hit: 0.

## Differential tests (seeds 991 and 992, not used for measured rows)

- n <= 63, (n, l) = (10,3), (12,3), (13,3), (14,4), (15,4), (17,5), (20,6): 2800 fresh instances (1400 planted, 1447 decomposable by the pair oracle); arms mw, del, del1, leaf, delleaf against unmodified fastcdcl and the pair oracle: 0 verdict mismatches; 7235 models checked, 0 bad, 0 non-liftable on admissible targets; control (`mw`) conflict counts differing from fastcdcl: 0; two-word control differing: 0; instances whose conflict count differs from fastcdcl per arm [mw, del, del1, leaf, delleaf]: [0, 0, 0, 1, 1].
- multi-word, (W, n, d) from (1, 17, 5) to (3, 131, 6), 11 cells: 210 targets (105 decomposable); solver arms mw, delleaf, del1, leaf against point-pair enumeration and the fast-leaf port: 0 verdict mismatches; 420 models checked, 0 bad; 0 unverified hits.
- selfcheck: {"test":"selfcheck","clmul_backend":"pclmulqdq","clmul_matches_portable":true,"f131_irreducible":true,"field_ok":true,"A_d4_to_d10":[10, 17, 34, 74, 131, 252, 519],"panel_A":[10, 17, 34, 74, 131, 252, 519],"A_matches_panel":true}
