# ANALYSIS (written by `ic_algebraic_floor analyze`)

Cost per relation in wall-clock ns (log2), one core, same binary and host. `measured` = every target finished; `lower_bound` = some target censored at its budget or the cell stopped early; intervals are 95% bootstrap percentiles.

## m = 4

### Cells and the floor

| n | d | \|F\| points | p (decomposable) | decomps/target | exhaustive = MITM | floor amortized log2 ns [95%] | floor online log2 ns |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 3 | 3 | 0.009 | 0.017 | 12/12 | 18.75 [18.56, 19.24] | 16.99 |
| 13 | 4 | 19 | 0.562 | 1.008 | 12/12 | 15.63 [15.59, 15.67] | 14.41 |
| 17 | 5 | 29 | 0.181 | 0.223 | 12/12 | 17.26 [17.09, 17.44] | 17.05 |
| 19 | 5 | 35 | 0.093 | 0.103 | 12/12 | 19.05 [18.81, 19.35] | 18.93 |
| 23 | 6 | 53 | 0.026 | 0.026 | 12/12 | 21.42 [20.92, 22.09] | 21.41 |
| 29 | 8 | 253 | 0.281 | 0.333 | 0/0 | 22.00 [21.86, 22.18] | 22.00 |
| 31 | 8 | 261 | 0.082 | 0.085 | 0/0 | 24.45 [24.17, 24.78] | 24.45 |

Floor slope (log2 ns per unit n, amortized): **0.351** [0.334, 0.364]; online 0.427; counted (group additions, amortized) 0.499.

### Arms

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| f4-chain | 11 | 3 | measured | 6/6 | 0 | 29.09 [28.19, 30.46] | 1.299e3 | 1.831e7 word_xors |
| f4-chain | 13 | 4 | measured | 6/6 | 0 | 26.11 [25.88, 26.31] | 1.427e3 | 3.906e6 word_xors |
| f4-chain | 17 | 5 | measured | 6/6 | 0 | 30.62 [30.36, 30.86] | 1.050e4 | 1.524e8 word_xors |
| f4-chain | 19 | 5 | measured | 6/6 | 0 | 32.52 [32.23, 32.88] | 1.133e4 | 6.680e8 word_xors |
| f4-chain | 23 | 6 | measured | 6/6 | 0 | 36.56 [36.08, 37.22] | 3.614e4 | 1.613e10 word_xors |
| f4-chain | 29 | 8 | measured | 6/6 | 0 | 38.13 [37.94, 38.33] | 7.152e4 | 6.572e10 word_xors |
| f4-chain | 31 | 8 | lower_bound | 6/6 | 6 | ≥ 40.48 | ≥ 6.699e4 | — |

**f4-chain**: measured cells 6, lower-bound cells 1; slope on measured cells 0.644 (floor on the same cells 0.298), Δ = arm − floor 95% [0.313, 0.372]; lower-bound fit slope 0.655 (valid as a lower bound), Δ 95% [0.279, 0.324]. **Verdict: grows faster than the floor.**

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| f6-ic | 11 | 3 | measured | 6/6 | 0 | 28.87 [28.05, 30.22] | 1.115e3 | 1.744e7 word_xors |
| f6-ic | 13 | 4 | measured | 6/6 | 0 | 26.33 [26.12, 26.52] | 1.665e3 | 3.906e6 word_xors |
| f6-ic | 17 | 5 | measured | 6/6 | 0 | 30.82 [30.59, 31.05] | 1.207e4 | 1.510e8 word_xors |
| f6-ic | 19 | 5 | measured | 6/6 | 0 | 32.57 [32.25, 32.93] | 1.173e4 | 6.645e8 word_xors |
| f6-ic | 23 | 6 | measured | 6/6 | 0 | 36.59 [36.10, 37.23] | 3.679e4 | 1.587e10 word_xors |
| f6-ic | 29 | 8 | measured | 6/6 | 0 | 38.14 [37.96, 38.35] | 7.198e4 | 6.505e10 word_xors |
| f6-ic | 31 | 8 | lower_bound | 6/6 | 6 | ≥ 40.48 | ≥ 6.672e4 | — |

**f6-ic**: measured cells 6, lower-bound cells 1; slope on measured cells 0.645 (floor on the same cells 0.298), Δ = arm − floor 95% [0.316, 0.371]; lower-bound fit slope 0.654 (valid as a lower bound), Δ 95% [0.279, 0.321]. **Verdict: grows faster than the floor.**

| arm | n | d | status | targets run (yes/no) | censored | log2 ns per relation [95%] | ÷ floor | counted per relation |
|---|--:|--:|---|--:|--:|--:|--:|--:|
| sat | 11 | 3 | measured | 6/6 | 0 | 32.03 [31.16, 33.37] | 9.947e3 | 2.069e5 conflicts |
| sat | 13 | 4 | lower_bound | 6/6 | 1 | ≥ 35.65 | ≥ 1.067e6 | — |
| sat | 17 | 5 | lower_bound | 6/6 | 9 | ≥ 39.19 | ≥ 3.999e6 | — |
| sat | 19 | 5 | lower_bound | 2/2 | 4 | ≥ 40.24 | ≥ 2.384e6 | — |
| sat | 23 | 6 | not_run | 0/0 | 0 | — | — | — |
| sat | 29 | 8 | not_run | 0/0 | 0 | — | — | — |
| sat | 31 | 8 | not_run | 0/0 | 0 | — | — | — |

**sat**: measured cells 1, lower-bound cells 3; slope on measured cells — (floor on the same cells —), Δ = arm − floor 95% —; lower-bound fit slope 0.997 (not a valid bound), Δ 95% [0.800, 0.962]. **Verdict: undetermined (fewer than 3 usable cells).**

