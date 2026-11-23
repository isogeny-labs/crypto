# ANALYSIS (written by `ic_algebraic_floor analyze`)

Cost per relation in wall-clock ns (log2), one core, same binary and host. `measured` = every target finished; `lower_bound` = some target censored at its budget or the cell stopped early; intervals are 95% bootstrap percentiles.

## m = 3

### Cells and the floor

| n | d | \|F\| points | p (decomposable) | decomps/target | exhaustive = MITM | floor amortized log2 ns [95%] | floor online log2 ns |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | 5 | 33 | 0.494 | 0.770 | 12/12 | 15.26 [15.25, 15.27] | 11.97 |
| 17 | 6 | 63 | 0.274 | 0.331 | 12/12 | 14.83 [14.80, 14.86] | 12.83 |
| 19 | 7 | 117 | 0.404 | 0.537 | 12/12 | 14.40 [14.37, 14.44] | 12.84 |
| 23 | 8 | 237 | 0.247 | 0.284 | 12/12 | 15.16 [15.10, 15.23] | 14.33 |
| 29 | 10 | 1017 | 0.279 | 0.323 | 0/0 | 17.29 [17.22, 17.37] | 16.79 |
| 31 | 11 | 2101 | 0.479 | 0.678 | 0/0 | 18.43 [18.40, 18.46] | 17.22 |
| 37 | 13 | 8191 | 0.488 | 0.668 | 0/0 | 20.36 [20.32, 20.40] | 19.51 |
| 41 | 14 | 16223 | 0.277 | 0.321 | 0/0 | 22.25 [22.17, 22.32] | 21.76 |

Floor slope (log2 ns per unit n, amortized): **0.275** [0.273, 0.277]; online 0.352; counted (group additions, amortized) 0.334.

### Arms

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| f4-chain | 13 | 5 | measured | 6/6 | 0 | 23.72 [23.50, 23.92] | 3.519e2 | 6.224e5 word_xors |
| f4-chain | 17 | 6 | measured | 6/6 | 0 | 26.46 [26.24, 26.65] | 3.159e3 | 7.580e6 word_xors |
| f4-chain | 19 | 7 | measured | 6/6 | 0 | 27.35 [27.06, 27.61] | 7.910e3 | 1.545e7 word_xors |
| f4-chain | 23 | 8 | measured | 6/6 | 0 | 29.64 [29.39, 29.89] | 2.286e4 | 1.324e8 word_xors |
| f4-chain | 29 | 10 | measured | 6/6 | 0 | 32.70 [32.51, 32.88] | 4.361e4 | 1.520e9 word_xors |
| f4-chain | 31 | 11 | measured | 6/6 | 0 | 33.37 [33.20, 33.52] | 3.139e4 | 2.524e9 word_xors |

**f4-chain**: measured cells 6, lower-bound cells 0; slope on measured cells 0.533 (floor on the same cells 0.193), Δ = arm − floor 95% [0.329, 0.352]; lower-bound fit slope 0.533 (not a valid bound), Δ 95% [0.329, 0.352]. **Verdict: grows faster than the floor.**

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| lin21 | 13 | 5 | measured | 6/6 | 0 | 15.69 [15.49, 15.90] | 1.353e0 | 4.185e1 trials |
| lin21 | 17 | 6 | measured | 6/6 | 0 | 17.75 [17.09, 18.50] | 7.590e0 | 1.845e2 trials |
| lin21 | 19 | 7 | measured | 6/6 | 0 | 18.59 [17.70, 19.48] | 1.821e1 | 1.851e2 trials |
| lin21 | 23 | 8 | measured | 6/6 | 0 | 21.05 [19.59, 22.21] | 5.909e1 | 7.776e2 trials |
| lin21 | 29 | 10 | measured | 6/6 | 0 | 22.02 [21.84, 22.19] | 2.647e1 | 2.921e3 trials |
| lin21 | 31 | 11 | measured | 6/6 | 0 | 22.50 [22.33, 22.65] | 1.684e1 | 2.948e3 trials |
| lin21 | 37 | 13 | measured | 6/6 | 0 | 24.55 [24.39, 24.70] | 1.829e1 | 9.960e3 trials |
| lin21 | 41 | 14 | measured | 6/6 | 0 | 26.93 [26.77, 27.08] | 2.557e1 | 4.589e4 trials |

**lin21**: measured cells 8, lower-bound cells 0; slope on measured cells 0.368 (floor on the same cells 0.275), Δ = arm − floor 95% [0.079, 0.114]; lower-bound fit slope 0.368 (not a valid bound), Δ 95% [0.079, 0.114]. **Verdict: grows faster than the floor.**

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| lin21-ht | 13 | 5 | measured | 6/6 | 0 | 15.17 [14.96, 15.41] | 9.399e-1 | 4.185e1 trials |
| lin21-ht | 17 | 6 | measured | 6/6 | 0 | 16.29 [16.11, 16.46] | 2.751e0 | 1.845e2 trials |
| lin21-ht | 19 | 7 | measured | 6/6 | 0 | 16.27 [16.11, 16.42] | 3.645e0 | 1.851e2 trials |
| lin21-ht | 23 | 8 | measured | 6/6 | 0 | 18.06 [17.83, 18.30] | 7.479e0 | 7.776e2 trials |
| lin21-ht | 29 | 10 | measured | 6/6 | 0 | 20.27 [19.60, 20.97] | 7.889e0 | 2.921e3 trials |
| lin21-ht | 31 | 11 | measured | 6/6 | 0 | 20.31 [20.09, 20.49] | 3.674e0 | 2.948e3 trials |
| lin21-ht | 37 | 13 | measured | 6/6 | 0 | 22.25 [22.03, 22.48] | 3.722e0 | 9.960e3 trials |
| lin21-ht | 41 | 14 | measured | 6/6 | 0 | 24.36 [24.11, 24.63] | 4.335e0 | 4.589e4 trials |

**lin21-ht**: measured cells 8, lower-bound cells 0; slope on measured cells 0.324 (floor on the same cells 0.275), Δ = arm − floor 95% [0.040, 0.057]; lower-bound fit slope 0.324 (not a valid bound), Δ 95% [0.040, 0.057]. **Verdict: grows faster than the floor.**

