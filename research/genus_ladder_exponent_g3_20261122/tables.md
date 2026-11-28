## T1 per-prime wall (s) and IC+SGE / rho ratio `[pcc:unverified]`

Old primes: R9 rows (6 curves x 2 seeds). New primes: this run (3 curves x 2 seeds). 95 % hierarchical bootstrap, 4000 draws.

| p | curves x seeds | runs certified | mean wall IC+SGE (s) | mean wall no-SGE (s) | mean wall rho (s) | IC+SGE / rho | 95 % interval | no-SGE / rho |
|--:|--:|:-:|--:|--:|--:|--:|:--|--:|
| 65537 | 6 x 2 | 36 (all) | 2.89 | 3.88 | 14.28 | 0.202 | [0.159, 0.271] | 0.272 |
| 131101 | 6 x 2 | 36 (all) | 8.48 | 10.89 | 32.51 | 0.261 | [0.220, 0.326] | 0.335 |
| 262147 | 6 x 2 | 36 (all) | 22.19 | 26.79 | 118.76 | 0.187 | [0.122, 0.324] | 0.226 |
| 524309 | 3 x 2 | 18 (all) | 56.77 | 67.58 | 480.20 | 0.118 | [0.055, 0.487] | 0.141 |
| 1048583 | 3 x 2 | 18 (all) | 150.07 | 169.67 | 817.16 | 0.184 | [0.107, 0.300] | 0.208 |

## T2 per-prime mean counts

| p | relation phase (cantor_mul+10 inv+rel_mul) | la_mul | rho (cantor_mul+10 inv) | mean rho walk steps |
|--:|--:|--:|--:|--:|
| 65537 | 3.075e+08 | 5.228e+07 | 2.249e+09 | 8.077e+06 |
| 131101 | 7.617e+08 | 1.496e+08 | 4.431e+09 | 1.592e+07 |
| 262147 | 1.848e+09 | 4.256e+08 | 1.672e+10 | 6.006e+07 |
| 524309 | 4.656e+09 | 1.193e+09 | 7.235e+10 | 2.599e+08 |
| 1048583 | 1.157e+10 | 3.306e+09 | 1.218e+11 | 4.378e+08 |

## T3 exponents over the five primes (log-log least squares on per-prime means; 95 % bootstrap over curves, then seeds, within prime)

| quantity | exponent | 95 % interval | reference |
|--|--:|:--|--|
| wall, IC+SGE(8,3) | 1.414 | [1.394, 1.433] |  |
| wall, negation rho | 1.556 | [1.277, 1.764] |  |
| wall, IC no SGE | 1.353 | [1.328, 1.379] |  |
| count, IC relation phase | 1.308 | [1.307, 1.309] | 4/3 |
| count, IC linear algebra (la_mul) | 1.496 | [1.492, 1.501] | 3/2 |
| count, rho | 1.555 | [1.271, 1.764] | 3/2 |
| wall exponent difference IC - rho (paired draws; secondary) | -0.142 | [-0.355, 0.141] | 0 |

Three-prime (R9 only) point exponents for reference: IC 1.471, rho 1.528.

Separation test (IC upper end < rho lower end): IC [1.394, 1.433], rho [1.277, 1.764] -> **DOES NOT HOLD**.

## T4 per-curve mean wall (s) at the new primes

| p | curve seed | IC+SGE | rho | ratio |
|--:|--:|--:|--:|--:|
| 524309 | 13 | 56.3 | 283.1 | 0.199 |
| 524309 | 19 | 56.9 | 1040.4 | 0.055 |
| 524309 | 86 | 57.0 | 117.1 | 0.487 |
| 1048583 | 2 | 152.0 | 568.1 | 0.268 |
| 1048583 | 27 | 153.5 | 532.0 | 0.289 |
| 1048583 | 117 | 144.7 | 1351.4 | 0.107 |
