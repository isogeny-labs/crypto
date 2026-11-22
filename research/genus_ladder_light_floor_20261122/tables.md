# Tables (every figure `[pcc:unverified]`; 6 fresh curves x 2 seeds = 12 runs per arm per prime; means over the 12 runs)

FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul; relation phase = cantor_mul + 10 cantor_inv + rel_mul. Paired ratio = mean of per-(curve, seed) ratios to the plain arm, hierarchical bootstrap 95%. Ratio to rho = mean FPM of the arm over the mean of R2's 12 rho runs on the same curves.


## p = 65537 (rho FPM_mid mean 2.96e+09, 12 runs)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 2.6e+04 | 3.2e+03 | - | - | 1.20e+08 | 3.07e+08 | 1.39e+09 | - | - | 0.469 [0.338, 0.719] |
| plain SGE(8,3) | 2.6e+04 | 3.2e+03 | 10.52 | 3.36e+04 -> 4.43e+04 | 5.24e+07 | 3.07e+08 | 7.79e+08 | 1 | 1 | 0.263 [0.190, 0.404] |
| ex(0.1,0.01) | 2.85e+04 | 3.09e+03 | 10.35 -> 8.85 | 2.73e+04 -> 3.64e+04 | 3.89e+07 | 3.38e+08 | 6.88e+08 | 0.884 [0.878, 0.890] | 0.742 [0.732, 0.753] | 0.233 [0.168, 0.357] |
| ex(0.2,0.01) | 3.11e+04 | 3.04e+03 | 10.13 -> 8.10 | 2.46e+04 -> 3.27e+04 | 3.35e+07 | 3.69e+08 | 6.70e+08 | 0.860 [0.855, 0.865] | 0.639 [0.631, 0.646] | 0.226 [0.163, 0.347] |
| ex(0.3,0.01) | 3.37e+04 | 3.02e+03 | 9.90 -> 7.59 | 2.29e+04 -> 3.01e+04 | 2.98e+07 | 3.99e+08 | 6.67e+08 | 0.857 [0.853, 0.861] | 0.568 [0.563, 0.575] | 0.226 [0.163, 0.345] |

## p = 131101 (rho FPM_mid mean 5.86e+09, 12 runs)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 5.06e+04 | 5.11e+03 | - | - | 3.08e+08 | 7.62e+08 | 3.53e+09 | - | - | 0.603 [0.396, 0.826] |
| plain SGE(8,3) | 5.06e+04 | 5.11e+03 | 11.63 | 5.95e+04 -> 7.89e+04 | 1.50e+08 | 7.62e+08 | 2.11e+09 | 1 | 1 | 0.360 [0.237, 0.494] |
| ex(0.1,0.01) | 5.56e+04 | 4.97e+03 | 11.35 -> 9.62 | 4.78e+04 -> 6.44e+04 | 1.13e+08 | 8.39e+08 | 1.86e+09 | 0.879 [0.874, 0.885] | 0.754 [0.746, 0.762] | 0.317 [0.208, 0.434] |
| ex(0.2,0.01) | 6.07e+04 | 4.91e+03 | 11.02 -> 8.71 | 4.28e+04 -> 5.77e+04 | 9.73e+07 | 9.15e+08 | 1.79e+09 | 0.848 [0.843, 0.853] | 0.649 [0.642, 0.656] | 0.305 [0.201, 0.418] |
| ex(0.3,0.01) | 6.57e+04 | 4.88e+03 | 10.74 -> 8.13 | 3.97e+04 -> 5.33e+04 | 8.73e+07 | 9.91e+08 | 1.78e+09 | 0.841 [0.836, 0.847] | 0.582 [0.574, 0.590] | 0.303 [0.199, 0.415] |

## p = 262147 (rho FPM_mid mean 1.29e+10, 12 runs)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 9.83e+04 | 8.16e+03 | - | - | 7.87e+08 | 1.85e+09 | 8.93e+09 | - | - | 0.694 [0.495, 1.147] |
| plain SGE(8,3) | 9.83e+04 | 8.16e+03 | 12.74 | 1.04e+05 -> 1.38e+05 | 4.23e+08 | 1.85e+09 | 5.66e+09 | 1 | 1 | 0.439 [0.314, 0.726] |
| ex(0.1,0.01) | 1.08e+05 | 7.97e+03 | 12.35 -> 10.34 | 8.24e+04 -> 1.12e+05 | 3.21e+08 | 2.03e+09 | 4.92e+09 | 0.870 [0.864, 0.875] | 0.758 [0.750, 0.766] | 0.382 [0.273, 0.633] |
| ex(0.2,0.01) | 1.18e+05 | 7.88e+03 | 11.94 -> 9.28 | 7.32e+04 -> 1.00e+05 | 2.74e+08 | 2.22e+09 | 4.69e+09 | 0.829 [0.825, 0.833] | 0.649 [0.644, 0.654] | 0.364 [0.260, 0.602] |
| ex(0.3,0.01) | 1.28e+05 | 7.8e+03 | 11.56 -> 8.58 | 6.69e+04 -> 9.12e+04 | 2.41e+08 | 2.40e+09 | 4.58e+09 | 0.809 [0.805, 0.814] | 0.571 [0.566, 0.577] | 0.356 [0.254, 0.588] |

(For the excess arms `nnz before` is the non-zeros of the core after the lightest-row selection, handed to SGE; `weight before` is the mean row weight of the peeled core before selection. LA share of FPM_mid, plain / ex(0.2) / ex(0.3) at 262147: 0.673 / 0.527 / 0.475.)


## Relation-phase ratio to plain (paired) and weight ratio

| p | E | relation phase / plain | mean weight after / plain |
|--:|--:|--|--:|
| 65537 | 0.1 | 1.101 [1.100, 1.101] | 0.841 |
| 65537 | 0.2 | 1.200 [1.198, 1.202] | 0.770 |
| 65537 | 0.3 | 1.300 [1.297, 1.303] | 0.721 |
| 131101 | 0.1 | 1.101 [1.100, 1.102] | 0.827 |
| 131101 | 0.2 | 1.201 [1.199, 1.202] | 0.749 |
| 131101 | 0.3 | 1.301 [1.299, 1.303] | 0.699 |
| 262147 | 0.1 | 1.100 [1.099, 1.101] | 0.812 |
| 262147 | 0.2 | 1.200 [1.199, 1.201] | 0.729 |
| 262147 | 0.3 | 1.300 [1.298, 1.301] | 0.673 |

## Exponent of mean la_mul in p (three primes, descriptive)

| arm | exponent |
|---|--:|
| plain SGE(8,3) | 1.506 |
| ex(0.1,0.01) | 1.522 |
| ex(0.2,0.01) | 1.518 |
| ex(0.3,0.01) (best E) | 1.510 |

FPM_mid exponent: plain 1.430, ex(0.3) 1.389.

