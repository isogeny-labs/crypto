# Tables (derived; Rust `fit/`)

Primary reference: **pooled** (142 runs). 

## Cover seeds 17-24 (n=8), primary reference (pooled)

| quantity | point | 95% interval | verdict | how far below rho (x) |
|:--|--:|--:|:--|--:|
| S/rho U1 | 0.00647 | [0.00545, 0.00799] | below | 154.5 [125.1, 183.5] |
| S/rho U2 | 0.01556 | [0.01310, 0.01921] | below | 64.3 [52.1, 76.4] |
| S+/rho U1 (rho unpriced) | 0.02685 | [0.02429, 0.02970] | below | 37.2 [33.7, 41.2] |
| S+/rho U2 (rho unpriced) | 0.06452 | [0.05837, 0.07137] | below | 15.5 [14.0, 17.1] |
| S+/rhoS+ U1 | 0.02261 | [0.02045, 0.02501] | below | 44.2 [40.0, 48.9] |
| S+/rhoS+ U2 | 0.04447 | [0.04023, 0.04919] | below | 22.5 [20.3, 24.9] |
| S/rho U1 (ratio of means) | 0.00671 | [0.00549, 0.00843] | below | 148.9 [118.7, 182.2] |
| S/rho U2 (ratio of means) | 0.01614 | [0.01319, 0.02025] | below | 62.0 [49.4, 75.8] |

### Section 5a band (sieve/rho; native priced at w GAE per operation on both arms)

| layer | unit | w=0 | w=0.1 | w=1 | order holds |
|:--|:--|--:|--:|--:|:--|
| A: V2 upper bound (cover adds+lookups+enum adds+unemitted; rho native+fp_adds) | U1 | 0.0065 [0.0054, 0.0080] | 0.1071 [0.0968, 0.1188] | 0.1214 [0.1097, 0.1346] | yes (below) |
| A: V2 upper bound (cover adds+lookups+enum adds+unemitted; rho native+fp_adds) | U2 | 0.0156 [0.0131, 0.0192] | 0.1165 [0.1053, 0.1293] | 0.1226 [0.1108, 0.1359] | yes (below) |
| B: as emitted (cover lookups; rho 6 native counters) | U1 | 0.0065 [0.0054, 0.0080] | 0.0093 [0.0085, 0.0103] | 0.0107 [0.0097, 0.0118] | yes (below) |
| B: as emitted (cover lookups; rho 6 native counters) | U2 | 0.0156 [0.0131, 0.0192] | 0.0119 [0.0108, 0.0132] | 0.0110 [0.0100, 0.0122] | yes (below) |

## Cover all 16 seeds (n=16), primary reference (pooled)

| quantity | point | 95% interval | verdict | how far below rho (x) |
|:--|--:|--:|:--|--:|
| S/rho U1 | 0.00655 | [0.00581, 0.00746] | below | 152.8 [134.0, 172.1] |
| S/rho U2 | 0.01573 | [0.01396, 0.01793] | below | 63.6 [55.8, 71.6] |
| S+/rho U1 (rho unpriced) | 0.02679 | [0.02436, 0.02954] | below | 37.3 [33.9, 41.0] |
| S+/rho U2 (rho unpriced) | 0.06439 | [0.05855, 0.07098] | below | 15.5 [14.1, 17.1] |
| S+/rhoS+ U1 | 0.02256 | [0.02051, 0.02487] | below | 44.3 [40.2, 48.7] |
| S+/rhoS+ U2 | 0.04438 | [0.04035, 0.04892] | below | 22.5 [20.4, 24.8] |
| S/rho U1 (ratio of means) | 0.00670 | [0.00587, 0.00770] | below | 149.3 [129.8, 170.2] |
| S/rho U2 (ratio of means) | 0.01610 | [0.01412, 0.01852] | below | 62.1 [54.0, 70.8] |

### Section 5a band (sieve/rho; native priced at w GAE per operation on both arms)

