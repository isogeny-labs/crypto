# Tables (every figure `[pcc:unverified]`; means over the runs of each cell: 6 at genus 4, 9 at genus 5)

FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul (g^2 = 16 / 25); relation phase = cantor_mul + 10 cantor_inv + rel_mul. Paired ratio = mean of per-(curve, seed) ratios to the plain arm, hierarchical bootstrap 95% (3 curves: coarse). Ratio to rho = mean FPM of the arm over the mean of the matching rho runs on the same curves (g5: curves 39 and 42 only).


## genus 4, p = 2053 (rho FPM_mid mean 1.81e+09, 6 runs on curves [40, 60, 109]; plain LA share 0.519)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 1.02e+03 | 598 | - | - | 4.21e+06 | 2.20e+07 | 8.93e+07 | - | - | 0.0493 [0.0406, 0.0627] |
| plain SGE(8,3) | 1.02e+03 | 598 | 7.98 | 4.77e+03 -> 5.99e+03 | 1.48e+06 | 2.20e+07 | 4.57e+07 | 1 | 1 | 0.0252 [0.0209, 0.0322] |
| ex(0.1,0.01) | 1.12e+03 | 586 | 7.96 -> 7.35 | 4.31e+03 -> 5.42e+03 | 1.30e+06 | 2.42e+07 | 4.50e+07 | 0.984 [0.958, 1.002] | 0.878 [0.830, 0.915] | 0.0248 [0.0204, 0.0317] |
| ex(0.2,0.01) | 1.22e+03 | 585 | 7.94 -> 7.05 | 4.12e+03 -> 5.21e+03 | 1.22e+06 | 2.63e+07 | 4.58e+07 | 1.001 [0.983, 1.012] | 0.821 [0.795, 0.847] | 0.0253 [0.0208, 0.0323] |
| ex(0.3,0.01) | 1.32e+03 | 588 | 7.91 -> 6.82 | 4.01e+03 -> 5.16e+03 | 1.20e+06 | 2.84e+07 | 4.77e+07 | 1.043 [1.028, 1.060] | 0.812 [0.784, 0.853] | 0.0263 [0.0217, 0.0336] |

## genus 4, p = 4099 (rho FPM_mid mean 6.42e+09, 6 runs on curves [72, 247, 418]; plain LA share 0.596)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 1.98e+03 | 1e+03 | - | - | 1.18e+07 | 5.11e+07 | 2.40e+08 | - | - | 0.0375 [0.0220, 0.0779] |
| plain SGE(8,3) | 1.98e+03 | 1e+03 | 9.01 | 9.04e+03 -> 1.17e+04 | 4.73e+06 | 5.11e+07 | 1.27e+08 | 1 | 1 | 0.0198 [0.0116, 0.0411] |
| ex(0.1,0.01) | 2.18e+03 | 979 | 8.95 -> 8.18 | 8.01e+03 -> 1.05e+04 | 4.11e+06 | 5.64e+07 | 1.22e+08 | 0.963 [0.955, 0.975] | 0.869 [0.854, 0.891] | 0.0190 [0.0111, 0.0396] |
| ex(0.2,0.01) | 2.38e+03 | 984 | 8.92 -> 7.78 | 7.65e+03 -> 1e+04 | 3.79e+06 | 6.14e+07 | 1.22e+08 | 0.963 [0.950, 0.971] | 0.803 [0.785, 0.815] | 0.0190 [0.0112, 0.0396] |
| ex(0.3,0.01) | 2.58e+03 | 982 | 8.87 -> 7.47 | 7.33e+03 -> 9.67e+03 | 3.58e+06 | 6.64e+07 | 1.24e+08 | 0.976 [0.960, 0.989] | 0.757 [0.736, 0.779] | 0.0193 [0.0113, 0.0400] |

## genus 5, p = 1031 (rho FPM_mid mean 2.26e+10, 6 runs on curves [39, 42]; plain LA share 0.225)

| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|--:|--:|--|--|--:|--:|--:|--|--|--|
| unmerged | 530 | 514 | - | - | 3.11e+06 | 7.26e+07 | 1.50e+08 | - | - | 0.0066 [0.0039, 0.0120] |
| plain SGE(8,3) | 530 | 514 | 6.02 | 3.09e+03 -> 4.2e+03 | 8.45e+05 | 7.26e+07 | 9.37e+07 | 1 | 1 | 0.0041 [0.0024, 0.0075] |
| ex(0.1,0.01) | 584 | 509 | 6.02 -> 6.00 | 3.05e+03 -> 4.18e+03 | 8.36e+05 | 7.96e+07 | 1.00e+08 | 1.072 [1.063, 1.082] | 0.990 [0.975, 1.005] | 0.0044 [0.0026, 0.0080] |
| ex(0.2,0.01) | 637 | 509 | 6.03 -> 6.00 | 3.05e+03 -> 4.18e+03 | 8.35e+05 | 8.63e+07 | 1.07e+08 | 1.143 [1.131, 1.157] | 0.989 [0.977, 1.004] | 0.0047 [0.0028, 0.0086] |
| ex(0.3,0.01) | 690 | 509 | 6.03 -> 6.00 | 3.05e+03 -> 4.18e+03 | 8.35e+05 | 9.33e+07 | 1.14e+08 | 1.219 [1.205, 1.231] | 0.989 [0.976, 1.003] | 0.0050 [0.0030, 0.0091] |

(`rows kept` of the plain/unmerged arms is the peeled core's row count; core columns: genus 4, p = 2053 577, genus 4, p = 4099 972, genus 5, p = 1031 497. The excess arms keep cols + margin rows, which is about the plain core at genus 5.)


## Relation-phase ratio to plain (paired), weight ratio, LA share

| cell | E | relation phase / plain | mean weight after / plain | LA share of the arm |
|---|--:|--|--:|--:|
| genus 4, p = 2053 | 0.1 | 1.098 [1.093, 1.105] | 0.921 | 0.463 |
| genus 4, p = 2053 | 0.2 | 1.196 [1.189, 1.201] | 0.883 | 0.425 |
| genus 4, p = 2053 | 0.3 | 1.291 [1.285, 1.300] | 0.854 | 0.404 |
| genus 4, p = 4099 | 0.1 | 1.102 [1.097, 1.109] | 0.909 | 0.538 |
| genus 4, p = 4099 | 0.2 | 1.201 [1.194, 1.208] | 0.864 | 0.497 |
| genus 4, p = 4099 | 0.3 | 1.299 [1.287, 1.308] | 0.829 | 0.463 |
| genus 5, p = 1031 | 0.1 | 1.096 [1.087, 1.108] | 0.996 | 0.208 |
| genus 5, p = 1031 | 0.2 | 1.188 [1.174, 1.206] | 0.996 | 0.195 |
| genus 5, p = 1031 | 0.3 | 1.286 [1.270, 1.300] | 0.996 | 0.183 |

## Extrapolation to E > 0.3 (a model fitted to the three measured E, not a run): FPM_mid over plain

| cell | slope of ln(la_mul) per unit E | E = 0.1 | 0.2 | 0.3 | 0.4 | 0.5 | 0.7 | 1.0 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| genus 4, p = 2053 | -0.391 | 0.978 | 1.009 | 1.040 | 1.072 | 1.105 | 1.172 | 1.276 |
| genus 4, p = 4099 | -0.690 | 0.961 | 0.966 | 0.975 | 0.985 | 0.997 | 1.028 | 1.085 |
| genus 5, p = 1031 | -0.007 | 1.068 | 1.145 | 1.221 | 1.298 | 1.375 | 1.528 | 1.758 |

## Scorecard inputs

Best E per cell (lowest paired FPM ratio): {'g4_2053': '0.1', 'g4_4099': '0.2', 'g5_1031': '0.1'}. Best arm over rho at g4 p = 4099: C_double_lp+pre+sge(8,3)+ex(0.2,0.01), 0.0190 [0.0112, 0.0396] (plain 0.0198 [0.0116, 0.0411]).
