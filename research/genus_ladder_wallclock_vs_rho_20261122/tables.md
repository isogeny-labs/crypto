## T1 wall ratio IC(2LP+pre+SGE(8,3)) / negation rho, paired, 6 curves x 2 seeds per prime, 95% hierarchical bootstrap `[pcc:unverified]`

| p | mean wall IC (s) | mean wall rho (s) | ratio, new binary, 1 thread | 95% interval | R8 count re-pricing | ratio without SGE | old binary (R2, threads 4) ratio | old interval |
|--:|--:|--:|--:|:--|--:|--:|--:|:--|
| 65537 | 2.89 | 14.28 | 0.202 | [0.158, 0.271] | 0.176 | 0.272 | 0.499 | [0.393, 0.694] |
| 131101 | 8.48 | 32.51 | 0.261 | [0.220, 0.324] | 0.228 | 0.335 | 0.682 | [0.532, 0.869] |
| 262147 | 22.19 | 118.76 | 0.187 | [0.121, 0.325] | 0.198 | 0.226 | 0.500 | [0.368, 0.806] |

## T2 mean wall per arm (s): new binary (this run, 1 thread, host otherwise idle) vs old binary (R2, 4 runs in parallel)

| p | IC+SGE new | IC+SGE old | IC no SGE new | IC no SGE old | rho new | rho old |
|--:|--:|--:|--:|--:|--:|--:|
| 65537 | 2.89 | 12.91 | 3.88 | 14.32 | 14.28 | 25.88 |
| 131101 | 8.48 | 32.13 | 10.89 | 35.65 | 32.51 | 47.13 |
| 262147 | 22.19 | 91.54 | 26.79 | 92.58 | 118.76 | 183.20 |

## T3 ns per counted operation (new binary)

Counted op = Cantor field multiplication + 10 x inversion (rho: all its counted work; IC relation phase: that plus `rel_mul`). IC relation-phase wall is not recorded separately: it is estimated as the arm total minus `la_mul` x R8 ns per `la_mul` (16.6/20.3/19.0 ns), so the IC relation-phase figure is an estimate.

| p | rho ns / counted op | rho ns / walk step | IC relation-phase ns / counted op (est.) | IC total ns / FPM_mid op (kappa 9) | est. LA share of IC wall | IC wall / `la_mul` (total, ns) |
|--:|--:|--:|--:|--:|--:|--:|
| 65537 | 6.35 | 1768 | 6.57 | 3.71 | 0.30 | 55.3 |
| 131101 | 7.34 | 2043 | 7.15 | 4.02 | 0.36 | 56.7 |
| 262147 | 7.10 | 1977 | 7.63 | 3.91 | 0.36 | 52.1 |

## T4 per-curve paired ratio (mean over 2 seeds)

| p | curve seed | IC s | rho s | ratio |
|--:|--:|--:|--:|--:|
| 65537 | 64 | 2.91 | 11.56 | 0.251 |
| 65537 | 86 | 2.73 | 9.02 | 0.303 |
| 65537 | 157 | 2.78 | 14.38 | 0.193 |
| 65537 | 158 | 3.16 | 17.10 | 0.185 |
| 65537 | 217 | 2.97 | 16.70 | 0.178 |
| 65537 | 250 | 2.80 | 16.93 | 0.165 |
| 131101 | 2 | 8.03 | 25.39 | 0.316 |
| 131101 | 14 | 8.61 | 43.67 | 0.197 |
| 131101 | 21 | 7.85 | 23.31 | 0.337 |
| 131101 | 41 | 8.41 | 35.44 | 0.237 |
| 131101 | 58 | 9.36 | 33.85 | 0.277 |
| 131101 | 61 | 8.62 | 33.43 | 0.258 |
| 262147 | 13 | 21.73 | 62.42 | 0.348 |
| 262147 | 19 | 22.10 | 117.04 | 0.189 |
| 262147 | 23 | 21.26 | 234.12 | 0.091 |
| 262147 | 69 | 22.10 | 160.85 | 0.137 |
| 262147 | 113 | 22.36 | 56.82 | 0.394 |
| 262147 | 206 | 23.61 | 81.29 | 0.290 |

## T5 exponent of mean wall in p (3 primes, descriptive, no interval)

| arm | new binary | old binary (R2) |
|--|--:|--:|
| IC + SGE(8,3) | 1.47 | 1.41 |
| negation rho | 1.53 | 1.41 |

## T6 V1 identity check against R2 (count fields of all 108 runs)

| p | runs | all certified | non-wall field differences | instance lines equal |
|--:|--:|:-:|--:|:-:|
| 65537 | 36 | True | 0 | True |
| 131101 | 36 | True | 0 | True |
| 262147 | 36 | True | 0 | True |