| layer | unit | w=0 | w=0.1 | w=1 | order holds |
|:--|:--|--:|--:|--:|:--|
| A: V2 upper bound (cover adds+lookups+enum adds+unemitted; rho native+fp_adds) | U1 | 0.0065 [0.0058, 0.0075] | 0.1069 [0.0973, 0.1178] | 0.1212 [0.1103, 0.1336] | yes (below) |
| A: V2 upper bound (cover adds+lookups+enum adds+unemitted; rho native+fp_adds) | U2 | 0.0157 [0.0140, 0.0179] | 0.1163 [0.1058, 0.1282] | 0.1223 [0.1113, 0.1348] | yes (below) |
| B: as emitted (cover lookups; rho 6 native counters) | U1 | 0.0065 [0.0058, 0.0075] | 0.0093 [0.0085, 0.0102] | 0.0106 [0.0097, 0.0117] | yes (below) |
| B: as emitted (cover lookups; rho 6 native counters) | U2 | 0.0157 [0.0140, 0.0179] | 0.0119 [0.0109, 0.0130] | 0.0110 [0.0100, 0.0121] | yes (below) |

## New reference runs (measured)

| seed | run | steps/sqrt(l) | s_harness | s_tuned |
|--:|--:|--:|--:|--:|
| 17 | 0 | 0.3257 | 0.3256 | 0.1355 |
| 17 | 1 | 0.7690 | 0.7689 | 0.3200 |
| 18 | 0 | 2.1009 | 2.1005 | 0.8741 |
| 18 | 1 | 0.6283 | 0.6282 | 0.2614 |
| 19 | 0 | 0.5448 | 0.5448 | 0.2267 |
| 19 | 1 | 1.0041 | 1.0039 | 0.4178 |
| 20 | 0 | 0.7318 | 0.7316 | 0.3045 |
| 20 | 1 | 0.8643 | 0.8641 | 0.3596 |
| 21 | 0 | 0.5238 | 0.5237 | 0.2179 |
| 21 | 1 | 0.3569 | 0.3569 | 0.1485 |
| 22 | 0 | 0.4225 | 0.4224 | 0.1758 |
| 22 | 1 | 1.2507 | 1.2504 | 0.5203 |
| 23 | 0 | 1.3978 | 1.3975 | 0.5815 |
| 23 | 1 | 0.2334 | 0.2333 | 0.0971 |
| 24 | 0 | 1.1296 | 1.1293 | 0.4699 |
| 24 | 1 | 0.6059 | 0.6058 | 0.2521 |

## Sieve rows (V2 sparse, measured)

| seed | S | S+ | band-A native/sqrt(l) | lookups/sqrt(l) |
|--:|--:|--:|--:|--:|
| 17 | 0.00600 | 0.02186 | 6.2666e0 | 1.4891e-1 |
| 18 | 0.00457 | 0.02478 | 6.9729e0 | 1.8428e-1 |
| 19 | 0.00499 | 0.02185 | 6.1998e0 | 1.5866e-1 |
| 20 | 0.01075 | 0.02667 | 7.8511e0 | 1.5125e-1 |
| 21 | 0.00503 | 0.02664 | 7.5082e0 | 1.9440e-1 |
| 22 | 0.00612 | 0.02664 | 7.5757e0 | 1.8262e-1 |
| 23 | 0.00505 | 0.02178 | 6.1875e0 | 1.5468e-1 |
| 24 | 0.00679 | 0.02789 | 7.9533e0 | 1.8942e-1 |
| 25 | 0.00560 | 0.02616 | 7.4110e0 | 1.8423e-1 |
| 26 | 0.00601 | 0.02835 | 8.0333e0 | 1.9608e-1 |
| 27 | 0.00613 | 0.02009 | 5.7895e0 | 1.3453e-1 |
| 28 | 0.00503 | 0.02417 | 6.8370e0 | 1.7392e-1 |
| 29 | 0.00796 | 0.02944 | 8.4420e0 | 1.9116e-1 |
| 30 | 0.00561 | 0.01966 | 5.6416e0 | 1.3676e-1 |
| 31 | 0.00569 | 0.02291 | 6.5319e0 | 1.5856e-1 |
| 32 | 0.00703 | 0.02759 | 7.8839e0 | 1.8445e-1 |
